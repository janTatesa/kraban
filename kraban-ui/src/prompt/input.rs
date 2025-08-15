use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::{Style, Stylize},
    widgets::Widget
};
use tui_textarea::{CursorMove, TextArea};

use super::Prompt;
use crate::keyhints::Keyhints;

#[derive(Debug)]
pub struct InputPrompt {
    text_area: TextArea<'static>,
    input_action: InputAction
}

#[derive(strum_macros::Display, Debug)]
pub enum InputAction {
    Rename,
    New
}

#[allow(clippy::large_enum_variant)]
pub enum Response {
    Update(InputPrompt),
    New(String),
    Rename(String)
}

impl InputPrompt {
    pub fn new(config: &Config, input_action: InputAction, text: String) -> Self {
        let mut text_area = match input_action {
            InputAction::Rename => {
                let mut text = TextArea::new(vec![text]);
                text.move_cursor(CursorMove::End);
                text
            }
            InputAction::New => {
                let mut text_area = TextArea::default();
                text_area.set_placeholder_text(text);
                text_area
            }
        };

        let style = Style::new().fg(config.app_color).reversed();
        text_area.set_selection_style(style);
        text_area.set_cursor_line_style(Style::new());

        Self {
            text_area,
            input_action
        }
    }

    pub fn on_key(mut self, key: KeyEvent) -> Response {
        if let KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } = key
        {
            let text = self.text_area.into_lines().remove(0);
            return match self.input_action {
                InputAction::New => Response::New(text),
                InputAction::Rename => Response::Rename(text)
            }
        }

        self.text_area.input(key);
        Response::Update(self)
    }
}

impl Prompt for InputPrompt {
    fn height(&self, _: &State, _: &Config) -> u16 { 1 }
    fn title(&self) -> &'static str {
        match self.input_action {
            InputAction::Rename => "Rename item",
            InputAction::New => "New item"
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, _: &State, _: &Config) {
        self.text_area.render(area, buf)
    }
}

impl Keyhints for InputPrompt {
    fn keyhints(&self, _: &State, _: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        [("Enter", "Submit"), ("Other", "Consult readme")]
    }
}
