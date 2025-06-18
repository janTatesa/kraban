mod column;
mod component;
mod tab;
mod view;

use kraban_lib::wrapping_usize::WrappingUsize;
use tab::TabView;

use crate::{Context, get};

#[derive(Default, Clone, Debug)]
pub struct TasksView {
    project_index: usize,
    tabs: Vec<TabView>,
    focused_tab: WrappingUsize,
}

impl TasksView {
    pub fn new(project: usize, context: Context) -> Self {
        let tabs = get!(context, tabs);
        let tab = WrappingUsize::new(tabs.len() - 1);
        Self {
            project_index: project,
            focused_tab: tab,
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
        context: Context,
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
            focused_tab: WrappingUsize::with_value(focused_tab, tabs.len()),
            tabs,
        }
    }
}
