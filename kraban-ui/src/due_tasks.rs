use std::iter;

use kraban_config::Config;
use kraban_lib::now;
use kraban_state::{DueTask, State};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Rect},
    style::Stylize,
    text::Line
};
use time::Date;

use crate::{
    keyhints::Keyhints,
    table::{Table, TableQuery, table},
    task::TasksView,
    utils::{
        DIFFICULTY_CONSTRAINT, DUE_DATE_CONSTRAINT, PRIORITY_CONSTRAINT, difficulty_to_line,
        due_date_to_line, priority_to_color, priority_to_line
    }
};

pub enum Response<'a> {
    SwitchToTasksView(TasksView<'a>),
    SwitchToProjectsView(DueTasksView),
    Update(DueTasksView)
}

#[derive(Default)]
pub struct DueTasksView(table!(DueTaskQuery));
impl DueTasksView {
    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool
    ) {
        self.0.render(area, buf, state, config, focused);
    }

    pub fn on_key<'a>(mut self, key: KeyEvent, state: &State, config: &'a Config) -> Response<'a> {
        const NONE: KeyModifiers = KeyModifiers::NONE;
        match (key.code, key.modifiers, self.0.selected(state, config)) {
            (KeyCode::Tab, NONE, _) => Response::SwitchToProjectsView(self),
            (KeyCode::Enter, NONE, Some(idx)) => {
                let selected = state.due_tasks(config).nth(idx).unwrap();
                Response::SwitchToTasksView(TasksView::with_specific_task(
                    selected.project_idx,
                    &selected.column_config.name,
                    selected.idx,
                    config
                ))
            }
            _ => {
                self.0.on_key(key);
                Response::Update(self)
            }
        }
    }
}

#[derive(Default)]
struct DueTaskQuery;
impl TableQuery<6> for DueTaskQuery {
    fn len(&self, state: &State, config: &Config) -> usize { state.due_tasks(config).count() }

    const CONSTRAINTS: [Constraint; 6] = [
        DUE_DATE_CONSTRAINT,
        Constraint::Fill(1),
        Constraint::Fill(1),
        PRIORITY_CONSTRAINT,
        DIFFICULTY_CONSTRAINT,
        Constraint::Fill(3)
    ];

    fn rows<'a>(
        &self,
        state: &'a State,
        config: &'a Config
    ) -> impl Iterator<Item = [Line<'a>; 6]> {
        let now = now();
        state
            .due_tasks(config)
            .map(move |task| due_task_rows(now, task))
    }
}

fn due_task_rows(now: Date, task: DueTask) -> [Line; 6] {
    let project_title_color = task
        .project
        .priority
        .map(priority_to_color)
        .unwrap_or_default();
    [
        due_date_to_line(task.due_date, now),
        Line::from(task.project.title.as_str()).fg(project_title_color),
        Line::from(task.column_config.name.as_str())
            .fg(task.column_config.color)
            .italic(),
        task.priority.map(priority_to_line).unwrap_or_default(),
        task.difficulty.map(difficulty_to_line).unwrap_or_default(),
        Line::from(task.title)
    ]
}

impl Keyhints for DueTasksView {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        let item_requiring_iter = self
            .0
            .keyhints(state, config)
            .into_iter()
            .chain(iter::once(("Enter", "Switch to task")));

        (self.0.len(state, config) != 0)
            .then_some(item_requiring_iter)
            .into_iter()
            .flatten()
            .chain(iter::once(("Tab", "Switch to project view")))
    }
}
