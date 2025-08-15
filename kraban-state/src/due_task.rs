use std::cmp::Ordering;

use kraban_config::{ColumnConfig, Config};
use time::Date;

use super::State;
use crate::{Difficulty, Priority, Project};

#[derive(PartialEq, Eq, Debug)]
pub struct DueTask<'a> {
    pub priority: Option<Priority>,
    pub due_date: Date,
    pub difficulty: Option<Difficulty>,
    pub title: &'a str,
    pub idx: usize,
    pub project_idx: usize,
    pub project: &'a Project,
    pub column_config: &'a ColumnConfig
}

impl Ord for DueTask<'_> {
    fn cmp(&self, other: &Self) -> Ordering { other.due_date.cmp(&self.due_date) }
}

impl PartialOrd for DueTask<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<'a> State {
    // TODO: Maybe iomit using vec smh
    pub fn due_tasks(&'a self, config: &'a Config) -> impl Iterator<Item = DueTask<'a>> {
        let mut vec = Vec::from_iter(
            config
                .column_configs()
                .filter(|column| !column.done_column)
                .flat_map(|column| self.column_due_tasks(column))
        );

        vec.sort();
        vec.reverse();
        vec.into_iter()
    }

    fn column_due_tasks(&'a self, column: &'a ColumnConfig) -> impl Iterator<Item = DueTask<'a>> {
        self.projects
            .iter()
            .enumerate()
            .flat_map(|(project_idx, project)| project.due_tasks_by_column(column, project_idx))
    }
}
