use chrono::{Datelike, Local};
use cli_log::error;
use time::OffsetDateTime;

pub type ChronoDate = chrono::DateTime<Local>;
// hate that there's two crates which do not fully implement my use case but whathever
pub fn chrono_date_to_time_date(chrono_date: ChronoDate) -> time::Date {
    let year = chrono_date.year();
    let month = time::Month::December.nth_next(chrono_date.month() as u8);
    let day = chrono_date.day();
    time::Date::from_calendar_date(year, month, day as u8).unwrap()
}

pub fn time_date_to_chrono_date(time_date: time::Date) -> ChronoDate {
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

pub fn now() -> time::Date {
    OffsetDateTime::now_local()
        .inspect_err(|e| error!("Failed to get local timezone {e}, using utc"))
        .unwrap_or(OffsetDateTime::now_utc())
        .date()
}
