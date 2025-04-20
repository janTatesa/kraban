use ratatui::{
    buffer::{Buffer, Cell},
    layout::{Constraint, Layout, Rect},
    style::Style,
};
use tap::Tap;

use crate::app::Context;

use super::TasksView;

impl TasksView {
    pub(super) fn render_tab(&self, area: Rect, buf: &mut Buffer, context: Context, tab: usize) {
        let column_constraints = vec![Constraint::Min(0); context.config.tabs[tab].columns.len()];
        let columns_len = column_constraints.len();
        Layout::horizontal(column_constraints)
            .split(area)
            .iter()
            .enumerate()
            .for_each(|(column, area)| {
                self.render_column_at(buf, *area, context, column, tab, columns_len)
            })
    }

    fn render_column_at(
        &self,
        buf: &mut Buffer,
        area: Rect,
        context: Context,
        index: usize,
        tab: usize,
        total_columns: usize,
    ) {
        let area = if index < total_columns - 1 {
            render_separator(area, buf, context);
            Rect {
                width: area.width - 1,
                ..area
            }
        } else {
            area
        };

        self.render_column(area, buf, context, tab, index);
    }
}

fn render_separator(area: Rect, buf: &mut Buffer, context: Context) {
    let separator_buf_area = Rect {
        width: 1,
        x: area.x + area.width - 1,
        ..area
    };
    let separator_buffer = Buffer::filled(
        separator_buf_area,
        Cell::new(ratatui::symbols::line::VERTICAL),
    )
    .tap_mut(|buf| {
        buf.set_style(
            separator_buf_area,
            Style::new().fg(context.config.app_color),
        )
    });
    buf.merge(&separator_buffer);
}
