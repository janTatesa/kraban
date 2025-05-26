use std::cmp::Ordering;

use crate::{Action, SwitchToIndex};

use super::{Difficulty, Priority, State};
use chrono::{Days, Local};
use cli_log::debug;
use derivative::Derivative;
use kraban_config::Config;
use kraban_lib::{chrono_date_to_time_date, compare_due_dates};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::Date;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Derivative, Clone)]
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
        if !self.due_date_manually_set && config.default_due_dates.enable {
            Some(chrono_date_to_time_date(Local::now().checked_add_days(
                Days::new(match priority? {
                    Priority::Low => config.default_due_dates.low,
                    Priority::Medium => config.default_due_dates.medium,
                    Priority::High => config.default_due_dates.high,
                } as u64),
            )?))
        } else {
            self.due_date
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then(compare_due_dates(self.due_date, other.due_date))
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
    ) -> Option<SwitchToIndex> {
        let list = self.projects[project].columns.get_mut(column.to_string());
        match action {
            Action::Delete => {
                debug!("{:?}", index);
                list.remove(index?);
                None
            }
            Action::ChangePriority(priority) => Self::modifing_action(index, list, |task| Task {
                priority,
                due_date: task.due_date_by_priority(priority, config),
                ..task
            }),
            Action::ChangeDifficulty(difficulty) => {
                Self::modifing_action(index, list, |task| Task { difficulty, ..task })
            }
            Action::New(title) => Some(SwitchToIndex(list.push(Task {
                title,
                ..Task::default()
            }))),
            Action::Rename(title) => {
                Self::modifing_action(index, list, |task| Task { title, ..task })
            }
            Action::MoveToColumn(column) => {
                let task = list.remove(index?);
                self.projects[project].columns.get_mut(column).push(task);
                None
            }
            Action::SetTaskDueDate(due_date) => Self::modifing_action(index, list, |task| Task {
                due_date,
                due_date_manually_set: true,
                ..task
            }),
        }
    }
}
