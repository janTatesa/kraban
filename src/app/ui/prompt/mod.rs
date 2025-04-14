mod delete;
mod enum_prompt;
mod input;
mod move_to_column;

pub use delete::DeleteConfirmation;
pub use enum_prompt::EnumPrompt;
pub use input::{InputAction, InputPrompt};
pub use move_to_column::MoveToColumnPrompt;

use super::{Component, Item};

pub trait Prompt: Component {
    fn height(&self) -> u16;
    fn title(&self, item: Item) -> String;
}
