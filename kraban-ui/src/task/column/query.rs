use kraban_config::Config;
use kraban_lib::now;
use kraban_state::{State, Task};
use ratatui::{layout::Constraint, text::Line};
use time::Date;

use crate::{
    table::TableQuery,
    utils::{
        DIFFICULTY_CONSTRAINT, DUE_DATE_CONSTRAINT, PRIORITY_CONSTRAINT, difficulty_to_line,
        due_date_to_line, priority_to_line
    }
};

#[derive(Debug, Clone)]
pub struct TaskTable<'a> {
    project_idx: usize,
    column: &'a str
}

impl<'a> TaskTable<'a> {
    pub fn new(project_idx: usize, column: &'a str) -> Self {
        Self {
            project_idx,
            column
        }
    }
}

impl TableQuery<4> for TaskTable<'_> {
    fn len(&self, state: &State, _: &Config) -> usize {
        state.projects()[self.project_idx]
            .columns
            .get(self.column)
            .len()
    }

    const CONSTRAINTS: [Constraint; 4] = [
        PRIORITY_CONSTRAINT,
        DIFFICULTY_CONSTRAINT,
        DUE_DATE_CONSTRAINT,
        Constraint::Min(0)
    ];

    fn rows<'a>(&self, state: &'a State, _: &'a Config) -> impl Iterator<Item = [Line<'a>; 4]> {
        let now = now();
        state.projects()[self.project_idx]
            .columns
            .get(self.column)
            .iter()
            .map(move |task| task_row(now, task))
    }
}

fn task_row(now: Date, task: &Task) -> [Line<'_>; 4] {
    [
        task.priority().map(priority_to_line).unwrap_or_default(),
        task.difficulty.map(difficulty_to_line).unwrap_or_default(),
        task.due_date()
            .map(|date| due_date_to_line(date, now))
            .unwrap_or_default(),
        task.title.as_str().into()
    ]
}
