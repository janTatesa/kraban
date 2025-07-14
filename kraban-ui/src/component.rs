use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use keyhints::KeyHints;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Clear, Widget},
};
use tap::Tap;
use widgets::main_block;

use crate::{
    Context, ViewTrait,
    action::Action,
    prompt::{PromptTrait, center_area},
    widgets::block_widget,
};

use super::{
    Ui,
    keyhints::{self, KeyHintsWidget},
    widgets,
};

#[enum_dispatch]
pub(crate) trait Component<'a>: Debug {
    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>>;
    fn key_hints(&self, context: Context) -> KeyHints;
    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, focused: bool);
}

impl<'a> Component<'a> for Ui<'a> {
    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        match (key_event, self.prompt_stack.last_mut()) {
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
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        match &self.prompt_stack.last() {
            Some(prompt) => prompt
                .key_hints(context)
                .tap_mut(|v| v.push(("Esc", "Exit Prompt"))),
            _ => self.view.key_hints(context),
        }
    }

    fn render(&mut self, terminal_area: Rect, buf: &mut Buffer, context: Context, _focused: bool) {
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
                    .unwrap_or(0),
            ),
        ])
        .split(terminal_area);

        let block = main_block(context.config).title(self.view.title(context));
        let block = match self.view.right_title() {
            Some(title) => block.title(Line::raw(title).right_aligned()),
            None => block,
        };

        let main_app_area = block.inner(layout[0]);
        block.render(layout[0], buf);
        if let Some(key_hints) = key_hints {
            key_hints.render(layout[1], buf)
        }

        self.view
            .render(main_app_area, buf, context, self.prompt_stack.is_empty());
        if let Some(prompt) = self.prompt_stack.last_mut() {
            buf.set_style(main_app_area, Style::default().dim());
            let prompt_area = center_area(
                main_app_area,
                Constraint::Length(prompt.width() + 2),
                Constraint::Length(prompt.height(context) + 2),
            );

            Clear.render(prompt_area, buf);
            let block = block_widget(context.config)
                .title(Line::from(prompt.title(self.view.current_item())).centered());
            let inner_prompt_area = block.inner(prompt_area);
            block.render(prompt_area, buf);
            prompt.render(inner_prompt_area, buf, context, true);
        }
    }
}

impl Ui<'_> {
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
}
