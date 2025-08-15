use std::cmp::Ordering;

use chrono::{Days, Local};
use kraban_config::Config;
use kraban_lib::chrono_date_to_time_date;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::Date;

use super::{Difficulty, Priority};
use crate::priority::SetPriority;

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
#[skip_serializing_none]
pub struct Task {
    priority: Option<Priority>,
    due_date: Option<Date>,
    pub difficulty: Option<Difficulty>,
    pub title: String,
    #[serde(default)]
    due_date_manually_set: bool
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            ..Self::default()
        }
    }

    pub fn priority(&self) -> Option<Priority> { self.priority }
    pub fn due_date(&self) -> Option<Date> { self.due_date }
    pub fn set_due_date(&mut self, due_date: Option<Date>) {
        self.due_date = due_date;
        self.due_date_manually_set = true;
    }
}

impl SetPriority for Task {
    fn set_priority(&mut self, priority: Option<Priority>, config: &Config) {
        self.priority = priority;
        if !self.due_date_manually_set {
            self.due_date = priority
                .map(|priority| match priority {
                    Priority::Low => config.default_due_dates.low,
                    Priority::Medium => config.default_due_dates.medium,
                    Priority::High => config.default_due_dates.high
                } as u64)
                .map(Days::new)
                .and_then(|days| Local::now().checked_add_days(days))
                .map(chrono_date_to_time_date)
        }
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
