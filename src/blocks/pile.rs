use std::collections::HashSet;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::blocks::{Block, Column, Direction, Gem, MAX_MATCHES_PER_DIRECTION, MIN_CONSECUTIVE_GEMS_TO_MATCH, MatchingStructure};

pub struct Pile {
    width: u8,
    height: u8,
    grid: Vec<Option<Gem>>,
    final_gem: Option<(usize, Gem)>,
    matched_positions: HashSet<(u8, u8)>,
}

impl Pile {
    pub fn new(width: u8, height: u8) -> Self {
        Self { width, height, grid: vec![], final_gem: None, matched_positions: HashSet::new() }
    }

    pub fn clear(&mut self) {
        self.grid = vec![None; (self.width * self.height) as usize];
        self.final_gem = None;
        self.matched_positions.clear();
    }

    pub fn will_next_position_fit(&self, x: u8, column: &Column) -> bool {
        let mut next_y_positions = [0; 3];
        for (idx, y) in column.next_y_positions().into_iter().enumerate() {
            next_y_positions[idx] = y;
        }

        let pile_height = i8::try_from(self.height).expect("Board height should fit in `i8`");
        if next_y_positions[2] >= pile_height {
            return false;
        }

        let stack_top_y = i8::try_from(self.find_stack_top_y(x)).expect("Stack top should fit in `i8`") - 1;
        for y in next_y_positions {
            if y > stack_top_y {
                return false;
            }
        }

        true
    }

    pub fn lock(&mut self, column: &Column) {
        for (gem_x, gem_y, gem) in column.gems() {
            if let Ok(gem_y) = u8::try_from(gem_y) {
                let idx = self.calculate_grid_idx(gem_x, gem_y);
                if let Some(slot) = self.grid.get_mut(idx) {
                    *slot = Some(gem);
                }
            } else {
                let last_gem_idx = self.calculate_grid_idx(gem_x, 0);
                self.final_gem = Some((last_gem_idx, gem));
            }
        }
    }

    pub fn lock_final_gem(&mut self) {
        if let Some((slot, gem)) = self.final_gem.and_then(|(idx, gem)| self.grid.get_mut(idx).map(|slot| (slot, gem))) {
            *slot = Some(gem);
        }
    }

    pub const fn is_overflowed(&self) -> bool {
        self.final_gem.is_some()
    }

    pub fn get(&self, x: u8, y: u8) -> Option<Gem> {
        let idx = self.calculate_grid_idx(x, y);
        self.grid.get(idx).and_then(|&g| g)
    }

    const fn calculate_grid_idx(&self, x: u8, y: u8) -> usize {
        (y * self.width + x) as usize
    }

    fn find_stack_top_y(&self, x: u8) -> u8 {
        (0..self.height).find(|&y| self.get(x, y).is_some()).unwrap_or(self.height)
    }

    // ============================================================================
    // Matches
    // ============================================================================
    pub fn find_matches(&mut self, structure: &MatchingStructure) -> u64 {
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

    pub fn clear_matches(&mut self) {
        for (x, y) in &self.matched_positions {
            let idx = self.calculate_grid_idx(*x, *y);
            if let Some(slot) = self.grid.get_mut(idx) {
                *slot = None;
            }
        }
        self.matched_positions.clear();
    }

    /// Shift Gems down
    pub fn apply_gravity(&mut self) {
        for x in 0..self.width {
            let mut write_pos = self.height.saturating_sub(1);
            for read_pos in (0..self.height).rev() {
                if self.get(x, read_pos).is_some() {
                    if read_pos != write_pos {
                        let read_idx = self.calculate_grid_idx(x, read_pos);
                        let write_idx = self.calculate_grid_idx(x, write_pos);
                        self.grid[write_idx] = self.grid[read_idx];
                        self.grid[read_idx] = None;
                    }
                    write_pos = write_pos.saturating_sub(1);
                }
            }
        }
    }

    #[allow(clippy::cast_possible_wrap)] // For the hypothetical 16-bit platforms (which are extremely rare) edge case.
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

    /// Count consecutive Gems of the same type in a given direction
    #[allow(clippy::maybe_infinite_iter)]
    #[allow(clippy::cast_possible_wrap)] // For the hypothetical 16-bit platforms (which are extremely rare) edge case.
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
// Rendering Widget
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
