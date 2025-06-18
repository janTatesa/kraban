use std::{borrow::Cow, mem};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::Widget,
};
use tap::{Pipe, Tap};
use tui_textarea::{CursorMove, TextArea};

use crate::{
    Component, Context, Item, StateAction,
    action::{Action, state_action},
    keyhints::KeyHints,
};

use super::{DEFAULT_WIDTH, PromptTrait};

#[derive(Debug)]
pub struct InputPrompt {
    text_area: TextArea<'static>,
    input_action: InputAction,
}

#[derive(strum_macros::Display, Debug)]
pub enum InputAction {
    Rename,
    New,
}

impl InputPrompt {
    pub fn new(context: Context, input_action: InputAction, text: String) -> Self {
        let text_area = match input_action {
            InputAction::Rename => {
                TextArea::new(vec![text]).tap_mut(|t| t.move_cursor(CursorMove::End))
            }
            InputAction::New => TextArea::default().tap_mut(|t| t.set_placeholder_text(text)),
        }
        .tap_mut(|t| {
            t.set_selection_style(Style::new().fg(context.config.app_color).reversed());
            t.set_cursor_line_style(Style::new());
        });

        Self {
            text_area,
            input_action,
        }
    }

    fn current_line_owned(&mut self) -> String {
        mem::take(&mut self.text_area)
            .into_lines()
            .into_iter()
            .next()
            .unwrap()
    }
}

impl PromptTrait for InputPrompt {
    fn height(&self, _context: Context) -> u16 {
        1
    }

    fn title(&self, item: Item) -> Cow<'static, str> {
        let item: &str = item.into();
        format!("{} {item}", self.input_action).into()
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH
    }
}

impl Component for InputPrompt {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Enter if !self.text_area.is_empty() => match self.input_action {
                InputAction::Rename => StateAction::Rename(self.current_line_owned()),
                InputAction::New => StateAction::New(self.current_line_owned()),
            }
            .pipe(state_action),
            KeyCode::Enter => None,
            _ => {
                self.text_area.input(key_event);
                None
            }
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Enter", "Submit"), ("Other", "Consult readme")]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, _context: Context, _focused: bool) {
        self.text_area.render(area, buf)
    }
}
