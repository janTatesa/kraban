mod component;
mod due_tasks;
mod projects;
mod view;

use std::fmt::Debug;

use super::list::ListState;
use crate::Context;

#[derive(Debug, Clone, Copy)]
pub enum MainView {
    Projects(ListState),
    DueTasks(ListState),
}

impl MainView {
    pub fn new(max_index: Option<usize>) -> Self {
        Self::Projects(ListState::new(max_index))
    }

    fn list_state(&self) -> &ListState {
        let (Self::Projects(list_state) | Self::DueTasks(list_state)) = self;
        list_state
    }

    fn list_state_mut(&mut self) -> &mut ListState {
        let (Self::Projects(list_state) | Self::DueTasks(list_state)) = self;
        list_state
    }
}

fn project_title(context: Context, index: usize) -> String {
    context.state.projects()[index].title.clone()
}
