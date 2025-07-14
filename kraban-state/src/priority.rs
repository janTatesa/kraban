use ratatui::{style::Color, text::Line};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

use crate::Action;

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

impl From<Priority> for Line<'_> {
    fn from(value: Priority) -> Self {
        let text: &str = value.into();
        Self::styled(text, Color::from(value))
    }
}

impl From<Priority> for Color {
    fn from(value: Priority) -> Self {
        match value {
            Priority::Low => Self::Green,
            Priority::Medium => Self::Yellow,
            Priority::High => Self::Red,
        }
    }
}

impl From<Option<Priority>> for Action<'_> {
    fn from(value: Option<Priority>) -> Self {
        Self::ChangePriority(value)
    }
}
