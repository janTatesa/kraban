use chrono::{Datelike, Local};
use color_eyre::Result;
use color_eyre::eyre::ContextCompat;
use std::{cmp::Ordering, fs, path::PathBuf, str::FromStr};
use tap::Tap;
use time::Date;

pub fn compare_due_dates(first: Option<Date>, second: Option<Date>) -> Ordering {
    match (first, second) {
        (Some(first), Some(second)) => second.cmp(&first),
        _ => first.cmp(&second),
    }
}

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

pub fn get_dir(dir: Dir, is_testing: bool) -> Result<PathBuf> {
    let path = if is_testing {
        PathBuf::from_str("testing-files").unwrap()
    } else {
        match dir {
            Dir::State => dirs::state_dir().or(dirs::data_dir()), // madOS doesn't have a state dir apparently
            Dir::Config => dirs::config_dir(),
        }
        .wrap_err_with(|| format!("Cannot get OS {dir} dir"))?
        .tap_mut(|p| p.push("kraban"))
    };

    fs::create_dir_all(&path)?;
    Ok(path)
}

#[derive(strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Dir {
    State,
    Config,
}

// If the iterator is empty, return an iterator of the default value
pub struct DefaultIter<T, I: Iterator<Item = T>> {
    default: Option<T>,
    iter: I,
}

impl<T, I> Iterator for DefaultIter<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => {
                self.default = None;
                Some(item)
            }
            None => self.default.take(),
        }
    }
}

pub trait IterExt: Iterator {
    fn default(self, default: Self::Item) -> DefaultIter<Self::Item, Self>
    where
        Self: Sized,
    {
        DefaultIter {
            default: Some(default),
            iter: self,
        }
    }
}

impl<T: Iterator> IterExt for T {}
