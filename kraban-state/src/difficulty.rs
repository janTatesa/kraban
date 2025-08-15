use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

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
    Easy
}
