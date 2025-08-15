use std::fmt::Debug;

use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Borders, Widget}
};

use crate::{due_tasks::DueTasksView, projects::ProjectsView, utils::block_widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainViewFocus {
    #[default]
    Projects,
    DueTasks
}

impl MainViewFocus {
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool,
        projects: &mut ProjectsView,
        due_tasks: &mut DueTasksView
    ) {
        const CONSTRAINTS: [Constraint; 2] =
            [Constraint::Percentage(40), Constraint::Percentage(60)];
        let layout = Layout::horizontal(CONSTRAINTS).split(area);
        let separator_block = block_widget(config).borders(Borders::RIGHT);
        let projects_area = separator_block.inner(layout[0]);
        separator_block.render(layout[0], buf);
        projects.render(
            projects_area,
            buf,
            state,
            config,
            *self == MainViewFocus::Projects && focused
        );

        due_tasks.render(
            layout[1],
            buf,
            state,
            config,
            *self == MainViewFocus::DueTasks && focused
        );
    }
}
