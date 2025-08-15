mod query;
mod render;

use std::iter;

use itertools::chain;
use kraban_config::{ColumnConfig, Config};
use kraban_state::{Column, Project, State, Task};
use query::TaskTable;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    style::Color
};

use crate::{
    TasksPrompt,
    keyhints::Keyhints,
    prompt::{
        delete::TaskDeleteConfirmation,
        difficulty::DifficultyPrompt,
        due_date::DueDatePrompt,
        input::{InputAction, InputPrompt},
        move_to_column::MoveToColumnPrompt,
        priority::PriorityPrompt
    },
    table::{Table, TableQuery, table}
};

pub struct ColumnView<'a> {
    color: Color,
    immutable: bool,
    column: &'a str,
    project_idx: usize,
    table: table!(TaskTable<'a>)
}

impl<'a> ColumnView<'a> {
    pub fn new(project_idx: usize, column: &'a ColumnConfig, task: usize) -> Self {
        let immutable = column.done_column;
        let color = column.color;
        let column = &column.name;
        let table = Table::new(task, TaskTable::new(project_idx, column));

        Self {
            color,
            immutable,
            table,
            project_idx,
            column
        }
    }

    fn modify_selected_column<T>(&self, state: &mut State, f: impl FnOnce(&mut Column) -> T) -> T {
        let f = |project: &mut Project| f(project.columns.get_mut(self.column));
        state.projects_mut().modify_item_at(self.project_idx, f)
    }

    pub fn push_task(&self, task: Task, state: &mut State) {
        self.modify_selected_column(state, |column| column.push(task));
    }

    pub fn modify_selected_task<T>(
        &self,
        state: &mut State,
        config: &Config,
        f: impl FnOnce(&mut Task) -> T
    ) -> Option<T> {
        let idx = self.table.selected(state, config)?;
        Some(self.modify_selected_column(state, |column| column.modify_item_at(idx, f)))
    }

    pub fn delete_selected_task(&self, state: &mut State, config: &Config) -> Option<Task> {
        let idx = self.table.selected(state, config)?;
        Some(self.modify_selected_column(state, |column| column.remove(idx)))
    }
}

impl<'a> ColumnView<'a> {
    pub fn on_key(
        &mut self,
        key: KeyEvent,
        state: &State,
        config: &Config
    ) -> Option<TasksPrompt<'a>> {
        let KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            ..
        } = key
        else {
            self.table.on_key(key);
            return None;
        };

        let task_idx = self.table.selected(state, config);
        let current_task =
            task_idx.map(|idx| &state.projects()[self.project_idx].columns.get(self.column)[idx]);

        match (code, current_task) {
            (KeyCode::Char('n'), _) => {
                let placeholder = "Enter new task name".to_owned();
                prompt(InputPrompt::new(config, InputAction::New, placeholder))
            }
            (KeyCode::Enter, Some(_)) => prompt(MoveToColumnPrompt::new(self.column)),
            _ if self.immutable => None,
            (KeyCode::Delete | KeyCode::Backspace, Some(_)) => prompt(TaskDeleteConfirmation::new(
                self.project_idx,
                self.column,
                task_idx?
            )),
            (KeyCode::Char('p'), Some(_)) => prompt(PriorityPrompt::new(None)),
            (KeyCode::Char('d'), Some(_)) => prompt(DifficultyPrompt::new(None)),
            (KeyCode::Char('r'), Some(current_task)) => prompt(InputPrompt::new(
                config,
                InputAction::Rename,
                current_task.title.clone()
            )),
            (KeyCode::Char('a'), Some(current_task)) => {
                prompt(DueDatePrompt::new(None, current_task.due_date()))
            }
            _ => {
                self.table.on_key(key);
                None
            }
        }
    }
}

fn prompt<'a, T: Into<TasksPrompt<'a>>>(prompt: T) -> Option<TasksPrompt<'a>> {
    Some(prompt.into())
}

impl Keyhints for ColumnView<'_> {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        const TASK_REQUIRING_MUTABLE_KEYHINTS: [(&str, &str); 5] = [
            ("Delete/Backspace", "Delete"),
            ("p", "Set priority"),
            ("d", "Set difficulty"),
            ("r", "Rename"),
            ("a", "Add due date")
        ];

        let iter_requiring_task = chain!(
            iter::once(("Enter", "Move task to column")),
            (!self.immutable)
                .then_some(TASK_REQUIRING_MUTABLE_KEYHINTS)
                .into_iter()
                .flatten(),
            self.table.keyhints(state, config)
        );

        chain![
            (!self.immutable).then_some(("n", "New")),
            (self.table.len(state, config) != 0)
                .then_some(iter_requiring_task)
                .into_iter()
                .flatten()
        ]
    }
}
