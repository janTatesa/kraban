use std::borrow::Cow;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

use crate::{
    Component, Context, Item, StateAction,
    action::{Action, state_action},
    keyhints::KeyHints,
};

use super::{DEFAULT_WIDTH, PromptTrait};

#[derive(Debug)]
pub struct DeleteConfirmation {
    pub name: String,
    pub item: Item,
}

impl PromptTrait for DeleteConfirmation {
    fn height(&self, _context: Context) -> u16 {
        1
    }

    fn title(&self, item: Item) -> Cow<'static, str> {
        let item: &str = item.into();
        format!("Delete {item}").into()
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

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context, _focused: bool) {
        let item_type: &str = self.item.into();
        let spans = [
            "Are you sure to delete ".into(),
            item_type.into(),
            " ".into(),
            Span::styled(&self.name, Style::new().fg(context.config.app_color)),
            "?".into(),
        ];

        Line::from_iter(spans).centered().render(area, buf);
    }
}
