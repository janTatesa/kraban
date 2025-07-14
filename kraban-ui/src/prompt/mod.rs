mod delete;
mod due_date;
mod enum_prompt;
mod input;
mod move_to_column;

use std::borrow::Cow;

use crate::{Context, action::Action, keyhints::KeyHints};

use super::Component;
pub use delete::DeleteConfirmation;
pub use due_date::DueDatePrompt;
use enum_dispatch::enum_dispatch;
pub use enum_prompt::{difficulty::DifficultyPrompt, priority::PriorityPrompt};
pub use input::{InputAction, InputPrompt};
use kraban_state::CurrentItem;
pub use move_to_column::MoveToColumnPrompt;
use ratatui::crossterm::event::KeyEvent;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
};

pub fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

const DEFAULT_WIDTH: u16 = 60;
#[enum_dispatch]
pub(crate) trait PromptTrait {
    fn height(&self, context: Context) -> u16;
    fn width(&self) -> u16;
    fn title(&self, item: CurrentItem) -> Cow<'static, str>;
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[enum_dispatch(PromptTrait, Component)]
#[derive(Debug)]
pub(crate) enum Prompt<'a> {
    DeleteConfirmation(DeleteConfirmation<'a>),
    DueDatePrompt,
    PriorityPrompt,
    DifficultyPrompt,
    InputPrompt,
    MoveToColumn(MoveToColumnPrompt<'a>),
}
