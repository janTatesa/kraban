use std::borrow::Cow;

use kraban_state::CurrentItem;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

use crate::{
    Component, Context, StateAction,
    action::{Action, state_action},
    get,
    keyhints::KeyHints,
};

use super::{DEFAULT_WIDTH, PromptTrait};

#[derive(Debug)]
pub struct DeleteConfirmation<'a>(pub CurrentItem<'a>);

impl PromptTrait for DeleteConfirmation<'_> {
    fn height(&self, _context: Context) -> u16 {
        1
    }

    fn title(&self, item: CurrentItem) -> Cow<'static, str> {
        let item: &str = item.as_ref();
        format!("Delete {item}").into()
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH
    }
}

impl Component<'_> for DeleteConfirmation<'_> {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action<'static>> {
        match key_event.code {
            KeyCode::Char('y' | 'Y') | KeyCode::Enter => state_action(StateAction::Delete),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Y/y/Enter", "Confirm")]
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, _focused: bool) {
        Line::from_iter([
            "Are you sure to delete ".into(),
            Span::raw::<&str>(self.0.as_ref()),
            " ".into(),
            Span::styled(
                match self.0 {
                    CurrentItem::Project(idx) => &get!(context, projects, idx.unwrap()).title,
                    CurrentItem::DueTask(idx) => &get!(context, due_tasks, idx.unwrap()).task.title,
                    CurrentItem::Task {
                        project,
                        column,
                        task,
                    } => &get!(context, projects, project, column, task.unwrap()).title,
                },
                Style::new().fg(context.config.app_color),
            ),
            "?".into(),
        ])
        .render(area, buf);
    }
}
