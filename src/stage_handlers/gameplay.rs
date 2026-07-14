use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game_state::GameState,
    stage_handlers::{FRAME_DURATION_GAMEPLAY, GameOverHandler, PausedHandler, Stage, StageHandler},
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
        FRAME_DURATION_GAMEPLAY.min(time_before_next_game_tick)
    }

    fn update(&mut self, game: &mut GameState) -> Option<Stage> {
        let tick_rate = game.current_tick_duration();

        // Use `while` instead of `if` to catch up if the process "hitched"
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

    pub fn has_level_blinked(&self) -> bool {
        self.level.blinked()
    }

    pub fn has_max_combo_blinked(&self) -> bool {
        self.max_combo.blinked()
    }

    pub fn has_highscore_blinked(&self) -> bool {
        let Some(highscore) = &self.highscore else {
            return false;
        };

        highscore.blinked()
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
        (u32::from(game.scoring().level()), u32::from(game.scoring().max_combo()), game.scoring().highscore())
    }
}

struct BlinkingLabel {
    value: u32,
    blink_time: Option<Instant>,
}

impl BlinkingLabel {
    const BLINK_DURATION: u64 = 475;
    const NUM_PHASES: u64 = 3;

    const fn new(initial_value: u32) -> Self {
        Self { value: initial_value, blink_time: None }
    }

    fn blinked(&self) -> bool {
        let Some(blink_time) = self.blink_time else {
            return false;
        };

        let elapsed_ms = blink_time.elapsed().as_millis() as u64;
        let blink_state = (elapsed_ms / Self::BLINK_DURATION) % 2;
        blink_state == 0 // black phase
    }

    fn update(&mut self, current_value: u32) {
        if current_value > self.value {
            self.value = current_value;
            self.start_blink_time();
        }

        if let Some(blink_time) = self.blink_time {
            let elapsed_ms = blink_time.elapsed().as_millis() as u64;
            if elapsed_ms >= Self::BLINK_DURATION * Self::NUM_PHASES {
                self.finish_blink_time();
            }
        }
    }

    fn start_blink_time(&mut self) {
        self.blink_time = Some(Instant::now());
    }

    const fn finish_blink_time(&mut self) {
        self.blink_time = None;
    }
}
