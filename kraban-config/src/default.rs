use std::fs;

use color_eyre::{eyre::Result, owo_colors::OwoColorize};

use crate::{Config, path};

impl Config {
    pub(crate) const DEFAULT: &str = include_str!("./default-config.toml");
    pub fn print_default() {
        println!("{}", Self::DEFAULT);
    }

    pub fn write_default() -> Result<()> {
        let dir = path()?;
        fs::write(&dir, Self::DEFAULT)?;
        println!("Wrote default config to {}", dir.display().green());
        Ok(())
    }
}
