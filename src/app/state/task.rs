use crate::app::Action;

use super::{Difficulty, Priority, State};
use cli_log::debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Derivative)]
#[derivative(PartialOrd, Ord)]
pub struct Task {
    // Priority and difficulty should be on top so it's sorted properly
    pub priority: Option<Priority>,
    pub difficulty: Option<Difficulty>,
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
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
                None
            }
            Action::ChangePriority(priority) => Self::modifing_action(index, list, |task| Task {
                priority: Some(priority),
                ..task
            }),
            Action::ChangeDifficulty(difficulty) => {
                Self::modifing_action(index, list, |task| Task {
                    difficulty: Some(difficulty),
                    ..task
                })
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
            _ => None,
        }
    }
}
