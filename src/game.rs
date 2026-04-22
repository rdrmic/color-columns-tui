use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Ok};

use crate::blocks::{Column, Pile};

pub struct Game {
    column_next: Column,
    column_falling: Option<Column>,
    pile: Pile,
    score: u16, // TODO Max of 65,535 is enough? (depends on ACCELERATION_SCORE_POINTS and maybe on ACCELERATION_FACTOR)
    current_tick_duration: Duration,
    pub is_column_locked: bool,
    rng: fastrand::Rng,
}

impl Game {
    pub const BOARD_WIDTH: u8 = 6;
    pub const BOARD_HEIGHT: u8 = 13;

    const INITIAL_TICK_DURATION: Duration = Duration::from_millis(750);
    const MIN_TICK_DURATION: Duration = Duration::from_millis(100);

    const ACCELERATION_SCORE_POINTS: u16 = 100; // new score points required for acceleration
    const ACCELERATION_FACTOR: f64 = 0.98; // reduce the current tick duration by 2%

    const FALLING_COLUMN_NOT_INITIALIZED_ERROR: &str = "Falling column must be initialized";

    pub fn new() -> anyhow::Result<Self> {
        let mut rng = create_rng()?;

        Ok(Self {
            column_next: Self::create_column(&mut rng),
            column_falling: None,
            pile: Pile::new(Self::BOARD_WIDTH, Self::BOARD_HEIGHT),
            score: 0,
            current_tick_duration: Self::INITIAL_TICK_DURATION,
            is_column_locked: false,
            rng,
        })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.pile.clear();
        self.score = 0;
        self.current_tick_duration = Self::INITIAL_TICK_DURATION;
        self.is_column_locked = false;
        self.transition_next_column_to_falling();
        self.rng = create_rng()?;

        Ok(())
    }

    pub fn tick(&mut self) -> bool {
        if self.pile.is_overflowed() {
            self.pile.lock_final_gem();
            return false;
        }

        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let x = column_falling.x();

        let is_column_descended_one_step = if self.pile.will_next_position_fit(x, column_falling) {
            column_falling.move_down(1);
            true
        } else {
            false
        };

        if !is_column_descended_one_step || !self.pile.will_next_position_fit(x, column_falling) {
            self.pile.lock(column_falling);

            if self.pile.is_overflowed() {
                self.pile.lock_final_gem();
                return false;
            }

            self.is_column_locked = true;
            self.transition_next_column_to_falling();
        }

        if self.score > 0 && self.score.is_multiple_of(Self::ACCELERATION_SCORE_POINTS) {
            self.accelerate(Self::ACCELERATION_FACTOR);
        }

        true
    }

    pub fn move_left(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let x = column_falling.x();
        if x > 0 && self.pile.will_next_position_fit(x - 1, column_falling) {
            column_falling.move_left();
        }
    }

    pub fn move_right(&mut self) {
        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let x = column_falling.x();
        if x < Self::BOARD_WIDTH - 1 && self.pile.will_next_position_fit(x + 1, column_falling) {
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
        let first_occupied_y = (0..Self::BOARD_HEIGHT).find(|&y| self.pile.get(x, y).is_some()).unwrap_or(Self::BOARD_HEIGHT);
        let first_occupied_y = i8::try_from(first_occupied_y).expect("Board height should fit in `i8`");

        let distance = first_occupied_y - 1 - column_falling.y_bottom();
        if distance > 0 {
            column_falling.move_down(distance);
        }
    }

    pub const fn get_next_column(&self) -> &Column {
        &self.column_next
    }

    pub const fn get_falling_column(&self) -> Option<&Column> {
        self.column_falling.as_ref()
    }

    pub const fn get_pile(&self) -> &Pile {
        &self.pile
    }

    pub const fn tick_rate(&self) -> Duration {
        if self.is_column_locked { Duration::ZERO } else { self.current_tick_duration }
        //self.current_tick_duration
    }

    fn create_column(rng: &mut fastrand::Rng) -> Column {
        Column::new(0, 0, rng)
    }

    fn transition_next_column_to_falling(&mut self) {
        let x = self.rng.u8(..Self::BOARD_WIDTH);

        let mut column_next = std::mem::replace(&mut self.column_next, Self::create_column(&mut self.rng));
        column_next.set_falling(x);

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
