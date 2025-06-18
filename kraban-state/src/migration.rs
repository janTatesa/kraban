use core::panic;

use cli_log::info;
use color_eyre::eyre::Result;
use hashbrown::HashMap;
use serde_json::Value;

use super::{Priority, Project, State, Task, defaultmap::DefaultMap, sorted_vec::ReversedSortedVec};
impl State {
    pub const BASILK_VERSION: u64 = 0;
    // IMPORTANT: update this everytime `State` is updated incompatibly
    pub const CURRENT_VERSION: u64 = 1;

    #[allow(clippy::match_overlapping_arm)]
    pub(super) fn from_version(version: u64, value: Value) -> Result<Self> {
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

    pub(super) fn from_basilk(mut value: Value) -> Self {
        Self {
            projects: value
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .map(process_basilk_project)
                .collect(),
            ..Self::default()
        }
    }
}

fn process_basilk_project(basilk_project: &mut Value) -> Project {
    let mut columns: DefaultMap<String, ReversedSortedVec<Task>> =
        DefaultMap::new(HashMap::with_capacity(3));
    basilk_project["tasks"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .map(process_basilk_task)
        .for_each(|(column, task)| {
            columns.get_mut(column).push(task);
        });
    Project {
        title: basilk_project["title"].as_str().unwrap().to_string(),
        columns,
        ..Project::default()
    }
}

fn process_basilk_task(basilk_task: &mut Value) -> (&str, Task) {
    (
        match basilk_task["status"].as_str().unwrap() {
            "UpNext" => "Backlog",
            "OnGoing" => "Doing",
            "Done" => "Done",
            _ => panic!(),
        },
        Task {
            title: basilk_task["title"].as_str().unwrap().to_string(),
            priority: match basilk_task["priority"].as_u64().unwrap() {
                0 => None,
                1 => Some(Priority::Low),
                2 => Some(Priority::Medium),
                3 => Some(Priority::High),
                _ => panic!(),
            },
            ..Task::default()
        },
    )
}
