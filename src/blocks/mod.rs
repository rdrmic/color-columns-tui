pub use block::{Gem, GemBlock};
pub use column::{Column, FallingColumnPlaceholder};
pub use pile::Pile;

mod block;
mod column;
mod pile;

// =============================================================================
// Searching for matches
// =============================================================================
const MIN_CONSECUTIVE_GEMS_TO_MATCH: usize = 3;

#[derive(Copy, Clone)]
pub enum MatchingStructure<'a> {
    Column(&'a Column),
    Pile,
}

#[derive(Copy, Clone)]
struct Direction {
    pub dx: i8,
    pub dy: i8,
}

impl Direction {
    const HORIZONTAL: Self = Self { dx: 1, dy: 0 };
    const VERTICAL: Self = Self { dx: 0, dy: 1 };
    const SLASH: Self = Self { dx: 1, dy: 1 };
    const BACKSLASH: Self = Self { dx: 1, dy: -1 };

    const ALL: [Self; 4] = [Self::HORIZONTAL, Self::VERTICAL, Self::SLASH, Self::BACKSLASH];
}

impl std::ops::Neg for Direction {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { dx: -self.dx, dy: -self.dy }
    }
}
