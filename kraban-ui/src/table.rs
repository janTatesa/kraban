use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use kraban_lib::wrapping_usize::WrappingUsize;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{self, Row, StatefulWidget, TableState, Widget},
};
use std::fmt::Debug;
use tap::{Pipe, Tap};

use crate::{Component, Context, action::Action, keyhints::KeyHints};

pub const LARGE_COLUMN_SIZE: u16 = 10;
pub const SMALL_COLUMN_SIZE: u16 = 6;
macro_rules! table {
    ($query:ident) => {
        Table<$query, {$query::COLUMNS}>
    };
}

pub(crate) use table;

#[derive(Debug, Default, Clone, Copy)]
pub struct Table<Q, const COLUMNS: usize> {
    selected: Option<WrappingUsize>,
    query: Q,
}

impl<Q: TableQuery<COLUMNS>, const COLUMNS: usize> Table<Q, COLUMNS> {
    pub fn with_default_index(idx: usize, query: Q) -> Self {
        Self {
            selected: Some(WrappingUsize::with_value(idx, idx)),
            query,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected.map(|s| s.value())
    }

    pub fn select(&mut self, index: usize) {
        self.selected = Some(WrappingUsize::with_value(index, index));
    }

    pub fn update_max_index(&mut self, context: Context) {
        let selected_old = self.selected.unwrap_or_default().value();
        self.selected = self
            .query
            .len(context)
            .checked_sub(1)
            .map(|len| WrappingUsize::with_value(selected_old, len))
    }

    pub fn query(&self) -> &Q {
        &self.query
    }
}

pub trait TableQuery<const COLUMNS: usize>: Debug {
    const COLUMNS: usize = COLUMNS;
    fn header(&self) -> [&'static str; COLUMNS];
    fn constraints(&self, context: Context) -> [Constraint; COLUMNS];
    fn rows<'a>(&self, context: Context<'a>) -> impl Iterator<Item = [Line<'a>; COLUMNS]>;
    fn on_key(&self, index: Option<usize>, key_event: KeyEvent, context: Context)
    -> Option<Action>;
    fn len(&self, context: Context) -> usize;
    fn keyhints(&self, context: Context) -> KeyHints;
}

impl<const COLUMNS: usize, Q> Component for Table<Q, COLUMNS>
where
    Q: TableQuery<COLUMNS>,
{
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE) => {
                self.selected = self.selected.map(|s| s.decrement());
                None
            }
            (KeyCode::Down, KeyModifiers::NONE) => {
                self.selected = self.selected.map(|s| s.increment());
                None
            }
            _ => self
                .query
                .on_key(self.selected.map(|s| s.value()), key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        vec![
            ("Up/Down", "Select previous/next"),
            ("Home/End", "Go to start/end"),
        ]
        .tap_mut(|v| v.extend(self.query.keyhints(context)))
    }

    fn render(&self, area: Rect, buf: &mut Buffer, ctx: Context, focused: bool) {
        let header = self.query.header().map(|s| s.dim().italic()).pipe(Row::new);
        let widths = self.query.constraints(ctx);
        let rows = self.query.rows(ctx).map(Row::new);
        let table = widgets::Table::new(rows, widths)
            .header(header)
            .column_spacing(1);
        match focused {
            true => StatefulWidget::render(
                table
                    .row_highlight_style(Style::new().on_black())
                    .highlight_symbol(">"),
                area,
                buf,
                &mut TableState::new().with_selected(self.selected()),
            ),
            false => Widget::render(table, area, buf),
        }
    }
}
