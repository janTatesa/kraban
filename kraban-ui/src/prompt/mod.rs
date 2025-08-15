pub mod delete;
pub mod difficulty;
pub mod due_date;
pub mod input;
pub mod move_to_column;
pub mod priority;

use enum_dispatch::enum_dispatch;
use kraban_config::Config;
use kraban_state::{Project, State, Task};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Clear, Widget}
};

use crate::{
    prompt::{
        delete::{ProjectDeleteConfirmation, TaskDeleteConfirmation},
        difficulty::DifficultyPrompt,
        due_date::DueDatePrompt,
        input::InputPrompt,
        move_to_column::MoveToColumnPrompt,
        priority::PriorityPrompt
    },
    utils::block_widget
};

fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

#[enum_dispatch]
trait Prompt {
    fn height(&self, state: &State, config: &Config) -> u16;
    fn width(&self) -> u16 { 60 }
    fn title(&self) -> &'static str;
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config);
}

#[allow(private_bounds)]
pub fn render_prompt<T: Prompt>(
    prompt: &mut T,
    area: Rect,
    buf: &mut Buffer,
    state: &State,
    config: &Config
) {
    buf.set_style(area, Style::default().dim());
    let prompt_area = center_area(
        area,
        Constraint::Length(prompt.width() + 2),
        Constraint::Length(prompt.height(state, config) + 2)
    );

    Clear.render(prompt_area, buf);
    let title = Line::from(prompt.title()).centered();
    let block = block_widget(config).title(title);
    let inner_prompt_area = block.inner(prompt_area);
    block.render(prompt_area, buf);
    prompt.render(inner_prompt_area, buf, state, config);
}

#[allow(clippy::large_enum_variant)]
#[enum_dispatch(Prompt)]
pub enum ProjectsPrompt {
    InputPrompt,
    PriorityPrompt(PriorityPrompt<Project>),
    ProjectDeleteConfirmation
}

#[allow(clippy::large_enum_variant)]
#[enum_dispatch(Prompt)]
pub enum TasksPrompt<'a> {
    InputPrompt,
    PriorityPrompt(PriorityPrompt<Task>),
    DifficultyPrompt,
    DueDatePrompt,
    MoveToColumnPrompt(MoveToColumnPrompt<'a>),
    TaskDeleteConfirmation(TaskDeleteConfirmation<'a>)
}
