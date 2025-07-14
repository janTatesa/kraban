pub mod difficulty;
pub mod priority;
mod query;

use crate::{Component, Context, StateAction, action::Action, keyhints::KeyHints, list::List};

use super::{DEFAULT_WIDTH, PromptTrait};
use kraban_state::CurrentItem;
use query::EnumPromptQuery;
use ratatui::crossterm::event::KeyEvent;
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
    StateAction<'static>: From<Option<T>>,
{
    pub fn new(current: Option<T>) -> Self {
        Self(List::new(EnumPromptQuery { current }))
    }
}

impl<T: EnumPromptMember> PromptTrait for EnumPrompt<T> {
    fn height(&self, _context: Context) -> u16 {
        T::COUNT as u16
    }

    fn title(&self, item: CurrentItem) -> Cow<'static, str> {
        let item: &str = item.as_ref();
        format!("Change {item} {}", T::type_name()).into()
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH / 2
    }
}

impl<T: EnumPromptMember> Component<'_> for EnumPrompt<T>
where
    StateAction<'static>: From<Option<T>>,
{
    fn key_hints(&self, context: Context) -> KeyHints {
        self.0.key_hints(context)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, context: Context, focused: bool) {
        self.0.render(area, buf, context, focused);
    }

    fn on_key<'a>(&mut self, key_event: KeyEvent, context: Context<'_, 'a>) -> Option<Action<'a>> {
        self.0.on_key(key_event, context)
    }
}
