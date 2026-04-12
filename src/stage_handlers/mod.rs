use ratatui::crossterm::event::KeyEvent;

use crate::game::Game;

mod gameover;
mod gameplay;
mod ready;

pub use gameover::GameOverHandler;
pub use gameplay::GameplayHandler;
pub use ready::ReadyHandler;

pub enum Stage {
    Ready(ReadyHandler),
    Gameplay(GameplayHandler),
    //Paused(PausedHandler),
    //Help(HelpHandler),
    GameOver(GameOverHandler),
}

pub trait StageHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage>;
    fn update(&mut self, game: &mut Game) -> Option<Stage>;
}

impl StageHandler for Stage {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        match self {
            Self::Ready(stage) => stage.handle_key_pressed_event(game, key_event),
            Self::Gameplay(stage) => stage.handle_key_pressed_event(game, key_event),
            Self::GameOver(stage) => stage.handle_key_pressed_event(game, key_event),
        }
    }

    fn update(&mut self, game: &mut Game) -> Option<Stage> {
        match self {
            Self::Ready(stage) => stage.update(game),
            Self::Gameplay(stage) => stage.update(game),
            Self::GameOver(stage) => stage.update(game),
        }
    }
}
