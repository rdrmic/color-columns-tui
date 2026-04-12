use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::Game,
    stage_handlers::{GameplayHandler, Stage, StageHandler},
};

pub struct GameOverHandler;

impl StageHandler for GameOverHandler {
    fn handle_key_pressed_event(&mut self, game: &mut Game, key_event: KeyEvent) -> Option<Stage> {
        if key_event.code == KeyCode::Enter {
            game.start();
            return Some(Stage::Gameplay(GameplayHandler::new()));
        }
        None
    }

    fn update(&mut self, _game: &mut Game) -> Option<Stage> {
        None
    }
}
