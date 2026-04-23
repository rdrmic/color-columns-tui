use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::Game,
    stage_handlers::{FRAME_DURATION_PAUSED, GameplayHandler, Stage, StageHandler},
};

const FLICKER_DURATION: u64 = FRAME_DURATION_PAUSED.as_millis() as u64;

pub struct PausedHandler {
    start_time: Instant,
}

impl PausedHandler {
    pub fn new() -> Self {
        Self { start_time: Instant::now() }
    }

    pub fn flicker_tick(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64 / FLICKER_DURATION
    }
}

impl StageHandler for PausedHandler {
    fn handle_key_pressed_event(&mut self, _game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            return Some(Stage::Gameplay(GameplayHandler::new()));
        }
        None
    }

    fn time_before_next_tick(&mut self, _game: &mut Game) -> Duration {
        FRAME_DURATION_PAUSED
    }

    fn update(&mut self, _game: &mut Game) -> Option<Stage> {
        None
    }
}
