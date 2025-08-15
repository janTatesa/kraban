use std::fmt::Display;

use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::Widget
};

use crate::{table::TableQuery, task::column::ColumnView};

fn optional_text(display: impl Display, condition: bool) -> String {
    if condition {
        format!(" ({display})")
    } else {
        String::default()
    }
}

impl<'a> ColumnView<'a> {
    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool
    ) {
        let column_len = self.table.len(state, config);
        let block_name = format!(
            "{}{}{}",
            self.column,
            optional_text(column_len, area.height <= column_len as u16,),
            optional_text("immutable", self.immutable)
        );

        Line::from(block_name).centered().render(area, buf);
        let style = if focused {
            Style::new().fg(self.color).reversed()
        } else {
            Style::new().fg(self.color).on_black()
        }
        .bold();

        buf.set_style(Rect { height: 1, ..area }, style);
        let area = Rect {
            y: area.y + 1,
            height: area.height - 1,
            ..area
        };

        self.table.render(area, buf, state, config, focused);
    }
}
