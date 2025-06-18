use std::fmt::Display;

use crossterm::event::KeyEvent;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::Widget,
};

use crate::{Component, Context, action::Action, keyhints::KeyHints, table::TableQuery};

use super::ColumnView;

fn optional_text(display: impl Display, condition: bool) -> String {
    match condition {
        true => format!(" ({display})"),
        false => String::default(),
    }
}

impl Component for ColumnView {
    fn render(&self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let column_len = self.list.query().len(context);
        let block_name = format!(
            "{}{}{}",
            self.list.query().column,
            optional_text(column_len, area.height <= column_len as u16,),
            optional_text("immutable", self.immutable)
        );

        Line::from(block_name).centered().render(area, buf);
        buf.set_style(
            Rect { height: 1, ..area },
            match focused {
                true => Style::new().fg(self.color).reversed(),
                false => Style::new().fg(self.color).on_black(),
            }
            .bold(),
        );

        let area = Rect {
            y: area.y + 1,
            height: area.height - 1,
            ..area
        };

        self.list.render(area, buf, context, focused);
    }

    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        self.list.on_key(key_event, context)
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        self.list.key_hints(context)
    }
}
