mod component;
mod keyhints;
mod list;
mod project;
mod prompt;
mod task;
mod widgets;

use super::{
    Context,
    config::Config,
    state::{CurrentList, Priority},
};
use cli_log::debug;
pub use component::Component;
use project::ProjectsView;
use prompt::Prompt;
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
        debug!("Performing {:?}", action);
        match action {
            Action::SwitchToView(view) => {
                self.view = view;
                None
            }
            Action::OpenPrompt(prompt) => {
                self.prompt = Some(prompt);
                None
            }
            Action::ClosePrompt => {
                self.prompt = None;
                None
            }
            _ => {
                self.prompt = None;
                self.view.handle_action(&action, context);
                Some(action)
            }
        }
    }
}

pub trait View: Component {
    fn item(&self) -> Item;
    fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a>;
    // TODO: implement this automatically
    fn handle_action(&mut self, action: &Action, context: Context);
}

#[derive(strum_macros::Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Item {
    Project,
    Task,
}

#[derive(Debug)]
pub enum Action {
    ClosePrompt,
    ShrinkList,
    Delete,
    ChangePriority(Priority),
    New(String),
    Rename(String),
    MoveToColumn(String),
    SwitchToView(Box<dyn View>),
    OpenPrompt(Box<dyn Prompt>),
    SwitchToIndex(usize),
}

impl From<Priority> for Span<'_> {
    fn from(value: Priority) -> Self {
        Span::styled(
            value.to_string(),
            Style::new().fg(match value {
                Priority::Low => Color::Green,
                Priority::Medium => Color::Yellow,
                Priority::High => Color::Red,
            }),
        )
    }
}
