//! Two separate color palettes are maintained to ensure the game renders correctly
//! out-of-the-box for a wide audience, without requiring players to install
//! custom terminal emulators.
//!
//! - macOS: The default macOS Terminal uses 8-bit (256) colors.
//!   Passing 24-bit RGB values makes the terminal guess the closest match,
//!   often resulting in washed-out or inaccurate colors. Precise 8-bit
//!   indexed colors are used here for predictable rendering.
//! - Other OSs: Linux and Windows terminals (even legacy ones, like cmd and
//!   PowerShell) support 24-bit true color natively, so the exact RGB
//!   arrays are used for them.

#[cfg(not(target_os = "macos"))]
mod standard;

#[cfg(not(target_os = "macos"))]
pub use standard::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

// =============================================================================
// RGB arrays and bytes used for uniform color rendering across OSs / terminals
// =============================================================================
#[rustfmt::skip]
pub(in crate::color_palettes) mod reusable_color_components {
    #[cfg(not(target_os = "macos"))]
    pub mod standard_components {
        // Gem colors
        pub const RASPBERRY:        [u8; 3] = [230,  20, 100];
        pub const MINT:             [u8; 3] = [  0, 215, 135];
        pub const TEAL:             [u8; 3] = [  0, 135, 175];
        pub const YELLOW:           [u8; 3] = [255, 215,  95];
        pub const VIOLET:           [u8; 3] = [135,   0, 255];
        pub const ORANGE:           [u8; 3] = [255, 100,   0];
        // Other colors
        pub const RED:              [u8; 3] = [230,  40,  50];
        pub const RED_BRIGHT:       [u8; 3] = [250,  15,  25];
        pub const GREEN:            [u8; 3] = [  0, 195,   0];
        pub const BLUE:             [u8; 3] = [  0, 125, 200];
        pub const BLUE_DARK:        [u8; 3] = [  6,  14,  18];
        pub const GREEN_KHAKI:      [u8; 3] = [175, 215, 135];
        pub const ORANGE_BRIGHT:    [u8; 3] = [255, 135,   0];
        pub const CYAN_LIGHT:       [u8; 3] = [175, 215, 215];
        pub const GRAY_MEDIUM:      [u8; 3] = [138, 138, 138];
        pub const GRAY_LIGHT:       [u8; 3] = [170, 170, 170];
    }
    #[cfg(not(target_os = "macos"))]
    pub use standard_components::*;

    #[cfg(target_os = "macos")]
    pub mod macos_components {
        // Gem colors
        pub const RASPBERRY:        u8 = 161;
        pub const MINT:             u8 =  42;
        pub const TEAL:             u8 =  31;
        pub const YELLOW:           u8 = 221;
        pub const VIOLET:           u8 =  93;
        pub const ORANGE:           u8 = 202;
        // Other colors
        pub const RED:              u8 = 160;
        pub const RED_BRIGHT:       u8 = 196;
        pub const GREEN:            u8 =  40;
        pub const BLUE:             u8 =  25;
        pub const BLUE_DARK:        u8 = 233;
        pub const GREEN_KHAKI:      u8 = 150;
        pub const ORANGE_BRIGHT:    u8 = 208;
        pub const CYAN_LIGHT:       u8 = 152;
        pub const GRAY_MEDIUM:      u8 = 245;
        pub const GRAY_LIGHT:       u8 = 248;
    }
    #[cfg(target_os = "macos")]
    pub use macos_components::*;
}

// =============================================================================
// Convenience arrays for message-fading math in `messages.rs`
// =============================================================================
#[cfg(not(target_os = "macos"))]
#[rustfmt::skip]
pub mod in_game_messages {
    use super::reusable_color_components::{GREEN, ORANGE_BRIGHT, GRAY_LIGHT, RED_BRIGHT};

    pub const GET_READY_COLOR_SOURCE:   [u8; 3] = GREEN;
    pub const LEVEL_UP_COLOR_SOURCE:    [u8; 3] = ORANGE_BRIGHT;
    pub const PAUSED_COLOR_SOURCE:      [u8; 3] = GRAY_LIGHT;
    pub const GAME_OVER_COLOR_SOURCE:   [u8; 3] = RED_BRIGHT;
}

#[cfg(target_os = "macos")]
#[rustfmt::skip]
pub mod in_game_messages {
    use ratatui::style::Color;

    use super::{
        byte_to_color,
        reusable_color_components::{GREEN, ORANGE_BRIGHT, GRAY_LIGHT, RED_BRIGHT},
    };

    pub const GET_READY_COLOR_SOURCE:   Color = byte_to_color(GREEN);
    pub const LEVEL_UP_COLOR_SOURCE:    Color = byte_to_color(ORANGE_BRIGHT);
    pub const PAUSED_COLOR_SOURCE:      Color = byte_to_color(GRAY_LIGHT);
    pub const GAME_OVER_COLOR_SOURCE:   Color = byte_to_color(RED_BRIGHT);
}

// =============================================================================
// Feature "dev-console" unique colors (`dev_console.rs`)
// =============================================================================
#[cfg(feature = "dev-console")]
#[rustfmt::skip]
pub mod dev_console {
    #[cfg(not(target_os = "macos"))]
    mod standard {
        use ratatui::style::Color;

        pub const DEV_CONSOLE_BORDER: Color =   Color::Rgb(0, 255, 0);
        pub const DEV_CONSOLE_GRAY: Color =     Color::Rgb(210, 210, 215);
        pub const DEV_CONSOLE_CYAN: Color =     Color::Rgb(0, 230, 230);
        pub const DEV_CONSOLE_YELLOW: Color =   Color::Rgb(200, 200, 0);
        pub const DEV_CONSOLE_RED: Color =      Color::Rgb(245, 75, 75);
    }
    #[cfg(not(target_os = "macos"))]
    pub use standard::*;

    #[cfg(target_os = "macos")]
    mod macos {
        use ratatui::style::Color;

        pub const DEV_CONSOLE_BORDER: Color =   Color::Indexed(46);
        pub const DEV_CONSOLE_GRAY: Color =     Color::Indexed(249);
        pub const DEV_CONSOLE_CYAN: Color =     Color::Indexed(44);
        pub const DEV_CONSOLE_YELLOW: Color =   Color::Indexed(220);
        pub const DEV_CONSOLE_RED: Color =      Color::Indexed(196);
    }
    #[cfg(target_os = "macos")]
    pub use macos::*;
}
#[cfg(feature = "dev-console")]
pub use dev_console::*;
