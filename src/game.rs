use std::time::Duration;

pub struct GameState {
    pub score: u32,
    pub position_y: u32,
    current_tick_rate: Duration,
}

impl GameState {
    const INITIAL_TICK_RATE: Duration = Duration::from_millis(500);
    const MIN_TICK_RATE: Duration = Duration::from_millis(50); // Speed cap

    pub fn new() -> Self {
        Self { score: 0, position_y: 0, current_tick_rate: Self::INITIAL_TICK_RATE }
    }

    pub fn tick(&mut self) {
        self.position_y += 1;

        // Example Acceleration Logic:
        // Every time the score increases, we reduce the tick duration by 2%
        // until we hit the MIN_TICK_RATE.
        if self.score > 0 && self.score.is_multiple_of(100) {
            self.accelerate(0.98);
        }
    }

    fn accelerate(&mut self, factor: f64) {
        let new_tick_rate = self.current_tick_rate.mul_f64(factor);

        // Ensure we don't go faster than our speed cap (MIN_TICK_RATE)
        self.current_tick_rate = new_tick_rate.max(Self::MIN_TICK_RATE);
    }

    // Getter so the App knows how long to wait
    pub const fn tick_rate(&self) -> Duration {
        self.current_tick_rate
    }

    pub fn move_left(&mut self) {
        //
    }

    pub fn move_right(&mut self) {
        //
    }
}
