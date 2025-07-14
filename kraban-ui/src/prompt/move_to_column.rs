use std::borrow::Cow;

use kraban_config::ColumnConfig;
use kraban_state::CurrentItem;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line};
use tap::Pipe;

use crate::{
    Component, Context, KeyNoModifiers, StateAction,
    action::{Action, state_action},
    get,
    keyhints::KeyHints,
    list::{List, ListQuery},
};

use super::{DEFAULT_WIDTH, PromptTrait};

#[derive(Debug)]
pub struct MoveToColumnPrompt<'a>(List<MoveToColumnQuery<'a>>);

#[derive(Debug)]
struct MoveToColumnQuery<'a> {
    current: &'a str,
}

impl MoveToColumnQuery<'_> {
    fn columns<'a>(&self, context: Context<'_, 'a>) -> impl Iterator<Item = &'a ColumnConfig> {
        get!(context, columns)
            .iter()
            .filter(|column| column.name != self.current)
    }
}

impl<'a> MoveToColumnPrompt<'a> {
    pub fn new(current: &'a str) -> Self {
        Self(List::new(MoveToColumnQuery { current }))
    }
}

impl ListQuery for MoveToColumnQuery<'_> {
    fn get_items<'a>(&self, context: Context<'a, 'a>) -> impl Iterator<Item = Line<'a>> {
        self.columns(context)
            .map(|column| Line::raw(&column.name).fg(column.color))
    }

    fn on_key<'a>(
        &self,
        index: usize,
        key: KeyEvent,
        context: Context<'_, 'a>,
    ) -> Option<Action<'a>> {
        match key.keycode_without_modifiers()? {
            KeyCode::Enter => self
                .columns(context)
                .map(|column| column.name.as_str())
                .nth(index)?
                .pipe(StateAction::MoveToColumn)
                .pipe(state_action),
            _ => None,
        }
    }

    fn keyhints(&self, _context: Context) -> KeyHints {
        vec![("Enter", "Pick column")]
    }
}

impl PromptTrait for MoveToColumnPrompt<'_> {
    fn height(&self, context: Context) -> u16 {
        get!(context, columns).len() as u16 - 1
    }

    fn title(&self, _item: CurrentItem) -> Cow<'static, str> {
        "Move task to column".into()
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH / 2
    }
}

impl<'a> Component<'a> for MoveToColumnPrompt<'a> {
    fn key_hints(&self, context: Context) -> KeyHints {
        self.0.key_hints(context)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        self.0.render(area, buf, context, focused);
    }

    fn on_key(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        self.0.on_key(key_event, context)
    }
}
