mod component;
mod query;

use kraban_config::Column;
use query::TaskTable;
use ratatui::style::Color;

use crate::{
    Context,
    table::{Table, TableQuery, table},
};

#[derive(Debug, Clone)]
pub struct ColumnView {
    color: Color,
    immutable: bool,
    list: table!(TaskTable),
}

impl ColumnView {
    pub fn new(project: usize, column: &Column, task: usize) -> Self {
        let immutable = column.done_column;
        Self {
            color: column.color,
            immutable,
            list: Table::with_default_index(
                task,
                TaskTable {
                    project_index: project,
                    column: column.name.clone(),
                    immutable,
                },
            ),
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.list.select(index);
    }

    pub fn update_max_index(&mut self, context: Context) {
        self.list.update_max_index(context);
    }

    pub fn selected(&self) -> (&str, Option<usize>) {
        (&self.list.query().column, self.list.selected())
    }
}
