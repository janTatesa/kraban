use clap::ArgMatches;
use cli_log::{debug, info};
use color_eyre::Result;
use kraban_config::Config;
use kraban_state::State;
use kraban_ui::Ui;
use kraban_ui::{Context, context};
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub struct App<'a> {
    state: State,
    running: bool,
    is_testing: bool,
    ui: Ui<'a>,
    config: &'a Config,
    terminal: DefaultTerminal,
}

impl<'a> App<'a> {
    pub fn run(config: &'a Config, cli: ArgMatches) -> Result<()> {
        let terminal = ratatui::init();
        let is_testing = *cli.get_one("testing").expect("Option has default value");
        let state = State::new(is_testing)?;
        Self {
            running: true,
            ui: Ui::default(),
            is_testing,
            state,
            config,
            terminal,
        }
        .main_loop()
    }

    fn main_loop(mut self) -> Result<()> {
        while self.running {
            if self.ui.in_main_view() {
                self.state.compile_due_tasks_list(self.config);
            }
            self.terminal.draw(|frame| {
                self.ui
                    .redraw(frame.area(), frame.buffer_mut(), context!(self))
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        let event = event::read()?;
        debug!("{event:?}");
        match event {
            Event::Key(
                key @ KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                },
            ) => self.on_key(key)?,
            Event::FocusGained => self.state = State::new(self.is_testing)?,
            _ => {}
        }
        Ok(())
    }

    fn on_key(&mut self, key_event: KeyEvent) -> Result<()> {
        if let KeyCode::Char('q') = key_event.code {
            info!("Quiting");
            self.running = false;
            return Ok(());
        }

        self.on_non_quit_key(key_event)
    }

    fn on_non_quit_key(&mut self, key_event: KeyEvent) -> Result<()> {
        let Some(action) = self.ui.get_action(key_event, context!(self)) else {
            return Ok(());
        };

        let current_item = self.ui.current_item();
        self.state.handle_action(current_item, action, self.config);
        self.state.save(self.is_testing)
    }
}
