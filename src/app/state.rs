//TODO: Maybe this should be a submodule of project
mod defaultmap;

mod migration;
mod project;
mod sorted_vec;
mod task;

use super::ui::Action;
use cli_log::info;
use color_eyre::Result;
pub use project::Project;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sorted_vec::SortedVec;
use std::{fs, io::ErrorKind, path::PathBuf};
pub use task::Task;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    projects: SortedVec<Project>,
}

impl State {
    pub fn new() -> Result<Self> {
        let mut value: Value = serde_json::from_str(
            match fs::read_to_string(path_to_state_file()?) {
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
        let state = if version == Self::BASILK_VERSION {
            value
        } else {
            value["state"].take()
        };

        Self::from_version(version, state)
    }

    const BASILK_VERSION: u64 = 0;
    // IMPORTANT: update this everytime `State` is updated
    const CURRENT_VERSION: u64 = 1;

    fn from_version(version: u64, value: Value) -> Result<Self> {
        info!(
            "Json version {version}, latest version {}",
            Self::CURRENT_VERSION
        );
        match version {
            Self::BASILK_VERSION => Ok(Self::from_basilk(value)),
            Self::CURRENT_VERSION => Ok(serde_json::from_value(value)?),
            Self::CURRENT_VERSION.. => unreachable!(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let json = json!({"version": Self::CURRENT_VERSION, "state": self});
        fs::write(path_to_state_file()?, serde_json::to_string(&json)?)?;
        Ok(())
    }

    pub fn handle_action(&mut self, current_list: CurrentList, action: Action) -> Option<Action> {
        match current_list {
            CurrentList::Projects(index) => self.handle_project_action(action, index),
            CurrentList::Tasks {
                project,
                column,
                index,
            } => self.handle_task_action(action, project, column, index),
        }
    }

    fn modifing_action<T: Ord, F: FnOnce(T) -> T>(
        index: Option<usize>,
        list: &mut SortedVec<T>,
        closure: F,
    ) -> Option<Action> {
        index.map(|index| Action::SwitchToIndex(list.map_item_at(index, closure)))
    }
}

pub enum CurrentList<'a> {
    Projects(Option<usize>),
    Tasks {
        project: usize,
        column: &'a str,
        index: Option<usize>,
    },
}

fn path_to_state_file() -> Result<PathBuf> {
    let mut path = dirs::state_dir().or(dirs::data_dir()).unwrap_or_default();
    path.push("kraban");
    fs::create_dir_all(&path)?;
    path.push("tasks.json");
    Ok(path)
}

type Column = SortedVec<Task>;

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub enum Priority {
    Low,
    Medium,
    High,
}
