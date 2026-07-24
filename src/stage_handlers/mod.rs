mod gameover;
mod gameplay;
mod instructions;
mod paused;
mod ready;

use std::time::Duration;

use ratatui::crossterm::event::KeyEvent;

use crate::game_state::GameState;

pub use gameover::GameOverHandler;
pub use gameplay::GameplayHandler;
pub use instructions::InstructionsHandler;
pub use paused::PausedHandler;
pub use ready::ReadyHandler;

const FAILED_TO_START_GAME_ERROR: &str = "Failed to start the game";

pub const FRAME_DURATION: Duration = Duration::from_millis(33);
const FRAME_DURATION_IDLE: Duration = Duration::from_hours(1);
const FRAME_DURATION_PAUSED: Duration = Duration::from_millis(76);

pub enum Stage {
    Ready(ReadyHandler),
    Gameplay(GameplayHandler),
    Paused(PausedHandler),
    Instructions(InstructionsHandler),
    GameOver(GameOverHandler),
}

pub trait StageHandler {
    fn time_before_next_tick(&mut self, game: &mut GameState) -> Duration;
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> Option<Stage>;
    fn update(&mut self, game: &mut GameState) -> Option<Stage>;
}

macro_rules! execute_handler_method {
    ($self:ident, $method:ident $(, $args:expr)*) => {
        match $self {
            Self::Ready(handler) => handler.$method($($args),*),
            Self::Gameplay(handler) => handler.$method($($args),*),
            Self::Paused(handler) => handler.$method($($args),*),
            Self::Instructions(handler) => handler.$method($($args),*),
            Self::GameOver(handler) => handler.$method($($args),*),
        }
    };
}

impl StageHandler for Stage {
    fn time_before_next_tick(&mut self, game: &mut GameState) -> Duration {
        execute_handler_method!(self, time_before_next_tick, game)
    }

    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> Option<Stage> {
        execute_handler_method!(self, handle_key_pressed_event, game, key_event)
    }

    fn update(&mut self, game: &mut GameState) -> Option<Stage> {
        execute_handler_method!(self, update, game)
    }
}
