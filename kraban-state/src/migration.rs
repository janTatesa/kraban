use core::panic;

use cli_log::info;
use color_eyre::eyre::Result;
use kraban_config::Config;
use serde_json::Value;

use super::{
    Priority, Project, State, Task, defaultmap::DefaultMap, sorted_vec::ReversedSortedVec
};
use crate::SetPriority;

impl State {
    pub const BASILK_VERSION: u64 = 0;
    // IMPORTANT: update this everytime `State` is updated incompatibly
    pub const CURRENT_VERSION: u64 = 1;

    #[allow(clippy::match_overlapping_arm)]
    pub(super) fn from_version(version: u64, value: Value, config: &Config) -> Result<Self> {
        info!(
            "Json version {version}, latest version {}",
            Self::CURRENT_VERSION
        );

        match version {
            Self::BASILK_VERSION => Ok(Self::from_basilk(value, config)),
            Self::CURRENT_VERSION => Ok(serde_json::from_value(value)?),
            Self::CURRENT_VERSION.. => unreachable!()
        }
    }

    pub(super) fn from_basilk(mut value: Value, config: &Config) -> Self {
        let projects = value
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .map(|basilk_project| process_basilk_project(basilk_project, config))
            .collect();
        Self {
            projects,
            should_save: false
        }
    }
}

fn process_basilk_project(basilk_project: &mut Value, config: &Config) -> Project {
    let mut columns: DefaultMap<String, ReversedSortedVec<Task>> = DefaultMap::with_capacity(3);
    basilk_project["tasks"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .map(|basilk_task| process_basilk_task(basilk_task, config))
        .for_each(|(column, task)| _ = columns.get_mut(column).push(task));

    let title = basilk_project["title"].as_str().unwrap().to_string();
    Project {
        title,
        columns,
        ..Project::default()
    }
}

fn process_basilk_task(basilk_task: &mut Value, config: &Config) -> (&'static str, Task) {
    // TODO: return result instead of panicking
    let mut task = Task::new(basilk_task["title"].as_str().unwrap().to_string());
    task.set_priority(
        match basilk_task["priority"].as_u64().unwrap() {
            0 => None,
            1 => Some(Priority::Low),
            2 => Some(Priority::Medium),
            3 => Some(Priority::High),
            _ => panic!()
        },
        config
    );

    let column_name = match basilk_task["status"].as_str().unwrap() {
        "UpNext" => "Backlog",
        "OnGoing" => "Doing",
        "Done" => "Done",
        _ => panic!("Not valid basilk state")
    };

    (column_name, task)
}
