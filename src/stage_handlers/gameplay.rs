use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::Game,
    stage_handlers::{GameOverHandler, Stage, StageHandler},
};

pub struct GameplayHandler {
    last_tick: Instant, // Tracks the last time the block moved down (gravity)
}

impl GameplayHandler {
    pub fn new() -> Self {
        Self { last_tick: Instant::now() }
    }

    /// Returns the duration until the next game tick should occur
    pub fn time_before_next_tick(&self, game: &Game) -> Duration {
        game.tick_rate().checked_sub(self.last_tick.elapsed()).unwrap_or(Duration::ZERO)
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
                let is_still_running = game.tick();
                if !is_still_running {
                    return Some(Stage::GameOver(GameOverHandler));
                }
            }
            KeyCode::Char(' ') => game.drop(),
            _ => (),
        }
        None
    }

    fn update(&mut self, game: &mut Game) -> Option<Stage> {
        let tick_rate = game.tick_rate();

        // Use `while` instead of `if` to catch up if the computer hitched
        while self.last_tick.elapsed() >= tick_rate {
            let is_running = game.tick();
            if !is_running {
                return Some(Stage::GameOver(GameOverHandler));
            }
            self.last_tick += tick_rate;
        }
        None
    }
}
