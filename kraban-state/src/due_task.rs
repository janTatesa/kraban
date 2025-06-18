use crate::Priority;

use super::{State, Task, sorted_vec::ReversedSortedVec};
use kraban_config::Config;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct DueTask {
    pub task: Task,
    pub index: usize,
    pub project_index: usize,
    pub project_title: String,
    pub project_priority: Option<Priority>,
    pub column_name: String,
    pub column_color: Color,
}

impl Ord for DueTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.task.due_date.cmp(&other.task.due_date).reverse()
    }
}

impl PartialOrd for DueTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn due_tasks(&self) -> &Vec<DueTask> {
        self.due_tasks
            .as_ref()
            .expect("Due tasks should have been refreshed")
            .inner()
    }

    // TODO: simplify this
    pub fn compile_due_tasks_list(&mut self, config: &Config) {
        if self.due_tasks.is_none() {
            let due_tasks = config
                .columns
                .iter()
                .filter(|column| !column.done_column)
                .flat_map(|column| {
                    self.projects.inner().iter().enumerate().flat_map(
                        move |(project_index, project)| {
                            project
                                .columns
                                .get(&column.name)
                                .inner()
                                .iter()
                                .enumerate()
                                .filter(|(_, task)| task.due_date.is_some())
                                .map(move |(index, task)| DueTask {
                                    task: task.clone(),
                                    project_title: project.title.clone(),
                                    column_name: column.name.clone(),
                                    column_color: column.color,
                                    index,
                                    project_priority: project.priority,
                                    project_index,
                                })
                        },
                    )
                });
            self.due_tasks = Some(ReversedSortedVec::new(due_tasks.collect()));
        }
    }
}
