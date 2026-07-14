use std::time::Instant;

use ratatui::style::Color;

use crate::{palette::in_game_messages, stage_handlers::FRAME_DURATION_GAMEPLAY};

pub struct Message {
    text: &'static str,
    color: MessageColor,
    opacity_percent: u8,
    num_ticks_while_opaque: u8,
    fade_percent_per_tick: u8,
    fading_time: Option<Instant>,
}

impl Message {
    pub const fn new_fading(text: &'static str, color: MessageColor, num_ticks_while_opaque: u8, fade_percent_per_tick: u8) -> Self {
        Self { text, color, opacity_percent: 100, num_ticks_while_opaque, fade_percent_per_tick, fading_time: None }
    }

    pub const fn new_permanent(text: &'static str, color: MessageColor) -> Self {
        Self { text, color, opacity_percent: 100, num_ticks_while_opaque: 0, fade_percent_per_tick: 0, fading_time: None }
    }

    pub const fn text(&self) -> &'static str {
        self.text
    }

    pub fn color(&self) -> Color {
        self.color.calculate(self.opacity_percent)
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

// =============================================================================
// Message colors
// =============================================================================
#[derive(Copy, Clone)]
pub enum MessageColor {
    GetReady,
    LevelUp,
    Paused,
    GameOver,
}

impl MessageColor {
    #[cfg(not(target_os = "macos"))]
    pub fn calculate(self, opacity_percent: u8) -> Color {
        // Returns `[u8; 3]` RGB components
        let rgb = match self {
            Self::GetReady => in_game_messages::RGB_GET_READY,
            Self::LevelUp => in_game_messages::RGB_LEVEL_UP,
            Self::Paused => in_game_messages::RGB_PAUSED,
            Self::GameOver => in_game_messages::RGB_GAME_OVER,
        };

        let opacity = u16::from(opacity_percent);

        let [r, g, b] = rgb.map(|channel| ((u16::from(channel) * opacity) / 100) as u8);
        Color::Rgb(r, g, b)
    }

    #[cfg(target_os = "macos")]
    pub const fn calculate(self, _opacity_percent: u8) -> Color {
        // Returns `Color::indexed` colors
        match self {
            Self::GetReady => in_game_messages::RGB_GET_READY,
            Self::LevelUp => in_game_messages::RGB_LEVEL_UP,
            Self::Paused => in_game_messages::RGB_PAUSED,
            Self::GameOver => in_game_messages::RGB_GAME_OVER,
        }
    }
}
