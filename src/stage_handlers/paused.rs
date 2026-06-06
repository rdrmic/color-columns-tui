use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::{GameState, Millis},
    messages::Message,
    stage_handlers::{FRAME_DURATION_PAUSED, GameplayHandler, Stage, StageHandler},
};

pub struct PausedHandler {
    start_time: Millis,
}

impl PausedHandler {
    const FLICKER_DURATION: Millis = FRAME_DURATION_PAUSED;

    pub const fn new(game: &mut GameState) -> Self {
        let message = Message::new_permanent("Paused...", [170, 170, 170]);
        game.set_message(Some(message));

        Self { start_time: game.current_time() }
    }

    pub const fn flicker_tick(&self, game: &GameState) -> u64 {
        let elapsed = game.current_time().saturating_sub(self.start_time);
        elapsed / Self::FLICKER_DURATION
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

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Millis {
        FRAME_DURATION_PAUSED
    }

    fn update(&mut self, _game: &mut GameState) -> Option<Stage> {
        None
    }
}
