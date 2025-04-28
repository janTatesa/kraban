use chrono::{Datelike, Days, Local, Months};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{
        Widget,
        calendar::{CalendarEventStore, Monthly},
    },
};
use tap::Tap;

use crate::app::{
    Action, Context,
    ui::{Component, Item, keyhints::KeyHints},
};

use super::Prompt;

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

impl Prompt for DueDatePrompt {
    fn height(&self) -> u16 {
        8 // Calendar has max 5 rows + month header + weekdays header
    }

    fn width(&self) -> u16 {
        22 // Each column has 2 characters with a space in between. Plus we add 2 for spacing
    }

    fn title(&self, _item: Item) -> String {
        "Change due date".to_string()
    }
}

const DAYS_IN_WEEK: u64 = 7;
impl Component for DueDatePrompt {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        self.current_date = match key_event.code {
            KeyCode::Tab => self.current_date.checked_add_months(Months::new(1)),
            KeyCode::BackTab => self.current_date.checked_sub_months(Months::new(1)),
            KeyCode::Right => self.current_date.checked_add_days(Days::new(1)),
            KeyCode::Left => self.current_date.checked_sub_days(Days::new(1)),
            KeyCode::Up => self.current_date.checked_sub_days(Days::new(DAYS_IN_WEEK)),
            KeyCode::Down => self.current_date.checked_add_days(Days::new(DAYS_IN_WEEK)),
            KeyCode::Enter => {
                return Some(Action::SetTaskDueDate(chrono_date_to_time_date(
                    self.current_date,
                )));
            }
            _ => None,
        }?;
        None
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![
            ("Tab/Backtab", "Switch month"),
            ("Arrows", "Pick day"),
            ("Enter", "Submit"),
        ]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context) {
        let date = chrono_date_to_time_date(self.current_date);
        let selected_style = Style::new().fg(context.config.app_color).reversed();
        let today_style = Style::new().fg(Color::Green).reversed();
        let old_style = Style::new().fg(Color::Yellow).reversed();
        let event_store = CalendarEventStore::default().tap_mut(|c| {
            c.add(chrono_date_to_time_date(Local::now()), today_style);
            if let Some(date) = self.old_date {
                c.add(date, old_style)
            }
            c.add(date, selected_style);
        });

        Monthly::new(date, event_store)
            .show_surrounding(Style::new().fg(Color::DarkGray))
            .show_weekdays_header(Style::new().fg(Color::Green).italic())
            .show_month_header(Style::new().fg(Color::Yellow).bold())
            .render(area, buf);
    }
}

type ChronoDate = chrono::DateTime<Local>;
// hate that there's two crates which do not fully implement my use case but whathever
fn chrono_date_to_time_date(chrono_date: ChronoDate) -> time::Date {
    let year = chrono_date.year();
    let month = time::Month::December.nth_next(chrono_date.month() as u8);
    let day = chrono_date.day();
    time::Date::from_calendar_date(year, month, day as u8).unwrap()
}

fn time_date_to_chrono_date(time_date: time::Date) -> ChronoDate {
    let year = time_date.year();
    let month = time_date.month() as u32;
    let day = time_date.day() as u32;
    ChronoDate::default()
        .with_year(year)
        .unwrap()
        .with_month(month)
        .unwrap()
        .with_day(day)
        .unwrap()
}
