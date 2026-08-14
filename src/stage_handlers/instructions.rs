use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    stage_handlers::{ReadyHandler, Stage, StageHandler, StageTransition},
};

pub struct InstructionsHandler;

impl StageHandler for InstructionsHandler {
    fn handle_key_pressed_event(&mut self, _game: &mut GameState, key_event: KeyEvent) -> StageTransition {
        if key_event.code == KeyCode::Enter {
            return Ok(Some(Stage::Ready(ReadyHandler)));
        }
        Ok(None)
    }
}
