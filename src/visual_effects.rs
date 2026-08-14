use std::time::Instant;

use crate::stage_handlers::FRAME_DURATION;

pub const FULL_OPACITY_PERCENT: u8 = 100;
pub const FADE_PERCENT_PER_TICK: u8 = 5;

#[inline(never)]
pub fn elapsed_phase(start_time: &Instant, phase_duration_ms: u64) -> u64 {
    start_time.elapsed().as_millis() as u64 / phase_duration_ms
}

// =============================================================================
// Blinking
// =============================================================================
#[derive(Copy, Clone)]
pub struct Blinking {
    start_time: Instant,
}

impl Blinking {
    const PHASE_DURATION_MS: u64 = 475;

    pub fn new() -> Self {
        Self { start_time: Instant::now() }
    }

    pub fn is_visible_phase(&self) -> bool {
        elapsed_phase(&self.start_time, Self::PHASE_DURATION_MS) & 1 != 0
    }

    pub fn is_finished(&self) -> bool {
        elapsed_phase(&self.start_time, Self::PHASE_DURATION_MS) >= 3
    }
}

// =============================================================================
// Fading
// =============================================================================
pub struct Fading {
    ticks_remaining: u8,
    last_update_time: Option<Instant>,
}

impl Fading {
    const NUM_FADE_TICKS: u8 = (FULL_OPACITY_PERCENT - 1) / FADE_PERCENT_PER_TICK + 1;

    pub const fn new(num_ticks_while_opaque: u8) -> Self {
        Self { ticks_remaining: num_ticks_while_opaque + Self::NUM_FADE_TICKS, last_update_time: None }
    }

    pub const fn opacity_percent(&self) -> u8 {
        let fade_ticks = if self.ticks_remaining < Self::NUM_FADE_TICKS { self.ticks_remaining } else { Self::NUM_FADE_TICKS };

        if FULL_OPACITY_PERCENT / FADE_PERCENT_PER_TICK * FADE_PERCENT_PER_TICK == FULL_OPACITY_PERCENT {
            fade_ticks * FADE_PERCENT_PER_TICK
        } else {
            FULL_OPACITY_PERCENT - (Self::NUM_FADE_TICKS - fade_ticks) * FADE_PERCENT_PER_TICK
        }
    }

    pub fn update(&mut self) -> bool {
        if let Some(last_update_time) = self.last_update_time
            && last_update_time.elapsed() < FRAME_DURATION
        {
            return true;
        }

        self.last_update_time = Some(Instant::now());

        if self.ticks_remaining <= 1 {
            return false;
        }

        self.ticks_remaining -= 1;
        true
    }
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn blinking_at(now: Instant, elapsed_ms: u64) -> Blinking {
        let Some(start_time) = now.checked_sub(Duration::from_millis(elapsed_ms)) else { panic!("test duration must fit in Instant") };
        Blinking { start_time }
    }

    #[test]
    fn blinking_uses_three_finite_label_phases_but_remains_periodic() {
        let now = Instant::now();

        let hidden = blinking_at(now, 100);
        assert!(!hidden.is_visible_phase());
        assert!(!hidden.is_finished());

        let visible = blinking_at(now, 600);
        assert!(visible.is_visible_phase());
        assert!(!visible.is_finished());

        let final_hidden = blinking_at(now, 1_100);
        assert!(!final_hidden.is_visible_phase());
        assert!(!final_hidden.is_finished());

        let periodic_visible = blinking_at(now, 1_600);
        assert!(periodic_visible.is_visible_phase());
        assert!(periodic_visible.is_finished());
    }

    #[test]
    fn fading_keeps_its_opaque_ticks_then_fades_to_five_percent() {
        let mut fading = Fading::new(1);

        assert_eq!(fading.opacity_percent(), FULL_OPACITY_PERCENT);
        assert!(fading.update());
        assert_eq!(fading.opacity_percent(), FULL_OPACITY_PERCENT);

        for opacity_percent in (1..Fading::NUM_FADE_TICKS).rev().map(|step| step * FADE_PERCENT_PER_TICK) {
            fading.last_update_time = None;
            assert!(fading.update());
            assert_eq!(fading.opacity_percent(), opacity_percent);
        }

        fading.last_update_time = None;
        assert!(!fading.update());
        assert_eq!(fading.opacity_percent(), FADE_PERCENT_PER_TICK);
    }
}
