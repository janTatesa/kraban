use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::app::Context;

use super::TasksView;
use crate::app::ui::{
    Action, Component, Item,
    keyhints::KeyHints,
    list::{ListState, WrappingUsize},
    open_prompt,
    project::ProjectsView,
    prompt::{
        ChangePriorityPrompt, DeleteConfirmation, InputAction, InputPrompt, MoveToColumnPrompt,
    },
};

impl TasksView {
    fn on_key_mutable(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Delete => self.focused_task.focused_item().and_then(|index| {
                open_prompt(DeleteConfirmation {
                    name: self.task_name(context, index),
                    item: Item::Task,
                })
            }),
            KeyCode::Char('n') => open_prompt(InputPrompt::new(
                context,
                InputAction::New,
                "Enter new task name".to_string(),
            )),
            KeyCode::Char('p') => self
                .focused_task
                .focused_item()
                .and_then(|_| open_prompt(ChangePriorityPrompt::new())),
            KeyCode::Char('r') => self.focused_task.focused_item().and_then(|index| {
                open_prompt(InputPrompt::new(
                    context,
                    InputAction::Rename,
                    self.task_name(context, index),
                ))
            }),
            _ => self.focused_task.on_key(key_event, context),
        }
    }

    fn reset_focused_task(&mut self, context: Context) {
        self.focused_task = ListState::new(self.get_current_column_len(context).checked_sub(1));
    }

    fn reset_focused_column(&mut self, context: Context) {
        self.focused_column = WrappingUsize::new(
            context.config.tabs[usize::from(self.focused_tab)]
                .columns
                .len()
                - 1,
        );
        self.reset_focused_task(context);
    }
}

impl Component for TasksView {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Left => {
                self.focused_column = self.focused_column.decrement();
                self.reset_focused_task(context);
                None
            }
            KeyCode::Right => {
                self.focused_column = self.focused_column.increment();
                self.reset_focused_task(context);
                None
            }
            KeyCode::BackTab => {
                self.focused_tab = self.focused_tab.decrement();
                self.reset_focused_column(context);
                self.focused_task =
                    ListState::new(self.get_current_column_len(context).checked_sub(1));
                None
            }
            KeyCode::Tab => {
                self.focused_tab = self.focused_tab.increment();
                self.reset_focused_column(context);
                None
            }
            KeyCode::Esc => Some(Action::SwitchToView(Box::new(ProjectsView::new(
                context.state.projects().len().checked_sub(1),
            )))),
            KeyCode::Enter => self.focused_task.focused_item().map(|_| {
                Action::OpenPrompt(Box::new(MoveToColumnPrompt::new(
                    context,
                    self.get_current_column(context.config).name.clone(),
                )))
            }),
            _ if !self.get_current_column(context.config).immutable => {
                self.on_key_mutable(key_event, context)
            }
            _ => self.focused_task.on_key(key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        let mut base = vec![
            ("Tab/Backtab", "Switch between tabs"),
            ("Left/Right", "Switch between columns"),
            ("Esc", "Back to projects view"),
            ("Enter", "Move task to column"),
        ];

        if !self.get_current_column(context.config).immutable {
            base.extend([
                ("Delete", "Delete"),
                ("n", "New"),
                ("p", "Set priority"),
                ("r", "Rename"),
            ]);
        }
        base
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context) {
        let tab_constraints = (0..context.config.tabs.len()).map(|tab| {
            if !context.config.collapse_unfocused_tabs || tab == usize::from(self.focused_tab) {
                Constraint::Min(0)
            } else {
                Constraint::Length(1)
            }
        });
        Layout::vertical(tab_constraints)
            .split(area)
            .iter()
            .enumerate()
            .for_each(|(tab, area)| self.render_tab(*area, buf, context, tab));
    }
}
