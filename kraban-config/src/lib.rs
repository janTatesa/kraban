mod default;

use std::{ops::Deref, path::PathBuf};

use color_eyre::Result;
use figment::{
    Figment,
    providers::{Data, Toml}
};
use kraban_lib::{Dir, get_dir};
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    pub tabs: Vec<TabConfig>,
    pub app_color: Color,
    pub collapse_unfocused_tabs: bool,
    pub show_key_hints: bool,
    pub always_open: AlwaysOpen,
    pub default_due_dates: DefaultDueDates
}

impl Config {
    pub fn column_configs(&self) -> impl Iterator<Item = &ColumnConfig> {
        self.tabs.iter().flat_map(|tab| tab.iter())
    }
}

#[derive(Default, Debug)]
pub struct TabConfig(Vec<ColumnConfig>);
impl Deref for TabConfig {
    type Target = Vec<ColumnConfig>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl TabConfig {
    pub fn get_column_idx(&self, column_name: &str) -> Option<usize> {
        self.iter().position(|column| column.name == column_name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnConfig {
    pub name: String,
    pub color: Color,
    pub done_column: bool
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DefaultDueDates {
    pub enable: bool,
    pub high: u16,
    pub medium: u16,
    pub low: u16
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct AlwaysOpen {
    pub priority: bool,
    pub difficulty: bool,
    pub due_date: bool
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct ColumnRaw {
    name: String,
    color: Color,
    tab: usize,
    #[serde(default)]
    done_column: bool
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigRaw {
    #[serde(alias = "column")]
    columns: Vec<ColumnRaw>,
    app_color: Color,
    collapse_unfocused_tabs: bool,
    show_key_hints: bool,
    always_open: AlwaysOpen,
    default_due_dates: DefaultDueDates
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = path()?;
        let raw: ConfigRaw = Figment::new()
            .merge(Data::<Toml>::string(Self::DEFAULT))
            .merge(Data::<Toml>::file(path))
            .extract()?;
        let ConfigRaw {
            columns,
            app_color,
            collapse_unfocused_tabs,
            show_key_hints,
            default_due_dates,
            always_open
        } = raw;

        let columns = columns.into_iter().map(|column| {
            (
                column.tab,
                ColumnConfig {
                    name: column.name,
                    color: column.color,
                    done_column: column.done_column
                }
            )
        });

        let tabs = columns.fold(Vec::new(), |mut tabs, (tab, column)| {
            if tab >= tabs.len() {
                tabs.resize_with(tab + 1, TabConfig::default);
            }
            tabs[tab].0.push(column);
            tabs
        });

        Ok(Self {
            tabs,
            app_color,
            collapse_unfocused_tabs,
            show_key_hints,
            always_open,
            default_due_dates
        })
    }
}

fn path() -> Result<PathBuf> {
    let mut dir = get_dir(Dir::Config)?;
    dir.push("kraban.toml");
    Ok(dir)
}
