use crate::app::ui::Action;

use super::{Priority, State};
use cli_log::debug;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    // Priority should be on top so it's sorted properly
    pub priority: Option<Priority>,
    pub title: String,
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
                Some(Action::ShrinkList)
            }
            Action::ChangePriority(priority) => Self::modifing_action(index, list, |task| Task {
                priority: Some(priority),
                ..task
            }),
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
                Some(Action::ShrinkList)
            }
            _ => None,
        }
    }
}
