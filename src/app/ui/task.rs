mod column;
mod component;
mod tab;
mod view;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::app::{
    Context,
    state::{Difficulty, Task},
};

use super::list::{ListState, WrappingUsize};

#[derive(Default, Clone, Copy, Debug)]
pub struct TasksView {
    project: usize,
    focused_tab: WrappingUsize,
    focused_column: WrappingUsize,
    focused_task: ListState,
}

impl TasksView {
    pub fn new(project: usize, context: Context) -> Self {
        let tabs = &context.config.tabs;
        let columns = &tabs.first().unwrap().columns;
        let column = WrappingUsize::new(columns.len() - 1);
        let tab = WrappingUsize::new(tabs.len() - 1);
        Self {
            project,
            focused_tab: tab,
            focused_column: column,
            focused_task: ListState::new(
                context
                    .state
                    .tasks(project, &columns.first().unwrap().name)
                    .len()
                    .checked_sub(1),
            ),
        }
    }

    fn task_name(&self, context: Context, index: usize) -> String {
        context
            .state
            .tasks(self.project, &self.get_current_column(context.config).name)[index]
            .title
            .clone()
    }
}

impl<'a> From<&'a Task> for Line<'a> {
    fn from(value: &'a Task) -> Self {
        let mut spans = Vec::with_capacity(1);
        // TODO: extract these to separate functions
        if let Some(priority) = value.priority {
            spans.extend([Span::raw("["), Span::from(priority), Span::raw("] ")]);
        }

        if let Some(difficulty) = value.difficulty {
            spans.extend([Span::raw("["), Span::from(difficulty), Span::raw("] ")]);
        }

        spans.push(Span::raw(&value.title));
        Line::from(spans)
    }
}

impl From<Difficulty> for Span<'static> {
    fn from(value: Difficulty) -> Self {
        let str: &str = value.into();
        Span::styled(
            str,
            Style::new().fg(match value {
                Difficulty::Easy => Color::Green,
                Difficulty::Normal => Color::Yellow,
                Difficulty::Hard => Color::Red,
            }),
        )
    }
}
