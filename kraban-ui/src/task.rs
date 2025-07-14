mod column;
mod component;
mod tab;
mod view;

use kraban_lib::wrapping_usize::WrappingUsize;
use tab::TabView;

use crate::{Context, get};

#[derive(Clone, Debug)]
pub struct TasksView<'a> {
    project_index: usize,
    tabs: Vec<TabView<'a>>,
    focused_tab: WrappingUsize,
}

impl<'a> TasksView<'a> {
    pub fn new(project: usize, context: Context<'_, 'a>) -> Self {
        let tabs = get!(context, tabs);
        let focused_tab = WrappingUsize::new(tabs.len() - 1);
        Self {
            project_index: project,
            focused_tab,
            tabs: tabs
                .iter()
                .enumerate()
                .map(|(index, tab)| TabView::new(project, index, tab))
                .collect(),
        }
    }

    pub fn with_specific_task(
        project: usize,
        column_name: &str,
        task: usize,
        context: Context<'_, 'a>,
    ) -> Self {
        let tabs = get!(context, tabs);

        let (focused_tab, column) = tabs
            .iter()
            .enumerate()
            .find_map(|(focused_tab, tab)| {
                Some((
                    focused_tab,
                    tab.columns
                        .iter()
                        .position(|column| column.name == column_name)?,
                ))
            })
            .unwrap();

        let tabs: Vec<TabView> = tabs
            .iter()
            .enumerate()
            .map(|(index, tab)| TabView::with_column_and_task(project, index, tab, column, task))
            .collect();

        Self {
            project_index: project,
            focused_tab: WrappingUsize::new_with_value(tabs.len() - 1, focused_tab),
            tabs,
        }
    }
}
