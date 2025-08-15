use itertools::Itertools;
use kraban_config::{ColumnConfig, Config};
use kraban_state::{Project, State};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Rect},
    style::Stylize,
    text::{Line, Span}
};

use crate::{
    keyhints::Keyhints,
    prompt::{
        ProjectsPrompt,
        delete::ProjectDeleteConfirmation,
        input::{InputAction, InputPrompt},
        priority::PriorityPrompt
    },
    table::{Table, TableQuery, table},
    task::TasksView,
    utils::{PRIORITY_CONSTRAINT, priority_to_line}
};

#[allow(clippy::large_enum_variant)]
pub enum Response<'a> {
    OpenPrompt(ProjectsView, ProjectsPrompt),
    SwitchToTasksView(TasksView<'a>),
    SwitchToDueTasksView(ProjectsView),
    Update(ProjectsView)
}

#[derive(Default)]
pub struct ProjectsView(table!(ProjectsTableQuery));
impl Keyhints for ProjectsView {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        const ITEM_REQUIRING_KEYHINTS: [(&str, &str); 4] = [
            ("Delete/Backspace", "Delete"),
            ("p", "Set priority"),
            ("r", "Rename"),
            ("Enter", "View project tasks")
        ];

        let item_requiring_iter = self
            .0
            .keyhints(state, config)
            .into_iter()
            .chain(ITEM_REQUIRING_KEYHINTS);

        (self.0.len(state, config) != 0)
            .then_some(item_requiring_iter)
            .into_iter()
            .flatten()
            .chain([("n", "New"), ("Tab", "Switch to due tasks view")])
    }
}

impl ProjectsView {
    pub fn new(idx: usize) -> Self { Self(Table::new(idx, ProjectsTableQuery)) }
    pub fn modify_selected_project<T>(
        &self,
        f: impl FnOnce(&mut Project) -> T,
        state: &mut State,
        config: &Config
    ) -> Option<T> {
        let idx = self.0.selected(state, config)?;
        Some(state.projects_mut().modify_item_at(idx, f))
    }

    pub fn delete_selected_project(&self, state: &mut State, config: &Config) -> Option<Project> {
        let idx = self.0.selected(state, config)?;
        Some(state.projects_mut().remove(idx))
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool
    ) {
        self.0.render(area, buf, state, config, focused)
    }

    pub fn on_key<'a>(mut self, key: KeyEvent, state: &State, config: &'a Config) -> Response<'a> {
        const NONE: KeyModifiers = KeyModifiers::NONE;
        match (key.code, key.modifiers, self.0.selected(state, config)) {
            (KeyCode::Delete | KeyCode::Backspace, NONE, Some(idx)) => {
                self.prompt(ProjectDeleteConfirmation::new(idx))
            }
            (KeyCode::Char('n'), NONE, _) => {
                let placeholder = "Enter new project name".to_string();
                self.prompt(InputPrompt::new(config, InputAction::New, placeholder))
            }
            (KeyCode::Char('p'), NONE, Some(_)) => self.prompt(PriorityPrompt::new(None)),
            (KeyCode::Char('r'), NONE, Some(idx)) => {
                let text = state.projects()[idx].title.clone();
                self.prompt(InputPrompt::new(config, InputAction::Rename, text))
            }
            (KeyCode::Enter, NONE, Some(idx)) => {
                Response::SwitchToTasksView(TasksView::new(idx, config))
            }
            (KeyCode::Tab, NONE, _) => Response::SwitchToDueTasksView(self),
            _ => {
                self.0.on_key(key);
                Response::Update(self)
            }
        }
    }

    pub fn prompt<T: Into<ProjectsPrompt>>(self, prompt: T) -> Response<'static> {
        Response::OpenPrompt(self, prompt.into())
    }
}

#[derive(Default)]
struct ProjectsTableQuery;
impl TableQuery<3> for ProjectsTableQuery {
    fn rows<'a>(
        &self,
        state: &'a State,
        config: &'a Config
    ) -> impl Iterator<Item = [Line<'a>; 3]> {
        state.projects().iter().map(move |project| {
            [
                project.priority.map(priority_to_line).unwrap_or_default(),
                Line::from_iter(tasks_count_hint(config, project)),
                project.title.as_str().into()
            ]
        })
    }

    fn len(&self, state: &State, _: &Config) -> usize { state.projects().len() }
    const CONSTRAINTS: [Constraint; 3] = [
        PRIORITY_CONSTRAINT,
        Constraint::Fill(1),
        Constraint::Fill(1)
    ];
}

#[allow(unstable_name_collisions)]
fn tasks_count_hint<'a>(config: &'a Config, project: &Project) -> impl Iterator<Item = Span<'a>> {
    config
        .column_configs()
        .filter_map(|column| task_count_hint_per_column(project, column))
        .flatten()
        .intersperse(" ".into())
}

fn task_count_hint_per_column<'a>(
    project: &Project,
    column: &'a ColumnConfig
) -> Option<[Span<'a>; 2]> {
    let len = project.columns.get(&column.name).len();
    (len != 0).then_some([
        len.to_string().fg(column.color).italic(),
        Span::raw(&column.name).fg(column.color).italic()
    ])
}
