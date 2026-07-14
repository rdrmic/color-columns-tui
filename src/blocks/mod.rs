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
const MAX_MATCHES_PER_DIRECTION: usize = 5;

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

pub mod num_matches_unpacking {
    const MATCH_BITS: u8 = 3;
    const MATCH_MASK: u16 = (1 << MATCH_BITS) - 1;

    const MAX_MATCHES_PER_DIRECTION: u8 = 4;
    const DIRECTION_BITS: u8 = MATCH_BITS * MAX_MATCHES_PER_DIRECTION;
    const DIRECTION_MASK: u64 = (1 << DIRECTION_BITS) - 1;

    const NUM_DIRECTIONS: u8 = 4;

    const MATCH_SHIFTS: [u8; 4] = [
        (MAX_MATCHES_PER_DIRECTION - 1) * MATCH_BITS,
        (MAX_MATCHES_PER_DIRECTION - 2) * MATCH_BITS,
        (MAX_MATCHES_PER_DIRECTION - 3) * MATCH_BITS,
        (MAX_MATCHES_PER_DIRECTION - 4) * MATCH_BITS,
    ];

    const DIRECTION_SHIFTS: [u8; 4] = [
        (NUM_DIRECTIONS - 1) * DIRECTION_BITS,
        (NUM_DIRECTIONS - 2) * DIRECTION_BITS,
        (NUM_DIRECTIONS - 3) * DIRECTION_BITS,
        (NUM_DIRECTIONS - 4) * DIRECTION_BITS,
    ];

    #[inline]
    const fn unpack_max_match_points_per_direction(points: u16) -> [u8; 4] {
        [
            ((points >> MATCH_SHIFTS[0]) & MATCH_MASK) as u8,
            ((points >> MATCH_SHIFTS[1]) & MATCH_MASK) as u8,
            ((points >> MATCH_SHIFTS[2]) & MATCH_MASK) as u8,
            ((points >> MATCH_SHIFTS[3]) & MATCH_MASK) as u8,
        ]
    }

    /// Unpack all 4 maximum matches points for all 4 directions into: [direction][points].
    #[inline]
    pub const fn unpack_matches_points(packed_points: u64) -> [[u8; 4]; 4] {
        [
            unpack_max_match_points_per_direction(((packed_points >> DIRECTION_SHIFTS[0]) & DIRECTION_MASK) as u16),
            unpack_max_match_points_per_direction(((packed_points >> DIRECTION_SHIFTS[1]) & DIRECTION_MASK) as u16),
            unpack_max_match_points_per_direction(((packed_points >> DIRECTION_SHIFTS[2]) & DIRECTION_MASK) as u16),
            unpack_max_match_points_per_direction(((packed_points >> DIRECTION_SHIFTS[3]) & DIRECTION_MASK) as u16),
        ]
    }
}
