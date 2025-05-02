use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::Widget,
};
use tap::Tap;
use tui_textarea::{CursorMove, TextArea};

use crate::{Action, Component, Context, Item, StateAction, keyhints::KeyHints, state_action};

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

    fn current_line(&self) -> &String {
        self.text_area.lines().first().unwrap()
    }
}

impl PromptTrait for InputPrompt {
    fn height(&self) -> u16 {
        1
    }

    fn title(&self, item: Item) -> String {
        format!("{} {item}", self.input_action)
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH
    }
}

impl Component for InputPrompt {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Enter if self.current_line() != &String::new() => {
                state_action(match self.input_action {
                    InputAction::Rename => StateAction::Rename(self.current_line().clone()),
                    InputAction::New => StateAction::New(self.current_line().clone()),
                })
            }
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

    fn render(&self, area: Rect, buf: &mut Buffer, _context: Context) {
        self.text_area.render(area, buf)
    }
}
