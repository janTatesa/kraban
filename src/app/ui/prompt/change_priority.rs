use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, text::Span, widgets::StatefulWidget};
use strum::IntoEnumIterator;

use crate::app::{
    Context,
    state::Priority,
    ui::{Action, Component, Item, keyhints::KeyHints, list::WrappingUsize, widgets::list_widget},
};

use super::Prompt;

#[derive(Debug)]
pub struct ChangePriorityPrompt(WrappingUsize);
impl ChangePriorityPrompt {
    pub const fn new() -> Self {
        Self(WrappingUsize::new(2))
    }
}

impl Prompt for ChangePriorityPrompt {
    fn height(&self) -> u16 {
        3
    }

    fn title(&self, item: Item) -> String {
        format!("Change {item} priority")
    }
}

impl Component for ChangePriorityPrompt {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Up => {
                self.0 = self.0.decrement();
                None
            }
            KeyCode::Down => {
                self.0 = self.0.increment();
                None
            }
            KeyCode::Enter => Some(Action::ChangePriority(
                Priority::iter().nth(self.0.into()).unwrap(),
            )),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![
            ("Up/Down", "Select previous/next"),
            ("Enter", "Pick priority"),
        ]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, _context: Context) {
        let list = list_widget(Priority::iter().map(Span::from));
        list.render(area, buf, &mut self.0.into());
    }
}
