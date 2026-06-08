use std::{collections::HashSet, time::Instant};

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::blocks::{Block, Column, Direction, Gem, MAX_MATCHES_PER_DIRECTION, MIN_CONSECUTIVE_GEMS_TO_MATCH, MatchingStructure};

pub struct Pile {
    width: u8,
    height: u8,
    grid: Vec<Option<Gem>>,
    matched_positions: HashSet<(u8, u8)>,
    blinking_matches: BlinkingMatches,
}

impl Pile {
    pub fn new(width: u8, height: u8) -> Self {
        Self { width, height, grid: vec![], matched_positions: HashSet::new(), blinking_matches: BlinkingMatches::new() }
    }

    pub fn clear(&mut self) {
        self.grid = vec![None; (self.width * self.height) as usize];
        self.matched_positions.clear();
        self.blinking_matches.reset();
    }

    pub fn lock(&mut self, column: Column) -> bool {
        for (gem_x, gem_y, gem) in column.gems() {
            if let Ok(gem_y) = u8::try_from(gem_y) {
                let idx = Self::calculate_grid_idx(gem_x, gem_y, self.width);
                if let Some(slot) = self.grid.get_mut(idx) {
                    *slot = Some(gem);
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
        (y * width + x) as usize
    }

    // ============================================================================
    // Matches
    // ============================================================================
    pub fn find_matches(&mut self, structure: MatchingStructure) -> u64 {
        let mut matches_counts_as_packed_bits = 0;
        let mut outer_shift_offset = 36;

        for direction in Direction::ALL {
            let mut match_counts_per_direction = 0;
            let mut shift_offset = 9;

            let mut matched_positions_per_direction = HashSet::with_capacity(MAX_MATCHES_PER_DIRECTION);

            match structure {
                MatchingStructure::Column(column) => {
                    for (x, y, _) in column.gems() {
                        if let Some(matched_positions) = u8::try_from(y).ok().and_then(|y| self.find_matches_from_gem_position(direction, x, y))
                            && !matched_positions.iter().all(|(x, y)| matched_positions_per_direction.contains(&(*x, *y)))
                        {
                            matched_positions_per_direction.extend(&matched_positions);

                            match_counts_per_direction |= (matched_positions.len() as u16) << shift_offset;
                            shift_offset -= 3;
                        }
                    }
                }
                MatchingStructure::Pile => {
                    for x in 0..self.width {
                        for y in 0..self.height {
                            if let Some(matched_positions) = self.get(x, y).and_then(|_| self.find_matches_from_gem_position(direction, x, y))
                                && !matched_positions.iter().all(|(x, y)| matched_positions_per_direction.contains(&(*x, *y)))
                            {
                                matched_positions_per_direction.extend(&matched_positions);

                                match_counts_per_direction |= (matched_positions.len() as u16) << shift_offset;
                                shift_offset -= 3;
                            }
                        }
                    }
                }
            }

            self.matched_positions.extend(&matched_positions_per_direction);

            matches_counts_as_packed_bits |= u64::from(match_counts_per_direction) << outer_shift_offset;
            outer_shift_offset -= 12;
        }

        matches_counts_as_packed_bits
    }

    pub fn clear_matches(&mut self) -> bool {
        if !self.matched_positions.is_empty() && !self.blinking_matches.is_active() {
            self.blinking_matches.start(self.width, &self.grid, &mut self.matched_positions);
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

    #[allow(clippy::cast_possible_wrap)]
    fn find_matches_from_gem_position(&self, (dx, dy): (i8, i8), x: u8, y: u8) -> Option<Vec<(u8, u8)>> {
        let gem = self.get(x, y)?;

        let count_forward = self.count_consecutive_gems(x, y, gem, dx, dy);
        let count_reverse = self.count_consecutive_gems(x, y, gem, -dx, -dy);
        let count = count_forward + count_reverse + 1; // +1 for the center Gem

        let mut matched_positions = Vec::<(u8, u8)>::with_capacity(MAX_MATCHES_PER_DIRECTION);

        if count >= MIN_CONSECUTIVE_GEMS_TO_MATCH {
            // Add center Gem position
            matched_positions.push((x, y));

            // Add forward direction matches
            for i in 1..=count_forward {
                let offset_x = i16::from(dx).saturating_mul(i as i16) as i8;
                let offset_y = i16::from(dy).saturating_mul(i as i16) as i8;

                if let (Some(nx), Some(ny)) = (x.checked_add_signed(offset_x), y.checked_add_signed(offset_y)) {
                    matched_positions.push((nx, ny));
                }
            }

            // Add reverse direction matches
            for i in 1..=count_reverse {
                let offset_x = i16::from(dx).saturating_mul(-(i as i16)) as i8;
                let offset_y = i16::from(dy).saturating_mul(-(i as i16)) as i8;

                if let (Some(nx), Some(ny)) = (x.checked_add_signed(offset_x), y.checked_add_signed(offset_y)) {
                    matched_positions.push((nx, ny));
                }
            }

            return Some(matched_positions);
        }

        None
    }

    /// Count consecutive Gems of the same type in a given direction.
    #[allow(clippy::maybe_infinite_iter)]
    fn count_consecutive_gems(&self, x: u8, y: u8, gem: Gem, dx: i8, dy: i8) -> usize {
        (1..)
            .take_while(|i| {
                let offset_x = i16::from(dx).saturating_mul(*i as i16) as i8;
                let offset_y = i16::from(dy).saturating_mul(*i as i16) as i8;

                let Some(nx) = x.checked_add_signed(offset_x) else {
                    return false;
                };
                let Some(ny) = y.checked_add_signed(offset_y) else {
                    return false;
                };

                if nx >= self.width || ny >= self.height {
                    return false;
                }

                self.get(nx, ny) == Some(gem)
            })
            .count()
    }
}

// ============================================================================
// Blinking matches
// ============================================================================
pub struct BlinkingMatches {
    matched_gems: Vec<(u8, u8, Gem)>,
    blink_time: Option<Instant>,
}

impl BlinkingMatches {
    const BLINK_DURATION: u64 = 442;
    const NUM_PHASES: u64 = 4;

    const fn new() -> Self {
        Self { matched_gems: Vec::new(), blink_time: None }
    }

    fn reset(&mut self) {
        self.matched_gems.clear();
        self.blink_time = None;
    }

    const fn is_active(&self) -> bool {
        self.blink_time.is_some()
    }

    fn start(&mut self, width: u8, grid: &[Option<Gem>], match_positions: &mut HashSet<(u8, u8)>) {
        for (x, y) in match_positions.drain() {
            let idx = Pile::calculate_grid_idx(x, y, width);
            if let Some(gem) = grid[idx] {
                self.matched_gems.push((x, y, gem));
            }
        }

        self.blink_time = Some(Instant::now());
    }

    fn update(&mut self, width: u8, grid: &mut [Option<Gem>]) -> bool {
        let Some(blink_time) = self.blink_time else {
            return false;
        };

        let elapsed_ms = blink_time.elapsed().as_millis() as u64;

        let is_finished = elapsed_ms >= Self::BLINK_DURATION * Self::NUM_PHASES;
        let is_black_phase = is_finished || !(elapsed_ms / Self::BLINK_DURATION).is_multiple_of(2);

        for &(x, y, original_gem) in &self.matched_gems {
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

// ============================================================================
// Widget rendering
// ============================================================================
impl Widget for &Pile {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let (Some(gem), Ok(y)) = (self.get(x, y), i8::try_from(y)) {
                    Block::new(x, y, gem).render(area, buf);
                }
            }
        }
    }
}
