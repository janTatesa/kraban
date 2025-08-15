use kraban_config::Config;
use kraban_state::State;
use ratatui::{
    style::{Style, Styled, Stylize},
    text::{Line, Span, Text}
};

use crate::{ProjectsPrompt, TasksPrompt, Ui, UiState, main_view::MainViewFocus};

fn keyhints_to_text<'a>(
    hints: impl Iterator<Item = (&'a str, &'a str)>,
    width: u16,
    config: &Config
) -> Text<'a> {
    let keybinding_style = Style::new().bold().fg(config.app_color);
    let hint_style = Style::new().reset().italic();
    let hints = hints.map(|(keybinding, hint)| {
        (
            keybinding.set_style(keybinding_style),
            format!(": {hint}").set_style(hint_style)
        )
    });

    // TODO: make this more declaractive
    let mut text: Text = Line::default().centered().into();
    let mut length = 0;
    for (keybinding, hint) in hints {
        let key_hint_length = keybinding.content.len() + hint.content.len() + 1;
        if length + key_hint_length > width.into() {
            text.lines.last_mut().unwrap().spans.pop();
            text.lines.push(Line::default().centered());
            length = 0;
        }

        length += key_hint_length;
        text.lines
            .last_mut()
            .unwrap()
            .extend([keybinding, hint, Span::raw(" ")]);
    }

    text.lines.last_mut().unwrap().spans.pop();
    text
}

pub trait Keyhints {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)>;
}

macro_rules! keyhints {
    ($self:expr, $width:expr, $state:expr, $config:expr, $hints_ident:ident, $extra_hints:expr, $($pat:pat,)*) => {
        match &$self.0 {
            $($pat => keyhints_to_text($hints_ident.keyhints($state, $config).into_iter().chain($extra_hints), $width, $config),)*
        }
    };
}

impl Ui<'_> {
    pub(crate) fn keyhints<'a>(
        &'a self,
        extra_hints: impl IntoIterator<Item = (&'a str, &'a str)>,
        width: u16,
        state: &State,
        config: &Config
    ) -> Text<'a> {
        keyhints!(
            self,
            width,
            state,
            config,
            hints,
            extra_hints,
            UiState::MainView(hints, _, MainViewFocus::Projects),
            UiState::MainView(_, hints, MainViewFocus::DueTasks),
            UiState::ProjectsPrompt(_, _, ProjectsPrompt::InputPrompt(hints)),
            UiState::TasksPrompt(_, TasksPrompt::InputPrompt(hints)),
            UiState::ProjectsPrompt(_, _, ProjectsPrompt::PriorityPrompt(hints)),
            UiState::TasksPrompt(_, TasksPrompt::PriorityPrompt(hints)),
            UiState::ProjectsPrompt(_, _, ProjectsPrompt::ProjectDeleteConfirmation(hints)),
            UiState::TasksView(hints),
            UiState::TasksPrompt(_, TasksPrompt::DifficultyPrompt(hints)),
            UiState::TasksPrompt(_, TasksPrompt::DueDatePrompt(hints)),
            UiState::TasksPrompt(_, TasksPrompt::MoveToColumnPrompt(hints)),
            UiState::TasksPrompt(_, TasksPrompt::TaskDeleteConfirmation(hints)),
        )
    }
}
