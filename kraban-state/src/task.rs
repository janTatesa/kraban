use core::panic;
use std::cmp::Ordering;

use crate::{Action, ItemToCreate, ReversedSortedVec};

use super::{Difficulty, Priority, State};
use chrono::{Days, Local};
use derivative::Derivative;
use kraban_config::Config;
use kraban_lib::{date::chrono_date_to_time_date, unwrap_or_ret};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::Date;

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Derivative, Clone, Debug)]
#[skip_serializing_none]
pub struct Task {
    // Priority and difficulty should be on top so it's sorted properly
    pub priority: Option<Priority>,
    pub due_date: Option<Date>,
    pub difficulty: Option<Difficulty>,
    pub title: String,
    #[serde(default)]
    pub due_date_manually_set: bool,
}

impl Task {
    fn due_date_by_priority(&self, priority: Option<Priority>, config: &Config) -> Option<Date> {
        if self.due_date_manually_set || !config.default_due_dates.enable {
            return self.due_date;
        }

        Some(chrono_date_to_time_date(Local::now().checked_add_days(
            Days::new(match priority? {
                Priority::Low => config.default_due_dates.low,
                Priority::Medium => config.default_due_dates.medium,
                Priority::High => config.default_due_dates.high,
            } as u64),
        )?))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then(self.difficulty.cmp(&other.difficulty))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn tasks(&self, project: usize, column: &str) -> &Vec<Task> {
        self.projects[project].columns.get(column).inner()
    }

    pub(super) fn handle_task_action(
        &mut self,
        action: Action,
        project: usize,
        column: &str,
        index: Option<usize>,
        config: &Config,
    ) {
        let list = self.projects[project].columns.get_mut(column);
        match action {
            Action::Delete => _ = list.remove(unwrap_or_ret!(index)),
            Action::ChangePriority(priority) => change_priority(index, config, list, priority),
            Action::ChangeDifficulty(difficulty) => change_difficulty(index, list, difficulty),
            Action::New(ItemToCreate::Task(task)) => _ = list.push(task),
            Action::New(_) => panic!("Cannot create project when in task view"),
            Action::Rename(title) => rename(index, list, title),
            Action::MoveToColumn(column) => {
                let task = list.remove(unwrap_or_ret!(index));
                self.projects[project].columns.get_mut(column).push(task);
            }
            Action::SetTaskDueDate(due_date) => change_due_date(index, list, due_date),
        }
    }
}

fn change_due_date(
    index: Option<usize>,
    list: &mut ReversedSortedVec<Task>,
    due_date: Option<Date>,
) {
    list.map_item_at(unwrap_or_ret!(index), |task| Task {
        due_date,
        due_date_manually_set: true,
        ..task
    });
}

fn change_priority(
    index: Option<usize>,
    config: &Config,
    list: &mut ReversedSortedVec<Task>,
    priority: Option<Priority>,
) {
    list.map_item_at(unwrap_or_ret!(index), |task| Task {
        priority,
        due_date: task.due_date_by_priority(priority, config),
        ..task
    });
}

fn change_difficulty(
    index: Option<usize>,
    list: &mut ReversedSortedVec<Task>,
    difficulty: Option<Difficulty>,
) {
    list.map_item_at(unwrap_or_ret!(index), |task| Task { difficulty, ..task });
}

fn rename(index: Option<usize>, list: &mut ReversedSortedVec<Task>, title: String) {
    list.map_item_at(unwrap_or_ret!(index), |task| Task { title, ..task });
}
