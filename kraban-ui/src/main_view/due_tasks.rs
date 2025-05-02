use std::iter;

use crossterm::event::{KeyCode, KeyEvent};
use kraban_lib::IterExt;
use kraban_state::DueTask;
use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};

use crate::{
    Action, Component, Context, item_trait,
    list::ListState,
    switch_to_view,
    task::{TasksView, date_to_span},
};

use super::MainView;

impl MainView {
    pub(super) fn on_key_due_tasks(
        &mut self,
        key_event: KeyEvent,
        context: Context,
    ) -> Option<Action> {
        let due_task =
            &context.state.due_tasks().inner()[self.list_state().focused_item().unwrap()];
        match key_event.code {
            KeyCode::Enter => switch_to_view(TasksView::with_specific_task(
                context
                    .state
                    .projects()
                    .iter()
                    .enumerate()
                    .find_map(|(index, project)| {
                        (project.title == due_task.project_title).then_some(index)
                    })
                    .unwrap(),
                &due_task.column_name,
                due_task.index,
                context,
            )),
            KeyCode::Tab => {
                *self = MainView::Projects(ListState::new(
                    context.state.projects().len().checked_sub(1),
                ));
                None
            }
            _ => self.list_state_mut().on_key(key_event, context),
        }
    }

    pub(super) fn due_task_list<'a>(&self, context: Context<'a>) -> impl Iterator<Item = Line<'a>> {
        context
            .state
            .due_tasks()
            .inner()
            .iter()
            .map(|task| due_task_to_line(task, context.config.app_color))
            .default("You have no scheduled tasks".italic().into())
    }
}

fn due_task_to_line(task: &DueTask, app_color: Color) -> Line<'_> {
    Line::from(
        task.task
            .due_date
            .into_iter()
            .map(date_to_span)
            .flat_map(item_trait)
            .chain(item_trait(task.project_title.clone().fg(app_color)))
            .chain(item_trait(
                Span::raw(task.column_name.clone())
                    .fg(task.column_color)
                    .italic(),
            ))
            .chain(task.task.priority.into_iter().flat_map(item_trait))
            .chain(task.task.difficulty.into_iter().flat_map(item_trait))
            .chain(iter::once(Span::raw(task.task.title.clone())))
            .collect::<Vec<Span<'_>>>(),
    )
}
