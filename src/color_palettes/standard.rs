#![cfg(not(target_os = "macos"))]

use ratatui::style::Color;

#[allow(clippy::wildcard_imports)]
use super::reusable_color_components::standard_components::*;

const fn rgb_to_color(rgb: [u8; 3]) -> Color {
    Color::Rgb(rgb[0], rgb[1], rgb[2])
}

// --- Gems ---
pub const GEM_RUBY: Color = rgb_to_color(RASPBERRY);
pub const GEM_EMERALD: Color = rgb_to_color(MINT);
pub const GEM_SAPPHIRE: Color = rgb_to_color(TEAL);
pub const GEM_TOPAZ: Color = rgb_to_color(YELLOW);
pub const GEM_AMETHYST: Color = rgb_to_color(VIOLET);
pub const GEM_AMBER: Color = rgb_to_color(ORANGE);

// --- Stats ---
pub const STAT_LEVEL: Color = rgb_to_color(ORANGE_BRIGHT);
pub const STAT_SCORE: Color = rgb_to_color(GREEN);
pub const STAT_MAX_COMBO: Color = rgb_to_color(BLUE);
pub const STAT_HIGHSCORE: Color = rgb_to_color(RED);
pub const STATS_VALUE: Color = rgb_to_color(CYAN_LIGHT);

// --- UI & layout ---
pub const UI_CANVAS_BG: Color = rgb_to_color(BLUE_DARK);
pub const UI_CANVAS_FG: Color = rgb_to_color(GREEN);
pub const UI_GAME_BORDER: Color = rgb_to_color(GRAY_MEDIUM);
pub const UI_LEGEND_KEY: Color = rgb_to_color(GREEN_KHAKI);
pub const UI_LEGEND_VALUE: Color = rgb_to_color(GRAY_LIGHT);
pub const UI_LEGEND_BORDER: Color = rgb_to_color(GRAY_MEDIUM);

// --- Console messages ---
pub const CONSOLE_TEXT_INFO: Color = rgb_to_color(GREEN);
pub const CONSOLE_TEXT_ERROR: Color = rgb_to_color(RED);
