//TODO: Use defaultmap as a dependency when https://github.com/JelteF/defaultmap/issues/19 is resolved
mod defaultmap;
mod due_task;
mod migration;
mod project;
mod sorted_vec;
mod task;

use std::{fs, io::ErrorKind, path::PathBuf};

use kraban_config::Config;
use kraban_lib::{Dir, get_dir};

use cli_log::info;
use color_eyre::Result;
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};
use tap::Tap;
use time::Date;

pub use due_task::DueTask;
pub use project::Project;
pub use sorted_vec::SortedVec;
pub use task::Task;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    projects: SortedVec<Project>,
    #[serde(skip)]
    due_tasks: Option<SortedVec<DueTask>>,
}

// When a task is moved due to the change of it's properties, the ui should switch to it
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
        config: &Config,
    ) -> Result<Option<SwitchToIndex>> {
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
        Ok(switch_to_index)
    }
    fn modifing_action<T: Ord, F: FnOnce(T) -> T>(
        index: Option<usize>,
        list: &mut SortedVec<T>,
        closure: F,
    ) -> Option<SwitchToIndex> {
        index.map(|index| SwitchToIndex(list.map_item_at(index, closure)))
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

impl From<Priority> for Span<'_> {
    fn from(value: Priority) -> Self {
        let text: &str = value.into();
        Span::styled(
            text,
            Style::new().fg(match value {
                Priority::Low => Color::Green,
                Priority::Medium => Color::Yellow,
                Priority::High => Color::Red,
            }),
        )
    }
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

impl From<Option<Difficulty>> for Action {
    fn from(value: Option<Difficulty>) -> Self {
        Self::ChangeDifficulty(value)
    }
}

impl From<Option<Priority>> for Action {
    fn from(value: Option<Priority>) -> Self {
        Self::ChangePriority(value)
    }
}

impl From<Difficulty> for Span<'static> {
    fn from(value: Difficulty) -> Self {
        let str: &str = value.into();
        Span::styled(
            str,
            Style::new().fg(match value {
                Difficulty::Easy => Color::Green,
                Difficulty::Normal => Color::Yellow,
                Difficulty::Hard => Color::Red,
            }),
        )
    }
}
