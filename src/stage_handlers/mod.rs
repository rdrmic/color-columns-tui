use std::time::Duration;

use ratatui::crossterm::event::KeyEvent;

use crate::game::Game;

mod gameover;
mod gameplay;
mod instructions;
mod paused;
mod ready;

pub use gameover::GameOverHandler;
pub use gameplay::GameplayHandler;
pub use instructions::InstructionsHandler;
pub use paused::PausedHandler;
pub use ready::ReadyHandler;

const FRAME_DURATION_IDLE: Duration = Duration::from_hours(1);
const FRAME_DURATION_GAMEPLAY: Duration = Duration::from_millis(16);
const FRAME_DURATION_PAUSED: Duration = Duration::from_millis(76);

pub enum Stage {
    Ready(ReadyHandler),
    Gameplay(GameplayHandler),
    Paused(PausedHandler),
    Instructions(InstructionsHandler),
    GameOver(GameOverHandler),
}

pub trait StageHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage>;
    fn time_before_next_tick(&mut self, game: &mut Game) -> Duration;
    fn update(&mut self, game: &mut Game) -> Option<Stage>;
}

impl StageHandler for Stage {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        match self {
            Self::Ready(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::Gameplay(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::Paused(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::Instructions(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::GameOver(handler) => handler.handle_key_pressed_event(game, key_event),
        }
    }

    fn time_before_next_tick(&mut self, game: &mut Game) -> Duration {
        match self {
            Self::Ready(handler) => handler.time_before_next_tick(game),
            Self::Gameplay(handler) => handler.time_before_next_tick(game),
            Self::Paused(handler) => handler.time_before_next_tick(game),
            Self::Instructions(handler) => handler.time_before_next_tick(game),
            Self::GameOver(handler) => handler.time_before_next_tick(game),
        }
    }

    fn update(&mut self, game: &mut Game) -> Option<Stage> {
        match self {
            Self::Ready(handler) => handler.update(game),
            Self::Gameplay(handler) => handler.update(game),
            Self::Paused(handler) => handler.update(game),
            Self::Instructions(handler) => handler.update(game),
            Self::GameOver(handler) => handler.update(game),
        }
    }
}
