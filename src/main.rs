mod app;
mod cli;

use std::{fs, panic, path::PathBuf, str::FromStr};

use app::{App, print_default_config, write_default_config};
use cli::cli;
use cli_log::init_cli_log;
use color_eyre::{
    Result,
    eyre::{ContextCompat, eyre},
};
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
    }
    let running_file_path = check_if_running(is_testing)?;
    fs::write(&running_file_path, "")?;
    set_panic_hook(running_file_path.clone());
    init_cli_log!();
    let terminal = ratatui::init();
    let result = App::run(terminal, cli);
    fs::remove_file(running_file_path)?;
    ratatui::restore();
    result
}

fn set_panic_hook(running_file_path: PathBuf) {
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        fs::remove_file(&running_file_path).unwrap();
        prev(info)
    }));
}

const ALREADY_RUNNING: &str = "Kraban is already running. It currently doesn't support multiple sessions due to conflicts (however it can have a normal and testing session simultaneously). If this is an error remove the file";
fn check_if_running(is_testing: bool) -> Result<PathBuf> {
    let running_file_path = get_dir(Dir::State, is_testing)?.tap_mut(|p| p.push("running"));
    if fs::exists(&running_file_path)? {
        return Err(eyre!("{ALREADY_RUNNING} {}", running_file_path.display()));
    }
    Ok(running_file_path)
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
