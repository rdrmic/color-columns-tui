use std::time::Instant;

use crate::stage_handlers::FRAME_DURATION;

// =============================================================================
// Blinking
// =============================================================================
pub struct Blinking {
    start_time: Option<Instant>,
}

impl Blinking {
    const PHASE_DURATION_MS: u64 = 475;
    const NUM_PHASES: u64 = 3;

    pub fn new() -> Self {
        Self { start_time: Some(Instant::now()) }
    }

    pub fn is_visible_phase(&self) -> bool {
        let Some(start_time) = self.start_time else {
            return true;
        };

        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        let phase = (elapsed_ms / Self::PHASE_DURATION_MS) % 2;

        phase != 0
    }

    pub fn update(&mut self) {
        let Some(start_time) = self.start_time else {
            return;
        };

        let elapsed_ms = start_time.elapsed().as_millis() as u64;

        if elapsed_ms >= Self::PHASE_DURATION_MS * Self::NUM_PHASES {
            self.start_time = None;
        }
    }
}

// =============================================================================
// Fading
// =============================================================================
pub struct Fading {
    opacity_percent: u8,
    num_ticks_while_opaque: u8,
    percent_per_tick: u8,
    last_update_time: Option<Instant>,
}

impl Fading {
    pub const fn new(num_ticks_while_opaque: u8, percent_per_tick: u8) -> Self {
        Self { opacity_percent: 100, num_ticks_while_opaque, percent_per_tick, last_update_time: None }
    }

    pub const fn opacity_percent(&self) -> u8 {
        self.opacity_percent
    }

    pub fn update(&mut self) -> bool {
        if self.percent_per_tick == 0 {
            return true;
        }

        let now = Instant::now();

        if let Some(last_update_time) = self.last_update_time
            && now.duration_since(last_update_time) < FRAME_DURATION
        {
            return true;
        }

        self.last_update_time = Some(now);

        if self.num_ticks_while_opaque > 0 {
            self.num_ticks_while_opaque -= 1;
            return true;
        }

        if self.opacity_percent > self.percent_per_tick {
            self.opacity_percent -= self.percent_per_tick;
            true
        } else {
            false
        }
    }
}
