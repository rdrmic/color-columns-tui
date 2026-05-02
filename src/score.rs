pub struct Scoring {
    score: u32,
    max_combo: u16,
    highscore: u32,
    accumulated_points: u16,
    cascade_count: u8,
}

impl Scoring {
    pub const fn new() -> Self {
        Self { score: 0, max_combo: 0, highscore: 0, accumulated_points: 0, cascade_count: 0 }
    }

    pub fn add(&mut self, points: [[u8; 4]; 4]) {
        let mut calculated_points_per_direction = [0; 4];
        for direction_points in points.into_iter().enumerate() {
            let points = direction_points.1.into_iter().filter(|p| *p > 0).map(u16::from).product::<u16>();
            calculated_points_per_direction[direction_points.0] = if points == 1 { 0 } else { points };
        }
        crate::dev_cyan!("calculated_points_per_direction: {calculated_points_per_direction:?}");

        let cascade_multiplier = self.calculate_cascade_multiplier();

        let calculated_points = calculated_points_per_direction.into_iter().filter(|p| *p > 0).product::<u16>() * cascade_multiplier;
        crate::dev_cyan!("calculated_points: {calculated_points:?}");

        self.accumulated_points += calculated_points;
        crate::dev_cyan!("self.accumulated_points: {:?}", self.accumulated_points);

        self.score += u32::from(self.accumulated_points);

        if self.accumulated_points > self.max_combo {
            self.max_combo = self.accumulated_points;
        }

        if self.score > self.highscore {
            self.highscore = self.score;
        }
    }

    pub const fn score(&self) -> u32 {
        self.score
    }

    pub const fn max_combo(&self) -> u16 {
        self.max_combo
    }

    pub const fn highscore(&self) -> u32 {
        self.highscore
    }

    pub const fn break_cascade_sequence(&mut self) {
        self.accumulated_points = 0;
        self.cascade_count = 0;
    }

    const fn calculate_cascade_multiplier(&mut self) -> u16 {
        // *1 *3 *4 *5 etc.
        self.cascade_count += 1;
        if self.cascade_count == 1 {
            return 1;
        }
        1 + self.cascade_count as u16
    }
}
