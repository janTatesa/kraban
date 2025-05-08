use std::fs;

use color_eyre::{Result, owo_colors::OwoColorize};
use figment::{
    Figment,
    providers::{Data, Toml},
};

use ratatui::style::Color;
use serde::Deserialize;
use tap::Tap;

use kraban_lib::{Dir, get_dir};

pub struct Config {
    pub tabs: Vec<Tab>,
    pub columns: Vec<Column>,
    pub app_color: Color,
    pub collapse_unfocused_tabs: bool,
    pub show_key_hints: bool,
    pub always_open_priority_prompt: bool,
    pub default_due_dates: DefaultDueDates,
}

const DEFAULT_CONFIG: &str = include_str!("./default-config.toml");
pub fn print_default_config() {
    println!("{DEFAULT_CONFIG}");
}

pub fn write_default_config(is_testing: bool) -> Result<()> {
    let dir = get_dir(Dir::Config, is_testing)?.tap_mut(|p| p.push("kraban.toml"));
    fs::write(&dir, DEFAULT_CONFIG)?;
    println!("Wrote default config to {}", dir.display().green());
    Ok(())
}

impl Config {
    pub fn new(is_testing: bool) -> Result<Self> {
        let path = get_dir(Dir::Config, is_testing)?.tap_mut(|p| p.push("kraban.toml"));

        let raw: ConfigRaw = Figment::new()
            .merge(Data::<Toml>::string(DEFAULT_CONFIG))
            .merge(Data::<Toml>::file(path))
            .extract()?;
        let ConfigRaw {
            columns,
            app_color,
            collapse_unfocused_tabs,
            show_key_hints,
            always_open_priority_prompt,
            default_due_dates,
        } = raw;

        let columns = columns.into_iter().map(|column| {
            (
                column.tab,
                Column {
                    name: column.name,
                    color: column.color,
                    done_column: column.done_column,
                },
            )
        });

        let tabs = columns.clone().fold(Vec::new(), |mut tabs, (tab, column)| {
            if tab >= tabs.len() {
                tabs.resize_with(tab + 1, Tab::default);
            }
            tabs[tab].columns.push(column);
            tabs
        });

        let columns = columns.map(|(_, column)| column).collect();
        Ok(Self {
            tabs,
            app_color,
            columns,
            collapse_unfocused_tabs,
            show_key_hints,
            always_open_priority_prompt,
            default_due_dates,
        })
    }
}

#[derive(Default)]
pub struct Tab {
    pub columns: Vec<Column>,
}

#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub color: Color,
    pub done_column: bool,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct ColumnRaw {
    name: String,
    color: Color,
    tab: usize,
    #[serde(default)]
    done_column: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigRaw {
    #[serde(alias = "column")]
    columns: Vec<ColumnRaw>,
    app_color: Color,
    collapse_unfocused_tabs: bool,
    show_key_hints: bool,
    always_open_priority_prompt: bool,
    default_due_dates: DefaultDueDates,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefaultDueDates {
    pub enable: bool,
    pub high: u16,
    pub medium: u16,
    pub low: u16,
}
