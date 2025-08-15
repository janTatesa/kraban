use std::iter;

use itertools::chain;
use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Borders, Widget}
};

use crate::{Ui, UiState, main_view::MainViewFocus, prompt::render_prompt, utils::block_widget};
impl<'a> Ui<'a> {
    fn render_view(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        match &mut self.0 {
            UiState::MainView(projects, due_tasks, main_view) => {
                main_view.render(area, buf, state, config, true, projects, due_tasks);
            }
            UiState::ProjectsPrompt(projects, due_tasks, projects_prompt) => {
                MainViewFocus::Projects
                    .render(area, buf, state, config, false, projects, due_tasks);
                buf.set_style(area, Style::new().dim());
                render_prompt(projects_prompt, area, buf, state, config);
            }
            UiState::TasksView(tasks_view) => tasks_view.render(area, buf, state, config, true),
            UiState::TasksPrompt(tasks_view, tasks_prompt) => {
                tasks_view.render(area, buf, state, config, false);
                buf.set_style(area, Style::new().dim());
                render_prompt(tasks_prompt, area, buf, state, config);
            }
        };
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        let extra_hints = chain![
            iter::once(("Ctrl-q", "Quit")),
            matches!(
                self.0,
                UiState::ProjectsPrompt(..) | UiState::TasksPrompt(..),
            )
            .then_some(("Esc", "Exit prompt"))
        ];

        let keyhints = config
            .show_key_hints
            .then(|| self.keyhints(extra_hints, area.width, state, config));
        let main_area = match keyhints {
            Some(keyhints) => {
                let layout: [_; 2] = Layout::vertical([
                    Constraint::Min(0),
                    Constraint::Length(keyhints.lines.len() as u16)
                ])
                .areas(area);
                keyhints.render(layout[1], buf);
                layout[0]
            }
            None => area
        };

        let main_block = block_widget(config)
            .title(
                concat!("kraban v", env!("CARGO_PKG_VERSION"))
                    .fg(config.app_color)
                    .into_centered_line()
            )
            .borders(Borders::TOP | Borders::BOTTOM);
        let view_area = main_block.inner(main_area);
        main_block.render(main_area, buf);
        self.render_view(view_area, buf, state, config);
    }
}
