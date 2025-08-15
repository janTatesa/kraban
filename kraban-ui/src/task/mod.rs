mod column;
mod tab;

use kraban_config::Config;
use kraban_lib::WrappingUsize;
use kraban_state::{Project, State, Task};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect}
};
use tab::TabView;

use crate::{TasksPrompt, keyhints::Keyhints, projects::ProjectsView};

pub struct TasksView<'a> {
    project_idx: usize,
    tabs: Vec<TabView<'a>>,
    focused_tab: WrappingUsize
}

#[allow(clippy::large_enum_variant)]
pub enum Response<'a> {
    OpenPrompt(TasksView<'a>, TasksPrompt<'a>),
    SwitchToProjectsView(ProjectsView),
    Update(TasksView<'a>)
}

impl<'a> TasksView<'a> {
    pub fn new(project_idx: usize, config: &'a Config) -> Self {
        let tab_configs = &config.tabs;
        let focused_tab = WrappingUsize::new(tab_configs.len() - 1);
        let tabs = tab_configs
            .iter()
            .enumerate()
            .map(|(idx, tab)| TabView::new(project_idx, idx, tab))
            .collect();

        Self {
            project_idx,
            focused_tab,
            tabs
        }
    }

    pub fn on_key(mut self, key: KeyEvent, state: &State, config: &Config) -> Response<'a> {
        const NONE: KeyModifiers = KeyModifiers::NONE;
        match (key.code, key.modifiers) {
            (KeyCode::Esc, NONE) => {
                return Response::SwitchToProjectsView(ProjectsView::new(self.project_idx))
            }
            (KeyCode::Tab, NONE) => self.focused_tab.increment(),
            (KeyCode::BackTab, NONE) => self.focused_tab.decrement(),
            _ => {
                if let Some(prompt) = self.tabs[*self.focused_tab].on_key(key, state, config) {
                    return Response::OpenPrompt(self, prompt)
                }
            }
        }

        Response::Update(self)
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &State,
        config: &Config,
        focused: bool
    ) {
        let constraints = (0..config.tabs.len()).map(|tab| self.tab_constraint(config, tab));
        for (tab, area) in Layout::vertical(constraints).split(area).iter().enumerate() {
            let focused = tab == *self.focused_tab && focused;
            self.tabs[tab].render(*area, buf, state, config, focused)
        }
    }

    fn tab_constraint(&mut self, config: &Config, tab: usize) -> Constraint {
        if !config.collapse_unfocused_tabs || tab == *self.focused_tab {
            Constraint::Min(0)
        } else {
            Constraint::Length(1)
        }
    }

    pub fn with_specific_task(
        project: usize,
        column_name: &str,
        task: usize,
        config: &'a Config
    ) -> Self {
        let tabs = &config.tabs;
        let (focused_tab, column) = tabs
            .iter()
            .enumerate()
            .find_map(|(focused_tab, tab)| Some((focused_tab, tab.get_column_idx(column_name)?)))
            .unwrap();

        let tabs: Vec<TabView> = tabs
            .iter()
            .enumerate()
            .map(|(idx, tab)| TabView::with_column_and_task(project, idx, tab, column, task))
            .collect();

        Self {
            project_idx: project,
            focused_tab: WrappingUsize::new_with_value(tabs.len() - 1, focused_tab),
            tabs
        }
    }

    pub fn push_task(&self, task: Task, state: &mut State) {
        self.tabs[*self.focused_tab].push_task(task, state)
    }

    pub fn modify_selected_task<T>(
        &self,
        state: &mut State,
        config: &Config,
        f: impl FnOnce(&mut Task) -> T
    ) -> Option<T> {
        self.tabs[*self.focused_tab].modify_selected_task(state, config, f)
    }

    pub fn delete_selected_task(&self, state: &mut State, config: &Config) -> Option<Task> {
        self.tabs[*self.focused_tab].delete_selected_task(state, config)
    }

    pub fn modify_selected_project<T>(
        &self,
        state: &mut State,
        f: impl FnOnce(&mut Project) -> T
    ) -> T {
        state.projects_mut().modify_item_at(self.project_idx, f)
    }
}

impl Keyhints for TasksView<'_> {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        [
            ("Tab/Backtab", "Switch between tabs"),
            ("Esc", "Back to main view")
        ]
        .into_iter()
        .chain(self.tabs[*self.focused_tab].keyhints(state, config))
    }
}
