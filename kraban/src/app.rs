use clap::ArgMatches;
use cli_log::{debug, info};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use kraban_config::Config;
use kraban_state::{State, SwitchToIndex};
use kraban_ui::Ui;
use kraban_ui::{Context, context};
use ratatui::DefaultTerminal;

pub struct App {
    state: State,
    running: bool,
    is_testing: bool,
    ui: Ui,
    config: Config,
    terminal: DefaultTerminal,
}

impl App {
    pub fn run(terminal: DefaultTerminal, cli: ArgMatches) -> Result<()> {
        let is_testing = *cli.get_one("testing").expect("Option has default value");
        let state = State::new(is_testing)?;
        let config = Config::new(*cli.get_one("testing").expect("Option has default value"))?;
        Self {
            running: true,
            ui: Ui::new(state.projects().len().checked_sub(1)),
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
                self.state.compile_due_tasks_list(&self.config);
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
                self.state.compile_due_tasks_list(&self.config);
                self.ui.refresh_on_state_change(context!(self));
            }
            _ => {}
        }
        Ok(())
    }

    fn on_key(&mut self, key_event: KeyEvent) -> Result<()> {
        if let KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } = key_event
        {
            info!("Quiting");
            self.running = false;
        } else {
            let Some(action) = self.ui.get_action(key_event, context!(self)) else {
                return Ok(());
            };
            let current_list = self.ui.current_list(&self.config);
            let action = self
                .state
                .handle_action(current_list, action, &self.config)?;
            self.ui.refresh_on_state_change(context!(self));
            if let Some(SwitchToIndex(index)) = action {
                self.ui.switch_to_index(index);
            }
            self.state.save(self.is_testing)?
        }
        Ok(())
    }
}
