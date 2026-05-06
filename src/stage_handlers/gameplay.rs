use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::Game,
    stage_handlers::{FRAME_DURATION_GAMEPLAY, GameOverHandler, PausedHandler, Stage, StageHandler},
};

pub struct GameplayHandler {
    last_tick: Instant, // tracks the last time the block moved down (gravity)
}

impl GameplayHandler {
    pub fn new() -> Self {
        Self { last_tick: Instant::now() }
    }

    fn try_updating_tick(&mut self, game: &mut Game, next_tick: Instant) -> Option<Stage> {
        if !game.tick() {
            return Some(Stage::GameOver(GameOverHandler::new()));
        }
        self.last_tick = next_tick;
        None
    }
}

impl StageHandler for GameplayHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        match key_event.code {
            KeyCode::Left => game.move_left(),
            KeyCode::Right => game.move_right(),
            KeyCode::Up => game.rotate_up(),
            KeyCode::Down => game.rotate_down(),
            KeyCode::Char('0') | KeyCode::Insert => {
                if let Some(gameover_stage) = self.try_updating_tick(game, Instant::now()) {
                    return Some(gameover_stage);
                }
            }
            KeyCode::Char(' ') => {
                game.drop();
                if let Some(gameover_stage) = self.try_updating_tick(game, Instant::now()) {
                    return Some(gameover_stage);
                }
            }
            KeyCode::Esc => return Some(Stage::Paused(PausedHandler::new())),
            _ => (),
        }
        None
    }

    fn time_before_next_tick(&mut self, game: &mut Game) -> Duration {
        let time_before_next_game_tick = game.tick_rate().checked_sub(self.last_tick.elapsed()).unwrap_or(Duration::ZERO);
        FRAME_DURATION_GAMEPLAY.min(time_before_next_game_tick)
    }

    fn update(&mut self, game: &mut Game) -> Option<Stage> {
        let tick_rate = game.tick_rate();

        // Use `while` instead of `if` to catch up if the computer hitched
        while self.last_tick.elapsed() >= tick_rate {
            if let Some(gameover_stage) = self.try_updating_tick(game, self.last_tick + tick_rate) {
                return Some(gameover_stage);
            }
        }

        None
    }
}
