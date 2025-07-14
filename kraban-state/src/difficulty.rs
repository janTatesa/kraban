use ratatui::{style::Color, text::Line};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

use crate::Action;

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

impl From<Difficulty> for Line<'static> {
    fn from(value: Difficulty) -> Self {
        let str: &str = value.into();
        Self::styled(
            str,
            match value {
                Difficulty::Easy => Color::Green,
                Difficulty::Normal => Color::Yellow,
                Difficulty::Hard => Color::Red,
            },
        )
    }
}

impl From<Option<Difficulty>> for Action<'_> {
    fn from(value: Option<Difficulty>) -> Self {
        Self::ChangeDifficulty(value)
    }
}
