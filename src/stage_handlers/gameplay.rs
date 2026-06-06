use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::{GameState, Millis},
    stage_handlers::{FRAME_DURATION_GAMEPLAY, GameOverHandler, PausedHandler, Stage, StageHandler},
};

pub struct GameplayHandler {
    gravity_time: Millis,
    blinking_labels: BlinkingLabels,
}

impl GameplayHandler {
    pub fn new(game: &GameState) -> Self {
        Self { gravity_time: game.current_time(), blinking_labels: BlinkingLabels::new(game) }
    }

    pub const fn blinking_labels(&self) -> &BlinkingLabels {
        &self.blinking_labels
    }

    fn try_updating_tick(&mut self, game: &mut GameState, next_tick: Millis) -> Option<Stage> {
        if !game.tick() {
            return Some(Stage::GameOver(GameOverHandler::new(game)));
        }
        self.gravity_time = next_tick;
        None
    }
}

impl StageHandler for GameplayHandler {
    fn handle_key_pressed_event(&mut self, game: &mut GameState, key_event: KeyEvent) -> Option<Stage> {
        match key_event.code {
            KeyCode::Left => game.move_left(),
            KeyCode::Right => game.move_right(),
            KeyCode::Up => game.rotate_up(),
            KeyCode::Down => game.rotate_down(),
            KeyCode::Char('0') | KeyCode::Insert => {
                if let Some(gameover_stage) = self.try_updating_tick(game, game.current_time()) {
                    return Some(gameover_stage);
                }
            }
            KeyCode::Char(' ') => {
                game.drop();
                if let Some(gameover_stage) = self.try_updating_tick(game, game.current_time()) {
                    return Some(gameover_stage);
                }
            }
            KeyCode::Esc => return Some(Stage::Paused(PausedHandler::new(game))),
            _ => (),
        }
        None
    }

    fn time_before_next_tick(&mut self, game: &mut GameState) -> Millis {
        let elapsed = game.current_time().saturating_sub(self.gravity_time);

        FRAME_DURATION_GAMEPLAY.min(game.current_tick_duration().saturating_sub(elapsed))
    }

    fn update(&mut self, game: &mut GameState) -> Option<Stage> {
        let tick_duration = game.current_tick_duration();

        // Use `while` instead of `if` to catch up if the process "hitched"
        while game.current_time().saturating_sub(self.gravity_time) >= tick_duration {
            if let Some(gameover_stage) = self.try_updating_tick(game, self.gravity_time.saturating_add(tick_duration)) {
                return Some(gameover_stage);
            }
        }

        self.blinking_labels.update(game);

        None
    }
}

// ============================================================================
// Blinking labels
// ============================================================================
pub struct BlinkingLabels {
    level: BlinkingLabel,
    max_combo: BlinkingLabel,
    highscore: Option<BlinkingLabel>,
}

impl BlinkingLabels {
    fn new(game: &GameState) -> Self {
        let (level, max_combo, highscore) = Self::get_label_values(game);

        Self {
            level: BlinkingLabel::new(level),
            max_combo: BlinkingLabel::new(max_combo),
            highscore: if highscore > 0 { Some(BlinkingLabel::new(highscore)) } else { None },
        }
    }

    pub const fn has_level_blinked(&self, game: &GameState) -> bool {
        self.level.blinked(game)
    }

    pub const fn has_max_combo_blinked(&self, game: &GameState) -> bool {
        self.max_combo.blinked(game)
    }

    pub const fn has_highscore_blinked(&self, game: &GameState) -> bool {
        let Some(highscore) = &self.highscore else {
            return false;
        };

        highscore.blinked(game)
    }

    fn update(&mut self, game: &GameState) {
        let (level, max_combo, highscore) = Self::get_label_values(game);

        self.level.update(game, level);
        self.max_combo.update(game, max_combo);
        if let Some(self_highscore) = self.highscore.as_mut() {
            self_highscore.update(game, highscore);
        }
    }

    fn get_label_values(game: &GameState) -> (u32, u32, u32) {
        (u32::from(game.scoring().level()), u32::from(game.scoring().max_combo()), game.scoring().highscore())
    }
}

struct BlinkingLabel {
    value: u32,
    blink_time: Option<Millis>,
}

impl BlinkingLabel {
    const BLINK_DURATION: Millis = 450;

    const fn new(initial_value: u32) -> Self {
        Self { value: initial_value, blink_time: None }
    }

    const fn blinked(&self, game: &GameState) -> bool {
        let Some(blink_time) = self.blink_time else {
            return false;
        };

        let elapsed = game.current_time().saturating_sub(blink_time);
        let blink_state = (elapsed / Self::BLINK_DURATION) % 2;
        blink_state == 0
    }

    const fn update(&mut self, game: &GameState, current_value: u32) {
        if current_value > self.value {
            self.value = current_value;
            self.start_blink_time(game);
        }

        if let Some(blink_time) = self.blink_time {
            let elapsed = game.current_time().saturating_sub(blink_time);
            if elapsed >= Self::BLINK_DURATION * 3 {
                self.finish_blink_time();
            }
        }
    }

    const fn start_blink_time(&mut self, game: &GameState) {
        self.blink_time = Some(game.current_time());
    }

    const fn finish_blink_time(&mut self) {
        self.blink_time = None;
    }
}
