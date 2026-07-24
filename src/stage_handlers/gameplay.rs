use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    stage_handlers::{FRAME_DURATION, GameOverHandler, PausedHandler, Stage, StageHandler},
    visual_effects::Blinking,
};

pub struct GameplayHandler {
    gravity_time: Instant,
    blinking_labels: BlinkingLabels,
}

impl GameplayHandler {
    pub fn new(game: &GameState) -> Self {
        Self { gravity_time: Instant::now(), blinking_labels: BlinkingLabels::new(game) }
    }

    pub const fn blinking_labels(&self) -> &BlinkingLabels {
        &self.blinking_labels
    }

    fn try_updating_tick(&mut self, game: &mut GameState, next_tick: Instant) -> Option<Stage> {
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
            KeyCode::Esc => return Some(Stage::Paused(PausedHandler::new(game))),
            _ => (),
        }
        None
    }

    fn time_before_next_tick(&mut self, game: &mut GameState) -> Duration {
        let time_before_next_game_tick = game.current_tick_duration().checked_sub(self.gravity_time.elapsed()).unwrap_or(Duration::ZERO);
        FRAME_DURATION.min(time_before_next_game_tick)
    }

    fn update(&mut self, game: &mut GameState) -> Option<Stage> {
        // Use `while` instead of `if` to catch up if the process "hitched"
        let tick_rate = game.current_tick_duration();
        while self.gravity_time.elapsed() >= tick_rate {
            if let Some(gameover_stage) = self.try_updating_tick(game, self.gravity_time + tick_rate) {
                return Some(gameover_stage);
            }
        }

        self.blinking_labels.update(game);

        if let Some(msg) = game.message_mut()
            && !msg.tick()
        {
            game.set_message(None);
        }

        None
    }
}

// =============================================================================
// Blinking labels
// =============================================================================
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

    pub fn is_level_visible(&self) -> bool {
        self.level.is_visible()
    }

    pub fn is_max_combo_visible(&self) -> bool {
        self.max_combo.is_visible()
    }

    pub fn is_highscore_visible(&self) -> bool {
        let Some(highscore) = &self.highscore else {
            return false;
        };

        highscore.is_visible()
    }

    fn update(&mut self, game: &GameState) {
        let (level, max_combo, highscore) = Self::get_label_values(game);

        self.level.update(level);
        self.max_combo.update(max_combo);
        if let Some(self_highscore) = self.highscore.as_mut() {
            self_highscore.update(highscore);
        }
    }

    fn get_label_values(game: &GameState) -> (u32, u32, u32) {
        let scoring = game.scoring();
        (u32::from(scoring.level()), u32::from(scoring.max_combo()), scoring.highscore())
    }
}

struct BlinkingLabel {
    value: u32,
    blinking: Option<Blinking>,
}

impl BlinkingLabel {
    const fn new(initial_value: u32) -> Self {
        Self { value: initial_value, blinking: None }
    }

    fn is_visible(&self) -> bool {
        let Some(blinking) = self.blinking.as_ref() else {
            return true;
        };

        blinking.is_visible_phase()
    }

    fn update(&mut self, current_value: u32) {
        if current_value > self.value {
            self.value = current_value;
            self.blinking = Some(Blinking::new());
        }

        if let Some(blinking) = self.blinking.as_mut() {
            blinking.update();
        }
    }
}
