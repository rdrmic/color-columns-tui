use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    messages::Message,
    stage_handlers::{FRAME_DURATION_PAUSED, StageHandler, StageTransition, enter_gameplay},
    visual_effects,
};

#[derive(Copy, Clone)]
pub struct PausedHandler {
    start_time: Instant,
}

impl PausedHandler {
    const FLICKER_DURATION: u64 = FRAME_DURATION_PAUSED.as_millis() as u64;

    pub fn new(game: &mut GameState) -> Self {
        let message = Message::new_paused();
        game.set_message(Some(message));

        Self { start_time: Instant::now() }
    }

    pub fn flicker_tick(&self) -> u64 {
        visual_effects::elapsed_phase(&self.start_time, Self::FLICKER_DURATION)
    }
}

impl StageHandler for PausedHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> StageTransition {
        if key_event.code != KeyCode::Enter {
            return Ok(None);
        }

        enter_gameplay(game, false, true)
    }

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Duration {
        FRAME_DURATION_PAUSED
    }
}
