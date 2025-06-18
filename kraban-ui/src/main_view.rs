mod component;
mod due_tasks;
mod projects;
mod view;

use std::fmt::Debug;

use due_tasks::DueTaskList;
use projects::ProjectsTable;

use crate::table::{Table, TableQuery, table};

#[derive(Debug, Clone, Copy, Default)]
pub struct MainView {
    projects: table!(ProjectsTable),
    due_tasks: table!(DueTaskList),
    focused_list: FocusedList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum FocusedList {
    #[default]
    Projects,
    DueTasks,
}

impl MainView {
    pub fn with_focused_project(project: usize) -> Self {
        Self {
            projects: Table::with_default_index(project, ProjectsTable),
            due_tasks: Table::default(),
            focused_list: FocusedList::Projects,
        }
    }
}
