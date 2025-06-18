use std::fs;

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use kraban_lib::dir::{Dir, get_dir};
use tap::Tap;

use crate::Config;

impl Config {
    pub(crate) const DEFAULT: &str = include_str!("./default-config.toml");
    pub fn print_default() {
        println!("{}", Self::DEFAULT);
    }

    pub fn write_default(is_testing: bool) -> Result<()> {
        let dir = get_dir(Dir::Config, is_testing)?.tap_mut(|p| p.push("kraban.toml"));
        fs::write(&dir, Self::DEFAULT)?;
        println!("Wrote default config to {}", dir.display().green());
        Ok(())
    }
}
