//TODO: Maybe this should be a submodule of project
mod defaultmap;

mod migration;
mod project;
mod sorted_vec;
mod task;

use crate::{Dir, get_dir};

use super::Action;
use cli_log::info;
use color_eyre::Result;
pub use project::Project;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sorted_vec::SortedVec;
use std::{fs, io::ErrorKind, path::PathBuf};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};
use tap::Tap;
pub use task::Task;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    projects: SortedVec<Project>,
}

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
        let state = if version == Self::BASILK_VERSION {
            value
        } else {
            value["state"].take()
        };

        Self::from_version(version, state)
    }

    const BASILK_VERSION: u64 = 0;
    // IMPORTANT: update this everytime `State` is updated incompatibly
    const CURRENT_VERSION: u64 = 1;

    #[allow(clippy::match_overlapping_arm)]
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
    ) -> Result<Option<Action>> {
        let action = match current_list {
            CurrentList::Projects(index) => self.handle_project_action(action, index),
            CurrentList::Tasks {
                project,
                column,
                index,
            } => self.handle_task_action(action, project, column, index),
        };
        Ok(action)
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

fn path_to_state_file(is_testing: bool) -> Result<PathBuf> {
    Ok(get_dir(Dir::State, is_testing)?.tap_mut(|p| p.push("tasks.json")))
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
    EnumCount,
    EnumIter,
    IntoStaticStr,
)]
pub enum Priority {
    Low,
    Medium,
    High,
}

// Idk whether tasks should be ordered easy to hard or hard to easy,
// but I currently stick to that easy is highest to do the easy stuff asap when I don't have motivation to do hard stuff
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
    EnumCount,
    EnumIter,
    IntoStaticStr,
)]
pub enum Difficulty {
    Hard,
    Normal,
    Easy,
}
