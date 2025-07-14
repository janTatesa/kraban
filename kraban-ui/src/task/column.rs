mod component;
mod query;

use kraban_config::ColumnConfig;
use query::TaskTable;
use ratatui::style::Color;

use crate::table::{Table, TableQuery};

#[derive(Debug, Clone)]
pub struct ColumnView<'a> {
    color: Color,
    immutable: bool,
    // TODO: use thhe table macro
    table: Table<TaskTable<'a>, { TaskTable::COLUMNS }>,
}

impl<'a> ColumnView<'a> {
    pub fn new(project: usize, column: &'a ColumnConfig, task: usize) -> Self {
        let immutable = column.done_column;
        Self {
            color: column.color,
            immutable,
            table: Table::new(
                task,
                TaskTable {
                    project_index: project,
                    column: &column.name,
                    immutable,
                },
            ),
        }
    }

    pub fn selected(&self) -> (&str, Option<usize>) {
        (self.table.query().column, self.table.selected())
    }
}
