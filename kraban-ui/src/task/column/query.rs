use kraban_state::CurrentItem;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Constraint, text::Line};

use crate::{
    Context, KeyNoModifiers,
    action::{Action, open_prompt},
    date_to_line, get,
    keyhints::KeyHints,
    no_property,
    prompt::{
        DeleteConfirmation, DifficultyPrompt, DueDatePrompt, InputAction, InputPrompt,
        MoveToColumnPrompt, PriorityPrompt,
    },
    table::{LARGE_COLUMN_SIZE, SMALL_COLUMN_SIZE, TableQuery},
};

#[derive(Debug, Clone)]
pub struct TaskTable<'a> {
    pub project_index: usize,
    pub column: &'a str,
    pub immutable: bool,
}

impl<'a> TableQuery<'a, 4> for TaskTable<'a> {
    fn on_key(
        &self,
        index: Option<usize>,
        key: KeyEvent,
        context: Context<'_, 'a>,
    ) -> Option<Action<'a>> {
        let key = key.keycode_without_modifiers()?;
        let current_task =
            index.map(|idx| get!(context, projects, self.project_index, self.column, idx));
        match key {
            KeyCode::Enter => open_prompt(MoveToColumnPrompt::new(self.column)),
            _ if self.immutable => None,
            KeyCode::Delete | KeyCode::Backspace => {
                open_prompt(DeleteConfirmation(CurrentItem::Task {
                    project: self.project_index,
                    column: self.column,
                    task: index,
                }))
            }
            KeyCode::Char('p') => open_prompt(PriorityPrompt::new(current_task?.priority)),
            KeyCode::Char('d') => open_prompt(DifficultyPrompt::new(current_task?.difficulty)),
            KeyCode::Char('r') => open_prompt(InputPrompt::new(
                context,
                InputAction::Rename,
                current_task?.title.clone(),
            )),
            KeyCode::Char('a') => open_prompt(DueDatePrompt::new(current_task?.due_date)),
            KeyCode::Char('n') => open_prompt(InputPrompt::new(
                context,
                InputAction::New,
                "Enter new task name".to_string(),
            )),
            _ => None,
        }
    }

    fn keyhints(&self, _context: Context) -> KeyHints {
        let mut base = vec![("Enter", "Move task to column")];
        if self.immutable {
            base.extend([
                ("Delete/Backspace", "Delete"),
                ("n", "New"),
                ("p", "Set priority"),
                ("d", "Set difficulty"),
                ("r", "Rename"),
                ("a", "Add due date"),
            ]);
        }
        base
    }

    fn len(&self, context: Context) -> usize {
        get!(context, projects, self.project_index, self.column).len()
    }

    fn header(&self) -> [&'static str; 4] {
        ["Prior.", "Diffi.", "Due date", "Name"]
    }

    fn constraints(&self, _context: Context) -> [Constraint; 4] {
        [
            Constraint::Length(SMALL_COLUMN_SIZE),
            Constraint::Length(SMALL_COLUMN_SIZE),
            Constraint::Length(LARGE_COLUMN_SIZE),
            Constraint::Min(0),
        ]
    }

    fn rows<'b>(&self, context: Context<'b, 'b>) -> impl Iterator<Item = [Line<'b>; 4]> {
        get!(context, projects, self.project_index, self.column)
            .iter()
            .map(|task| {
                [
                    task.priority
                        .map(|priority| priority.into())
                        .unwrap_or(no_property()),
                    task.difficulty
                        .map(|difficulty| difficulty.into())
                        .unwrap_or(no_property()),
                    task.due_date.map(date_to_line).unwrap_or(no_property()),
                    task.title.as_str().into(),
                ]
            })
    }
}
