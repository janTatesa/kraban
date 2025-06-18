mod delete;
mod due_date;
mod enum_prompt;
mod input;
mod move_to_column;

use std::borrow::Cow;

use crate::{Context, ViewTrait, action::Action, keyhints::KeyHints};

use super::{Component, Item, Ui, widgets::block_widget};
use crossterm::event::KeyEvent;
pub use delete::DeleteConfirmation;
pub use due_date::DueDatePrompt;
use enum_dispatch::enum_dispatch;
pub use enum_prompt::{difficulty::DifficultyPrompt, priority::PriorityPrompt};
pub use input::{InputAction, InputPrompt};
pub use move_to_column::MoveToColumnPrompt;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{Clear, Widget},
};

fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl Ui {
    pub(crate) fn render_prompt(
        &self,
        area: Rect,
        buf: &mut Buffer,
        prompt: &Prompt,
        context: Context,
    ) {
        let prompt_area = center_area(
            area,
            Constraint::Length(prompt.width() + 2),
            Constraint::Length(prompt.height(context) + 2),
        );

        Clear.render(prompt_area, buf);
        let block = block_widget(context.config)
            .title(Line::from(prompt.title(self.view.item())).centered());
        let inner_prompt_area = block.inner(prompt_area);
        block.render(prompt_area, buf);
        prompt.render(inner_prompt_area, buf, context, true);
    }
}

const DEFAULT_WIDTH: u16 = 60;
#[enum_dispatch]
pub(crate) trait PromptTrait {
    fn height(&self, context: Context) -> u16;
    fn width(&self) -> u16;
    fn title(&self, item: Item) -> Cow<'static, str>;
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[enum_dispatch(PromptTrait, Component)]
#[derive(Debug)]
pub(crate) enum Prompt {
    DeleteConfirmation,
    DueDatePrompt,
    PriorityPrompt,
    DifficultyPrompt,
    InputPrompt,
    MoveToColumnPrompt,
}
