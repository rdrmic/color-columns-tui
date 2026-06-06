use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    errors::Context,
    game_state::{GameState, Millis},
    messages::Message,
    stage_handlers::{FAILED_TO_START_GAME_ERROR, FRAME_DURATION_IDLE, GameplayHandler, InstructionsHandler, Stage, StageHandler},
};

pub struct ReadyHandler;

impl ReadyHandler {
    pub const fn new(game: &mut GameState) -> Self {
        let message = Message::new_fading("Get ready!", [0, 170, 0], 1, 5);
        game.set_message(Some(message));

        Self
    }
}

impl StageHandler for ReadyHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> Option<Stage> {
        match key_event.code {
            KeyCode::Enter => {
                game.start().context(FAILED_TO_START_GAME_ERROR).ok()?;
                Some(Stage::Gameplay(GameplayHandler::new(game)))
            }
            KeyCode::F(1) => Some(Stage::Instructions(InstructionsHandler)),
            _ => None,
        }
    }

    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Millis {
        FRAME_DURATION_IDLE
    }

    fn update(&mut self, _game: &mut GameState) -> Option<Stage> {
        None
    }
}
