use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::{GameState, Millis},
    stage_handlers::{FRAME_DURATION_IDLE, ReadyHandler, Stage, StageHandler},
};

pub struct InstructionsHandler;

impl StageHandler for InstructionsHandler {
    fn handle_key_pressed_event(&mut self, _game: &mut GameState, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            return Some(Stage::Ready(ReadyHandler));
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
