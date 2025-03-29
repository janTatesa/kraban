mod keyhints;
mod list;
mod project;
mod prompt;
mod task;
mod widgets;

use super::{
    config::Config,
    state::{CurrentList, Priority},
    Context,
};
use cli_log::debug;
use crossterm::event::{KeyCode, KeyEvent};
use keyhints::{KeyHints, KeyHintsWidget};
use project::ProjectsView;
use prompt::Prompt;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Span, Text},
    widgets::{Clear, Widget},
};
use std::fmt::Debug;
use tap::Tap;
use widgets::{block_widget, main_block};

#[derive(Debug)]
pub struct Ui {
    view: Box<dyn View>,
    prompt: Option<Box<dyn Prompt>>,
}

fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

fn open_prompt<T: Prompt + 'static>(prompt: T) -> Option<Action> {
    Some(Action::OpenPrompt(Box::new(prompt)))
}

impl Ui {
    pub fn new(context: Context) -> Self {
        Self {
            view: Box::new(ProjectsView::new(
                context.state.projects().len().checked_sub(1),
            )),
            prompt: None,
        }
    }

    // Context cannot be used because state would be referenced both mutably and immutably
    pub fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a> {
        self.view.current_list(config)
    }

    fn key_hints_widget(&self, context: Context, width: u16) -> Text<'static> {
        let key_hints = self
            .key_hints(context)
            .tap_mut(|hints| hints.push(("Ctrl-q", "Quit")));
        KeyHintsWidget {
            hints: key_hints,
            keybinding_style: Style::new().bold().fg(context.config.app_color),
            hint_style: Style::new().reset().italic(),
        }
        .into_text(width)
    }
    fn render_prompt(&self, area: Rect, buf: &mut Buffer, prompt: &dyn Prompt, context: Context) {
        let prompt_area = center_area(
            area,
            Constraint::Percentage(60),
            Constraint::Length(prompt.height() + 2),
        );

        Clear.render(prompt_area, buf);
        let block = block_widget(context.config).title(prompt.title(self.view.item()));
        let inner_prompt_area = block.inner(prompt_area);
        block.render(prompt_area, buf);
        prompt.render(inner_prompt_area, buf, context);
    }

    pub fn handle_action(&mut self, action: Action, context: Context) -> Option<Action> {
        debug!("Performing {:?}", action);
        match action {
            Action::SwitchToView(view) => {
                self.view = view;
                None
            }
            Action::OpenPrompt(prompt) => {
                self.prompt = Some(prompt);
                None
            }
            _ => {
                self.prompt = None;
                self.view.handle_action(&action, context);
                Some(action)
            }
        }
    }
}

impl Component for Ui {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        if let KeyCode::Esc = key_event.code {
            self.prompt = None;
        }

        match &mut self.prompt {
            Some(prompt) => prompt.on_key(key_event, context),
            None => self.view.on_key(key_event, context),
        }
        .and_then(|action| self.handle_action(action, context))
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        match &self.prompt {
            Some(prompt) => prompt
                .key_hints(context)
                .tap_mut(|v| v.push(("Esc", "Exit Prompt"))),
            None => self.view.key_hints(context),
        }
    }

    fn render(&self, terminal_area: Rect, buf: &mut Buffer, context: Context) {
        let key_hints = context
            .config
            .show_key_hints
            .then(|| self.key_hints_widget(context, terminal_area.width));
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(
                key_hints
                    .as_ref()
                    .map(|k| k.lines.len() as u16)
                    .unwrap_or_default(),
            ),
        ])
        .split(terminal_area);

        let block = main_block(context.config);

        let main_app_area = block.inner(layout[0]);
        block.render(layout[0], buf);
        if let Some(k) = key_hints {
            k.render(layout[1], buf)
        }
        let mut view_buffer = Buffer::empty(main_app_area);
        self.view.render(main_app_area, &mut view_buffer, context);
        if let Some(prompt) = &self.prompt {
            view_buffer.set_style(main_app_area, Style::default().dim());
            buf.merge(&view_buffer);
            self.render_prompt(main_app_area, buf, &**prompt, context);
        } else {
            buf.merge(&view_buffer);
        }
    }
}

trait View: Component {
    fn item(&self) -> Item;
    fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a>;
    // TODO: implement this automatically
    fn handle_action(&mut self, action: &Action, context: Context);
}

pub trait Component: Debug {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action>;
    fn key_hints(&self, context: Context) -> KeyHints;
    fn render(&self, area: Rect, buf: &mut Buffer, context: Context);
}

#[derive(strum_macros::Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Item {
    Project,
    Task,
}

#[derive(Debug)]
#[allow(private_interfaces)]
pub enum Action {
    ShrinkList,
    Delete,
    ChangePriority(Priority),
    New(String),
    Rename(String),
    MoveToColumn(String),
    SwitchToView(Box<dyn View>),
    OpenPrompt(Box<dyn Prompt>),
    SwitchToIndex(usize),
}

impl From<Priority> for Span<'_> {
    fn from(value: Priority) -> Self {
        Span::styled(
            value.to_string(),
            Style::new().fg(match value {
                Priority::Low => Color::Green,
                Priority::Medium => Color::Yellow,
                Priority::High => Color::Red,
            }),
        )
    }
}
