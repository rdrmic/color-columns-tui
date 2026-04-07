use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::blocks::{Block, Gem};

pub struct Column {
    pub blocks: [Block; 3],
}

impl Column {
    pub fn new(x: u8, y: i8, rng: &mut fastrand::Rng) -> Self {
        Self {
            blocks: [
                Block::new(x, y, Gem::random(rng)),
                Block::new(x, y + 1, Gem::random(rng)),
                Block::new(x, y + 2, Gem::random(rng)),
            ],
        }
    }

    pub fn move_down(&mut self) {
        for block in &mut self.blocks {
            block.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        for block in &mut self.blocks {
            if block.x > 0 {
                block.x -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        for block in &mut self.blocks {
            block.x += 1;
        }
    }

    pub const fn rotate_up(&mut self) {
        let temp = self.blocks[0].variant;
        self.blocks[0].variant = self.blocks[1].variant;
        self.blocks[1].variant = self.blocks[2].variant;
        self.blocks[2].variant = temp;
    }

    pub const fn rotate_down(&mut self) {
        let temp = self.blocks[2].variant;
        self.blocks[2].variant = self.blocks[1].variant;
        self.blocks[1].variant = self.blocks[0].variant;
        self.blocks[0].variant = temp;
    }

    pub fn drop(&mut self, distance: i8) {
        if distance > 0 {
            for block in &mut self.blocks {
                block.y += distance;
            }
        }
    }
}

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for block in &self.blocks {
            block.render(area, buf);
        }
    }
}
