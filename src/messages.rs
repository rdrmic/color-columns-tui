use std::time::Instant;

use ratatui::style::Color;

use crate::stage_handlers::FRAME_DURATION_GAMEPLAY;

pub struct Message {
    text: &'static str,
    rgb: [u8; 3],
    opacity_percent: u8,
    num_ticks_while_opaque: u8,
    fade_percent_per_tick: u8,
    fading_time: Option<Instant>,
}

impl Message {
    pub const fn new_fading(text: &'static str, rgb: [u8; 3], num_ticks_while_opaque: u8, fade_percent_per_tick: u8) -> Self {
        Self { text, rgb, opacity_percent: 100, num_ticks_while_opaque, fade_percent_per_tick, fading_time: None }
    }

    pub const fn new_permanent(text: &'static str, rgb: [u8; 3]) -> Self {
        Self { text, rgb, opacity_percent: 100, num_ticks_while_opaque: 0, fade_percent_per_tick: 0, fading_time: None }
    }

    pub const fn text(&self) -> &'static str {
        self.text
    }

    pub fn color(&self) -> Color {
        let opacity_percent = u16::from(self.opacity_percent);

        let [r, g, b] = self.rgb.map(|channel| ((u16::from(channel) * opacity_percent) / 100) as u8);
        Color::Rgb(r, g, b)
    }

    pub fn tick(&mut self) -> bool {
        if self.fade_percent_per_tick == 0 {
            return true;
        }

        let now = Instant::now();
        if let Some(fading_time) = self.fading_time
            && now.duration_since(fading_time) < FRAME_DURATION_GAMEPLAY
        {
            return true;
        }
        self.fading_time = Some(now);

        if self.num_ticks_while_opaque > 0 {
            self.num_ticks_while_opaque -= 1;
            return true;
        }

        if self.opacity_percent > self.fade_percent_per_tick {
            self.opacity_percent -= self.fade_percent_per_tick;
            true
        } else {
            false
        }
    }
}
