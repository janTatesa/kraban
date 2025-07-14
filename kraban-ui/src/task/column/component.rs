use std::fmt::Display;

use ratatui::crossterm::event::KeyEvent;

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

impl<'a> Component<'a> for ColumnView<'a> {
    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let column_len = self.table.query().len(context);
        let block_name = format!(
            "{}{}{}",
            self.table.query().column,
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

        self.table.render(area, buf, context, focused);
    }

    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        self.table.on_key(key_event, context)
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        self.table.key_hints(context)
    }
}
