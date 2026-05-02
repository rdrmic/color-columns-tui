mod block;
mod column;
mod pile;

pub use block::{Block, Gem};
pub use column::Column;
pub use num_matches_unpacking::all_directions_match_counts;
pub use pile::Pile;

// ============================================================================
// Searching for matches
// ============================================================================
const MIN_CONSECUTIVE_GEMS_TO_MATCH: usize = 3;
const MAX_MATCHES_PER_DIRECTION: usize = 5;

#[derive(Debug)]
pub enum MatchingStructure<'a> {
    Column(&'a Column),
    Pile,
}

struct Direction;

impl Direction {
    const HORIZONTAL: (i8, i8) = (1, 0);
    const VERTICAL: (i8, i8) = (0, 1);
    const SLASH: (i8, i8) = (1, 1);
    const BACKSLASH: (i8, i8) = (1, -1);

    const ALL: [(i8, i8); 4] = [Self::HORIZONTAL, Self::VERTICAL, Self::SLASH, Self::BACKSLASH];
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

    /// Unpack a direction match counts into max possible 4 counts
    #[inline]
    const fn direction_match_counts(counts: u16) -> [u8; 4] {
        [
            ((counts >> MATCH_SHIFTS[0]) & MATCH_MASK) as u8,
            ((counts >> MATCH_SHIFTS[1]) & MATCH_MASK) as u8,
            ((counts >> MATCH_SHIFTS[2]) & MATCH_MASK) as u8,
            ((counts >> MATCH_SHIFTS[3]) & MATCH_MASK) as u8,
        ]
    }

    /// Unpack all 4 directions counts pack into: [direction][matches]
    #[inline]
    pub const fn all_directions_match_counts(counts: u64) -> [[u8; 4]; 4] {
        [
            direction_match_counts(((counts >> DIRECTION_SHIFTS[0]) & DIRECTION_MASK) as u16),
            direction_match_counts(((counts >> DIRECTION_SHIFTS[1]) & DIRECTION_MASK) as u16),
            direction_match_counts(((counts >> DIRECTION_SHIFTS[2]) & DIRECTION_MASK) as u16),
            direction_match_counts(((counts >> DIRECTION_SHIFTS[3]) & DIRECTION_MASK) as u16),
        ]
    }
}
