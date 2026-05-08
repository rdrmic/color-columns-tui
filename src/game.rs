use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    blocks::{Column, MatchingStructure, Pile, all_directions_match_counts},
    errors::{self, Context},
    scoring::Scoring,
};

enum GameplayState {
    FallingColumn,
    ClearingMatches,
    ApplyingHangingBlocksGravity,
}

pub struct Game {
    column_next: Column,
    column_falling: Option<Column>,
    pile: Pile,
    scoring: Scoring,
    current_tick_duration: Duration,
    gameplay_state: GameplayState,
    app_state_dir_path: Option<PathBuf>,
    rng: fastrand::Rng,
}

impl Game {
    pub const BOARD_WIDTH: u8 = 6;
    pub const BOARD_HEIGHT: u8 = 13;

    const INITIAL_TICK_DURATION: Duration = Duration::from_millis(750);
    const MIN_TICK_DURATION: Duration = Duration::from_millis(100);

    const ACCELERATION_SCORE_POINTS: u8 = 100; // new score points required for acceleration
    const ACCELERATION_FACTOR: u8 = 98; // reduce the current tick duration by 2%

    const FALLING_COLUMN_NOT_INITIALIZED_ERROR: &str = "Falling column must be initialized";

    pub fn new(app_state_dir_path: Option<&Path>) -> Result<Self, errors::Error> {
        let app_state_dir_path = app_state_dir_path.map(PathBuf::from);

        let mut rng = create_rng()?;

        Ok(Self {
            column_next: Self::create_column(&mut rng),
            column_falling: None,
            pile: Pile::new(Self::BOARD_WIDTH, Self::BOARD_HEIGHT),
            scoring: Scoring::new(app_state_dir_path.as_deref())?,
            current_tick_duration: Self::INITIAL_TICK_DURATION,
            gameplay_state: GameplayState::FallingColumn,
            app_state_dir_path,
            rng,
        })
    }

    pub fn start(&mut self) -> Result<(), errors::Error> {
        self.pile.clear();
        self.scoring = Scoring::new(self.app_state_dir_path.as_deref())?;
        self.current_tick_duration = Self::INITIAL_TICK_DURATION;
        self.gameplay_state = GameplayState::FallingColumn;
        self.transition_next_column_to_falling();
        self.rng = create_rng()?;

        Ok(())
    }

    pub fn tick(&mut self) -> bool {
        let is_running = match self.gameplay_state {
            GameplayState::FallingColumn => self.tick_falling_column(),
            GameplayState::ClearingMatches => self.tick_clearing_matches(),
            GameplayState::ApplyingHangingBlocksGravity => self.tick_applying_hanging_blocks_gravity(),
        };
        if !is_running {
            self.write_highscore_to_file();
            return false;
        }
        true
    }

    pub fn move_left(&mut self) {
        let available_distance = self.get_available_distance_for_fall(-1);
        if available_distance > 0 {
            let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);
            column_falling.move_left();
        }
    }

    pub fn move_right(&mut self) {
        let available_distance = self.get_available_distance_for_fall(1);
        if available_distance > 0 {
            let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);
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
        let available_distance = self.get_available_distance_for_fall(0);
        if available_distance > 0 {
            let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);
            column_falling.move_down(available_distance);
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
        self.current_tick_duration
    }

    pub const fn score(&self) -> u32 {
        self.scoring.score()
    }

    pub const fn max_combo(&self) -> u16 {
        self.scoring.max_combo()
    }

    pub const fn highscore(&self) -> u32 {
        self.scoring.highscore()
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

    fn get_available_distance_for_fall(&self, x_offset: i8) -> i8 {
        let column_falling = self.column_falling.as_ref().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        let x_as_i8 = i8::try_from(column_falling.x()).expect("`x` should fit in `i8`");
        let x = if x_offset == 0 {
            x_as_i8
        } else {
            let x_moved = x_as_i8 + x_offset;
            if x_moved < 0 || x_moved >= i8::try_from(Self::BOARD_WIDTH).expect("Board width should fit in `i8`") {
                return -1;
            }
            x_moved
        };

        let x = u8::try_from(x).expect("At this point, `x` must fit in `u8`");

        let first_occupied_y = (0..Self::BOARD_HEIGHT).find(|&y| self.pile.get(x, y).is_some()).unwrap_or(Self::BOARD_HEIGHT);
        let first_occupied_y = i8::try_from(first_occupied_y).expect("Board height should fit in `i8`") - 1;

        first_occupied_y - column_falling.y_bottom()
    }

    fn accelerate(&mut self) {
        let current_ms = self.current_tick_duration.as_millis() as u64;
        let new_ms = (current_ms * u64::from(Self::ACCELERATION_FACTOR)) / 100;
        self.current_tick_duration = Duration::from_millis(new_ms).max(Self::MIN_TICK_DURATION);
    }

    fn write_highscore_to_file(&self) {
        let _ = self.scoring.write_highscore_to_file(self.app_state_dir_path.as_deref()).inspect_err(|e| log::error!("{e}"));
    }
    // ============================================================================
    // Ticks
    // ============================================================================
    fn tick_falling_column(&mut self) -> bool {
        let mut available_distance = self.get_available_distance_for_fall(0);

        let column_falling = self.column_falling.as_mut().expect(Self::FALLING_COLUMN_NOT_INITIALIZED_ERROR);

        if available_distance > 0 {
            column_falling.move_down(1);
            available_distance -= 1;

            self.scoring.break_cascade_sequence();
        }

        if available_distance == 0 {
            let is_column_locked = self.pile.lock(column_falling);

            let match_counts = self.pile.find_matches(&MatchingStructure::Column(column_falling));
            if match_counts > 0 {
                self.gameplay_state = GameplayState::ClearingMatches;

                let points = all_directions_match_counts(match_counts);
                self.scoring.add(points);
            } else {
                if !is_column_locked {
                    return false;
                }
                self.gameplay_state = GameplayState::FallingColumn;
            }

            self.transition_next_column_to_falling();
        }

        true
    }

    fn tick_clearing_matches(&mut self) -> bool {
        self.pile.clear_matches();

        self.gameplay_state = if self.pile.has_hanging_blocks() { GameplayState::ApplyingHangingBlocksGravity } else { GameplayState::FallingColumn };

        true
    }

    fn tick_applying_hanging_blocks_gravity(&mut self) -> bool {
        self.pile.apply_hanging_blocks_gravity();

        let match_counts = self.pile.find_matches(&MatchingStructure::Pile);
        if match_counts > 0 {
            let points = all_directions_match_counts(match_counts);
            self.scoring.add(points);

            self.gameplay_state = GameplayState::ClearingMatches;
        } else {
            self.gameplay_state = GameplayState::FallingColumn;
        }

        self.scoring.count_in_accumulated_points();
        if self.scoring.score() > 0 && self.scoring.score().is_multiple_of(u32::from(Self::ACCELERATION_SCORE_POINTS)) {
            self.accelerate();
        }

        true
    }
}

#[rustfmt::skip]
fn create_rng() -> Result<fastrand::Rng, errors::Error> {
    let now = SystemTime::now();
    let seed = now.duration_since(UNIX_EPOCH)
        .context("Failed to generate random seed from system time: system clock is set before year 1970")?
        .as_millis() as u64;

    log::debug!("Using random seed: {seed}");
    Ok(fastrand::Rng::with_seed(seed))
}
