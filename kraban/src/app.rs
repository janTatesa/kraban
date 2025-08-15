use cli_log::debug;
use color_eyre::Result;
use kraban_config::Config;
use kraban_state::State;
use kraban_ui::{Response, Ui};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind}
};

pub struct App<'a> {
    state: State,
    ui: Ui<'a>,
    config: &'a Config,
    terminal: DefaultTerminal
}

impl<'a> App<'a> {
    pub fn run(config: &'a Config) -> Result<()> {
        let terminal = ratatui::init();
        let state = State::new(config)?;
        Self {
            ui: Ui::default(),
            state,
            config,
            terminal
        }
        .main_loop()
    }

    fn main_loop(mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.area();
            let buf = frame.buffer_mut();
            self.ui.render(area, buf, &self.state, self.config)
        })?;

        if let Some(mut app) = self.handle_crossterm_events()? {
            app.state.save_if_needed()?;
            return app.main_loop();
        }

        Ok(())
    }

    fn handle_crossterm_events(mut self) -> Result<Option<Self>> {
        let event = event::read()?;
        debug!("{event:?}");
        match event {
            Event::Key(
                key @ KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                }
            ) => return self.on_key(key),
            Event::FocusGained => self.state = State::new(self.config)?,
            _ => {}
        }

        Ok(Some(self))
    }

    fn on_key(self, key: KeyEvent) -> Result<Option<Self>> {
        let App {
            mut state,
            ui,
            config,
            terminal
        } = self;

        let app = match ui.on_key(key, &mut state, config) {
            Response::Quit => None,
            Response::Update(ui) => Some(Self {
                state,
                ui,
                config,
                terminal
            })
        };

        Ok(app)
    }
}
