use core::panic;

use crate::{Action, ItemToCreate, ReversedSortedVec};

use super::{Column, Priority, State, defaultmap::DefaultMap};
use derivative::Derivative;
use kraban_lib::unwrap_or_ret;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    pub columns: DefaultMap<String, Column>,
}

impl State {
    pub fn projects(&self) -> &Vec<Project> {
        self.projects.inner()
    }

    pub(super) fn handle_project_action(&mut self, action: Action, index: Option<usize>) {
        let projects = &mut self.projects;
        match action {
            Action::Delete => _ = projects.remove(unwrap_or_ret!(index)),
            Action::ChangePriority(priority) => {
                change_priority(unwrap_or_ret!(index), projects, priority)
            }
            Action::New(ItemToCreate::Project(project)) => _ = self.projects.push(project),
            Action::Rename(title) => rename(unwrap_or_ret!(index), projects, title),
            action => panic!("Cannot perform {action:?} when in projects view"),
        }
    }
}

fn rename(index: usize, projects: &mut ReversedSortedVec<Project>, title: String) {
    projects.map_item_at(index, |project| Project { title, ..project });
}

fn change_priority(
    index: usize,
    projects: &mut ReversedSortedVec<Project>,
    priority: Option<Priority>,
) {
    projects.map_item_at(index, |project| Project {
        priority,
        ..project
    });
}
