mod config;
mod state;
mod ui;

use clap::ArgMatches;
use cli_log::{debug, info};
use color_eyre::Result;
use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use state::State;
use ui::{Component, Ui};

pub struct App {
    state: State,
    running: bool,
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
    pub fn run(terminal: DefaultTerminal, _cli: ArgMatches) -> Result<()> {
        let state = State::new()?;
        let config = Config::new()?;
        Self {
            running: true,
            ui: Ui::new(Context {
                state: &state,
                config: &config,
            }),
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
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                debug!("Handling key event {:?}", key);
                self.on_key(key)?
            }
        }
        Ok(())
    }

    fn on_key(&mut self, key_event: KeyEvent) -> Result<()> {
        if let KeyCode::Char('q') = key_event.code {
            self.quit()
        } else {
            let Some(action) = self.ui.on_key(key_event, context!(self)) else {
                return Ok(());
            };
            let current_list = self.ui.current_list(&self.config);
            if let Some(action) = self.state.handle_action(current_list, action) {
                self.ui.handle_action(action, context!(self));
            }
            Ok(())
        }
    }

    fn quit(&mut self) -> Result<()> {
        info!("Quiting");
        self.state.save()?;
        self.running = false;
        Ok(())
    }
}
