use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::Widget
};

use super::Prompt;
use crate::keyhints::Keyhints;

pub struct TaskDeleteConfirmation<'a> {
    project_idx: usize,
    column: &'a str,
    task_idx: usize
}

pub enum Response<T> {
    Delete,
    Update(T)
}

impl<'a> TaskDeleteConfirmation<'a> {
    pub fn new(project_idx: usize, column: &'a str, task_idx: usize) -> Self {
        Self {
            project_idx,
            column,
            task_idx
        }
    }

    pub fn on_key(self, key: KeyEvent) -> Response<Self> {
        if let KeyEvent {
            code: KeyCode::Enter | KeyCode::Char('Y' | 'y'),
            modifiers: KeyModifiers::NONE,
            ..
        } = key
        {
            return Response::Delete;
        }

        Response::Update(self)
    }
}

pub struct ProjectDeleteConfirmation {
    project_idx: usize
}

impl ProjectDeleteConfirmation {
    pub fn new(project_idx: usize) -> Self { Self { project_idx } }
    pub fn on_key(self, key: KeyEvent) -> Response<Self> {
        if let KeyEvent {
            code: KeyCode::Enter | KeyCode::Char('Y') | KeyCode::Char('y'),
            modifiers: KeyModifiers::NONE,
            ..
        } = key
        {
            return Response::Delete;
        }

        Response::Update(self)
    }
}

impl Prompt for TaskDeleteConfirmation<'_> {
    fn height(&self, _: &State, _: &Config) -> u16 { 1 }
    fn title(&self) -> &'static str { "Delete task" }
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        let task_name = state.projects()[self.project_idx].columns.get(self.column)[self.task_idx]
            .title
            .as_str()
            .fg(config.app_color)
            .italic();

        let spans = ["Are you sure to delete task ".into(), task_name, "?".into()];
        Line::from_iter(spans).render(area, buf);
    }
}

impl Prompt for ProjectDeleteConfirmation {
    fn height(&self, _: &State, _: &Config) -> u16 { 1 }
    fn title(&self) -> &'static str { "Delete project" }
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        let project_name = state.projects()[self.project_idx]
            .title
            .as_str()
            .fg(config.app_color)
            .italic();

        let spans = [
            "Are you sure to delete project ".into(),
            project_name,
            "?".into()
        ];

        Line::from_iter(spans).render(area, buf);
    }
}

const KEYHINTS: [(&str, &str); 1] = [("Y/y/Enter", "Confirm")];
impl Keyhints for TaskDeleteConfirmation<'_> {
    fn keyhints(&self, _: &State, _: &Config) -> impl IntoIterator<Item = (&str, &str)> { KEYHINTS }
}

impl Keyhints for ProjectDeleteConfirmation {
    fn keyhints(&self, _: &State, _: &Config) -> impl IntoIterator<Item = (&str, &str)> { KEYHINTS }
}
