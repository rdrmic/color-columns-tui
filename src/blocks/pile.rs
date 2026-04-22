use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::blocks::{Block, Column, Gem};

pub struct Pile {
    width: u8,
    height: u8,
    grid: Vec<Option<Gem>>,
    final_gem: Option<(usize, Gem)>,
}

impl Pile {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height, grid: vec![], final_gem: None }
    }

    pub fn clear(&mut self) {
        self.grid = vec![None; (self.width * self.height) as usize];
        self.final_gem = None;
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
}

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
