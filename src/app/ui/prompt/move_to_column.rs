use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Span, ToSpan},
    widgets::StatefulWidget,
};

use crate::app::{
    Context,
    ui::{Action, Component, Item, keyhints::KeyHints, list::WrappingUsize, widgets::list_widget},
};

use super::Prompt;

#[derive(Debug)]
pub struct MoveToColumnPrompt {
    selected: WrappingUsize,
    current_column: String,
}
impl MoveToColumnPrompt {
    pub fn new(context: Context, current_column: String) -> Self {
        Self {
            selected: WrappingUsize::new(context.config.all_columns.len().saturating_sub(2)),
            current_column,
        }
    }

    fn columns<'a>(&self, context: Context<'a>) -> Vec<Span<'a>> {
        context
            .config
            .all_columns
            .iter()
            .filter_map(|column| {
                if column.name == self.current_column {
                    None
                } else {
                    Some(column.name.to_span().fg(column.color))
                }
            })
            .collect()
    }
}

impl Prompt for MoveToColumnPrompt {
    fn height(&self) -> u16 {
        self.selected.max() as u16 + 1
    }

    fn title(&self, _item: Item) -> String {
        "Move task to column".to_string()
    }
}

impl Component for MoveToColumnPrompt {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Up => {
                self.selected = self.selected.decrement();
                None
            }
            KeyCode::Down => {
                self.selected = self.selected.increment();
                None
            }
            KeyCode::Enter => Some(Action::MoveToColumn(
                self.columns(context)
                    .get::<usize>(self.selected.into())
                    .unwrap()
                    .to_string(),
            )),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![
            ("Up/Down", "Select previous/next"),
            ("Enter", "Pick column"),
        ]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context) {
        list_widget(self.columns(context)).render(area, buf, &mut self.selected.into());
    }
}
