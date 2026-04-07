use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Ok};

use crate::blocks::Column;

// TODO pub
pub struct GameState {
    pub column_falling: Column,
    pub score: u32,
    //pub rng: fastrand::Rng,
    current_tick_rate: Duration,
}

impl GameState {
    pub const BOARD_WIDTH: u8 = 6;
    pub const BOARD_HEIGHT: i8 = 13;

    const INITIAL_TICK_RATE: Duration = Duration::from_millis(750);
    const MIN_TICK_RATE: Duration = Duration::from_millis(100); // Speed cap

    pub fn new() -> anyhow::Result<Self> {
        let mut rng = create_rng()?;

        let column_falling_x = rng.u8(..Self::BOARD_WIDTH);

        Ok(Self {
            column_falling: Column::new(column_falling_x, -3, &mut rng),
            score: 0,
            //rng,
            current_tick_rate: Self::INITIAL_TICK_RATE,
        })
    }

    pub fn tick(&mut self) {
        // Determine if we can move down
        let bottom_y = self.column_falling.blocks[2].y;
        if bottom_y < Self::BOARD_HEIGHT - 1 {
            self.column_falling.move_down();
        } else {
            // Landing logic will be implemented in a future step
        }

        // Example Acceleration Logic:
        // Every time the score increases, we reduce the tick duration by 2%
        // until we hit the MIN_TICK_RATE.
        if self.score > 0 && self.score.is_multiple_of(100) {
            self.accelerate(0.98);
        }
    }

    pub fn move_left(&mut self) {
        let x = self.column_falling.blocks[0].x;
        if x > 0 {
            self.column_falling.move_left();
        }
    }

    pub fn move_right(&mut self) {
        let x = self.column_falling.blocks[0].x;
        if x < Self::BOARD_WIDTH - 1 {
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
        let bottom_y = self.column_falling.blocks[2].y;
        let distance = Self::BOARD_HEIGHT - 1 - bottom_y;

        self.column_falling.drop(distance);

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
