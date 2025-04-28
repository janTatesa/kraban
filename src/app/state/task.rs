use std::cmp::Ordering;

use crate::app::{Action, Date};

use super::{Difficulty, Priority, State};
use cli_log::debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Derivative, Clone)]
pub struct Task {
    // Priority and difficulty should be on top so it's sorted properly
    pub priority: Option<Priority>,
    pub due_date: Option<Date>,
    pub difficulty: Option<Difficulty>,
    pub title: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        let due_date_comparison = self.due_date.cmp(&other.due_date);
        let due_date_comparison = if let (Some(_), Some(_)) = (self.due_date, other.due_date) {
            due_date_comparison.reverse()
        } else {
            due_date_comparison
        };

        self.priority
            .cmp(&other.priority)
            .then(due_date_comparison)
            .then(self.difficulty.cmp(&other.difficulty))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn tasks(&self, project: usize, column: &str) -> &Vec<Task> {
        self.projects[project].columns.get(column).inner()
    }

    pub(super) fn handle_task_action(
        &mut self,
        action: Action,
        project: usize,
        column: &str,
        index: Option<usize>,
    ) -> Option<Action> {
        let list = self.projects[project].columns.get_mut(column.to_string());
        match action {
            Action::Delete => {
                debug!("{:?}", index);
                list.remove(index?);
                None
            }
            Action::ChangePriority(priority) => {
                Self::modifing_action(index, list, |task| Task { priority, ..task })
            }
            Action::ChangeDifficulty(difficulty) => {
                Self::modifing_action(index, list, |task| Task { difficulty, ..task })
            }
            Action::New(title) => Some(Action::SwitchToIndex(list.push(Task {
                title,
                ..Task::default()
            }))),
            Action::Rename(title) => {
                Self::modifing_action(index, list, |task| Task { title, ..task })
            }
            Action::MoveToColumn(column) => {
                let task = list.remove(index?);
                self.projects[project].columns.get_mut(column).push(task);
                None
            }
            Action::SetTaskDueDate(due_date) => Self::modifing_action(index, list, |task| Task {
                due_date: Some(due_date),
                ..task
            }),
            _ => None,
        }
    }
}
