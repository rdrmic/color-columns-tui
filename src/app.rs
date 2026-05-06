use std::path::Path;

use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyEventKind},
    },
};

#[cfg(feature = "dev-console")]
use crate::logging;

use crate::{
    errors::{self, Context},
    game::Game,
    rendering,
    stage_handlers::{ReadyHandler, Stage, StageHandler},
};

pub struct App {
    is_running: bool,
    stage: Stage,
    game: Game,
}

impl App {
    pub fn new(app_state_dir_path: Option<&Path>) -> Result<Self, errors::Error> {
        Ok(Self { is_running: true, stage: Stage::Ready(ReadyHandler), game: Game::new(app_state_dir_path)? })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), errors::Error> {
        log::info!("Main loop is starting...");

        while self.is_running {
            terminal
                .draw(|frame| {
                    rendering::render(frame, &self.stage, &self.game);
                })
                .context("Failed to draw to terminal")?;

            let event_waiting_time = self.stage.time_before_next_tick(&mut self.game);
            if crossterm::event::poll(event_waiting_time)? {
                self.handle_events(&crossterm::event::read()?);
            }

            self.tick();
        }

        Ok(())
    }

    fn tick(&mut self) {
        if let Some(next_stage) = self.stage.update(&mut self.game) {
            self.stage = next_stage;
        }
    }

    fn handle_events(&mut self, event: &Event) {
        match event {
            // Keyboard events
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.handle_key_pressed_event(key_event),

            // Mouse events for "dev-console" scrolling
            #[cfg(feature = "dev-console")]
            Event::Mouse(mouse_event)
                if matches!(mouse_event.kind, crossterm::event::MouseEventKind::ScrollUp | crossterm::event::MouseEventKind::ScrollDown) =>
            {
                logging::dev_console::handle_mouse_scroll_event(*mouse_event);
            }
            _ => (),
        }
    }

    fn handle_key_pressed_event(&mut self, key_event: &KeyEvent) {
        // Global keys
        if let KeyCode::Char('q' | 'Q') = key_event.code {
            self.is_running = false;
            return;
        }

        // Stages keys
        if let Some(next_stage) = self.stage.handle_key_pressed_event(&mut self.game, *key_event) {
            self.stage = next_stage;
            #[cfg(feature = "dev-console")]
            return;
        }

        // "dev-console" keys
        #[cfg(feature = "dev-console")]
        logging::dev_console::handle_key_pressed_event(key_event);
    }
}
