use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Ok};

use crate::blocks::{Column, Pile};

// TODO pub
pub struct GameState {
    column_falling: Column,
    pile: Pile,
    score: u32,
    rng: fastrand::Rng,
    current_tick_rate: Duration,
}

impl GameState {
    pub const BOARD_WIDTH: u8 = 6;
    pub const BOARD_HEIGHT: i8 = 13;

    const INITIAL_TICK_RATE: Duration = Duration::from_millis(750);
    const MIN_TICK_RATE: Duration = Duration::from_millis(100); // Speed cap

    pub fn new() -> anyhow::Result<Self> {
        let mut rng = create_rng()?;

        Ok(Self {
            column_falling: Self::create_column(&mut rng),
            pile: Pile::new(Self::BOARD_WIDTH, Self::BOARD_HEIGHT as u8),
            score: 0,
            rng,
            current_tick_rate: Self::INITIAL_TICK_RATE,
        })
    }

    fn create_column(rng: &mut fastrand::Rng) -> Column {
        let x = rng.u8(..Self::BOARD_WIDTH);
        Column::new(x, -3, rng)
    }

    fn spawn_new_column(&mut self) {
        self.column_falling = Self::create_column(&mut self.rng);
    }

    pub fn tick(&mut self) -> bool {
        let next_y_bottom = self.column_falling.y_bottom() + 1;
        let x = self.column_falling.x();

        // Check collision with floor OR pile
        if next_y_bottom < Self::BOARD_HEIGHT && !self.pile.is_occupied(x, next_y_bottom) {
            self.column_falling.move_down(1);
        } else {
            let is_locked = self.lock_column();
            if !is_locked {
                return false;
            }
        }

        // Example Acceleration Logic:
        // Every time the score increases, we reduce the tick duration by 2%
        // until we hit the MIN_TICK_RATE.
        if self.score > 0 && self.score.is_multiple_of(100) {
            self.accelerate(0.98);
        }
        true
    }

    #[allow(clippy::cast_sign_loss)]
    fn lock_column(&mut self) -> bool {
        let is_set = self.pile.set(&self.column_falling);
        if !is_set {
            return false;
        }

        self.spawn_new_column();

        self.tick();
        true
    }

    pub fn move_left(&mut self) {
        let x = self.column_falling.x();
        let y_bottom = self.column_falling.y_bottom();

        // If the bottom gem can move left, the whole column can move left
        if x > 0 && !self.pile.is_occupied(x - 1, y_bottom) {
            self.column_falling.move_left();
        }
    }

    pub fn move_right(&mut self) {
        let x = self.column_falling.x();
        let y_bottom = self.column_falling.y_bottom();

        // If the bottom gem can move right, the whole column can move right
        if x < Self::BOARD_WIDTH - 1 && !self.pile.is_occupied(x + 1, y_bottom) {
            self.column_falling.move_right();
        }
    }

    pub const fn rotate_up(&mut self) {
        self.column_falling.rotate_up();
    }

    pub const fn rotate_down(&mut self) {
        self.column_falling.rotate_down();
    }

    pub fn drop(&mut self) {
        let x = self.column_falling.x();

        // Find the first occupied row in the target column (top-down)
        let first_occupied_y = (0..Self::BOARD_HEIGHT).find(|&y| self.pile.is_occupied(x, y)).unwrap_or(Self::BOARD_HEIGHT);

        // Target is the row immediately above the first occupied gem (or the floor)
        let distance = (first_occupied_y - 1) - self.column_falling.y_bottom();

        if distance > 0 {
            self.column_falling.move_down(distance);
        }

        self.tick();
    }

    fn accelerate(&mut self, factor: f64) {
        let new_tick_rate = self.current_tick_rate.mul_f64(factor);

        // Ensure we don't go faster than our speed cap (MIN_TICK_RATE)
        self.current_tick_rate = new_tick_rate.max(Self::MIN_TICK_RATE);
    }

    // Getter so the App knows how long to wait
    pub const fn tick_rate(&self) -> Duration {
        self.current_tick_rate
    }

    pub const fn get_falling_column(&self) -> &Column {
        &self.column_falling
    }

    pub const fn get_pile(&self) -> &Pile {
        &self.pile
    }
}

fn create_rng() -> anyhow::Result<fastrand::Rng> {
    let now = SystemTime::now();
    let seed = now
        .duration_since(UNIX_EPOCH)
        .with_context(|| format!("System clock is set before 1970? Current time: {now:?}"))
        .context("Failed to generate a random seed from system time")?
        .as_millis() as u64;
    Ok(fastrand::Rng::with_seed(seed))
}
