use color_eyre::Result;
use std::fs;

use ratatui::style::Color;
use serde::Deserialize;
use tap::Tap;

pub struct Config {
    pub tabs: Vec<Tab>,
    pub all_columns: Vec<Column>,
    pub app_color: Color,
    pub collapse_unfocused_tabs: bool,
    pub show_key_hints: bool,
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = dirs::config_dir().unwrap_or_default().tap_mut(|p| {
            p.push("kraban");
            p.push("kraban.toml")
        });
        if !fs::exists(&path)? {
            fs::write(&path, include_str!("../../default-config.toml"))?;
        }

        let raw: ConfigRaw = toml::from_str(&fs::read_to_string(path)?)?;
        let ConfigRaw {
            column,
            app_color,
            collapse_unfocused_tabs,
            show_key_hints,
        } = raw;
        let mut tabs: Vec<Tab> = Vec::new();
        for (tab, column) in column.into_iter().map(|column| {
            (
                column.tab,
                Column {
                    name: column.name,
                    color: column.color,
                    immutable: column.immutable,
                },
            )
        }) {
            if tab >= tabs.len() {
                tabs.resize_with(tab + 1, Tab::default);
            }
            tabs[tab].columns.push(column);
        }
        let all_columns = tabs.iter().fold(Vec::new(), |acc, tab| {
            acc.tap_mut(|acc| acc.extend(tab.columns.clone()))
        });
        Ok(Self {
            tabs,
            app_color,
            all_columns,
            collapse_unfocused_tabs,
            show_key_hints,
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
    pub immutable: bool,
}

#[derive(Deserialize)]
struct ColumnRaw {
    name: String,
    color: Color,
    tab: usize,
    #[serde(default)]
    immutable: bool,
}

#[derive(Deserialize)]
struct ConfigRaw {
    column: Vec<ColumnRaw>,
    app_color: Color,
    collapse_unfocused_tabs: bool,
    show_key_hints: bool,
}
