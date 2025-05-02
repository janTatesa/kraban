use crate::{Action, SwitchToIndex};

use super::{Column, Priority, State, defaultmap::DefaultMap};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize, Default)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {
    // Priority should be on top so it's sorted properly
    pub priority: Option<Priority>,
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub title: String,
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub columns: DefaultMap<String, Column>,
}

impl State {
    pub fn projects(&self) -> &Vec<Project> {
        self.projects.inner()
    }

    pub(super) fn handle_project_action(
        &mut self,
        action: Action,
        index: Option<usize>,
    ) -> Option<SwitchToIndex> {
        match action {
            Action::Delete => {
                index.map(|index| self.projects.remove(index));
                None
            }
            Action::ChangePriority(priority) => {
                Self::modifing_action(index, &mut self.projects, |project| Project {
                    priority,
                    ..project
                })
            }
            Action::New(title) => Some(SwitchToIndex(self.projects.push(Project {
                title,
                ..Project::default()
            }))),
            Action::Rename(title) => Self::modifing_action(index, &mut self.projects, |project| {
                Project { title, ..project }
            }),
            Action::MoveToColumn(_) => panic!("Project cannot be moved to a column"),
            _ => None,
        }
    }
}
