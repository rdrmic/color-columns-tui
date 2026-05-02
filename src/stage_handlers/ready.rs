use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    errors::Context,
    game::Game,
    stage_handlers::{FRAME_DURATION_IDLE, GameplayHandler, InstructionsHandler, Stage, StageHandler},
};

pub struct ReadyHandler;

impl StageHandler for ReadyHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        match key_event.code {
            KeyCode::Enter => {
                game.start().context("Failed to start the game").ok()?;
                Some(Stage::Gameplay(GameplayHandler::new()))
            }
            KeyCode::F(1) => Some(Stage::Instructions(InstructionsHandler)),
            _ => None,
        }
    }

    fn time_before_next_tick(&mut self, _game: &mut Game) -> Duration {
        FRAME_DURATION_IDLE
    }

    fn update(&mut self, _game: &mut Game) -> Option<Stage> {
        None
    }
}
