use clap::{crate_name, crate_version};
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::config::Config;

pub fn block_widget(config: &Config) -> Block<'static> {
    Block::new()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(config.app_color))
        .borders(Borders::all())
}

pub fn list_widget<'a, T>(items: T) -> List<'a>
where
    T: IntoIterator,
    T::Item: Into<ListItem<'a>>,
{
    List::new(items)
        .highlight_style(Style::new().bold().on_black())
        .highlight_symbol(">")
}

pub(super) fn main_block(config: &Config) -> Block<'static> {
    block_widget(config).title(
        format!("{} v{}", crate_name!(), crate_version!())
            .fg(config.app_color)
            .into_centered_line(),
    )
}
