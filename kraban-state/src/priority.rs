use kraban_config::Config;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

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
    High
}

pub trait SetPriority {
    fn set_priority(&mut self, priority: Option<Priority>, config: &Config);
}
