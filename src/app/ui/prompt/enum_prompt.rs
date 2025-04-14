use std::marker::PhantomData;

use crossterm::event::{KeyCode, KeyEvent};
use derivative::Derivative;
use ratatui::{buffer::Buffer, layout::Rect, text::Span, widgets::StatefulWidget};
use strum::{EnumCount, IntoEnumIterator};

use crate::app::{
    Context,
    state::{Difficulty, Priority},
    ui::{Action, Component, Item, keyhints::KeyHints, list::WrappingUsize, widgets::list_widget},
};

use super::Prompt;

pub trait EnumPromptMember:
    EnumCount + IntoEnumIterator + Into<Span<'static>> + Into<Action>
{
    fn type_name() -> &'static str;
    fn keyhint() -> &'static str;
}

impl EnumPromptMember for Difficulty {
    fn type_name() -> &'static str {
        "difficulty"
    }

    fn keyhint() -> &'static str {
        "Pick difficulty"
    }
}

impl From<Difficulty> for Action {
    fn from(value: Difficulty) -> Self {
        Self::ChangeDifficulty(value)
    }
}

impl EnumPromptMember for Priority {
    fn type_name() -> &'static str {
        "priority"
    }

    fn keyhint() -> &'static str {
        "Pick priority"
    }
}

impl From<Priority> for Action {
    fn from(value: Priority) -> Self {
        Self::ChangePriority(value)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct EnumPrompt<T: EnumPromptMember>(WrappingUsize, PhantomData<T>);

impl<T: EnumPromptMember> EnumPrompt<T> {
    pub const fn new() -> Self {
        Self(WrappingUsize::new(T::COUNT - 1), PhantomData)
    }
}

impl<T: EnumPromptMember> Prompt for EnumPrompt<T>
where
    T:,
{
    fn height(&self) -> u16 {
        T::COUNT as u16
    }

    fn title(&self, item: Item) -> String {
        format!("Change {item} {}", T::type_name())
    }
}

impl<T: EnumPromptMember> Component for EnumPrompt<T> {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Up => {
                self.0 = self.0.decrement();
                None
            }
            KeyCode::Down => {
                self.0 = self.0.increment();
                None
            }
            KeyCode::Enter => T::iter().nth(self.0.into()).map(|t| -> Action { t.into() }),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Up/Down", "Select previous/next"), ("Enter", T::keyhint())]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, _context: Context) {
        let list = list_widget(T::iter().map(|item| -> Span<'static> { item.into() }));
        list.render(area, buf, &mut self.0.into());
    }
}
