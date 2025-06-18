use kraban_config::Config;
use kraban_state::State;

#[derive(Clone, Copy, Debug)]
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

#[macro_export]
macro_rules! get {
    // Projects
    ($context:expr, projects) => {
        $context.state.projects()
    };

    ($context:expr, projects, $project:expr) => {
        &get!($context, projects)[$project]
    };

    ($context:expr, projects, $project:expr, $column:expr) => {
        get!($context, projects, $project)
            .columns
            .get($column)
            .inner()
    };

    ($context:expr, projects, $project:expr, $column:expr, $task: expr) => {
        &get!($context, projects, $project, $column)[$task]
    };

    // Due tasks
    ($context:expr, due_tasks) => {
        $context.state.due_tasks()
    };

    ($context:expr, due_tasks, $task:expr) => {
        &get!($context, due_tasks)[$task]
    };

    // Tabs
    ($context:expr, tabs) => {
        &$context.config.tabs
    };
    ($context:expr, tabs, $tab:expr) => {
        get!($context, tabs)[$tab].columns
    };
    ($context:expr, tabs, $tab:expr, $column:expr) => {
        $context.config.tabs[$tab].columns[$column]
    };

    // Columns
    ($context:expr, columns) => {
        $context.config.columns
    };

    ($context:expr, columns, $column:expr) => {
        &get!(context, columns)[$column]
    };
}
