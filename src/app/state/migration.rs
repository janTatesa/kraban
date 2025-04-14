use core::panic;
use std::collections::HashMap;

use serde_json::Value;

use super::{Priority, Project, State, Task, defaultmap::DefaultMap, sorted_vec::SortedVec};
impl State {
    pub(super) fn from_basilk(mut value: Value) -> Self {
        Self {
            projects: value
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .map(process_basilk_project)
                .collect(),
        }
    }
}

fn process_basilk_project(basilk_project: &mut Value) -> Project {
    let mut columns: DefaultMap<String, SortedVec<Task>> =
        DefaultMap::new(HashMap::with_capacity(3));
    let tasks = basilk_project["tasks"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .map(process_basilk_task);
    for (column, task) in tasks {
        columns.get_mut(column.to_string()).push(task);
    }
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
            difficulty: None,
        },
    )
}
