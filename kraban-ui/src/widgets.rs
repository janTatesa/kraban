use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders},
};

use kraban_config::Config;

pub fn block_widget(config: &Config) -> Block<'static> {
    Block::new()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(config.app_color))
        .borders(Borders::all())
}

pub(super) fn main_block(config: &Config) -> Block<'static> {
    block_widget(config)
        .title(
            concat!("kraban v", env!("CARGO_PKG_VERSION"))
                .fg(config.app_color)
                .into_centered_line(),
        )
        .borders(Borders::TOP | Borders::BOTTOM)
}
