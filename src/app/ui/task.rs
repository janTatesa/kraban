mod column;
mod component;
mod tab;
mod view;

use cli_log::error;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, ToSpan},
};
use time::{Date, OffsetDateTime};

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

    fn current_task(&self, context: Context, index: usize) -> Task {
        context
            .state
            .tasks(self.project, &self.get_current_column(context.config).name)[index]
            .clone()
    }
}

impl<'a> From<&'a Task> for Line<'a> {
    fn from(value: &'a Task) -> Self {
        let spans =
            value
                .priority
                .iter()
                .flat_map(|priority| [Span::raw("["), Span::from(*priority), Span::raw("] ")])
                .chain(value.difficulty.iter().flat_map(|difficulty| {
                    [Span::raw("["), Span::from(*difficulty), Span::raw("] ")]
                }))
                .chain(
                    value
                        .due_date
                        .iter()
                        .flat_map(|date| [Span::raw("["), date_to_span(date), Span::raw("] ")]),
                )
                .chain([Span::raw(&value.title)]);
        Line::from_iter(spans)
    }
}

fn date_to_span(date: &Date) -> Span<'_> {
    let duration = *date
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

    date.to_span().fg(color)
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
