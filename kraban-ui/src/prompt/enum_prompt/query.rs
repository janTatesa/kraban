use std::iter;

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Stylize, text::Line};
use tap::Pipe;

use crate::{
    Context, KeyNoModifiers, StateAction,
    action::{Action, state_action},
    keyhints::KeyHints,
    list::ListQuery,
};

use super::EnumPromptMember;

#[derive(Debug, Clone, Copy)]
pub struct EnumPromptQuery<T> {
    pub current: Option<T>,
}

impl<T: EnumPromptMember> EnumPromptQuery<T> {
    fn variants(self) -> impl Iterator<Item = Option<T>> {
        iter::once(None)
            .chain(T::iter().map(Some))
            .filter(move |variant| *variant != self.current)
    }
}

fn enum_variant_to_line<'a, T: EnumPromptMember>(variant: Option<T>) -> Line<'a> {
    match variant {
        Some(variant) => variant.into(),
        None => Line::raw("None").dim(),
    }
}

impl<T: EnumPromptMember> ListQuery for EnumPromptQuery<T>
where
    StateAction<'static>: From<Option<T>>,
{
    fn get_items<'a>(&self, _context: Context<'a, 'a>) -> impl Iterator<Item = Line<'a>> {
        self.variants().map(enum_variant_to_line)
    }

    fn on_key(
        &self,
        index: usize,
        key_event: KeyEvent,
        _context: Context,
    ) -> Option<Action<'static>> {
        match key_event.keycode_without_modifiers()? {
            KeyCode::Enter => state_action(self.variants().nth(index)?.into()),
            KeyCode::Char(char) => self
                .variants()
                .find(|variant| {
                    char == enum_variant_to_str(*variant)
                        .chars()
                        .next()
                        .unwrap()
                        .to_ascii_lowercase()
                })?
                .pipe(StateAction::from)
                .pipe(state_action),
            _ => None,
        }
    }

    fn keyhints(&self, _context: Context) -> KeyHints {
        let mut hints = vec![("Enter", T::keyhint())];
        hints.extend(
            self.variants()
                .map(enum_variant_to_str)
                .map(|variant| (&variant[0..1], variant)),
        );
        hints
    }
}

fn enum_variant_to_str<T: EnumPromptMember>(variant: Option<T>) -> &'static str {
    match variant {
        Some(variant) => variant.into(),
        None => "None",
    }
}
