pub mod difficulty;
pub mod priority;
mod query;

use crate::{
    Component, Context, Item, StateAction, action::Action, keyhints::KeyHints, list::List,
};

use super::{DEFAULT_WIDTH, PromptTrait};
use crossterm::event::KeyEvent;
use query::EnumPromptQuery;
use ratatui::{buffer::Buffer, layout::Rect, text::Line};

use std::{borrow::Cow, fmt::Debug};
use strum::{EnumCount, IntoEnumIterator};

pub trait EnumPromptMember:
    EnumCount + IntoEnumIterator + Into<Line<'static>> + Debug + Eq + Copy + Into<&'static str>
{
    fn type_name() -> &'static str;
    fn keyhint() -> &'static str;
}

#[derive(Debug)]
pub struct EnumPrompt<T: EnumPromptMember>(List<EnumPromptQuery<T>>);

impl<T: EnumPromptMember> EnumPrompt<T>
where
    StateAction: From<Option<T>>,
{
    pub const fn new(current: Option<T>) -> Self {
        Self(List::new(T::COUNT, EnumPromptQuery { current }))
    }
}

impl<T: EnumPromptMember> PromptTrait for EnumPrompt<T> {
    fn height(&self, _context: Context) -> u16 {
        T::COUNT as u16
    }

    fn title(&self, item: Item) -> Cow<'static, str> {
        let item: &str = item.into();
        format!("Change {item} {}", T::type_name()).into()
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH / 2
    }
}

impl<T: EnumPromptMember> Component for EnumPrompt<T>
where
    StateAction: From<Option<T>>,
{
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
