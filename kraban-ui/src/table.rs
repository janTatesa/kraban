use std::ops::Deref;

use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{self, Row, StatefulWidget, TableState, Widget}
};

use crate::keyhints::Keyhints;

macro_rules! table {(
    $query:ident $(< $lt:lifetime >)?
) => (
    Table<$query $(<$lt>)?, { $query::COLUMNS }>
)}

pub(crate) use table;

pub struct Table<Q, const COLUMNS: usize> {
    selected: TableState,
    query: Q
}

impl<Q: Default, const COLUMNS: usize> Default for Table<Q, COLUMNS> {
    fn default() -> Self {
        Self {
            selected: TableState::default(),
            query: Q::default()
        }
    }
}

impl<Q, const COLUMNS: usize> Deref for Table<Q, COLUMNS> {
    type Target = Q;

    fn deref(&self) -> &Self::Target { &self.query }
}

impl<Q: TableQuery<COLUMNS>, const COLUMNS: usize> Table<Q, COLUMNS> {
    pub fn new(idx: usize, query: Q) -> Self {
        let selected = TableState::new().with_selected(idx);
        Self { selected, query }
    }

    pub fn selected(&self, state: &State, config: &Config) -> Option<usize> {
        self.selected
            .selected()
            .min(self.query.len(state, config).checked_sub(1))
    }
}

pub trait TableQuery<const COLUMNS: usize> {
    const COLUMNS: usize = COLUMNS;
    const CONSTRAINTS: [Constraint; COLUMNS];
    fn rows<'a>(
        &self,
        state: &'a State,
        config: &'a Config
    ) -> impl Iterator<Item = [Line<'a>; COLUMNS]>;
    fn len(&self, state: &State, config: &Config) -> usize;
}

impl<const COLUMNS: usize, Q> Table<Q, COLUMNS>
where
    Q: TableQuery<COLUMNS>
{
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
            KeyCode::Up => self.selected.select_previous(),
            KeyCode::Down => self.selected.select_next(),
            KeyCode::Home => self.selected.select_first(),
            KeyCode::End => self.selected.select_last(),
            _ => {}
        }
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool
    ) {
        let widths = Q::CONSTRAINTS;
        let rows = self.query.rows(state, config).map(Row::new);
        let table = widgets::Table::new(rows, widths)
            .column_spacing(1)
            .row_highlight_style(Style::new().on_black())
            .highlight_symbol(">");

        if self.selected.selected().is_none() {
            self.selected.select(Some(0));
        }

        if focused {
            StatefulWidget::render(table, area, buf, &mut self.selected)
        } else {
            Widget::render(table, area, buf)
        }
    }
}

impl<const COLUMNS: usize, Q> Keyhints for Table<Q, COLUMNS> {
    fn keyhints(&self, _: &State, _: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        [
            ("Up/Down", "Select previous/next"),
            ("Home/End", "Go to start/end")
        ]
    }
}
