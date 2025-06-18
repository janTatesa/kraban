use core::panic;

use crate::{Action, ReversedSortedVec, SwitchToIndex};

use super::{Column, Priority, State, defaultmap::DefaultMap};
use derivative::Derivative;
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

    pub(super) fn handle_project_action(
        &mut self,
        action: Action,
        index: Option<usize>,
    ) -> Option<SwitchToIndex> {
        let projects = &mut self.projects;
        match action {
            Action::Delete => _ = projects.remove(index?),
            Action::ChangePriority(priority) => change_priority(index?, projects, priority),
            Action::New(title) => return self.new_project(title),
            Action::Rename(title) => rename(index?, projects, title),
            action => panic!("Cannot perform {action:?} when in projects view"),
        }
        None
    }

    fn new_project(&mut self, title: String) -> Option<SwitchToIndex> {
        Some(SwitchToIndex(self.projects.push(Project {
            title,
            ..Project::default()
        })))
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
