use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span, ToSpan},
    widgets::Widget,
};

use crate::{Action, Component, Context, Item, StateAction, keyhints::KeyHints, state_action};

use super::{DEFAULT_WIDTH, PromptTrait};

#[derive(Debug)]
pub struct DeleteConfirmation {
    pub name: String,
    pub item: Item,
}

impl PromptTrait for DeleteConfirmation {
    fn height(&self) -> u16 {
        1
    }

    fn title(&self, item: Item) -> String {
        format!("Delete {item}")
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH
    }
}

impl Component for DeleteConfirmation {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Char('y' | 'Y') | KeyCode::Enter => state_action(StateAction::Delete),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Y/y/Enter", "Confirm")]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context) {
        Line::from_iter([
            "Are you sure to delete ".to_span(),
            self.item.to_string().to_span(),
            " ".to_span(),
            Span::styled(&self.name, Style::new().fg(context.config.app_color)),
            "?".to_span(),
        ])
        .centered()
        .render(area, buf);
    }
}
