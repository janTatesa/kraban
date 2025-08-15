use kraban_config::Config;
use kraban_state::{Difficulty, Priority};
use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders}
};
use time::Date;

pub const fn priority_to_color(priority: Priority) -> Color {
    match priority {
        Priority::Low => Color::Green,
        Priority::Medium => Color::Yellow,
        Priority::High => Color::Red
    }
}

pub fn priority_to_line<'a>(priority: Priority) -> Line<'a> {
    let text = match priority {
        Priority::Low => "!",
        Priority::Medium => "!!",
        Priority::High => "!!!"
    };

    Line::raw(text).fg(priority_to_color(priority))
}

pub fn difficulty_to_line<'a>(difficulty: Difficulty) -> Line<'a> {
    let color = difficulty_to_color(difficulty);

    let text = match difficulty {
        Difficulty::Hard => "***",
        Difficulty::Normal => "**",
        Difficulty::Easy => "*"
    };

    Line::raw(text).fg(color)
}

pub fn difficulty_to_color(difficulty: Difficulty) -> Color {
    match difficulty {
        Difficulty::Hard => Color::Red,
        Difficulty::Normal => Color::Yellow,
        Difficulty::Easy => Color::Green
    }
}

pub fn due_date_to_line(due_date: Date, now: Date) -> Line<'static> {
    let duration = due_date - now;
    let color = match duration.whole_days() {
        ..0 => Color::Red,
        0 => Color::Yellow,
        1..7 => Color::Green,
        7..30 => Color::Blue,
        _ => Color::Magenta
    };

    due_date.to_string().fg(color).underlined().into()
}

pub fn block_widget(config: &Config) -> Block<'static> {
    Block::new()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(config.app_color))
        .borders(Borders::all())
}

pub const PRIORITY_CONSTRAINT: Constraint = Constraint::Length(3);
pub const DIFFICULTY_CONSTRAINT: Constraint = Constraint::Length(3);
pub const DUE_DATE_CONSTRAINT: Constraint = Constraint::Length(10);
