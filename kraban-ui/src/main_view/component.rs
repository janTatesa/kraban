use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Text},
    widgets::{Borders, StatefulWidget, Widget},
};
use tap::Tap;

use super::MainView;
use crate::{
    Action, Component, Context,
    keyhints::KeyHints,
    list::ListState,
    widgets::{block_widget, list_widget},
};

impl Component for MainView {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match self {
            MainView::Projects(_) => self.on_key_projects(key_event, context),
            MainView::DueTasks(_) => self.on_key_due_tasks(key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        self.list_state().key_hints(context).tap_mut(|v| {
            v.extend(match self {
                MainView::Projects(_) => [
                    ("Delete/Backspace", "Delete"),
                    ("n", "New"),
                    ("p", "Set priority"),
                    ("r", "Rename"),
                    ("Enter", "View project tasks"),
                ]
                .iter(),
                MainView::DueTasks(_) => [("Enter", "Switch to task")].iter(),
            });
            v.push(("Tab", "Switch between lists"));
        })
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context) {
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(area);
        let projects = self.project_list(context);
        let due_tasks = self.due_task_list(context);
        let block = block_widget(context.config).borders(Borders::RIGHT);
        let projects_area = block.inner(layout[0]);
        block.render(layout[0], buf);
        match self {
            MainView::Projects(list_state) => render_tabs(
                *list_state,
                projects,
                due_tasks,
                projects_area,
                layout[1],
                buf,
            ),
            MainView::DueTasks(list_state) => render_tabs(
                *list_state,
                due_tasks,
                projects,
                layout[1],
                projects_area,
                buf,
            ),
        }
    }
}

fn render_tabs<'a>(
    list_state: ListState,
    list: impl Iterator<Item = Line<'a>>,
    text: impl Iterator<Item = Line<'a>>,
    list_area: Rect,
    text_area: Rect,
    buf: &mut Buffer,
) {
    StatefulWidget::render(list_widget(list), list_area, buf, &mut list_state.into());
    Widget::render(Text::from_iter(text), text_area, buf);
}
