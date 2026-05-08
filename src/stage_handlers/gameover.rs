use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    errors::Context,
    game::Game,
    stage_handlers::{FAILED_TO_START_GAME_ERROR, FRAME_DURATION_IDLE, GameplayHandler, Stage, StageHandler},
};

pub struct GameOverHandler;

impl StageHandler for GameOverHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            game.start().context(FAILED_TO_START_GAME_ERROR).ok()?;
            return Some(Stage::Gameplay(GameplayHandler::new()));
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
