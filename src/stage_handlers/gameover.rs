use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    errors::Context,
    game_state::{GameState, Millis},
    messages::Message,
    stage_handlers::{FAILED_TO_START_GAME_ERROR, FRAME_DURATION_IDLE, GameplayHandler, Stage, StageHandler},
};

pub struct GameOverHandler;

impl GameOverHandler {
    pub const fn new(game: &mut GameState) -> Self {
        let message = Message::new_permanent("Game over!", [241, 76, 76]);
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

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Millis {
        FRAME_DURATION_IDLE
    }

    fn update(&mut self, _game: &mut GameState) -> Option<Stage> {
        None
    }
}
