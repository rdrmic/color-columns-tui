use ratatui::style::Color;

use crate::{
    palette::in_game_messages,
    visual_effects::{Blinking, Fading},
};

pub struct Message {
    attributes: MessageAttributes,
    blinking: Option<Blinking>,
    fading: Option<Fading>,
}

#[rustfmt::skip]
impl Message {
    pub const fn new_fading(
        msg_type: MessageType,
        num_ticks_while_opaque: u8,
        fade_percent_per_tick: u8,
    ) -> Self {
        Self {
            attributes: msg_type.attributes(),
            blinking: None,
            fading: Some(Fading::new(
                num_ticks_while_opaque,
                fade_percent_per_tick,
            )),
        }
    }

    pub fn new_blinking(msg_type: MessageType) -> Self {
        Self {
            attributes: msg_type.attributes(),
            blinking: Some(Blinking::new()),
            fading: None,
        }
    }

    pub const fn new_permanent(msg_type: MessageType) -> Self {
        Self {
            attributes: msg_type.attributes(),
            blinking: None,
            fading: None,
        }
    }

    pub const fn text(&self) -> &'static str {
        self.attributes.text
    }

    pub fn color(&self) -> Color {
        let opacity_percent = self.fading.as_ref().map_or(100, Fading::opacity_percent);
        self.attributes.color.with_opacity(opacity_percent)
    }

    pub const fn blinking(&self) -> Option<&Blinking> {
        self.blinking.as_ref()
    }

    pub fn tick(&mut self) -> bool {
        if let Some(blinking) = self.blinking.as_mut() {
            blinking.update();
        }

        if let Some(fading) = self.fading.as_mut() {
            return fading.update();
        }

        true
    }
}

// =============================================================================
// Message attributes: text and color
// =============================================================================
struct MessageAttributes {
    text: &'static str,
    color: MessageColor,
}

#[derive(Copy, Clone)]
pub enum MessageType {
    GetReady,
    LevelUp,
    Paused,
    GameOver,
}

impl MessageType {
    #[rustfmt::skip]
    const fn attributes(self) -> MessageAttributes {
        match self {
            Self::GetReady => MessageAttributes { text: "Get ready!", color: MessageColor::new(in_game_messages::GET_READY_COLOR_SOURCE) },
            Self::LevelUp  => MessageAttributes { text: "Level up!",  color: MessageColor::new(in_game_messages::LEVEL_UP_COLOR_SOURCE) },
            Self::Paused   => MessageAttributes { text: "Paused...",  color: MessageColor::new(in_game_messages::PAUSED_COLOR_SOURCE) },
            Self::GameOver => MessageAttributes { text: "Game over!", color: MessageColor::new(in_game_messages::GAME_OVER_COLOR_SOURCE) },
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum ColorSource {
    Rgb([u8; 3]),
    Indexed(Color),
}

#[derive(Copy, Clone)]
struct MessageColor {
    source: ColorSource,
}

impl MessageColor {
    #[cfg(not(target_os = "macos"))]
    const fn new(source: [u8; 3]) -> Self {
        Self { source: ColorSource::Rgb(source) }
    }

    #[cfg(target_os = "macos")]
    const fn new(source: Color) -> Self {
        Self { source: ColorSource::Indexed(source) }
    }

    fn with_opacity(self, opacity_percent: u8) -> Color {
        match self.source {
            ColorSource::Rgb(rgb_components) => {
                let [r, g, b] = rgb_components.map(|channel| Self::scale(channel, opacity_percent));
                Color::Rgb(r, g, b)
            }
            // Cannot apply opacity to 8-bit (indexed) colors
            ColorSource::Indexed(color) => color,
        }
    }

    fn scale(channel: u8, percent: u8) -> u8 {
        ((u16::from(channel) * u16::from(percent)) / 100) as u8
    }
}
