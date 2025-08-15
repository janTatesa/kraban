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

use color_eyre::Result;
pub use difficulty::Difficulty;
pub use due_task::DueTask;
use kraban_config::Config;
use kraban_lib::{Dir, get_dir};
pub use priority::{Priority, SetPriority};
pub use project::Project;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
pub use sorted_vec::ReversedSortedVec;
pub use task::Task;

use crate::defaultmap::DefaultMap;

pub type Columns = DefaultMap<String, Column>;
pub type Column = ReversedSortedVec<Task>;
pub type Projects = ReversedSortedVec<Project>;
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct State {
    projects: Projects,
    #[serde(skip)]
    should_save: bool
}

impl State {
    pub fn new(config: &Config) -> Result<Self> {
        let mut value: Value = serde_json::from_str(
            match fs::read_to_string(path()?) {
                Ok(contents) => contents,
                Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::default()),
                error => error?
            }
            .as_str()
        )?;

        if value.is_null() {
            return Ok(Self::default());
        }

        let version = value["version"].as_u64().unwrap_or(Self::BASILK_VERSION);
        let state = match version {
            Self::BASILK_VERSION => value,
            _ => value["state"].take()
        };

        Self::from_version(version, state, config)
    }

    pub fn save_if_needed(&mut self) -> Result<()> {
        if self.should_save {
            let json = json!({"version": Self::CURRENT_VERSION, "state": self});
            let contents = serde_json::to_string(&json)?;
            let path = path()?;
            fs::write(path, contents)?;
            self.should_save = false;
        }

        Ok(())
    }

    pub fn projects(&self) -> &Projects { &self.projects }
    pub fn projects_mut(&mut self) -> &mut Projects {
        self.should_save = true;
        &mut self.projects
    }
}

#[derive(strum_macros::AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum CurrentItem<'a> {
    Project(Option<usize>),
    DueTask(Option<usize>),
    Task {
        project: usize,
        column: &'a str,
        task: Option<usize>
    }
}

fn path() -> Result<PathBuf> {
    let mut path = get_dir(Dir::State)?;
    path.push("tasks.json");
    Ok(path)
}
