#![cfg(target_os = "macos")]

use ratatui::style::Color;

#[allow(clippy::wildcard_imports)]
use super::reusable_color_components::macos_components::*;

pub(super) const fn byte_to_color(byte: u8) -> Color {
    Color::Indexed(byte)
}

// --- Gems ---
pub const GEM_RUBY: Color = byte_to_color(RGB_RED);
pub const GEM_EMERALD: Color = byte_to_color(RGB_CYAN_PASTEL);
pub const GEM_SAPPHIRE: Color = byte_to_color(RGB_BLUE);
pub const GEM_TOPAZ: Color = byte_to_color(RGB_YELLOW);
pub const GEM_AMETHYST: Color = byte_to_color(RGB_PURPLE);
pub const GEM_AMBER: Color = byte_to_color(RGB_ORANGE);

// --- Stats ---
pub const STAT_LEVEL: Color = byte_to_color(RGB_ORANGE_BRIGHT);
pub const STAT_SCORE: Color = byte_to_color(RGB_GREEN); // TODO make it more pale
pub const STAT_MAX_COMBO: Color = byte_to_color(RGB_BLUE);
pub const STAT_HIGHSCORE: Color = byte_to_color(RGB_RED);
pub const STATS_VALUE: Color = byte_to_color(RGB_CYAN_LIGHT);

// --- UI & layout ---
pub const UI_CANVAS_BG: Color = byte_to_color(RGB_BLUE_DARK);
pub const UI_CANVAS_FG: Color = byte_to_color(RGB_GREEN);
pub const UI_GAME_BORDER: Color = byte_to_color(RGB_GRAY_MEDIUM);
pub const UI_LEGEND_KEY: Color = byte_to_color(RGB_GREEN_KHAKI);
pub const UI_LEGEND_VALUE: Color = byte_to_color(RGB_GRAY_LIGHT);
pub const UI_LEGEND_BORDER: Color = byte_to_color(RGB_GRAY_MEDIUM); // TODO bright green?

// --- Console messages ---
pub const CONSOLE_TEXT_INFO: Color = byte_to_color(RGB_GREEN);
pub const CONSOLE_TEXT_ERROR: Color = byte_to_color(RGB_RED); // TODO make it more red
pub const CONSOLE_TEXT_BORDER: Color = byte_to_color(RGB_GRAY_DARK);
