use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use kraban_lib::wrapping_usize::WrappingUsize;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{List as ListWidget, ListItem, ListState, StatefulWidget, Widget},
};
use tap::Tap;

use std::fmt::Debug;

use crate::{Context, action::Action};

use super::{Component, keyhints::KeyHints};

pub trait ListQuery: Debug {
    fn get_items<'a>(&self, context: Context<'a>) -> impl Iterator<Item = Line<'a>>;
    fn on_key(&self, index: usize, key_event: KeyEvent, context: Context) -> Option<Action>;
    fn keyhints(&self, context: Context) -> KeyHints;
}

impl<Q: ListQuery> Component for List<Q> {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        let selected = &mut self.selected;
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Down, KeyModifiers::NONE) => *selected = selected.increment(),
            (KeyCode::Up, KeyModifiers::NONE) => *selected = selected.decrement(),
            (KeyCode::Home, KeyModifiers::NONE) => *selected = selected.set_value(0),
            (KeyCode::End, KeyModifiers::NONE) => *selected = selected.set_value(0).decrement(),
            _ => return self.query.on_key(self.selected(), key_event, context),
        }
        None
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        vec![
            ("Up/Down", "Select previous/next"),
            ("Home/End", "Go to start/end"),
        ]
        .tap_mut(|v| v.extend(self.query.keyhints(context)))
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let selected = self.selected();
        let list = self.query.get_items(context);
        match focused {
            true => StatefulWidget::render(
                list_widget(list),
                area,
                buf,
                &mut ListState::default().with_selected(Some(selected)),
            ),
            false => Widget::render(Text::from_iter(list), area, buf),
        }
    }
}

fn list_widget<'a, T>(items: T) -> ListWidget<'a>
where
    T: IntoIterator,
    T::Item: Into<ListItem<'a>>,
{
    ListWidget::new(items)
        .highlight_style(Style::new().on_black())
        .highlight_symbol(">")
}

#[derive(Debug, Clone, Copy)]
pub struct List<Q> {
    selected: WrappingUsize,
    query: Q,
}

impl<Q> List<Q> {
    pub const fn new(len: usize, query: Q) -> Self {
        Self::with_selected(0, len, query)
    }

    pub const fn with_selected(selected: usize, len: usize, query: Q) -> Self {
        Self {
            selected: WrappingUsize::with_value(selected, len - 1),
            query,
        }
    }

    pub fn selected(&self) -> usize {
        self.selected.value()
    }
}
