mod component;

use super::column::ColumnView;
use kraban_config::TabConfig;
use kraban_lib::wrapping_usize::WrappingUsize;

#[derive(Debug, Clone)]
pub struct TabView<'a> {
    tab_index: usize,
    columns: Vec<ColumnView<'a>>,
    focused: WrappingUsize,
}

impl<'a> TabView<'a> {
    pub fn new(project: usize, tab_index: usize, tab: &'a TabConfig) -> Self {
        Self::with_column_and_task(project, tab_index, tab, 0, 0)
    }

    pub fn with_column_and_task(
        project: usize,
        tab_index: usize,
        tab: &'a TabConfig,
        column_index: usize,
        task: usize,
    ) -> Self {
        Self {
            tab_index,
            columns: tab
                .columns
                .iter()
                .map(|column| ColumnView::new(project, column, task))
                .collect(),
            focused: WrappingUsize::new_with_value(tab.columns.len() - 1, column_index),
        }
    }

    pub fn get_column_and_task_index(&self) -> (&str, Option<usize>) {
        let column = self.focused.value();
        self.columns[column].selected()
    }
}
