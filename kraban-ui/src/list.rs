use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    fn get_items<'a>(&self, context: Context<'a, 'a>) -> impl Iterator<Item = Line<'a>>;
    fn on_key<'a>(
        &self,
        index: usize,
        key_event: KeyEvent,
        context: Context<'_, 'a>,
    ) -> Option<Action<'a>>;
    fn keyhints(&self, context: Context) -> KeyHints;
}

impl<'a, Q: ListQuery> Component<'a> for List<Q> {
    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        let selected = &mut self.selected;
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Down, KeyModifiers::NONE) => selected.select_next(),
            (KeyCode::Up, KeyModifiers::NONE) => selected.select_previous(),
            (KeyCode::Home, KeyModifiers::NONE) => selected.select_first(),
            (KeyCode::End, KeyModifiers::NONE) => selected.select_last(),
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

    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let list = self.query.get_items(context);
        match focused {
            true => StatefulWidget::render(list_widget(list), area, buf, &mut self.selected),
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

#[derive(Debug, Clone)]
pub struct List<Q> {
    selected: ListState,
    query: Q,
}

impl<Q> List<Q> {
    pub fn new(query: Q) -> Self {
        Self {
            selected: ListState::default().with_selected(Some(0)),
            query,
        }
    }

    pub fn selected(&self) -> usize {
        self.selected.selected().unwrap()
    }
}
