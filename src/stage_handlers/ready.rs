use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    messages::Message,
    stage_handlers::{InstructionsHandler, Stage, StageHandler, StageTransition, enter_gameplay},
};

pub struct ReadyHandler;

impl ReadyHandler {
    pub const fn new(game: &mut GameState) -> Self {
        let message = Message::new_get_ready();
        game.set_message(Some(message));

        Self
    }
}

impl StageHandler for ReadyHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> StageTransition {
        match key_event.code {
            KeyCode::Enter => enter_gameplay(game, true, false),
            KeyCode::F(1) => Ok(Some(Stage::Instructions(InstructionsHandler))),
            _ => Ok(None),
        }
    }
}
