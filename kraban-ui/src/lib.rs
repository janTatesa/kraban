mod component;
mod keyhints;
mod list;
mod main_view;
mod prompt;
mod task;
mod view;
mod widgets;

use cli_log::info;
use component::Component;
use crossterm::event::KeyEvent;
use kraban_config::Config;
use kraban_state::{CurrentList, Priority, State};
use main_view::MainView;
use prompt::{EnumPrompt, Prompt};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Span};
use std::fmt::Debug;
use view::{Item, View, ViewTrait};

type StateAction = kraban_state::Action;
#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub state: &'a State,
    pub config: &'a Config,
}

#[macro_export]
macro_rules! context {
    ($self:expr) => {
        Context {
            state: &$self.state,
            config: &$self.config,
        }
    };
}

#[derive(Debug)]
pub struct Ui {
    view: View,
    prompt: Option<Prompt>,
}

fn switch_to_view<T: Into<View>>(view: T) -> Option<Action> {
    Some(Action::SwitchToView(view.into()))
}

fn open_prompt<T: Into<Prompt>>(prompt: T) -> Option<Action> {
    Some(Action::OpenPrompt(prompt.into()))
}

fn state_action(state_action: StateAction) -> Option<Action> {
    Some(Action::State(state_action))
}

#[derive(Debug)]
enum Action {
    ClosePrompt,
    SwitchToView(View),
    OpenPrompt(Prompt),
    State(StateAction),
}

impl Ui {
    pub fn new(projects_len: Option<usize>) -> Self {
        Self {
            view: View::from(MainView::new(projects_len)),
            prompt: None,
        }
    }

    // Context cannot be used because state would be referenced both mutably and immutably
    pub fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a> {
        self.view.current_list(config)
    }

    pub fn redraw(&self, area: Rect, buf: &mut Buffer, context: Context) {
        self.render(area, buf, context);
    }

    pub fn get_action(&mut self, key_event: KeyEvent, context: Context) -> Option<StateAction> {
        let action = self.on_key(key_event, context)?;
        self.handle_action(action, context)
    }

    pub fn in_main_view(&self) -> bool {
        matches!(self.view, View::MainView(_))
    }

    fn handle_action(&mut self, action: Action, context: Context) -> Option<StateAction> {
        info!("Performing {:?}", action);
        match action {
            Action::SwitchToView(view) => self.view = view,
            Action::OpenPrompt(prompt) => self.prompt = Some(prompt),
            Action::ClosePrompt => self.prompt = None,
            Action::State(action @ StateAction::New(_)) => {
                self.prompt = None;
                if context.config.always_open_priority_prompt {
                    let enum_prompt: EnumPrompt<Priority> = EnumPrompt::new(None);
                    self.prompt = Some(enum_prompt.into());
                };
                return Some(action);
            }
            Action::State(action) => {
                // So far all prompts exit after doing their actions
                self.prompt = None;
                return Some(action);
            }
        }
        None
    }

    pub fn switch_to_index(&mut self, index: usize) {
        self.view.switch_to_index(index);
    }

    pub fn refresh_on_state_change(&mut self, context: Context) {
        self.view.refresh_on_state_change(context);
    }
}

// Traits like difficulty priority etc
fn item_trait<'a, T>(item_trait: T) -> [Span<'a>; 3]
where
    Span<'a>: From<T>,
{
    [
        Span::raw("[").dark_gray(),
        Span::from(item_trait),
        Span::raw("] ").dark_gray(),
    ]
}
