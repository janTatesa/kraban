use std::{borrow::Cow, collections::HashMap, iter};

use crate::{
    Component, Context, StateAction,
    action::{Action, state_action},
    keyhints::KeyHints,
};
use chrono::{Days, Local, Months};
use itertools::chain;
use kraban_lib::date::{ChronoDate, chrono_date_to_time_date, time_date_to_chrono_date};
use kraban_state::CurrentItem;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{
        Widget,
        calendar::{CalendarEventStore, Monthly},
    },
};
use tap::Pipe;

use super::PromptTrait;

#[derive(Debug, Clone, Copy)]
pub struct DueDatePrompt {
    old_date: Option<time::Date>,
    current_date: ChronoDate,
}

impl DueDatePrompt {
    pub fn new(old_date: Option<time::Date>) -> Self {
        let current_date = old_date
            .map(time_date_to_chrono_date)
            .unwrap_or(Local::now());
        Self {
            old_date,
            current_date,
        }
    }
}

impl PromptTrait for DueDatePrompt {
    fn height(&self, _context: Context) -> u16 {
        8 // Calendar has max 5 rows + month header + weekdays header
    }

    fn width(&self) -> u16 {
        22 // Each column has 2 characters with a space in between. Plus we add 2 for spacing
    }

    fn title(&self, _item: CurrentItem) -> Cow<'static, str> {
        "Change due date".into()
    }
}

const DAYS_IN_WEEK: u64 = 7;
impl Component<'_> for DueDatePrompt {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action<'static>> {
        self.current_date = match key_event.code {
            KeyCode::Tab => self.current_date.checked_add_months(Months::new(1)),
            KeyCode::BackTab => self.current_date.checked_sub_months(Months::new(1)),
            KeyCode::Right => self.current_date.checked_add_days(Days::new(1)),
            KeyCode::Left => self.current_date.checked_sub_days(Days::new(1)),
            KeyCode::Up => self.current_date.checked_sub_days(Days::new(DAYS_IN_WEEK)),
            KeyCode::Down => self.current_date.checked_add_days(Days::new(DAYS_IN_WEEK)),
            KeyCode::Enter => {
                return self
                    .current_date
                    .pipe(chrono_date_to_time_date)
                    .pipe(Some)
                    .pipe(StateAction::SetTaskDueDate)
                    .pipe(state_action);
            }
            KeyCode::Delete | KeyCode::Backspace => {
                return state_action(StateAction::SetTaskDueDate(None));
            }

            _ => None,
        }?;
        None
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![
            ("Delete/Backspace", "Delete due date"),
            ("Tab/Backtab", "Switch month"),
            ("Arrows", "Pick day"),
            ("Enter", "Submit"),
        ]
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, _focused: bool) {
        let selected_date = chrono_date_to_time_date(self.current_date);
        let selected_style = Style::new().fg(context.config.app_color).reversed();
        let today = chrono_date_to_time_date(Local::now());
        let today_style = Style::new().fg(Color::Green).reversed();
        let old_date_style = Style::new().fg(Color::Yellow).reversed();
        let events = chain![
            iter::once((today, today_style)),
            self.old_date.map(|old_date| (old_date, old_date_style)),
            iter::once((selected_date, selected_style)),
        ];

        let event_store = CalendarEventStore(HashMap::from_iter(events));
        Monthly::new(selected_date, event_store)
            .show_surrounding(Style::new().fg(Color::DarkGray))
            .show_weekdays_header(Style::new().fg(Color::Green).italic())
            .show_month_header(Style::new().fg(Color::Yellow).bold())
            .render(area, buf);
    }
}
