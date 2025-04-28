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

use super::{DEFAULT_WIDTH, Prompt};

pub trait EnumPromptMember: EnumCount + IntoEnumIterator + Into<Span<'static>>
where
    Option<Self>: Into<Action>,
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

impl From<Option<Difficulty>> for Action {
    fn from(value: Option<Difficulty>) -> Self {
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

impl From<Option<Priority>> for Action {
    fn from(value: Option<Priority>) -> Self {
        Self::ChangePriority(value)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct EnumPrompt<T: EnumPromptMember>(WrappingUsize, PhantomData<T>)
where
    Action: From<Option<T>>;

impl<T: EnumPromptMember> EnumPrompt<T>
where
    Action: From<Option<T>>,
{
    pub const fn new() -> Self {
        Self(WrappingUsize::new(T::COUNT), PhantomData)
    }
}

impl<T: EnumPromptMember> Prompt for EnumPrompt<T>
where
    Action: From<Option<T>>,
{
    fn height(&self) -> u16 {
        T::COUNT as u16 + 1
    }

    fn title(&self, item: Item) -> String {
        format!("Change {item} {}", T::type_name())
    }

    fn width(&self) -> u16 {
        DEFAULT_WIDTH / 2
    }
}

impl<T: EnumPromptMember> Component for EnumPrompt<T>
where
    Action: From<Option<T>>,
{
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
            KeyCode::Enter => Some(Action::from(
                [None]
                    .into_iter()
                    .chain(T::iter().map(Some))
                    .nth(self.0.into())
                    .unwrap(),
            )),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Up/Down", "Select previous/next"), ("Enter", T::keyhint())]
    }

    fn render(&self, area: Rect, buf: &mut Buffer, _context: Context) {
        let list = list_widget(
            [Span::raw("None")]
                .into_iter()
                .chain(T::iter().map(|item| -> Span<'static> { item.into() })),
        );
        list.render(area, buf, &mut self.0.into());
    }
}
