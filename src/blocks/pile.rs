use std::time::Instant;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    blocks::{Column, Direction, Gem, GemBlock, MAX_MATCHES_PER_DIRECTION, MIN_CONSECUTIVE_GEMS_TO_MATCH, MatchingStructure},
    game_state::{BOARD_HEIGHT, BOARD_WIDTH},
};

const NUM_GRID_CELLS: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize;

pub struct Pile {
    width: u8,
    height: u8,
    grid: [Option<Gem>; NUM_GRID_CELLS],
    matched_positions: [u64; 2],
    blinking_matches: BlinkingMatches,
}

impl Pile {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height, grid: [None; NUM_GRID_CELLS], matched_positions: [0; 2], blinking_matches: BlinkingMatches::new() }
    }

    pub const fn clear(&mut self) {
        self.grid = [None; NUM_GRID_CELLS];
        self.matched_positions = [0; 2];
        self.blinking_matches.reset();
    }

    pub fn lock(&mut self, column: Column) -> bool {
        for gem_block in column.gem_blocks() {
            if let Ok(gem_y) = u8::try_from(gem_block.y) {
                let idx = Self::calculate_grid_idx(gem_block.x, gem_y, self.width);
                if let Some(slot) = self.grid.get_mut(idx) {
                    *slot = Some(gem_block.gem);
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn get(&self, x: u8, y: u8) -> Option<Gem> {
        let idx = Self::calculate_grid_idx(x, y, self.width);
        *self.grid.get(idx)?
    }

    const fn calculate_grid_idx(x: u8, y: u8, width: u8) -> usize {
        y as usize * width as usize + x as usize
    }

    // =============================================================================
    // Matches
    // =============================================================================
    pub fn find_matches(&mut self, structure: MatchingStructure) -> u64 {
        // Initialize with a dummy value, but only read up to `check_count`
        let mut gems_to_check = [GemBlock::new(0, 0, Gem::Ruby); NUM_GRID_CELLS];
        let mut check_count = 0;

        match structure {
            MatchingStructure::Column(column) => {
                for gem_block in column.gem_blocks() {
                    if gem_block.y >= 0 {
                        gems_to_check[check_count] = gem_block;
                        check_count += 1;
                    }
                }
            }
            MatchingStructure::Pile => {
                for x in 0..self.width {
                    for y in 0..self.height {
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

        let mut matches_counts_as_packed_bits = 0;
        let mut outer_shift_offset = 36;
        let mut matched_positions_buffer = [(0, 0); MAX_MATCHES_PER_DIRECTION];

        for direction in Direction::ALL {
            let mut match_counts_per_direction = 0;
            let mut shift_offset = 9;

            let mut matched_positions_per_direction = [0u64; 2];

            for &gem_block in &gems_to_check[..check_count] {
                if let Some(len) = self.find_matches_from_gem_position(gem_block, direction, &mut matched_positions_buffer) {
                    let mut new_match = false;
                    for &(mx, my) in &matched_positions_buffer[..len] {
                        let idx = Self::calculate_grid_idx(mx, my, self.width);
                        // Bitmask check
                        if (matched_positions_per_direction[idx >> 6] & (1 << (idx & 63))) == 0 {
                            new_match = true;
                            break;
                        }
                    }

                    if new_match {
                        for &(mx, my) in &matched_positions_buffer[..len] {
                            let idx = Self::calculate_grid_idx(mx, my, self.width);
                            // Bitmask set
                            matched_positions_per_direction[idx >> 6] |= 1 << (idx & 63);
                            self.matched_positions[idx >> 6] |= 1u64 << (idx & 63);
                        }
                        match_counts_per_direction |= (len as u16) << shift_offset;
                        shift_offset -= 3;
                    }
                }
            }

            matches_counts_as_packed_bits |= u64::from(match_counts_per_direction) << outer_shift_offset;
            outer_shift_offset -= 12;
        }

        matches_counts_as_packed_bits
    }

    pub fn clear_matches(&mut self) -> bool {
        if self.matched_positions != [0; 2] && !self.blinking_matches.is_active() {
            self.blinking_matches.start(self.width, self.height, &self.grid, &mut self.matched_positions);
        }
        self.blinking_matches.update(self.width, &mut self.grid)
    }

    pub fn has_hanging_gems(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y).is_some() {
                    for y_bellow in (y + 1)..self.height {
                        if self.get(x, y_bellow).is_none() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Scans through all rows from top to bottom (`y_read_pos`).
    /// Whenever it finds a gem, it moves it down to the lowest available row (`y_write_pos`).
    pub fn apply_hanging_gems_gravity(&mut self) {
        for x in 0..self.width {
            let mut y_write_pos = self.height.saturating_sub(1);
            for y_read_pos in (0..self.height).rev() {
                if self.get(x, y_read_pos).is_some() {
                    if y_read_pos != y_write_pos {
                        let write_idx = Self::calculate_grid_idx(x, y_write_pos, self.width);
                        let read_idx = Self::calculate_grid_idx(x, y_read_pos, self.width);
                        self.grid[write_idx] = self.grid[read_idx];
                        self.grid[read_idx] = None;
                    }
                    y_write_pos = y_write_pos.saturating_sub(1);
                }
            }
        }
    }

    fn find_matches_from_gem_position(
        &self,
        gem_block: GemBlock,
        direction: Direction,
        matched_positions_buffer: &mut [(u8, u8); MAX_MATCHES_PER_DIRECTION],
    ) -> Option<usize> {
        let count_forward = self.count_consecutive_gems(gem_block, direction);
        let count_reverse = self.count_consecutive_gems(gem_block, -direction);
        let count = count_forward + count_reverse + 1;

        if count >= MIN_CONSECUTIVE_GEMS_TO_MATCH as i8 {
            let gem_block_base_y = gem_block.y.unsigned_abs();

            let mut len = 0;

            matched_positions_buffer[len] = (gem_block.x, gem_block_base_y);
            len += 1;

            len = Self::scan_for_matches(count_forward, direction, [gem_block.x, gem_block_base_y], matched_positions_buffer, len);
            len = Self::scan_for_matches(count_reverse, -direction, [gem_block.x, gem_block_base_y], matched_positions_buffer, len);

            return Some(len);
        }
        None
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Binary golf - shaves off 64 B")]
    fn scan_for_matches(
        consecutive_gems_count: i8,
        direction: Direction,
        gem_block_coordinates: [u8; 2],
        matched_positions_buffer: &mut [(u8, u8); MAX_MATCHES_PER_DIRECTION],
        buffer_offset: usize,
    ) -> usize {
        let mut buffer_idx = buffer_offset;
        for i in 1..=consecutive_gems_count {
            let scan_x = gem_block_coordinates[0].wrapping_add_signed(direction.dx * i);
            let scan_y = gem_block_coordinates[1].wrapping_add_signed(direction.dy * i);

            matched_positions_buffer[buffer_idx] = (scan_x, scan_y);
            buffer_idx += 1;
        }
        buffer_idx
    }

    /// Count consecutive Gems of the same type in a given direction.
    fn count_consecutive_gems(&self, gem_block: GemBlock, direction: Direction) -> i8 {
        let mut x = gem_block.x;
        let mut y = gem_block.y.unsigned_abs();

        let mut count = 0_i8;
        loop {
            let scan_x = x.wrapping_add_signed(direction.dx);
            let scan_y = y.wrapping_add_signed(direction.dy);

            if scan_x < self.width && self.get(scan_x, scan_y) == Some(gem_block.gem) {
                x = scan_x;
                y = scan_y;
                count += 1;
            } else {
                break;
            }
        }
        count
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

    fn start(&mut self, width: u8, height: u8, grid: &[Option<Gem>], match_positions: &mut [u64; 2]) {
        for y in 0..height {
            for x in 0..width {
                let idx = Pile::calculate_grid_idx(x, y, width);
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

    fn update(&mut self, width: u8, grid: &mut [Option<Gem>]) -> bool {
        let Some(blink_time) = self.blink_time else {
            return false;
        };

        let elapsed_ms = blink_time.elapsed().as_millis() as u64;

        let is_finished = elapsed_ms >= Self::BLINK_DURATION * Self::NUM_PHASES;
        let is_black_phase = is_finished || !(elapsed_ms / Self::BLINK_DURATION).is_multiple_of(2);

        for &(x, y, original_gem) in &self.matched_gems[..self.matched_count] {
            let idx = Pile::calculate_grid_idx(x, y, width);
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
        for y in 0..self.height {
            for x in 0..self.width {
                if let (Some(gem), Ok(y)) = (self.get(x, y), i8::try_from(y)) {
                    GemBlock::new(x, y, gem).render(area, buf);
                }
            }
        }
    }
}
