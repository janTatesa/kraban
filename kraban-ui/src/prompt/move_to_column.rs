use kraban_config::{ColumnConfig, Config};
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::Stylize,
    text::Line
};

use super::Prompt;
use crate::{
    keyhints::Keyhints,
    list::{List, ListQuery}
};

#[derive(Debug)]
pub struct MoveToColumnPrompt<'a>(List<MoveToColumnQuery<'a>>);

#[derive(Debug)]
struct MoveToColumnQuery<'a> {
    current: &'a str
}

pub enum Response<'a> {
    MoveToColumn(&'a str),
    Update(MoveToColumnPrompt<'a>)
}

impl MoveToColumnQuery<'_> {
    fn columns<'a>(&self, config: &'a Config) -> impl Iterator<Item = &'a ColumnConfig> {
        config
            .column_configs()
            .filter(|column| column.name != self.current)
    }
}

impl<'a> MoveToColumnPrompt<'a> {
    pub fn new(current: &'a str) -> Self { Self(List::new(MoveToColumnQuery { current })) }
    pub fn on_key(mut self, key: KeyEvent, config: &'a Config) -> Response<'a> {
        if let KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } = key
        {
            let column = &self.0.columns(config).nth(self.0.selected()).unwrap().name;
            return Response::MoveToColumn(column);
        }

        self.0.on_key(key);
        Response::Update(self)
    }
}

impl ListQuery for MoveToColumnQuery<'_> {
    fn get_items<'a>(&self, _: &State, config: &'a Config) -> impl Iterator<Item = Line<'a>> {
        self.columns(config)
            .map(|column| Line::raw(&column.name).fg(column.color))
    }
}

impl Prompt for MoveToColumnPrompt<'_> {
    fn height(&self, _: &State, config: &Config) -> u16 {
        config.column_configs().count() as u16 - 1
    }

    fn title(&self) -> &'static str { "Move task to column" }
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        self.0.render(area, buf, state, config);
    }
}

impl Keyhints for MoveToColumnPrompt<'_> {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        self.0
            .keyhints(state, config)
            .into_iter()
            .chain([("Enter", "Pick column")])
    }
}
