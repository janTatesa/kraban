use std::borrow::Cow;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line};
use tap::Pipe;

use crate::{
    Component, Context, Item, KeyNoModifiers, StateAction,
    action::{Action, state_action},
    get,
    keyhints::KeyHints,
    list::{List, ListQuery},
};

use super::{DEFAULT_WIDTH, PromptTrait};

#[derive(Debug)]
pub struct MoveToColumnPrompt(List<MoveToColumnQuery>);

#[derive(Debug)]
struct MoveToColumnQuery {
    current: String,
}

impl MoveToColumnPrompt {
    pub fn new(context: Context, current: String) -> Self {
        Self(List::new(
            get!(context, columns).len() - 1,
            MoveToColumnQuery { current },
        ))
    }
}

impl ListQuery for MoveToColumnQuery {
    fn get_items<'a>(&self, context: Context<'a>) -> impl Iterator<Item = Line<'a>> {
        get!(context, columns)
            .iter()
            .filter(|column| column.name != self.current)
            .map(|column| Line::raw(&column.name).fg(column.color))
    }

    fn on_key(&self, index: usize, key: KeyEvent, context: Context) -> Option<Action> {
        match key.keycode_without_modifiers()? {
            KeyCode::Enter => self
                .get_items(context)
                .nth(index)?
                .to_string()
                .pipe(StateAction::MoveToColumn)
                .pipe(state_action),
            _ => None,
        }
    }

    fn keyhints(&self, _context: Context) -> KeyHints {
        vec![("Enter", "Pick column")]
    }
}

impl PromptTrait for MoveToColumnPrompt {
    fn height(&self, context: Context) -> u16 {
        get!(context, columns).len() as u16 - 1
    }

    fn title(&self, _item: Item) -> Cow<'static, str> {
        "Move task to column".into()
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH / 2
    }
}

impl Component for MoveToColumnPrompt {
    fn key_hints(&self, context: Context) -> KeyHints {
        self.0.key_hints(context)
    }

    fn render(&self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        self.0.render(area, buf, context, focused);
    }

    fn on_key(&mut self, key_event: KeyEvent, context: Context) -> Option<Action> {
        self.0.on_key(key_event, context)
    }
}
