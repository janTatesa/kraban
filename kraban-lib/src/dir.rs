use std::{fs, path::PathBuf};

#[cfg(not(debug_assertions))]
use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::Result;

#[cfg(debug_assertions)]
pub fn get_dir(_dir: Dir) -> Result<PathBuf> {
    let path = PathBuf::from_iter(["testing-files"]);
    fs::create_dir_all(&path)?;
    Ok(path)
}

#[cfg(not(debug_assertions))]
pub fn get_dir(dir: Dir) -> Result<PathBuf> {
    let mut path = match dir {
        Dir::State => dirs::state_dir().or(dirs::data_dir()), // madOS doesn't have a state dir apparently
        Dir::Config => dirs::config_dir()
    }
    .wrap_err_with(|| format!("Cannot get OS {dir} dir"))?;

    path.push("kraban");

    fs::create_dir_all(&path)?;
    Ok(path)
}

#[derive(strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Dir {
    State,
    Config
}
