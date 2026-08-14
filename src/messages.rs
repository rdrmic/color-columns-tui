use ratatui::style::Color;

#[cfg(not(target_os = "macos"))]
use crate::visual_effects::FADE_PERCENT_PER_TICK;
use crate::{
    palette::in_game_messages,
    visual_effects::{Blinking, FULL_OPACITY_PERCENT, Fading},
};

const MSG_GET_READY: &str = "Get ready!";
const MSG_LEVEL_UP: &str = "Level up!";
const MSG_PAUSED: &str = "Paused...";
const MSG_GAME_OVER: &str = "Game over!";

pub struct Message {
    kind: MessageKind,
}

enum MessageKind {
    GetReady(Fading),
    LevelUp(Fading),
    Paused,
    GameOver(Blinking),
}

impl Message {
    pub const fn new_get_ready() -> Self {
        Self { kind: MessageKind::GetReady(Fading::new(1)) }
    }

    pub const fn new_level_up() -> Self {
        Self { kind: MessageKind::LevelUp(Fading::new(28)) }
    }

    pub const fn new_paused() -> Self {
        Self { kind: MessageKind::Paused }
    }

    pub fn new_game_over() -> Self {
        Self { kind: MessageKind::GameOver(Blinking::new()) }
    }

    pub const fn text(&self) -> &'static str {
        match self.kind {
            MessageKind::GetReady(_) => MSG_GET_READY,
            MessageKind::LevelUp(_) => MSG_LEVEL_UP,
            MessageKind::Paused => MSG_PAUSED,
            MessageKind::GameOver(_) => MSG_GAME_OVER,
        }
    }

    pub fn color(&self) -> Color {
        match &self.kind {
            MessageKind::GetReady(fading) => MessageColor::new(in_game_messages::GET_READY_COLOR_SOURCE).with_opacity(fading.opacity_percent()),
            MessageKind::LevelUp(fading) => MessageColor::new(in_game_messages::LEVEL_UP_COLOR_SOURCE).with_opacity(fading.opacity_percent()),
            MessageKind::Paused => MessageColor::new(in_game_messages::PAUSED_COLOR_SOURCE).with_opacity(FULL_OPACITY_PERCENT),
            MessageKind::GameOver(_) => MessageColor::new(in_game_messages::GAME_OVER_COLOR_SOURCE).with_opacity(FULL_OPACITY_PERCENT),
        }
    }

    pub const fn blinking(&self) -> Option<&Blinking> {
        match &self.kind {
            MessageKind::GameOver(blinking) => Some(blinking),
            _ => None,
        }
    }

    pub fn tick(&mut self) -> bool {
        match &mut self.kind {
            MessageKind::GetReady(fading) | MessageKind::LevelUp(fading) => fading.update(),
            MessageKind::Paused | MessageKind::GameOver(_) => true,
        }
    }
}

// =============================================================================
// Opacity scaling reduction factors
// =============================================================================
#[cfg(not(target_os = "macos"))]
const OPACITY_REDUCTION_FACTOR: u8 = greatest_common_divisor(FULL_OPACITY_PERCENT, FADE_PERCENT_PER_TICK);

#[cfg(not(target_os = "macos"))]
const COLOR_REDUCTION_FACTOR: u8 = {
    const fn rgb_greatest_common_divisor([r, g, b]: [u8; 3]) -> u8 {
        greatest_common_divisor(greatest_common_divisor(r, g), b)
    }

    let mut factor = FULL_OPACITY_PERCENT / OPACITY_REDUCTION_FACTOR;
    factor = greatest_common_divisor(factor, rgb_greatest_common_divisor(in_game_messages::GET_READY_COLOR_SOURCE));
    factor = greatest_common_divisor(factor, rgb_greatest_common_divisor(in_game_messages::LEVEL_UP_COLOR_SOURCE));
    factor = greatest_common_divisor(factor, rgb_greatest_common_divisor(in_game_messages::PAUSED_COLOR_SOURCE));
    greatest_common_divisor(factor, rgb_greatest_common_divisor(in_game_messages::GAME_OVER_COLOR_SOURCE))
};

#[cfg(not(target_os = "macos"))]
const SCALE_DENOMINATOR: u8 = FULL_OPACITY_PERCENT / OPACITY_REDUCTION_FACTOR / COLOR_REDUCTION_FACTOR;

#[cfg(not(target_os = "macos"))]
const fn greatest_common_divisor(mut a: u8, mut b: u8) -> u8 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// =============================================================================
// Message colors handling
// =============================================================================
#[cfg(not(target_os = "macos"))]
#[derive(Copy, Clone)]
struct MessageColor([u8; 3]);

#[cfg(not(target_os = "macos"))]
impl MessageColor {
    const fn new(source: [u8; 3]) -> Self {
        Self(source)
    }

    fn with_opacity(self, opacity_percent: u8) -> Color {
        let [r, g, b] = self.0;
        Color::Rgb(Self::scale(r, opacity_percent), Self::scale(g, opacity_percent), Self::scale(b, opacity_percent))
    }

    // Cancel compile-time common factors before multiplying. The factors are
    // derived from the fade increment and every color passed to this method,
    // so the result remains exact if either changes.
    fn scale(channel: u8, opacity_percent: u8) -> u8 {
        debug_assert!(channel.is_multiple_of(COLOR_REDUCTION_FACTOR));
        debug_assert!(opacity_percent.is_multiple_of(OPACITY_REDUCTION_FACTOR));

        ((u16::from(channel / COLOR_REDUCTION_FACTOR) * u16::from(opacity_percent / OPACITY_REDUCTION_FACTOR)) / u16::from(SCALE_DENOMINATOR)) as u8
    }
}

#[cfg(target_os = "macos")]
#[derive(Copy, Clone)]
struct MessageColor(Color);

#[cfg(target_os = "macos")]
impl MessageColor {
    const fn new(source: Color) -> Self {
        Self(source)
    }

    // Opacity cannot be applied to 8-bit indexed colors.
    const fn with_opacity(self, _opacity_percent: u8) -> Color {
        self.0
    }
}
