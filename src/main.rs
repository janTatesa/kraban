mod app;

use app::App;
use clap::{ArgMatches, command};
use cli_log::init_cli_log;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_cli_log!();
    let cli = cli();
    let terminal = ratatui::init();
    let result = App::run(terminal, cli);
    ratatui::restore();
    result
}

fn cli() -> ArgMatches {
    command!().get_matches()
}
