use crate::app::Context;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use keyhints::KeyHints;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Clear, Widget},
};
use std::fmt::Debug;
use tap::Tap;
use widgets::main_block;

use super::{
    Action, Ui,
    keyhints::{self, KeyHintsWidget},
    prompt::Prompt,
    widgets::{self, block_widget},
};

pub trait Component: Debug {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action>;
    fn key_hints(&self, context: Context) -> KeyHints;
    fn render(&self, area: Rect, buf: &mut Buffer, context: Context);
}

impl Component for Ui {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match (key_event, &mut self.prompt) {
            (
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    ..
                },
                Some(_),
            ) => Some(Action::ClosePrompt),
            (_, Some(prompt)) => prompt.on_key(key_event, context),
            _ => self.view.on_key(key_event, context),
        }
        .and_then(|action| self.handle_action(action, context))
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        match &self.prompt {
            Some(prompt) => prompt
                .key_hints(context)
                .tap_mut(|v| v.push(("Esc", "Exit Prompt"))),
            _ => self.view.key_hints(context),
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
        if let Some(key_hints) = key_hints {
            key_hints.render(layout[1], buf)
        }

        let mut view_buffer = Buffer::empty(main_app_area);
        self.view.render(main_app_area, &mut view_buffer, context);
        match &self.prompt {
            Some(prompt) => {
                view_buffer.set_style(main_app_area, Style::default().dim());
                buf.merge(&view_buffer);
                self.render_prompt(main_app_area, buf, &**prompt, context);
            }
            _ => {
                buf.merge(&view_buffer);
            }
        }
    }
}

fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl Ui {
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
}
