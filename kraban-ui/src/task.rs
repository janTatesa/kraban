mod column;
mod component;
mod tab;
mod view;

use std::iter;

use cli_log::error;
use kraban_state::Task;
use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};
use time::{Date, OffsetDateTime};

use crate::Context;

use super::{
    item_trait,
    list::{ListState, WrappingUsize},
};

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

    pub fn with_specific_task(
        project: usize,
        column_name: &str,
        task: usize,
        context: Context,
    ) -> Self {
        let tabs = &context.config.tabs;

        let (tab_number, column_number) = tabs
            .iter()
            .enumerate()
            .find_map(|(tab_number, tab)| {
                Some((
                    tab_number,
                    tab.columns
                        .iter()
                        .position(|column| column.name == column_name)?,
                ))
            })
            .unwrap();

        Self {
            project,
            focused_tab: WrappingUsize::with_value(tab_number, tabs.len() - 1),
            focused_column: WrappingUsize::with_value(
                column_number,
                tabs[tab_number].columns.len() - 1,
            ),
            focused_task: ListState::with_default_index(
                task,
                context.state.projects()[project]
                    .columns
                    .get(column_name)
                    .len()
                    - 1,
            ),
        }
    }

    fn current_task(&self, context: Context, index: usize) -> Task {
        context
            .state
            .tasks(self.project, &self.get_current_column(context.config).name)[index]
            .clone()
    }
}

fn display_task(task: &Task) -> Line<'_> {
    let spans = task
        .priority
        .into_iter()
        .flat_map(item_trait)
        .chain(task.difficulty.into_iter().flat_map(item_trait))
        .chain(
            task.due_date
                .into_iter()
                .map(date_to_span)
                .flat_map(item_trait),
        )
        .chain(iter::once(task.title.clone().into()));
    Line::from_iter(spans)
}

pub fn date_to_span(date: Date) -> Span<'static> {
    let duration = date
        - OffsetDateTime::now_local()
            .inspect(|e| error!("Failed to get local timezone using utc {e}"))
            .unwrap_or(OffsetDateTime::now_utc())
            .date();
    let color = match duration.whole_days() {
        ..0 => Color::Red,
        0 => Color::Yellow,
        1..7 => Color::Green,
        7..30 => Color::Blue,
        _ => Color::Magenta,
    };

    date.to_string().fg(color).underlined()
}
