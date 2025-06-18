//TODO: Use defaultmap as a dependency when https://github.com/JelteF/defaultmap/issues/19 is resolved
mod defaultmap;
mod difficulty;
mod due_task;
mod migration;
mod priority;
mod project;
mod sorted_vec;
mod task;

use std::{fs, io::ErrorKind, path::PathBuf};

use kraban_config::Config;
use kraban_lib::dir::{Dir, get_dir};

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tap::Tap;
use time::Date;

pub use difficulty::Difficulty;
pub use due_task::DueTask;
pub use priority::Priority;
pub use project::Project;
pub use sorted_vec::ReversedSortedVec;
pub use task::Task;

type Column = ReversedSortedVec<Task>;
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct State {
    projects: ReversedSortedVec<Project>,
    due_tasks: Option<ReversedSortedVec<DueTask>>,
}

// When a task/project is created, the ui should switch to it
pub struct SwitchToIndex(pub usize);

impl State {
    pub fn new(is_testing: bool) -> Result<Self> {
        let mut value: Value = serde_json::from_str(
            match fs::read_to_string(path_to_state_file(is_testing)?) {
                Ok(contents) => contents,
                Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::default()),
                error => error?,
            }
            .as_str(),
        )?;
        if value.is_null() {
            return Ok(Self::default());
        }
        let version = value["version"].as_u64().unwrap_or(Self::BASILK_VERSION);
        let state = match version {
            Self::BASILK_VERSION => value,
            _ => value["state"].take(),
        };

        Self::from_version(version, state)
    }

    pub fn save(&self, is_testing: bool) -> Result<()> {
        let json = json!({"version": Self::CURRENT_VERSION, "state": self});
        fs::write(
            path_to_state_file(is_testing)?,
            serde_json::to_string(&json)?,
        )?;
        Ok(())
    }

    pub fn handle_action(
        &mut self,
        current_list: CurrentList,
        action: Action,
        config: &Config,
    ) -> Option<SwitchToIndex> {
        let switch_to_index = match current_list {
            CurrentList::Projects(index) => self.handle_project_action(action, index),
            CurrentList::Tasks {
                project,
                column,
                index,
            } => self.handle_task_action(action, project, column, index, config),
            // TODO: Due task list should have actions like normal task list does
            CurrentList::DueTasks(_) => None,
        };

        // The list might have been changed
        // TODO: Set it to none only when it actually changes
        self.due_tasks = None;
        switch_to_index
    }
}

pub enum CurrentList<'a> {
    Projects(Option<usize>),
    DueTasks(Option<usize>),
    Tasks {
        project: usize,
        column: &'a str,
        index: Option<usize>,
    },
}

fn path_to_state_file(is_testing: bool) -> Result<PathBuf> {
    Ok(get_dir(Dir::State, is_testing)?.tap_mut(|p| p.push("tasks.json")))
}

#[derive(Debug)]
pub enum Action {
    Delete,
    ChangePriority(Option<Priority>),
    ChangeDifficulty(Option<Difficulty>),
    New(String),
    Rename(String),
    MoveToColumn(String),
    SetTaskDueDate(Option<Date>),
}
