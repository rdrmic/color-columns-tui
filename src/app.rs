use std::time::Duration;

use anyhow::Context;
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyEventKind},
    },
};

use crate::rendering;
use crate::stage_handlers::{Stage, StageHandler};
use crate::{game::Game, stage_handlers::ReadyHandler};

pub struct App {
    is_running: bool,
    stage: Stage,
    game: Game,
}

impl App {
    const FRAME_DURATION_GAMEPLAY: Duration = Duration::from_millis(16);
    const FRAME_DURATION_IDLE: Duration = Duration::from_hours(1);

    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { is_running: true, stage: Stage::Ready(ReadyHandler), game: Game::new()? })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        log::info!("Main loop is starting...");

        while self.is_running {
            terminal
                .draw(|frame| {
                    rendering::render(frame, &self.stage, &self.game);
                })
                .context("Failed to draw to terminal")?;

            // The poll waiting time is a MINIMUM of our frame rate and logic rate
            // This ensures we wake up exactly when the block needs to fall
            let event_waiting_time = match &self.stage {
                Stage::Gameplay(gameplay_handler) => {
                    let time_before_next_game_tick = gameplay_handler.time_before_next_tick(&self.game);
                    Self::FRAME_DURATION_GAMEPLAY.min(time_before_next_game_tick)
                }
                _ => Self::FRAME_DURATION_IDLE,
            };
            if crossterm::event::poll(event_waiting_time)? {
                self.handle_events(&crossterm::event::read()?);
            }

            self.tick();
        }

        Ok(())
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
                use crate::logging;
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

        // "dev-console" keys
        #[cfg(feature = "dev-console")]
        {
            use crate::logging;
            if logging::dev_console::handle_key_pressed_event(key_event) {
                return;
            }
        }

        // Stages keys
        if let Some(next_state) = self.stage.handle_key_pressed_event(&mut self.game, *key_event) {
            self.stage = next_state;
        }
    }

    fn tick(&mut self) {
        if let Some(next_state) = self.stage.update(&mut self.game) {
            self.stage = next_state;
        }
    }
}
