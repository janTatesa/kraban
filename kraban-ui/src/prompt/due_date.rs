use std::{collections::HashMap, iter};

use chrono::{Days, Local, Months};
use itertools::chain;
use kraban_config::Config;
use kraban_lib::{ChronoDate, chrono_date_to_time_date, time_date_to_chrono_date};
use kraban_state::{State, Task};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{
        Widget,
        calendar::{CalendarEventStore, Monthly}
    }
};

use super::Prompt;
use crate::keyhints::Keyhints;

pub struct DueDatePrompt {
    old_date: Option<time::Date>,
    currently_creating: Option<Task>,
    current_date: ChronoDate
}

const DAYS_IN_WEEK: u64 = 7;
const WEEK: Days = Days::new(DAYS_IN_WEEK);

pub enum Response {
    Update(DueDatePrompt),
    SetDueDate(Option<time::Date>),
    ModifyCurrentlyCreatedTask(Task)
}

impl DueDatePrompt {
    pub fn new(currently_creating: Option<Task>, old_date: Option<time::Date>) -> Self {
        let current_date = old_date
            .map(time_date_to_chrono_date)
            .unwrap_or(Local::now());
        Self {
            old_date,
            current_date,
            currently_creating
        }
    }

    pub fn on_key(mut self, key: KeyEvent) -> Response {
        if key.modifiers == KeyModifiers::NONE {
            self.current_date = match key.code {
                KeyCode::Tab => self.current_date.checked_add_months(Months::new(1)),
                KeyCode::BackTab => self.current_date.checked_sub_months(Months::new(1)),
                KeyCode::Right => self.current_date.checked_add_days(Days::new(1)),
                KeyCode::Left => self.current_date.checked_sub_days(Days::new(1)),
                KeyCode::Up => self.current_date.checked_sub_days(WEEK),
                KeyCode::Down => self.current_date.checked_add_days(WEEK),
                KeyCode::Enter => {
                    let date = chrono_date_to_time_date(self.current_date);
                    return Self::due_date_selected(self.currently_creating, Some(date));
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    return Self::due_date_selected(self.currently_creating, None)
                }
                _ => None
            }
            .unwrap_or(self.current_date);
        }

        Response::Update(self)
    }

    fn due_date_selected(
        currently_creating: Option<Task>,
        due_date: Option<time::Date>
    ) -> Response {
        match currently_creating {
            Some(mut task) => {
                task.set_due_date(due_date);
                Response::ModifyCurrentlyCreatedTask(task)
            }
            None => Response::SetDueDate(due_date)
        }
    }
}

impl Prompt for DueDatePrompt {
    fn height(&self, _: &State, _: &Config) -> u16 {
        const HEADERS: u16 = 2;
        const MAX_ROWS: u16 = 6;
        const HEIGHT: u16 = HEADERS + MAX_ROWS;
        HEIGHT
    }

    fn width(&self) -> u16 {
        const CHARACTERS_PER_COLUMN: u16 = 2;
        const SPACING: u16 = 2;
        const WIDTH: u16 = (DAYS_IN_WEEK as u16 * (CHARACTERS_PER_COLUMN + 1)) - 1 + SPACING;
        WIDTH
    }

    fn title(&self) -> &'static str { "Change due date" }
    fn render(&mut self, area: Rect, buf: &mut Buffer, _: &State, config: &Config) {
        let selected_date = chrono_date_to_time_date(self.current_date);
        let selected_style = Style::new().fg(config.app_color).reversed();
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

impl Keyhints for DueDatePrompt {
    fn keyhints(&self, _: &State, _: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        [
            ("Delete/Backspace", "Delete due date"),
            ("Tab/Backtab", "Switch month"),
            ("Arrows", "Pick day"),
            ("Enter", "Submit")
        ]
    }
}
