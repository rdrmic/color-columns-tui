use std::time::Duration;

use anyhow::Context;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::Game,
    stage_handlers::{FRAME_DURATION_IDLE, GameplayHandler, Stage, StageHandler},
};

pub struct GameOverHandler {
    is_stage_initialized: bool,
}

impl GameOverHandler {
    pub const fn new() -> Self {
        Self { is_stage_initialized: false }
    }
}

impl StageHandler for GameOverHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            game.start().context("Failed to start the game").ok()?;
            return Some(Stage::Gameplay(GameplayHandler::new()));
        }
        None
    }

    fn time_before_next_tick(&mut self, _game: &mut Game) -> Duration {
        if self.is_stage_initialized {
            self.is_stage_initialized = true;
            FRAME_DURATION_IDLE
        } else {
            Duration::ZERO
        }
    }

    fn update(&mut self, _game: &mut Game) -> Option<Stage> {
        None
    }
}
