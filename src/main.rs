mod app;
mod cli;

use std::{fs, io::stdout, path::PathBuf, str::FromStr};

use app::{
    App,
    config::{print_default_config, write_default_config},
};
use cli::cli;
use cli_log::init_cli_log;
use color_eyre::{
    Result,
    eyre::{Context, ContextCompat},
};
use crossterm::{event::EnableFocusChange, execute};
use tap::Tap;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = cli();
    let is_testing = *cli.get_one("testing").expect("Option has default value");
    if *cli
        .get_one("print_default_config")
        .expect("Option has default value")
    {
        print_default_config();
        return Ok(());
    }
    if *cli
        .get_one("write_default_config")
        .expect("Option has default value")
    {
        return write_default_config(is_testing);
    };
    init_cli_log!();
    let terminal = ratatui::init();
    let result = execute!(stdout(), EnableFocusChange)
        .wrap_err("Failed to enable focus change")
        .and(App::run(terminal, cli));
    ratatui::restore();
    result
}

fn get_dir(dir: Dir, is_testing: bool) -> Result<PathBuf> {
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
enum Dir {
    State,
    Config,
}
