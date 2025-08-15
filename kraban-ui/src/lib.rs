mod due_tasks;
mod keyhints;
mod list;
mod main_view;
mod projects;
mod prompt;
mod render;
mod table;
mod task;
mod utils;

use kraban_config::{AlwaysOpen, Config};
use kraban_state::{Project, SetPriority, State, Task};
use main_view::MainViewFocus;
use projects::ProjectsView;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use task::TasksView;

use crate::{
    due_tasks::DueTasksView,
    prompt::{
        ProjectsPrompt, TasksPrompt, delete,
        difficulty::{self, DifficultyPrompt},
        due_date::{self, DueDatePrompt},
        input, move_to_column,
        priority::{self, PriorityPrompt}
    }
};

pub struct Ui<'a>(UiState<'a>);
enum UiState<'a> {
    MainView(ProjectsView, DueTasksView, MainViewFocus),
    ProjectsPrompt(ProjectsView, DueTasksView, ProjectsPrompt),
    TasksView(TasksView<'a>),
    TasksPrompt(TasksView<'a>, TasksPrompt<'a>)
}

impl<'a> From<UiState<'a>> for Response<'a> {
    fn from(value: UiState<'a>) -> Self { Self::Update(Ui(value)) }
}

impl Default for Ui<'_> {
    fn default() -> Self {
        Self(UiState::MainView(
            ProjectsView::default(),
            DueTasksView::default(),
            MainViewFocus::Projects
        ))
    }
}

#[allow(clippy::large_enum_variant)]
pub enum Response<'a> {
    Quit,
    Update(Ui<'a>)
}

impl<'a> Ui<'a> {
    pub fn on_key(self, key: KeyEvent, state: &mut State, config: &'a Config) -> Response<'a> {
        match self.0 {
            _ if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL => {
                return Response::Quit
            }
            UiState::MainView(projects, due_tasks, MainViewFocus::Projects) => {
                match projects.on_key(key, state, config) {
                    projects::Response::OpenPrompt(projects, projects_prompt) => {
                        UiState::ProjectsPrompt(projects, due_tasks, projects_prompt)
                    }
                    projects::Response::SwitchToTasksView(tasks) => UiState::TasksView(tasks),
                    projects::Response::Update(projects) => {
                        UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                    }
                    projects::Response::SwitchToDueTasksView(projects) => {
                        UiState::MainView(projects, due_tasks, MainViewFocus::DueTasks)
                    }
                }
            }
            UiState::MainView(projects, due_tasks, MainViewFocus::DueTasks) => {
                match due_tasks.on_key(key, state, config) {
                    due_tasks::Response::SwitchToTasksView(tasks_view) => {
                        UiState::TasksView(tasks_view)
                    }
                    due_tasks::Response::SwitchToProjectsView(due_tasks) => {
                        UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                    }
                    due_tasks::Response::Update(due_tasks) => {
                        UiState::MainView(projects, due_tasks, MainViewFocus::DueTasks)
                    }
                }
            }
            UiState::ProjectsPrompt(projects, due_tasks, ..)
                if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE =>
            {
                UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
            }
            UiState::TasksPrompt(tasks_view, ..)
                if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE =>
            {
                UiState::TasksView(tasks_view)
            }

            UiState::ProjectsPrompt(projects, due_tasks, ProjectsPrompt::InputPrompt(prompt)) => {
                match prompt.on_key(key) {
                    input::Response::Update(prompt) => {
                        UiState::ProjectsPrompt(projects, due_tasks, prompt.into())
                    }
                    input::Response::Rename(title) => {
                        projects.modify_selected_project(
                            |project| project.title = title,
                            state,
                            config
                        );

                        UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                    }
                    input::Response::New(title) => {
                        if config.always_open.priority {
                            UiState::ProjectsPrompt(
                                projects,
                                due_tasks,
                                PriorityPrompt::new(Some(Project::new(title))).into()
                            )
                        } else {
                            state.projects_mut().push(Project::new(title));
                            UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                        }
                    }
                }
            }
            UiState::ProjectsPrompt(
                projects,
                due_tasks,
                ProjectsPrompt::PriorityPrompt(prompt)
            ) => match prompt.on_key(key, config) {
                priority::Response::Update(prompt) => {
                    UiState::ProjectsPrompt(projects, due_tasks, prompt.into())
                }
                priority::Response::ModifyCurrentlyCreatedItem(project) => {
                    state.projects_mut().push(project);
                    UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                }
                priority::Response::SetPriority(priority) => {
                    projects.modify_selected_project(
                        |project| project.priority = priority,
                        state,
                        config
                    );

                    UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                }
            },
            UiState::ProjectsPrompt(
                projects,
                due_tasks,
                ProjectsPrompt::ProjectDeleteConfirmation(prompt)
            ) => match prompt.on_key(key) {
                delete::Response::Delete => {
                    projects.delete_selected_project(state, config);
                    UiState::MainView(projects, due_tasks, MainViewFocus::Projects)
                }
                delete::Response::Update(prompt) => {
                    UiState::ProjectsPrompt(projects, due_tasks, prompt.into())
                }
            },
            UiState::TasksView(tasks_view) => match tasks_view.on_key(key, state, config) {
                task::Response::OpenPrompt(tasks_view, tasks_prompt) => {
                    UiState::TasksPrompt(tasks_view, tasks_prompt)
                }
                task::Response::SwitchToProjectsView(projects_view) => UiState::MainView(
                    projects_view,
                    DueTasksView::default(),
                    MainViewFocus::Projects
                ),
                task::Response::Update(tasks_view) => UiState::TasksView(tasks_view)
            },
            UiState::TasksPrompt(tasks_view, TasksPrompt::InputPrompt(prompt)) => {
                match prompt.on_key(key) {
                    input::Response::Update(prompt) => {
                        UiState::TasksPrompt(tasks_view, prompt.into())
                    }
                    input::Response::New(title) => match config.always_open {
                        AlwaysOpen { priority: true, .. } => UiState::TasksPrompt(
                            tasks_view,
                            PriorityPrompt::new(Some(Task::new(title))).into()
                        ),
                        AlwaysOpen {
                            difficulty: true, ..
                        } => UiState::TasksPrompt(
                            tasks_view,
                            DifficultyPrompt::new(Some(Task::new(title))).into()
                        ),
                        AlwaysOpen { due_date: true, .. } => UiState::TasksPrompt(
                            tasks_view,
                            DueDatePrompt::new(Some(Task::new(title)), None).into()
                        ),
                        _ => {
                            tasks_view.push_task(Task::new(title), state);
                            UiState::TasksView(tasks_view)
                        }
                    },
                    input::Response::Rename(title) => {
                        tasks_view.modify_selected_task(state, config, |task| task.title = title);
                        UiState::TasksView(tasks_view)
                    }
                }
            }
            UiState::TasksPrompt(tasks_view, TasksPrompt::PriorityPrompt(prompt)) => {
                match prompt.on_key(key, config) {
                    priority::Response::Update(prompt) => {
                        UiState::TasksPrompt(tasks_view, prompt.into())
                    }
                    priority::Response::ModifyCurrentlyCreatedItem(task) => {
                        match config.always_open {
                            AlwaysOpen {
                                difficulty: true, ..
                            } => UiState::TasksPrompt(
                                tasks_view,
                                DifficultyPrompt::new(Some(task)).into()
                            ),
                            AlwaysOpen { due_date: true, .. } => {
                                let old_date = task.due_date();
                                UiState::TasksPrompt(
                                    tasks_view,
                                    DueDatePrompt::new(Some(task), old_date).into()
                                )
                            }
                            _ => {
                                tasks_view.push_task(task, state);
                                UiState::TasksView(tasks_view)
                            }
                        }
                    }
                    priority::Response::SetPriority(priority) => {
                        let f = |task: &mut Task| task.set_priority(priority, config);
                        tasks_view.modify_selected_task(state, config, f);
                        UiState::TasksView(tasks_view)
                    }
                }
            }
            UiState::TasksPrompt(tasks_view, TasksPrompt::DifficultyPrompt(prompt)) => match prompt
                .on_key(key)
            {
                difficulty::Response::Update(prompt) => {
                    UiState::TasksPrompt(tasks_view, prompt.into())
                }
                difficulty::Response::ModifyCurrentlyCreatedTask(task) => {
                    if config.always_open.due_date {
                        let old_date = task.due_date();
                        let due_date_prompt = DueDatePrompt::new(Some(task), old_date);
                        UiState::TasksPrompt(tasks_view, due_date_prompt.into())
                    } else {
                        tasks_view.push_task(task, state);
                        UiState::TasksView(tasks_view)
                    }
                }
                difficulty::Response::SetDifficulty(difficulty) => {
                    tasks_view
                        .modify_selected_task(state, config, |task| task.difficulty = difficulty);
                    UiState::TasksView(tasks_view)
                }
            },
            UiState::TasksPrompt(tasks_view, TasksPrompt::DueDatePrompt(prompt)) => {
                match prompt.on_key(key) {
                    due_date::Response::Update(prompt) => {
                        UiState::TasksPrompt(tasks_view, prompt.into())
                    }
                    due_date::Response::SetDueDate(date) => {
                        tasks_view
                            .modify_selected_task(state, config, |task| task.set_due_date(date));
                        UiState::TasksView(tasks_view)
                    }
                    due_date::Response::ModifyCurrentlyCreatedTask(task) => {
                        tasks_view.push_task(task, state);
                        UiState::TasksView(tasks_view)
                    }
                }
            }
            UiState::TasksPrompt(tasks_view, TasksPrompt::MoveToColumnPrompt(prompt)) => {
                match prompt.on_key(key, config) {
                    move_to_column::Response::MoveToColumn(column) => {
                        if let Some(task) = tasks_view.delete_selected_task(state, config) {
                            let f =
                                |project: &mut Project| project.columns.get_mut(column).push(task);
                            tasks_view.modify_selected_project(state, f);
                        }

                        UiState::TasksView(tasks_view)
                    }
                    move_to_column::Response::Update(move_to_column_prompt) => {
                        UiState::TasksPrompt(tasks_view, move_to_column_prompt.into())
                    }
                }
            }
            UiState::TasksPrompt(tasks_view, TasksPrompt::TaskDeleteConfirmation(prompt)) => {
                match prompt.on_key(key) {
                    delete::Response::Delete => {
                        tasks_view.delete_selected_task(state, config);
                        UiState::TasksView(tasks_view)
                    }
                    delete::Response::Update(prompt) => {
                        UiState::TasksPrompt(tasks_view, prompt.into())
                    }
                }
            }
        }
        .into()
    }
}
