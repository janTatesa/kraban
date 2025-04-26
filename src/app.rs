pub mod config;
mod state;
mod ui;

use clap::ArgMatches;
use cli_log::{debug, info};
use color_eyre::Result;
use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use state::{Difficulty, Priority, State};
use ui::{Component, Prompt, Ui, View};

pub struct App {
    state: State,
    running: bool,
    is_testing: bool,
    ui: Ui,
    terminal: DefaultTerminal,
    config: Config,
}

#[derive(Clone, Copy)]
struct Context<'a> {
    state: &'a State,
    config: &'a Config,
}

macro_rules! context {
    ($self:expr) => {
        Context {
            state: &$self.state,
            config: &$self.config,
        }
    };
}

impl App {
    pub fn run(terminal: DefaultTerminal, cli: ArgMatches) -> Result<()> {
        let is_testing = *cli.get_one("testing").expect("Option has default value");
        let state = State::new(is_testing)?;
        let config = Config::new(&cli)?;
        Self {
            running: true,
            ui: Ui::new(Context {
                state: &state,
                config: &config,
            }),
            is_testing,
            terminal,
            state,
            config,
        }
        .main_loop()
    }

    fn main_loop(mut self) -> Result<()> {
        while self.running {
            self.terminal.draw(|frame| {
                self.ui
                    .render(frame.area(), frame.buffer_mut(), context!(self))
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        let event = event::read()?;
        debug!("{:?}", event);
        match event {
            Event::Key(
                key @ KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                },
            ) => self.on_key(key)?,
            Event::FocusGained => {
                self.state = State::new(self.is_testing)?;
                self.ui.refresh_on_state_change(context!(self));
            }
            _ => {}
        }
        Ok(())
    }

    fn on_key(&mut self, key_event: KeyEvent) -> Result<()> {
        if let KeyCode::Char('q') = key_event.code {
            info!("Quiting");
            self.running = false;
        } else {
            let Some(action) = self.ui.on_key(key_event, context!(self)) else {
                return Ok(());
            };
            let current_list = self.ui.current_list(&self.config);
            let action = self.state.handle_action(current_list, action)?;
            self.ui.refresh_on_state_change(context!(self));
            if let Some(action) = action {
                self.ui.handle_action(action);
            }
            self.state.save(self.is_testing)?
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Action {
    ClosePrompt,
    Delete,
    ChangePriority(Option<Priority>),
    ChangeDifficulty(Option<Difficulty>),
    New(String),
    Rename(String),
    MoveToColumn(String),
    SwitchToView(Box<dyn View>),
    OpenPrompt(Box<dyn Prompt>),
    SwitchToIndex(usize),
}
