mod gameover;
mod gameplay;
mod instructions;
mod paused;
mod ready;

use std::time::Duration;

use ratatui::crossterm::event::KeyEvent;

use crate::{
    errors::{Context, Error},
    game_state::GameState,
};

pub use gameover::GameOverHandler;
pub use gameplay::GameplayHandler;
pub use instructions::InstructionsHandler;
pub use paused::PausedHandler;
pub use ready::ReadyHandler;

const ERROR_FAILED_TO_START_GAME: &str = "Failed to start the game";

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

type StageTransition = Result<Option<Stage>, Error>;

#[cold]
#[inline(never)]
fn enter_gameplay(game: &mut GameState, restart: bool, clear_message: bool) -> StageTransition {
    if restart {
        game.start().context(ERROR_FAILED_TO_START_GAME)?;
    }
    if clear_message {
        game.set_message(None);
    }
    Ok(Some(Stage::Gameplay(GameplayHandler::new(game))))
}

pub trait StageHandler {
    fn time_before_next_tick(&mut self, _game: &mut GameState) -> Duration {
        FRAME_DURATION_IDLE
    }

    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> StageTransition;

    fn update(&mut self, _game: &mut GameState) -> Option<Stage> {
        None
    }
}

impl StageHandler for Stage {
    fn time_before_next_tick(&mut self, game: &mut GameState) -> Duration {
        match self {
            Self::Ready(_) | Self::Instructions(_) => FRAME_DURATION_IDLE,
            Self::Gameplay(handler) => handler.time_before_next_tick(game),
            Self::Paused(_) => FRAME_DURATION_PAUSED,
            Self::GameOver(_) => FRAME_DURATION,
        }
    }

    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> StageTransition {
        match self {
            Self::Ready(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::Gameplay(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::Paused(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::Instructions(handler) => handler.handle_key_pressed_event(game, key_event),
            Self::GameOver(handler) => handler.handle_key_pressed_event(game, key_event),
        }
    }

    fn update(&mut self, game: &mut GameState) -> Option<Stage> {
        match self {
            Self::Gameplay(handler) => handler.update(game),
            _ => None,
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::*;

    const ENTER: KeyEvent = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);

    #[test]
    fn shared_gameplay_entry_preserves_each_stage_transition() {
        let mut game = GameState::new(None).expect("game without persistence should initialize");

        let mut ready = ReadyHandler::new(&mut game);
        let next_stage = ready.handle_key_pressed_event(&mut game, ENTER).expect("ready transition should succeed").expect("ready should enter gameplay");
        assert!(matches!(next_stage, Stage::Gameplay(_)));
        assert_eq!(game.message().map(crate::messages::Message::text), Some("Get ready!"));

        let mut paused = PausedHandler::new(&mut game);
        let next_stage = paused.handle_key_pressed_event(&mut game, ENTER).expect("resume transition should succeed").expect("pause should enter gameplay");
        assert!(matches!(next_stage, Stage::Gameplay(_)));
        assert!(game.message().is_none());

        let mut game_over = GameOverHandler::new(&mut game);
        let next_stage =
            game_over.handle_key_pressed_event(&mut game, ENTER).expect("restart transition should succeed").expect("game over should enter gameplay");
        assert!(matches!(next_stage, Stage::Gameplay(_)));
        assert!(game.message().is_none());
    }
}
