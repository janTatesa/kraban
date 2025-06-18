use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Borders, Widget},
};
use tap::Tap;

use super::{FocusedList, MainView};
use crate::{
    Component, Context, KeyNoModifiers, action::Action, keyhints::KeyHints, widgets::block_widget,
};

impl Component for MainView {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        if let Some(KeyCode::Tab) = key_event.keycode_without_modifiers() {
            self.focused_list = match self.focused_list {
                FocusedList::Projects => FocusedList::DueTasks,
                FocusedList::DueTasks => FocusedList::Projects,
            };

            return None;
        }

        match self.focused_list {
            FocusedList::Projects => self.projects.on_key(key_event, context),
            FocusedList::DueTasks => self.due_tasks.on_key(key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        match self.focused_list {
            FocusedList::Projects => self.projects.key_hints(context),
            FocusedList::DueTasks => self.due_tasks.key_hints(context),
        }
        .tap_mut(|v| v.push(("Tab", "Switch between lists")))
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let layout = Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);
        let separator_block = block_widget(context.config).borders(Borders::RIGHT);
        let projects_area = separator_block.inner(layout[0]);
        separator_block.render(layout[0], buf);
        self.projects.render(
            projects_area,
            buf,
            context,
            self.focused_list == FocusedList::Projects && focused,
        );

        self.due_tasks.render(
            layout[1],
            buf,
            context,
            self.focused_list == FocusedList::DueTasks && focused,
        );
    }
}
