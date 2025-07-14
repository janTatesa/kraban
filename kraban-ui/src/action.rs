use std::ops::BitAnd;

use cli_log::info;
use itertools::chain;
use kraban_state::{CurrentItem, ItemToCreate, Project, Task};
use ratatui::crossterm::event::KeyEvent;
use tap::Pipe;

use crate::{
    Component, Context, Prompt, Ui, ViewTrait,
    prompt::{DifficultyPrompt, DueDatePrompt, PriorityPrompt},
    view::View,
};

pub(crate) fn switch_to_view<'a, T: Into<View<'a>>>(view: T) -> Option<Action<'a>> {
    Some(Action::SwitchToView(view.into()))
}

pub(crate) fn open_prompt<'a, T: Into<Prompt<'a>>>(prompt: T) -> Option<Action<'a>> {
    Some(Action::OpenPrompt(prompt.into()))
}

pub(crate) fn state_action<'a>(state_action: StateAction<'a>) -> Option<Action<'a>> {
    Some(Action::State(state_action))
}

pub(crate) type StateAction<'a> = kraban_state::Action<'a>;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Action<'a> {
    ClosePrompt,
    New(String),
    SwitchToView(View<'a>),
    OpenPrompt(Prompt<'a>),
    State(StateAction<'a>),
}

impl<'a> Ui<'a> {
    pub fn get_action(
        &mut self,
        key_event: KeyEvent,
        context: Context<'_, 'a>,
    ) -> Option<StateAction<'a>> {
        let action = self.on_key(key_event, context)?;

        info!("Performing {action:#?}");
        // So far all prompts exit after doing their actions
        self.prompt_stack.pop();
        match (action, self.item_to_create.take()) {
            (Action::SwitchToView(view), _) => self.view = view,
            (Action::OpenPrompt(prompt), _) => self.prompt_stack.push(prompt),
            (Action::State(action), _) => return Some(action),
            (Action::New(title), _) => {
                self.item_to_create = Some(match self.view.current_item() {
                    CurrentItem::Project(_) => Project {
                        title,
                        ..Default::default()
                    }
                    .pipe(ItemToCreate::Project),
                    CurrentItem::DueTask(_) => todo!(),
                    CurrentItem::Task { .. } => Task {
                        title,
                        ..Default::default()
                    }
                    .pipe(ItemToCreate::Task),
                });
                let always_open = context.config.always_open;
                let in_task_view = matches!(self.view.current_item(), CurrentItem::Task { .. });
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
                ))
            }
            _ => {}
        }

        None
    }
}
