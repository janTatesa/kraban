use std::ops::BitAnd;

use cli_log::info;
use crossterm::event::KeyEvent;
use itertools::chain;

use crate::{
    Component, Context, Item, Prompt, Ui, ViewTrait,
    prompt::{DifficultyPrompt, DueDatePrompt, PriorityPrompt},
    view::View,
};

pub(crate) fn switch_to_view<T: Into<View>>(view: T) -> Option<Action> {
    Some(Action::SwitchToView(view.into()))
}

pub(crate) fn open_prompt<T: Into<Prompt>>(prompt: T) -> Option<Action> {
    Some(Action::OpenPrompt(prompt.into()))
}

pub(crate) fn state_action(state_action: StateAction) -> Option<Action> {
    Some(Action::State(state_action))
}

pub(crate) type StateAction = kraban_state::Action;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Action {
    ClosePrompt,
    SwitchToView(View),
    OpenPrompt(Prompt),
    State(StateAction),
}

impl Ui {
    pub(crate) fn handle_action(
        &mut self,
        action: Action,
        context: Context,
    ) -> Option<StateAction> {
        info!("Performing {action:#?}");
        // So far all prompts exit after doing their actions
        self.prompt_stack.pop();
        match action {
            Action::SwitchToView(view) => self.view = view,
            Action::OpenPrompt(prompt) => self.prompt_stack.push(prompt),
            Action::State(action) => {
                self.prompt_stack.pop();
                if let StateAction::New(_) = action {
                    let always_open = context.config.always_open;
                    let in_task_view = self.view.item() == Item::Task;
                    self.prompt_stack.extend(chain!(
                        always_open
                            .due_date
                            .bitand(in_task_view)
                            .then_some(DueDatePrompt::new(None).into()),
                        always_open
                            .difficulty
                            .bitand(in_task_view)
                            .then_some(DifficultyPrompt::new(None).into()),
                        always_open
                            .priority
                            .then_some(PriorityPrompt::new(None).into())
                    ));
                }

                return Some(action);
            }
            _ => {}
        }
        None
    }

    pub fn get_action(&mut self, key_event: KeyEvent, context: Context) -> Option<StateAction> {
        let action = self.on_key(key_event, context)?;
        self.handle_action(action, context)
    }
}
