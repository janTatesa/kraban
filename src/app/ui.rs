mod component;
mod keyhints;
mod list;
mod project;
mod prompt;
mod task;
mod widgets;

use super::{
    Action, Context,
    config::Config,
    state::{CurrentList, Priority},
};
use cli_log::info;
pub use component::Component;
use project::ProjectsView;
use prompt::EnumPrompt;
pub use prompt::Prompt;
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Ui {
    view: Box<dyn View>,
    prompt: Option<Box<dyn Prompt>>,
}

fn open_prompt<T: Prompt + 'static>(prompt: T) -> Option<Action> {
    Some(Action::OpenPrompt(Box::new(prompt)))
}

impl Ui {
    pub fn new(context: Context) -> Self {
        Self {
            view: Box::new(ProjectsView::new(
                context.state.projects().len().checked_sub(1),
            )),
            prompt: None,
        }
    }

    // Context cannot be used because state would be referenced both mutably and immutably
    pub fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a> {
        self.view.current_list(config)
    }

    pub fn handle_action(&mut self, action: Action, context: Context) -> Option<Action> {
        info!("Performing {:?}", action);
        match action {
            Action::SwitchToView(view) => self.view = view,
            Action::OpenPrompt(prompt) => self.prompt = Some(prompt),
            Action::ClosePrompt => self.prompt = None,
            Action::SwitchToIndex(index) => self.view.switch_to_index(index),
            action @ Action::New(_) => {
                self.prompt = None;
                if context.config.always_open_priority_prompt {
                    let enum_prompt: EnumPrompt<Priority> = EnumPrompt::new(None);
                    self.prompt = Some(Box::new(enum_prompt));
                };
                return Some(action);
            }
            _ => {
                self.prompt = None;
                return Some(action);
            }
        }
        None
    }

    pub fn refresh_on_state_change(&mut self, context: Context) {
        self.view.refresh_on_state_change(context);
    }
}

pub trait View: Component {
    fn item(&self) -> Item;
    fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a>;
    fn refresh_on_state_change(&mut self, context: Context);
    fn switch_to_index(&mut self, index: usize);
}

#[derive(strum_macros::Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Item {
    Project,
    Task,
}

impl From<Priority> for Span<'_> {
    fn from(value: Priority) -> Self {
        let text: &str = value.into();
        Span::styled(
            text,
            Style::new().fg(match value {
                Priority::Low => Color::Green,
                Priority::Medium => Color::Yellow,
                Priority::High => Color::Red,
            }),
        )
    }
}
