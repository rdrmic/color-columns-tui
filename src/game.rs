use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Ok};

use crate::blocks::{Column, Pile};

pub struct Game {
    column_next: Column,
    column_falling: Option<Column>,
    pile: Pile,
    score: u16, // TODO Max of 65,535 is enough? (depends on ACCELERATION_SCORE_POINTS and maybe on ACCELERATION_FACTOR)
    current_tick_duration: Duration,
    rng: fastrand::Rng,
}

impl Game {
    pub const BOARD_WIDTH: u8 = 6;
    pub const BOARD_HEIGHT: i8 = 13;

    const INITIAL_TICK_DURATION: Duration = Duration::from_millis(750);
    const MIN_TICK_DURATION: Duration = Duration::from_millis(100);

    const ACCELERATION_SCORE_POINTS: u16 = 100; // New score points required for acceleration
    const ACCELERATION_FACTOR: f64 = 0.98; // Reduce the current tick duration by 2%

    const FALLING_COLUMN_NOT_INITIALIZED_ERROR: &str = "Falling column must be initialized";

    pub fn new() -> anyhow::Result<Self> {
        let mut rng = create_rng()?;

        Ok(Self {
            column_next: Self::create_column(&mut rng),
            column_falling: None,
            pile: Pile::new(Self::BOARD_WIDTH, Self::BOARD_HEIGHT as u8),
            score: 0,
            current_tick_duration: Self::INITIAL_TICK_DURATION,
            rng,
        })
    }

    pub fn start(&mut self) {
        self.transition_next_column_to_falling();
        self.pile.clear();
        self.score = 0;
        self.current_tick_duration = Self::INITIAL_TICK_DURATION;
    }

    pub fn tick(&mut self) -> bool {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let next_y_bottom = column_falling.y_bottom() + 1;
        let x = column_falling.x();

        if next_y_bottom < Self::BOARD_HEIGHT && !self.pile.is_occupied(x, next_y_bottom) {
            column_falling.move_down(1);
        } else {
            let is_locked = self.pile.lock(column_falling);
            if !is_locked {
                return false;
            }

            self.transition_next_column_to_falling();
            self.tick();
        }

        if self.score > 0 && self.score.is_multiple_of(Self::ACCELERATION_SCORE_POINTS) {
            self.accelerate(Self::ACCELERATION_FACTOR);
        }
        true
    }

    pub fn move_left(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let x = column_falling.x();
        let y_bottom = column_falling.y_bottom();

        // If the bottom Gem can move left, the whole column can move left
        if x > 0 && !self.pile.is_occupied(x - 1, y_bottom) {
            column_falling.move_left();
        }
    }

    pub fn move_right(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let x = column_falling.x();
        let y_bottom = column_falling.y_bottom();

        // If the bottom Gem can move right, the whole column can move right
        if x < Self::BOARD_WIDTH - 1 && !self.pile.is_occupied(x + 1, y_bottom) {
            column_falling.move_right();
        }
    }

    pub const fn rotate_up(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);
        column_falling.rotate_up();
    }

    pub const fn rotate_down(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);
        column_falling.rotate_down();
    }

    pub fn drop(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);
        let x = column_falling.x();

        // Find the first occupied row in the target column (top-down)
        let first_occupied_y = (0..Self::BOARD_HEIGHT).find(|&y| self.pile.is_occupied(x, y)).unwrap_or(Self::BOARD_HEIGHT);

        // Target is the row immediately above the first occupied Gem (or the floor)
        let distance = (first_occupied_y - 1) - column_falling.y_bottom();
        if distance > 0 {
            column_falling.move_down(distance);
        }
        self.tick();
    }

    pub const fn _get_next_column(&self) -> &Column {
        &self.column_next
    }

    pub const fn get_falling_column(&self) -> Option<&Column> {
        self.column_falling.as_ref()
    }

    pub const fn get_pile(&self) -> &Pile {
        &self.pile
    }

    pub const fn tick_rate(&self) -> Duration {
        self.current_tick_duration
    }

    fn create_column(rng: &mut fastrand::Rng) -> Column {
        let x = rng.u8(..Self::BOARD_WIDTH);
        Column::new(x, -3, rng)
    }

    fn transition_next_column_to_falling(&mut self) {
        let column_next = std::mem::replace(&mut self.column_next, Self::create_column(&mut self.rng));
        self.column_falling = Some(column_next);
    }

    fn accelerate(&mut self, factor: f64) {
        let new_tick_rate = self.current_tick_duration.mul_f64(factor);
        self.current_tick_duration = new_tick_rate.max(Self::MIN_TICK_DURATION);
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
