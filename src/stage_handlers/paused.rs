use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    messages::{Message, MessageType},
    stage_handlers::{FRAME_DURATION_PAUSED, GameplayHandler, Stage, StageHandler},
};

#[derive(Copy, Clone)]
pub struct PausedHandler {
    start_time: Instant,
}

impl PausedHandler {
    const FLICKER_DURATION: u64 = FRAME_DURATION_PAUSED.as_millis() as u64;

    pub fn new(game: &mut GameState) -> Self {
        let message = Message::new_permanent(MessageType::Paused);
        game.set_message(Some(message));

        Self { start_time: Instant::now() }
    }

    pub fn flicker_tick(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64 / Self::FLICKER_DURATION
    }
}

impl StageHandler for PausedHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            game.set_message(None);
            return Some(Stage::Gameplay(GameplayHandler::new(game)));
        }
        None
    }

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Duration {
        FRAME_DURATION_PAUSED
    }

    fn update(&mut self, _game: &mut GameState) -> Option<Stage> {
        None
    }
}
