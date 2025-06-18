use std::{fs, path::PathBuf, str::FromStr};

use color_eyre::eyre::{ContextCompat, Result};
use tap::Tap;

pub fn get_dir(dir: Dir, is_testing: bool) -> Result<PathBuf> {
    let path = match is_testing {
        true => PathBuf::from_str("testing-files").unwrap(),
        false => {
            match dir {
                Dir::State => dirs::state_dir().or(dirs::data_dir()), // madOS doesn't have a state dir apparently
                Dir::Config => dirs::config_dir(),
            }
            .wrap_err_with(|| format!("Cannot get OS {dir} dir"))?
            .tap_mut(|p| p.push("kraban"))
        }
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
