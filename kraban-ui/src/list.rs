use std::{fmt::Debug, ops::Deref};

use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{List as ListWidget, ListState, StatefulWidget}
};

use crate::keyhints::Keyhints;
pub trait ListQuery {
    fn get_items<'a>(&self, state: &'a State, config: &'a Config)
    -> impl Iterator<Item = Line<'a>>;
}

impl<Q: ListQuery> List<Q> {
    pub fn on_key(&mut self, key: KeyEvent) {
        let KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            ..
        } = key
        else {
            return
        };

        match code {
            KeyCode::Down => self.selected.select_next(),
            KeyCode::Up => self.selected.select_previous(),
            KeyCode::Home => self.selected.select_first(),
            KeyCode::End => self.selected.select_last(),
            _ => {}
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        ListWidget::new(self.query.get_items(state, config))
            .highlight_style(Style::new().on_black())
            .highlight_symbol(">")
            .render(area, buf, &mut self.selected)
    }
}
#[derive(Debug, Clone)]
pub struct List<Q> {
    selected: ListState,
    query: Q
}

impl<Q> Deref for List<Q> {
    type Target = Q;
    fn deref(&self) -> &Self::Target { &self.query }
}

impl<Q> List<Q> {
    pub fn new(query: Q) -> Self {
        Self {
            selected: ListState::default().with_selected(Some(0)),
            query
        }
    }

    pub fn selected(&self) -> usize { self.selected.selected().unwrap() }
}

impl<Q: ListQuery> Keyhints for List<Q> {
    fn keyhints(&self, _: &State, _: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        [
            ("Up/Down", "Select previous/next"),
            ("Home/End", "Go to start/end")
        ]
    }
}
