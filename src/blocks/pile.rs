use std::time::Instant;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    blocks::{Column, Direction, Gem, GemBlock, MIN_CONSECUTIVE_GEMS_TO_MATCH, MatchingStructure},
    game_state::{BOARD_HEIGHT, BOARD_WIDTH},
};

const NUM_GRID_CELLS: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize;

pub struct Pile {
    grid: [Option<Gem>; NUM_GRID_CELLS],
    matched_positions: [u64; 2],
    blinking_matches: BlinkingMatches,
}

impl Pile {
    pub const fn new() -> Self {
        Self { grid: [None; NUM_GRID_CELLS], matched_positions: [0; 2], blinking_matches: BlinkingMatches::new() }
    }

    pub const fn clear(&mut self) {
        self.grid = [None; NUM_GRID_CELLS];
        self.matched_positions = [0; 2];
        self.blinking_matches.reset();
    }

    pub fn lock(&mut self, column: Column) -> bool {
        for gem_block in column.gem_blocks() {
            let Ok(gem_y) = u8::try_from(gem_block.y) else {
                return false;
            };
            if gem_block.x >= BOARD_WIDTH || gem_y >= BOARD_HEIGHT {
                return false;
            }

            let idx = Self::calculate_grid_idx(gem_block.x, gem_y);
            self.grid[idx] = Some(gem_block.gem);
        }
        true
    }

    pub const fn get(&self, x: u8, y: u8) -> Option<Gem> {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            return None;
        }

        self.grid[Self::calculate_grid_idx(x, y)]
    }

    const fn calculate_grid_idx(x: u8, y: u8) -> usize {
        y as usize * BOARD_WIDTH as usize + x as usize
    }

    // =============================================================================
    // Matches
    // =============================================================================
    pub fn find_matches(&mut self, structure: MatchingStructure) -> Option<u32> {
        // Initialize with a dummy value, but only read up to `check_count`
        let mut gems_to_check = [GemBlock::new(0, 0, Gem::Ruby); NUM_GRID_CELLS];
        let mut check_count = 0;

        match structure {
            MatchingStructure::Column(column) => {
                for gem_block in column.gem_blocks() {
                    if gem_block.x < BOARD_WIDTH && gem_block.y >= 0 && gem_block.y.unsigned_abs() < BOARD_HEIGHT {
                        gems_to_check[check_count] = gem_block;
                        check_count += 1;
                    }
                }
            }
            MatchingStructure::Pile => {
                for x in 0..BOARD_WIDTH {
                    for y in 0..BOARD_HEIGHT {
                        if let Some(gem) = self.get(x, y) {
                            #[allow(clippy::cast_possible_wrap)]
                            let y_i8 = y as i8;

                            gems_to_check[check_count] = GemBlock::new(x, y_i8, gem);
                            check_count += 1;
                        }
                    }
                }
            }
        }

        let mut match_points = 1_u32;
        let mut has_matches = false;

        for direction in Direction::ALL {
            let mut matched_positions_per_direction = [0; 2];

            for &gem_block in &gems_to_check[..check_count] {
                let Some((length, run_positions)) = self.find_matches_from_gem_position(gem_block, direction) else {
                    continue;
                };

                let is_new_match =
                    (run_positions[0] & !matched_positions_per_direction[0]) != 0 || (run_positions[1] & !matched_positions_per_direction[1]) != 0;

                if is_new_match {
                    matched_positions_per_direction[0] |= run_positions[0];
                    matched_positions_per_direction[1] |= run_positions[1];
                    self.matched_positions[0] |= run_positions[0];
                    self.matched_positions[1] |= run_positions[1];
                    match_points = match_points.saturating_mul(u32::from(length));
                    has_matches = true;
                }
            }
        }

        has_matches.then_some(match_points)
    }

    pub fn clear_matches(&mut self) -> bool {
        if self.matched_positions != [0; 2] && !self.blinking_matches.is_active() {
            self.blinking_matches.start(&self.grid, &mut self.matched_positions);
        }
        self.blinking_matches.update(&mut self.grid)
    }

    pub fn has_hanging_gems(&self) -> bool {
        for x in 0..BOARD_WIDTH {
            let mut occupied_seen = false;
            for y in 0..BOARD_HEIGHT {
                if self.get(x, y).is_some() {
                    occupied_seen = true;
                } else if occupied_seen {
                    return true;
                }
            }
        }
        false
    }

    /// Scans through all rows from top to bottom (`y_read_pos`).
    /// Whenever it finds a gem, it moves it down to the lowest available row (`y_write_pos`).
    pub fn apply_hanging_gems_gravity(&mut self) {
        for x in 0..BOARD_WIDTH {
            let mut y_write_pos = BOARD_HEIGHT - 1;
            for y_read_pos in (0..BOARD_HEIGHT).rev() {
                if self.get(x, y_read_pos).is_some() {
                    if y_read_pos != y_write_pos {
                        let write_idx = Self::calculate_grid_idx(x, y_write_pos);
                        let read_idx = Self::calculate_grid_idx(x, y_read_pos);
                        self.grid[write_idx] = self.grid[read_idx];
                        self.grid[read_idx] = None;
                    }
                    y_write_pos = y_write_pos.saturating_sub(1);
                }
            }
        }
    }

    fn find_matches_from_gem_position(&self, gem_block: GemBlock, direction: Direction) -> Option<(u8, [u64; 2])> {
        let y = gem_block.y.unsigned_abs();
        let mut run_positions = [0; 2];
        Self::mark_position(&mut run_positions, gem_block.x, y);

        let length = 1 + self.scan_matching_gems(gem_block, direction, &mut run_positions) + self.scan_matching_gems(gem_block, -direction, &mut run_positions);

        (usize::from(length) >= MIN_CONSECUTIVE_GEMS_TO_MATCH).then_some((length, run_positions))
    }

    fn scan_matching_gems(&self, gem_block: GemBlock, direction: Direction, run_positions: &mut [u64; 2]) -> u8 {
        let mut x = gem_block.x;
        let mut y = gem_block.y.unsigned_abs();
        let mut count = 0;

        loop {
            let scan_x = x.wrapping_add_signed(direction.dx);
            let scan_y = y.wrapping_add_signed(direction.dy);

            if scan_x >= BOARD_WIDTH || self.get(scan_x, scan_y) != Some(gem_block.gem) {
                break;
            }

            Self::mark_position(run_positions, scan_x, scan_y);
            x = scan_x;
            y = scan_y;
            count += 1;
        }

        count
    }

    const fn mark_position(positions: &mut [u64; 2], x: u8, y: u8) {
        let idx = Self::calculate_grid_idx(x, y);
        positions[idx >> 6] |= 1 << (idx & 63);
    }
}

// =============================================================================
// Blinking matches
// =============================================================================
struct BlinkingMatches {
    matched_gems: [(u8, u8, Gem); NUM_GRID_CELLS],
    matched_count: usize,
    blink_time: Option<Instant>,
}

impl BlinkingMatches {
    const BLINK_DURATION: u64 = 374;
    const NUM_PHASES: u64 = 4;

    const fn new() -> Self {
        Self { matched_gems: [(0, 0, Gem::Ruby); NUM_GRID_CELLS], matched_count: 0, blink_time: None }
    }

    const fn reset(&mut self) {
        self.matched_count = 0;
        self.blink_time = None;
    }

    const fn is_active(&self) -> bool {
        self.blink_time.is_some()
    }

    fn start(&mut self, grid: &[Option<Gem>], match_positions: &mut [u64; 2]) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let idx = Pile::calculate_grid_idx(x, y);
                // Check using the 2-word mask
                if let Some(gem) = grid[idx]
                    && (match_positions[idx >> 6] & (1u64 << (idx & 63))) != 0
                {
                    self.matched_gems[self.matched_count] = (x, y, gem);
                    self.matched_count += 1;
                }
            }
        }
        // Clear the mask
        *match_positions = [0; 2];
        self.blink_time = Some(Instant::now());
    }

    fn update(&mut self, grid: &mut [Option<Gem>]) -> bool {
        let Some(blink_time) = self.blink_time else {
            return false;
        };

        let elapsed_ms = blink_time.elapsed().as_millis() as u64;

        let is_finished = elapsed_ms >= Self::BLINK_DURATION * Self::NUM_PHASES;
        let is_black_phase = is_finished || !(elapsed_ms / Self::BLINK_DURATION).is_multiple_of(2);

        for &(x, y, original_gem) in &self.matched_gems[..self.matched_count] {
            let idx = Pile::calculate_grid_idx(x, y);
            grid[idx] = if is_black_phase { None } else { Some(original_gem) };
        }

        if is_finished {
            self.reset();
            return false;
        }
        true
    }
}

// =============================================================================
// Widget rendering
// =============================================================================
impl Widget for &Pile {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if let (Some(gem), Ok(y)) = (self.get(x, y), i8::try_from(y)) {
                    GemBlock::new(x, y, gem).render(area, buf);
                }
            }
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn pile_with_gems(gems: &[(u8, u8, Gem)]) -> Pile {
        let mut pile = Pile::new();
        for &(x, y, gem) in gems {
            pile.grid[Pile::calculate_grid_idx(x, y)] = Some(gem);
        }
        pile
    }

    #[test]
    fn scores_each_run_only_once() {
        let mut pile = pile_with_gems(&[(0, 0, Gem::Ruby), (1, 0, Gem::Ruby), (2, 0, Gem::Ruby)]);

        assert_eq!(pile.find_matches(MatchingStructure::Pile), Some(3));
    }

    #[test]
    fn multiplies_intersecting_directions() {
        let mut pile = pile_with_gems(&[(1, 2, Gem::Ruby), (2, 2, Gem::Ruby), (3, 2, Gem::Ruby), (2, 1, Gem::Ruby), (2, 3, Gem::Ruby)]);

        assert_eq!(pile.find_matches(MatchingStructure::Pile), Some(9));
    }

    #[test]
    fn supports_runs_longer_than_the_old_packed_representation() {
        let mut pile = Pile::new();
        for y in 0..BOARD_HEIGHT {
            pile.grid[Pile::calculate_grid_idx(0, y)] = Some(Gem::Ruby);
        }

        assert_eq!(pile.find_matches(MatchingStructure::Pile), Some(u32::from(BOARD_HEIGHT)));
    }

    #[test]
    fn handles_many_distinct_runs_without_overflow() {
        let mut pile = Pile::new();
        for y in 0..7 {
            let gem = if y % 2 == 0 { Gem::Ruby } else { Gem::Emerald };
            for x in 0..BOARD_WIDTH {
                pile.grid[Pile::calculate_grid_idx(x, y)] = Some(gem);
            }
        }

        assert_eq!(pile.find_matches(MatchingStructure::Pile), Some(279_936));
    }

    #[test]
    fn saturates_match_points_at_the_score_limit() {
        let mut pile = Pile::new();
        for y in 0..BOARD_HEIGHT {
            let gem = if y % 2 == 0 { Gem::Ruby } else { Gem::Emerald };
            for x in 0..BOARD_WIDTH {
                pile.grid[Pile::calculate_grid_idx(x, y)] = Some(gem);
            }
        }

        assert_eq!(pile.find_matches(MatchingStructure::Pile), Some(u32::MAX));
    }

    #[test]
    fn rejects_coordinates_outside_the_fixed_board() {
        let pile = Pile::new();

        assert!(pile.get(BOARD_WIDTH, 0).is_none());
        assert!(pile.get(0, BOARD_HEIGHT).is_none());
    }

    #[test]
    fn detects_only_gaps_below_gems_as_hanging() {
        let compact = pile_with_gems(&[(0, BOARD_HEIGHT - 2, Gem::Ruby), (0, BOARD_HEIGHT - 1, Gem::Ruby)]);
        let hanging = pile_with_gems(&[(0, BOARD_HEIGHT - 2, Gem::Ruby)]);

        assert!(!compact.has_hanging_gems());
        assert!(hanging.has_hanging_gems());
    }
}
