use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::Game,
    stage_handlers::{FRAME_DURATION_IDLE, ReadyHandler, Stage, StageHandler},
};

pub struct InstructionsHandler;

impl StageHandler for InstructionsHandler {
    fn handle_key_pressed_event(&mut self, _game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            return Some(Stage::Ready(ReadyHandler));
        }
        None
    }

    fn time_before_next_tick(&mut self, _game: &mut Game) -> Duration {
        FRAME_DURATION_IDLE
    }

    fn update(&mut self, _game: &mut Game) -> Option<Stage> {
        None
    }
}
