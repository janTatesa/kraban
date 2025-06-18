use crossterm::event::{KeyCode, KeyEvent};
use itertools::Itertools;
use kraban_lib::iter::IterExt;
use ratatui::{
    layout::Constraint,
    style::Stylize,
    text::{Line, Span},
};
use tap::Pipe;

use crate::{
    Context, Item, KeyNoModifiers,
    action::{Action, open_prompt, switch_to_view},
    get,
    keyhints::KeyHints,
    no_property,
    prompt::{DeleteConfirmation, InputAction, InputPrompt, PriorityPrompt},
    table::{SMALL_COLUMN_SIZE, TableQuery},
    task::TasksView,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct ProjectsTable;

impl TableQuery<3> for ProjectsTable {
    fn on_key(
        &self,
        index: Option<usize>,
        key_event: KeyEvent,
        context: Context,
    ) -> Option<Action> {
        match key_event.keycode_without_modifiers()? {
            KeyCode::Delete | KeyCode::Backspace => open_prompt(DeleteConfirmation {
                name: get!(context, projects, index?).title.clone(),
                item: Item::Project,
            }),
            KeyCode::Char('n') => open_prompt(InputPrompt::new(
                context,
                InputAction::New,
                "Enter new project name".to_string(),
            )),

            KeyCode::Char('p') => get!(context, projects, index?)
                .priority
                .pipe(PriorityPrompt::new)
                .pipe(open_prompt),
            KeyCode::Char('r') => open_prompt(InputPrompt::new(
                context,
                InputAction::Rename,
                get!(context, projects, index?).title.clone(),
            )),
            KeyCode::Enter => switch_to_view(TasksView::new(index?, context)),
            _ => None,
        }
    }

    #[allow(unstable_name_collisions)]
    fn rows<'a>(&self, context: Context<'a>) -> impl Iterator<Item = [Line<'a>; 3]> {
        get!(context, projects).iter().map(move |project| {
            [
                project.priority.map(Line::from).unwrap_or(no_property()),
                Line::from_iter(
                    get!(context, columns)
                        .iter()
                        .filter_map(|column| {
                            let len = project.columns.get(&column.name).len();
                            match len != 0 {
                                true => Some([
                                    len.to_string().fg(column.color).italic(),
                                    Span::raw(&column.name).fg(column.color).italic(),
                                ]),
                                false => None,
                            }
                        })
                        .flatten()
                        .intersperse(" ".into())
                        .default("None".italic().dim()),
                ),
                project.title.as_str().into(),
            ]
        })
    }

    fn keyhints(&self, _context: Context) -> KeyHints {
        vec![
            ("Delete/Backspace", "Delete"),
            ("n", "New"),
            ("p", "Set priority"),
            ("r", "Rename"),
            ("Enter", "View project tasks"),
        ]
    }

    fn len(&self, context: Context) -> usize {
        get!(context, projects).len()
    }

    fn header(&self) -> [&'static str; 3] {
        ["Prior.", "Tasks", "Title"]
    }

    fn constraints(&self, _context: Context) -> [Constraint; 3] {
        [
            Constraint::Length(SMALL_COLUMN_SIZE),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]
    }
}
