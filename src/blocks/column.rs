use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::blocks::{Gem, GemBlock};

#[derive(Copy, Clone)]
pub struct Column {
    x: u8,
    y: i8,
    gems: [Gem; 3],
    is_falling: bool,
}

impl Column {
    pub fn new(x: u8, y: i8, rng: &mut fastrand::Rng) -> Self {
        Self { x, y, gems: [Gem::random(rng), Gem::random(rng), Gem::random(rng)], is_falling: false }
    }

    pub const fn set_falling(&mut self, x: u8) {
        self.x = x;
        self.y = -3;
        self.is_falling = true;
    }

    #[rustfmt::skip]
    pub const fn gem_blocks(self) -> [GemBlock; 3] {
        [
            GemBlock::new(self.x, self.y,     self.gems[0]),
            GemBlock::new(self.x, self.y + 1, self.gems[1]),
            GemBlock::new(self.x, self.y + 2, self.gems[2]),
        ]
    }

    pub const fn next_y_positions(self) -> [i8; 3] {
        [self.y + 1, self.y + 2, self.y + 3]
    }

    pub const fn y_bottom(self) -> i8 {
        self.y + 2
    }

    pub const fn x(self) -> u8 {
        self.x
    }

    pub const fn move_down(&mut self, distance: i8) {
        self.y += distance;
    }

    pub const fn move_left(&mut self) {
        self.x -= 1;
    }

    pub const fn move_right(&mut self) {
        self.x += 1;
    }

    pub const fn rotate_up(&mut self) {
        self.gems.rotate_left(1);
    }

    pub const fn rotate_down(&mut self) {
        self.gems.rotate_right(1);
    }
}

// =============================================================================
// Falling Column placeholder
// =============================================================================
pub trait FallingColumnPlaceholder {
    fn placeholder() -> Self;
}

impl FallingColumnPlaceholder for Column {
    fn placeholder() -> Self {
        Self { x: 0, y: i8::MIN, gems: [Gem::Emerald, Gem::Emerald, Gem::Emerald], is_falling: false }
    }
}

// =============================================================================
// Widget rendering
// =============================================================================
impl Widget for Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for gem_block in self.gem_blocks() {
            if self.is_falling && gem_block.y < 0 {
                continue;
            }
            gem_block.render(area, buf);
        }
    }
}
