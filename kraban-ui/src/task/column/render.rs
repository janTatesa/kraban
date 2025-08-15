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
        let len_msg = (area.height <= column_len as u16)
            .then_some(format!(" ({column_len})"))
            .unwrap_or_default();
        let block_name = format!("{}{len_msg}", self.column);

        Line::from(block_name).centered().render(area, buf);
        let style = if focused {
            Style::new().reversed()
        } else {
            Style::new().on_black()
        }
        .bold()
        .fg(self.color);

        buf.set_style(Rect { height: 1, ..area }, style);
        let area = Rect {
            y: area.y + 1,
            height: area.height - 1,
            ..area
        };

        self.table.render(area, buf, state, config, focused);
    }
}
