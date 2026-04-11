#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::blocks::{Block, Column, Gem};

pub struct Pile {
    width: u8,
    height: u8,
    grid: Vec<Option<Gem>>,
}

impl Pile {
    pub fn new(width: u8, height: u8) -> Self {
        Self { width, height, grid: vec![None; (width as usize) * (height as usize)] }
    }

    pub fn is_occupied(&self, x: u8, y: i8) -> bool {
        if y < 0 {
            crate::dev_red!("pile::is_occupied: y is negative: {}", y);
            return false;
        } // Still in the "buffer" zone above the board

        // if x >= self.width || y >= self.height as i8 {
        //     if x >= self.width {
        //         crate::dev_red!("pile::is_occupied: x is out of bounds: {}", x);
        //     }
        //     if y >= self.height as i8 {
        //         crate::dev_red!("pile::is_occupied: y is out of bounds: {}", y);
        //     }
        //     return true;
        // } // Out of bounds

        self.get(x, y as u8).is_some()
    }

    pub fn set(&mut self, column: &Column) -> bool {
        for (gem_x, gem_y, gem) in column.view_gems() {
            if gem_y >= 0 {
                let idx = self.calculate_grid_idx(gem_x, gem_y as u8);
                if let Some(slot) = self.grid.get_mut(idx) {
                    *slot = Some(gem);
                }
            } else {
                return false;
            }
        }
        true
    }

    fn get(&self, x: u8, y: u8) -> Option<Gem> {
        let idx = self.calculate_grid_idx(x, y);
        self.grid.get(idx).and_then(|&g| g)
    }

    const fn calculate_grid_idx(&self, x: u8, y: u8) -> usize {
        (y * self.width + x) as usize
    }
}

impl Widget for &Pile {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(gem) = self.get(x, y) {
                    let block = Block::new(x, y as i8, gem);
                    block.render(area, buf);
                }
            }
        }
    }
}
