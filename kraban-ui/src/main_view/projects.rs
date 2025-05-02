use std::iter;

use crossterm::event::{KeyCode, KeyEvent};
use kraban_lib::IterExt;
use kraban_state::Priority;
use ratatui::{
    style::Stylize,
    text::{Line, ToSpan},
};

use crate::{
    Action, Component, Context, Item, item_trait,
    list::ListState,
    open_prompt,
    prompt::{DeleteConfirmation, EnumPrompt, InputAction, InputPrompt},
    switch_to_view,
    task::TasksView,
};

use super::{MainView, project_title};

impl MainView {
    pub(super) fn on_key_projects(
        &mut self,
        key_event: KeyEvent,
        context: Context,
    ) -> Option<Action> {
        match key_event.code {
            KeyCode::Tab => {
                *self = MainView::DueTasks(ListState::new(
                    context.state.due_tasks().len().checked_sub(1),
                ));
                None
            }
            KeyCode::Delete | KeyCode::Backspace => {
                self.list_state_mut().focused_item().and_then(|index| {
                    open_prompt(DeleteConfirmation {
                        name: project_title(context, index),
                        item: Item::Project,
                    })
                })
            }
            KeyCode::Char('n') => open_prompt(InputPrompt::new(
                context,
                InputAction::New,
                "Enter new project name".to_string(),
            )),
            // TODO: This is both in task and project and therefore violates DRY, fix that
            KeyCode::Char('p') => self.list_state_mut().focused_item().and_then(|index| {
                open_prompt({
                    let priority_prompt: EnumPrompt<Priority> =
                        EnumPrompt::new(context.state.projects()[index].priority);
                    priority_prompt
                })
            }),
            KeyCode::Char('r') => self.list_state_mut().focused_item().and_then(|index| {
                open_prompt(InputPrompt::new(
                    context,
                    InputAction::Rename,
                    project_title(context, index),
                ))
            }),
            KeyCode::Enter => self
                .list_state_mut()
                .focused_item()
                .and_then(|project| switch_to_view(TasksView::new(project, context))),
            _ => self.list_state_mut().on_key(key_event, context),
        }
    }

    pub(super) fn project_list<'a>(&self, context: Context<'a>) -> impl Iterator<Item = Line<'a>> {
        let projects = 0..context.state.projects().len();
        projects.map(move |index| display_project(index, context))
    }
}

fn display_project(index: usize, context: Context) -> Line<'_> {
    let project = &context.state.projects()[index];

    //TODO: simplify this
    //This will use intersperse when stabilised
    let column_numbers = context
        .config
        .columns
        .iter()
        .filter_map(|column| {
            let len = project.columns.get(&column.name).len();
            len.ne(&0).then(|| {
                [
                    " ".to_span(),
                    len.to_string().fg(column.color).italic(),
                    " ".to_span(),
                    column.name.clone().fg(column.color).italic(),
                ]
            })
        })
        .flatten()
        .skip(1)
        .default("No tasks".italic());

    let spans = project
        .priority
        .into_iter()
        .flat_map(item_trait)
        .chain(iter::once("[".dark_gray()))
        .chain(column_numbers)
        .chain(iter::once("] ".dark_gray()))
        .chain(iter::once(project.title.to_span()));
    Line::from_iter(spans)
}
