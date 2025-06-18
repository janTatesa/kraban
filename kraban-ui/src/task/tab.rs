mod component;

use kraban_config::Tab;
use kraban_lib::wrapping_usize::WrappingUsize;

use crate::Context;

use super::column::ColumnView;

#[derive(Debug, Clone)]
pub struct TabView {
    tab_index: usize,
    columns: Vec<ColumnView>,
    focused: WrappingUsize,
}

impl TabView {
    pub fn new(project: usize, tab_index: usize, tab: &Tab) -> Self {
        Self::with_column_and_task(project, tab_index, tab, 0, 0)
    }

    pub fn set_task_index(&mut self, index: usize) {
        self.columns[self.focused.value()].set_index(index);
    }

    pub fn with_column_and_task(
        project: usize,
        tab_index: usize,
        tab: &Tab,
        column: usize,
        task: usize,
    ) -> Self {
        Self {
            tab_index,
            columns: tab
                .columns
                .iter()
                .map(|column| ColumnView::new(project, column, task))
                .collect(),
            focused: WrappingUsize::with_value(column, tab.columns.len() - 1),
        }
    }

    pub fn update_column_max_index(&mut self, context: Context) {
        self.columns[self.focused.value()].update_max_index(context);
    }

    pub fn get_column_and_task_index(&self) -> (&str, Option<usize>) {
        let column = self.focused.value();
        self.columns[column].selected()
    }
}
