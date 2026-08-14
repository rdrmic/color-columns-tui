use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    messages::Message,
    stage_handlers::{FRAME_DURATION, StageHandler, StageTransition, enter_gameplay},
};

pub struct GameOverHandler;

impl GameOverHandler {
    pub fn new(game: &mut GameState) -> Self {
        let message = Message::new_game_over();
        game.set_message(Some(message));

        Self
    }
}

impl StageHandler for GameOverHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> StageTransition {
        if key_event.code != KeyCode::Enter {
            return Ok(None);
        }

        enter_gameplay(game, true, true)
    }

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Duration {
        FRAME_DURATION
    }
}
