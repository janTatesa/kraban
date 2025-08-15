mod app;
mod cli;

use std::io::stdout;

use app::App;
use clap::Parser;
use cli_log::init_cli_log;
use color_eyre::{Result, eyre::Context};
use kraban_config::Config;
use ratatui::crossterm::{event::EnableFocusChange, execute};

use crate::cli::Cli;
fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    if cli.print_default_config {
        Config::print_default();
        return Ok(());
    }

    if cli.write_defaul_config {
        return Config::write_default();
    }

    init_cli_log!();
    let config = Config::new()?;
    let result = execute!(stdout(), EnableFocusChange)
        .wrap_err("Failed to enable focus change")
        .and(App::run(&config));
    ratatui::restore();
    result
}
