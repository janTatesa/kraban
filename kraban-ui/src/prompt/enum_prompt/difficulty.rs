use kraban_state::Difficulty;

use super::{EnumPrompt, EnumPromptMember};

impl EnumPromptMember for Difficulty {
    fn type_name() -> &'static str {
        "difficulty"
    }

    fn keyhint() -> &'static str {
        "Pick difficulty"
    }
}

pub type DifficultyPrompt = EnumPrompt<Difficulty>;
