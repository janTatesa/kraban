use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::{Buffer, Cell},
    layout::{Constraint, Layout, Rect},
    style::Style,
};
use tap::Tap;

use crate::{
    Component, Context, action::Action, get, keyhints::KeyHints, task::column::ColumnView,
};

use super::TabView;

fn render_column_at(
    buf: &mut Buffer,
    area: Rect,
    context: Context,
    should_render_separator: bool,
    column_view: &mut ColumnView,
    focused: bool,
) {
    let area = area.tap_mut(|area| {
        if should_render_separator {
            render_separator(*area, buf, context);
            area.width -= 1;
        }
    });

    column_view.render(area, buf, context, focused);
}

impl<'a> Component<'a> for TabView<'a> {
    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let columns_len = get!(context, tabs, self.tab_index).len();
        let column_constraints = vec![Constraint::Min(0); columns_len];
        Layout::horizontal(column_constraints)
            .split(area)
            .iter()
            .zip(&mut self.columns)
            .enumerate()
            .for_each(|(index, (area, column_view))| {
                render_column_at(
                    buf,
                    *area,
                    context,
                    columns_len - 1 > index,
                    column_view,
                    focused && index == self.focused.value(),
                )
            })
    }

    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        let column_view = &mut self.columns[self.focused.value()];
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Left, KeyModifiers::NONE) => self.focused.decrement(),
            (KeyCode::Right, KeyModifiers::NONE) => self.focused.increment(),
            _ => return column_view.on_key(key_event, context),
        }
        None
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        self.columns[self.focused.value()]
            .key_hints(context)
            .tap_mut(|v| v.push(("Left/Right", "Switch between columns")))
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
