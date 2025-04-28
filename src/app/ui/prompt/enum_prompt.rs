use crossterm::event::{KeyCode, KeyEvent};
use derivative::Derivative;
use ratatui::{buffer::Buffer, layout::Rect, text::Span, widgets::StatefulWidget};
use std::{fmt::Debug, iter};
use strum::{EnumCount, IntoEnumIterator};
use tap::Tap;

use crate::app::{
    Context,
    state::{Difficulty, Priority},
    ui::{Action, Component, Item, keyhints::KeyHints, list::WrappingUsize, widgets::list_widget},
};

use super::{DEFAULT_WIDTH, Prompt};

pub trait EnumPromptMember:
    EnumCount + IntoEnumIterator + Into<Span<'static>> + Debug + Eq + Copy + Into<&'static str>
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
pub struct EnumPrompt<T: EnumPromptMember>
where
    Action: From<Option<T>>,
{
    selected: WrappingUsize,
    current: Option<T>,
}

impl<T: EnumPromptMember> EnumPrompt<T>
where
    Action: From<Option<T>>,
{
    pub const fn new(current: Option<T>) -> Self {
        Self {
            selected: WrappingUsize::new(T::COUNT - 1),
            current,
        }
    }

    fn variants(&self) -> impl Iterator<Item = Option<T>> {
        iter::once(None)
            .chain(T::iter().map(Some))
            .filter(|variant| *variant != self.current)
    }
}

impl<T: EnumPromptMember> Prompt for EnumPrompt<T>
where
    Action: From<Option<T>>,
{
    fn height(&self) -> u16 {
        T::COUNT as u16
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
                self.selected = self.selected.decrement();
                None
            }
            KeyCode::Down => {
                self.selected = self.selected.increment();
                None
            }
            KeyCode::Enter => Some(Action::from(
                self.variants().nth(self.selected.into()).unwrap(),
            )),
            KeyCode::Char(char) => Some(Action::from(self.variants().find(|variant| {
                char.eq_ignore_ascii_case(&enum_variant_to_str(*variant).chars().next().unwrap())
            })?)),
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Up/Down", "Select previous/next"), ("Enter", T::keyhint())].tap_mut(|vec| {
            vec.extend(self.variants().map(|variant| {
                (
                    &enum_variant_to_str(variant)[0..1],
                    enum_variant_to_str(variant),
                )
            }))
        })
    }

    fn render(&self, area: Rect, buf: &mut Buffer, _context: Context) {
        let list = list_widget(self.variants().map(enum_variant_to_span));
        list.render(area, buf, &mut self.selected.into());
    }
}

fn enum_variant_to_span<T: EnumPromptMember>(variant: Option<T>) -> Span<'static> {
    match variant {
        Some(variant) => variant.into(),
        None => Span::raw("None"),
    }
}

fn enum_variant_to_str<T: EnumPromptMember>(variant: Option<T>) -> &'static str {
    match variant {
        Some(variant) => variant.into(),
        None => "None",
    }
}
