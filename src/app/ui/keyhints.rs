use ratatui::{
    style::{Style, Styled},
    text::{Line, Span, Text},
};

pub type KeyHints = Vec<(&'static str, &'static str)>;
pub struct KeyHintsWidget {
    pub hints: KeyHints,
    pub keybinding_style: Style,
    pub hint_style: Style,
}

impl KeyHintsWidget {
    pub fn into_text(self, width: u16) -> Text<'static> {
        let key_hints = self.hints.iter().map(|(keybinding, hint)| {
            (
                keybinding.set_style(self.keybinding_style),
                format!(": {hint}").set_style(self.hint_style),
            )
        });
        // TODO: This doesn't use iterators, UGLY UGLY UGLY
        let mut lines = vec![Line::default().centered()];
        let mut length = 0;
        for (keybinding, hint) in key_hints {
            let key_hint_length = keybinding.content.len() + hint.content.len() + 1;
            if length + key_hint_length > width.into() {
                lines.last_mut().unwrap().spans.pop();
                lines.push(Line::default().centered());
                length = 0;
            }
            length += key_hint_length;
            lines
                .last_mut()
                .unwrap()
                .extend([keybinding, hint, Span::raw(" ")]);
        }
        lines.last_mut().unwrap().spans.pop();
        Text::from(lines)
    }
}
