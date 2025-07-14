use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    ($query:ty) => {
        Table<$query, {<$query>::COLUMNS}>
    };
}

pub(crate) use table;

#[derive(Debug, Clone)]
pub struct Table<Q, const COLUMNS: usize> {
    selected: TableState,
    query: Q,
}

impl<'a, Q: TableQuery<'a, COLUMNS>, const COLUMNS: usize> Table<Q, COLUMNS> {
    pub fn new(idx: usize, query: Q) -> Self {
        Self {
            selected: TableState::default().with_selected(idx),
            query,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected.selected()
    }

    pub fn query(&self) -> &Q {
        &self.query
    }
}

pub trait TableQuery<'a, const COLUMNS: usize>: Debug {
    const COLUMNS: usize = COLUMNS;
    fn header(&self) -> [&'static str; COLUMNS];
    fn constraints(&self, context: Context) -> [Constraint; COLUMNS];
    fn rows<'b>(&self, context: Context<'b, 'b>) -> impl Iterator<Item = [Line<'b>; COLUMNS]>;
    fn on_key(
        &self,
        index: Option<usize>,
        key_event: KeyEvent,
        context: Context<'_, 'a>,
    ) -> Option<Action<'a>>;
    fn len(&self, context: Context) -> usize;
    fn keyhints(&self, context: Context) -> KeyHints;
}

impl<'a, const COLUMNS: usize, Q> Component<'a> for Table<Q, COLUMNS>
where
    Q: TableQuery<'a, COLUMNS>,
{
    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE) => {
                self.selected.select_previous();
                None
            }
            (KeyCode::Down, KeyModifiers::NONE) => {
                self.selected.select_next();
                None
            }
            (KeyCode::Home, KeyModifiers::NONE) => {
                self.selected.select_first();
                None
            }
            (KeyCode::End, KeyModifiers::NONE) => {
                self.selected.select_last();
                None
            }

            _ => self
                .query
                .on_key(self.selected.selected(), key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        vec![
            ("Up/Down", "Select previous/next"),
            ("Home/End", "Go to start/end"),
        ]
        .tap_mut(|v| v.extend(self.query.keyhints(context)))
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, ctx: Context, focused: bool) {
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
                &mut self.selected,
            ),
            false => Widget::render(table, area, buf),
        }
    }
}
