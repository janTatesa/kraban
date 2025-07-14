use kraban_state::DueTask;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Constraint,
    style::{Color, Stylize},
    text::{Line, ToLine},
};
use tap::Pipe;

use crate::{
    Context, KeyNoModifiers,
    action::{Action, switch_to_view},
    date_to_line, get,
    keyhints::KeyHints,
    no_property,
    table::{LARGE_COLUMN_SIZE, SMALL_COLUMN_SIZE, TableQuery},
    task::TasksView,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct DueTaskTable;
impl TableQuery<'_, 6> for DueTaskTable {
    fn on_key<'a>(
        &self,
        index: Option<usize>,
        key: KeyEvent,
        context: Context<'_, 'a>,
    ) -> Option<Action<'a>> {
        let due_task = get!(context, due_tasks, index?);
        match key.keycode_without_modifiers()? {
            KeyCode::Enter => switch_to_task(context, due_task),
            _ => None,
        }
    }

    fn keyhints(&self, _context: Context) -> KeyHints {
        vec![("Enter", "Switch to task")]
    }

    fn len(&self, context: Context) -> usize {
        get!(context, due_tasks).len()
    }

    fn header(&self) -> [&'static str; 6] {
        ["Due date", "Project", "Column", "Prior.", "Diffi.", "Title"]
    }

    fn constraints(&self, _context: Context) -> [Constraint; 6] {
        [
            Constraint::Length(LARGE_COLUMN_SIZE),
            Constraint::Length(LARGE_COLUMN_SIZE),
            Constraint::Length(LARGE_COLUMN_SIZE),
            Constraint::Length(SMALL_COLUMN_SIZE),
            Constraint::Length(SMALL_COLUMN_SIZE),
            Constraint::Min(0),
        ]
    }

    fn rows<'a>(&self, context: Context<'a, 'a>) -> impl Iterator<Item = [Line<'a>; 6]> {
        get!(context, due_tasks).iter().map(|task| {
            [
                task.task
                    .due_date
                    .map(date_to_line)
                    .unwrap_or(no_property()),
                task.project_title
                    .as_str()
                    .pipe(Line::raw)
                    .fg::<Color>(task.project_priority.map(|p| p.into()).unwrap_or_default()),
                task.column_name.to_line().fg(task.column_color).italic(),
                task.task.priority.map(Line::from).unwrap_or(no_property()),
                task.task
                    .difficulty
                    .map(Line::from)
                    .unwrap_or(no_property()),
                task.task.title.as_str().pipe(Line::raw),
            ]
        })
    }
}

fn switch_to_task<'a>(context: Context<'_, 'a>, due_task: &DueTask) -> Option<Action<'a>> {
    TasksView::with_specific_task(
        due_task.project_index,
        &due_task.column_name,
        due_task.index,
        context,
    )
    .pipe(switch_to_view)
}
