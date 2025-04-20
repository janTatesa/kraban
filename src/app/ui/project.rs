use std::fmt::Debug;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Span, ToSpan},
    widgets::StatefulWidget,
};
use tap::Tap;

use crate::app::{
    Context,
    config::Config,
    state::{CurrentList, Priority},
};

use super::{
    Action, Component, Item, View,
    keyhints::KeyHints,
    list::ListState,
    open_prompt,
    prompt::{DeleteConfirmation, EnumPrompt, InputAction, InputPrompt},
    task::TasksView,
    widgets::list_widget,
};

#[derive(Debug, Clone, Copy)]
pub struct ProjectsView(ListState);

impl ProjectsView {
    pub fn new(max_index: Option<usize>) -> Self {
        Self(ListState::new(max_index))
    }
}

impl View for ProjectsView {
    fn item(&self) -> Item {
        Item::Project
    }

    fn current_list<'a>(&self, _config: &'a Config) -> CurrentList<'a> {
        CurrentList::Projects(self.0.focused_item())
    }

    fn refresh_on_state_change(&mut self, context: Context) {
        self.0
            .update_max_index(context.state.projects().len().checked_sub(1));
    }

    fn switch_to_index(&mut self, index: usize) {
        self.0.switch_to_index(index);
    }
}

fn project_title(context: Context, index: usize) -> String {
    context.state.projects()[index].title.clone()
}

impl Component for ProjectsView {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Delete => self.0.focused_item().and_then(|index| {
                open_prompt(DeleteConfirmation {
                    name: project_title(context, index),
                    item: Item::Project,
                })
            }),
            KeyCode::Char('n') => open_prompt(InputPrompt::new(
                context,
                InputAction::New,
                "Enter new project name".to_string(),
            )),
            // TODO: This is both in task and project and therefore violates DRY, fix that
            KeyCode::Char('p') => self.0.focused_item().and_then(|_| {
                open_prompt({
                    let priority_prompt: EnumPrompt<Priority> = EnumPrompt::new();
                    priority_prompt
                })
            }),
            KeyCode::Char('r') => self.0.focused_item().and_then(|index| {
                open_prompt(InputPrompt::new(
                    context,
                    InputAction::Rename,
                    project_title(context, index),
                ))
            }),
            KeyCode::Enter => self
                .0
                .focused_item()
                .map(|project| Action::SwitchToView(Box::new(TasksView::new(project, context)))),
            _ => self.0.on_key(key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        self.0.key_hints(context).tap_mut(|v| {
            v.extend([
                ("Delete", "Delete"),
                ("n", "New"),
                ("p", "Set priority"),
                ("r", "Rename"),
                ("Enter", "View project tasks"),
            ])
        })
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context) {
        let items =
            (0..context.state.projects().len()).map(|index| display_project(index, context));
        let list = list_widget(items);
        list.render(area, buf, &mut self.0.into());
    }
}

fn display_project(index: usize, context: Context) -> Line {
    let project = &context.state.projects()[index];
    let mut spans = Vec::with_capacity(1);
    if let Some(priority) = project.priority {
        spans.extend([Span::raw("["), Span::from(priority), Span::raw("] ")]);
    }
    let column_numbers: Vec<Span> = ["[".to_span()]
        .into_iter()
        .chain(
            context
                .config
                .columns
                .iter()
                .filter_map(|column| {
                    let len = project.columns.get(&column.name).len();
                    if len == 0 {
                        None
                    } else {
                        Some([
                            len.to_string().fg(column.color).bold(),
                            column.name.clone().fg(column.color).bold(),
                        ])
                    }
                })
                .flatten()
                .flat_map(|span| [" ".to_span(), span])
                .skip(1),
        )
        .chain(["] ".to_span()])
        .collect();
    if column_numbers.len() > 2 {
        spans.extend(column_numbers);
    }
    spans.push(project.title.to_span());
    Line::from(spans)
}
