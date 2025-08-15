use kraban_config::{Config, TabConfig};
use kraban_lib::WrappingUsize;
use kraban_state::{State, Task};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    symbols::line
};

use super::column::ColumnView;
use crate::{TasksPrompt, keyhints::Keyhints};

#[derive(Default)]
pub struct TabView<'a> {
    tab_idx: usize,
    columns: Vec<ColumnView<'a>>,
    focused_column: WrappingUsize
}

impl<'a> TabView<'a> {
    pub fn new(project: usize, tab_idx: usize, tab: &'a TabConfig) -> Self {
        Self::with_column_and_task(project, tab_idx, tab, 0, 0)
    }

    pub fn on_key(
        &mut self,
        key: KeyEvent,
        state: &State,
        config: &Config
    ) -> Option<TasksPrompt<'a>> {
        match (key.code, key.modifiers) {
            (KeyCode::Left, KeyModifiers::NONE) => self.focused_column.decrement(),
            (KeyCode::Right, KeyModifiers::NONE) => self.focused_column.increment(),
            _ => return self.columns[*self.focused_column].on_key(key, state, config)
        }

        None
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool
    ) {
        let columns_len = config.tabs[self.tab_idx].len();
        let column_constraints = vec![Constraint::Min(0); columns_len];

        let areas = Layout::horizontal(column_constraints).split(area);
        let columns = areas.iter().copied().zip(&mut self.columns).enumerate();

        for (idx, (mut area, column_view)) in columns {
            if columns_len - 1 > idx {
                render_separator(area, buf, config);
                area.width -= 1;
            }

            column_view.render(
                area,
                buf,
                state,
                config,
                focused && idx == *self.focused_column
            );
        }
    }

    pub fn with_column_and_task(
        project: usize,
        tab_idx: usize,
        tab: &'a TabConfig,
        column_idx: usize,
        task: usize
    ) -> Self {
        Self {
            tab_idx,
            columns: tab
                .iter()
                .map(|column| ColumnView::new(project, column, task))
                .collect(),
            focused_column: WrappingUsize::new_with_value(tab.len() - 1, column_idx)
        }
    }

    pub fn push_task(&self, task: Task, state: &mut State) {
        self.columns[*self.focused_column].push_task(task, state)
    }

    pub fn modify_selected_task<T>(
        &self,
        state: &mut State,
        config: &Config,
        f: impl FnOnce(&mut Task) -> T
    ) -> Option<T> {
        self.columns[*self.focused_column].modify_selected_task(state, config, f)
    }

    pub fn delete_selected_task(&self, state: &mut State, config: &Config) -> Option<Task> {
        self.columns[*self.focused_column].delete_selected_task(state, config)
    }
}

fn render_separator(area: Rect, buf: &mut Buffer, config: &Config) {
    let separator_buf_area = Rect {
        width: 1,
        x: area.x + area.width - 1,
        ..area
    };

    for pos in separator_buf_area.positions() {
        buf[pos].set_symbol(line::VERTICAL).set_fg(config.app_color);
    }
}

impl Keyhints for TabView<'_> {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        [
            ("Up/Down", "Select previous/next"),
            ("Home/End", "Go to start/end")
        ]
        .into_iter()
        .chain(self.columns[*self.focused_column].keyhints(state, config))
    }
}
