use kraban_state::Priority;

use super::{EnumPrompt, EnumPromptMember};

impl EnumPromptMember for Priority {
    fn type_name() -> &'static str {
        "priority"
    }

    fn keyhint() -> &'static str {
        "Pick priority"
    }
}

pub type PriorityPrompt = EnumPrompt<Priority>;
