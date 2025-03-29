use app::App;
use clap::{command, ArgMatches};
use cli_log::init_cli_log;
mod app;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_cli_log!();
    let terminal = ratatui::init();
    let cli = cli();
    let result = App::run(terminal, cli);
    ratatui::restore();
    result
}

fn cli() -> ArgMatches {
    command!().get_matches()
}
