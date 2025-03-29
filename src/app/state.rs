mod defaultmap;
mod migration;
mod sorted_vec;

use std::{fs, io::ErrorKind, path::PathBuf};

use cli_log::{debug, info};
use color_eyre::Result;
use defaultmap::DefaultMap;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sorted_vec::SortedVec;

use super::ui::Action;

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

    fn handle_task_action(
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
                Some(Action::ShrinkList)
            }
            Action::ChangePriority(priority) => Self::modifing_action(index, list, |task| Task {
                priority: Some(priority),
                ..task
            }),
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
                Some(Action::ShrinkList)
            }
            _ => None,
        }
    }

    fn handle_project_action(&mut self, action: Action, index: Option<usize>) -> Option<Action> {
        match action {
            Action::Delete => {
                index.map(|index| self.projects.remove(index));
                Some(Action::ShrinkList)
            }
            Action::ChangePriority(priority) => {
                Self::modifing_action(index, &mut self.projects, |project| Project {
                    priority: Some(priority),
                    ..project
                })
            }
            Action::New(title) => Some(Action::SwitchToIndex(self.projects.push(Project {
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

    fn modifing_action<T: Ord, F: FnOnce(T) -> T>(
        index: Option<usize>,
        list: &mut SortedVec<T>,
        closure: F,
    ) -> Option<Action> {
        index.map(|index| Action::SwitchToIndex(list.map_item_at(index, closure)))
    }

    pub fn projects(&self) -> &Vec<Project> {
        self.projects.inner()
    }

    pub fn tasks(&self, project: usize, column: &str) -> &Vec<Task> {
        self.projects[project].columns.get(column).inner()
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
    let mut path = dirs::state_dir().unwrap_or_default();
    path.push("kraban");
    fs::create_dir_all(&path)?;
    path.push("tasks.json");
    Ok(path)
}

#[derive(Derivative, Serialize, Deserialize, Default)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {
    // Priority should be on top so it's sorted properly
    pub priority: Option<Priority>,
    pub title: String,
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub columns: DefaultMap<String, Column>,
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

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    // Priority should be on top so it's sorted properly
    pub priority: Option<Priority>,
    pub title: String,
}
