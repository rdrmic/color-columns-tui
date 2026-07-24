use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    errors::Context,
    game_state::GameState,
    messages::{Message, MessageType},
    stage_handlers::{FAILED_TO_START_GAME_ERROR, FRAME_DURATION, GameplayHandler, Stage, StageHandler},
};

pub struct GameOverHandler;

impl GameOverHandler {
    pub fn new(game: &mut GameState) -> Self {
        let message = Message::new_blinking(MessageType::GameOver);
        game.set_message(Some(message));

        Self
    }
}

impl StageHandler for GameOverHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            game.start().context(FAILED_TO_START_GAME_ERROR).ok()?;
            game.set_message(None);
            return Some(Stage::Gameplay(GameplayHandler::new(game)));
        }
        None
    }

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Duration {
        FRAME_DURATION
    }

    fn update(&mut self, _game: &mut GameState) -> Option<Stage> {
        None
    }
}
