use std::fmt::Display;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{StatefulWidget, Widget},
};

use crate::app::{
    Context,
    config::{Column, Config},
    ui::widgets::list_widget,
};

use super::TasksView;

fn optional_text(display: impl Display, condition: bool) -> String {
    if condition {
        format!(" ({display})")
    } else {
        String::default()
    }
}

impl TasksView {
    pub(super) fn render_column(
        &self,
        area: Rect,
        buf: &mut Buffer,
        context: Context,
        tab: usize,
        column: usize,
    ) {
        let focused =
            tab == usize::from(self.focused_tab) && column == usize::from(self.focused_column);
        let columns = &context.config.tabs[tab].columns;
        let column = columns.get(column).unwrap();
        let tasks = context.state.tasks(self.project, &column.name);
        let column_len = tasks.len();
        let block_name = format!(
            "{}{}{}",
            column.name,
            optional_text(column_len, area.height <= column_len as u16,),
            optional_text("immutable", column.immutable)
        );
        Line::from(block_name).centered().render(area, buf);
        buf.set_style(
            Rect { height: 1, ..area },
            if focused {
                Style::new().fg(column.color).reversed()
            } else {
                Style::new().fg(column.color).on_black()
            }
            .bold(),
        );
        let column = tasks.iter().map(Line::from);
        let area = Rect {
            y: area.y + 1,
            height: area.height.saturating_sub(1),
            ..area
        };
        if focused {
            StatefulWidget::render(
                list_widget(column),
                area,
                buf,
                &mut self.focused_task.into(),
            );
        } else {
            Text::from(column.collect::<Vec<Line>>()).render(area, buf);
        }
    }

    pub(super) fn get_current_column<'a>(&self, config: &'a Config) -> &'a Column {
        config
            .tabs
            .get::<usize>(self.focused_tab.into())
            .unwrap()
            .columns
            .get::<usize>(self.focused_column.into())
            .unwrap()
    }

    pub(super) fn get_current_column_len(&self, context: Context) -> usize {
        context
            .state
            .tasks(self.project, &self.get_current_column(context.config).name)
            .len()
    }
}
