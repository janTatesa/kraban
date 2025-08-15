use derivative::Derivative;
use kraban_config::{ColumnConfig, Config};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Priority;
use crate::{Columns, DueTask, SetPriority};

#[derive(Derivative, Serialize, Deserialize, Default, Debug)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
#[skip_serializing_none]
pub struct Project {
    // Priority should be on top so it's sorted properly
    pub priority: Option<Priority>,
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub title: String,
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub columns: Columns
}

impl Project {
    pub fn new(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }

    pub(crate) fn due_tasks_by_column<'a>(
        &'a self,
        column_config: &'a ColumnConfig,
        idx_of_self: usize
    ) -> impl Iterator<Item = DueTask<'a>> {
        self.columns
            .get(&column_config.name)
            .iter()
            .enumerate()
            .filter_map(|(i, task)| Some((i, task, task.due_date()?)))
            .map(move |(idx, task, due_date)| DueTask {
                project: self,
                column_config,
                idx,
                project_idx: idx_of_self,
                priority: task.priority(),
                due_date,
                difficulty: task.difficulty,
                title: &task.title
            })
    }
}

impl SetPriority for Project {
    fn set_priority(&mut self, priority: Option<Priority>, _: &Config) { self.priority = priority }
}
