use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};
use tap::{Pipe, Tap};

use crate::{
    Component, Context,
    action::{Action, switch_to_view},
    keyhints::KeyHints,
    main_view::MainView,
};

use super::TasksView;

impl Component for TasksView {
    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::BackTab => {
                self.focused_tab = self.focused_tab.decrement();
                None
            }
            KeyCode::Tab => {
                self.focused_tab = self.focused_tab.increment();
                None
            }
            // TODO: this maybe could also switch bask to duetask list
            KeyCode::Esc => self
                .project_index
                .pipe(MainView::with_focused_project)
                .pipe(switch_to_view),
            _ => self.tabs[self.focused_tab.value()].on_key(key_event, context),
        }
    }

    fn key_hints(&self, context: Context) -> KeyHints {
        self.tabs[self.focused_tab.value()]
            .key_hints(context)
            .tap_mut(|v| {
                v.extend([
                    ("Tab/Backtab", "Switch between tabs"),
                    ("Esc", "Back to main view"),
                ])
            })
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        let tab_constraints = (0..context.config.tabs.len()).map(|tab| {
            match !context.config.collapse_unfocused_tabs || tab == self.focused_tab.value() {
                true => Constraint::Min(0),
                false => Constraint::Length(1),
            }
        });

        Layout::vertical(tab_constraints)
            .split(area)
            .iter()
            .enumerate()
            .for_each(|(tab, area)| {
                self.tabs[tab].render(
                    *area,
                    buf,
                    context,
                    tab == self.focused_tab.value() && focused,
                )
            });
    }
}
