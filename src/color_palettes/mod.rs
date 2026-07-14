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
        pub const RGB_RED:                  [u8; 3] = [239,  71, 111];
        pub const RGB_CYAN_PASTEL:          [u8; 3] = [  6, 214, 160];
        pub const RGB_BLUE:                 [u8; 3] = [ 17, 138, 194];
        pub const RGB_YELLOW:               [u8; 3] = [255, 209, 102];
        pub const RGB_PURPLE:               [u8; 3] = [131,  56, 236];
        pub const RGB_ORANGE:               [u8; 3] = [247, 127,   0];
        // Other colors
        pub const RGB_BLUE_DARK:            [u8; 3] = [  6,  14,  18];
        pub const RGB_GREEN:                [u8; 3] = [  0, 195,   0];
        pub const RGB_GREEN_KHAKI:          [u8; 3] = [175, 215, 135];
        pub const RGB_ORANGE_BRIGHT:        [u8; 3] = [255, 135,   0];
        pub const RGB_CYAN_LIGHT:           [u8; 3] = [175, 215, 215];
        pub const RGB_GRAY_DARK:            [u8; 3] = [ 88,  88,  88];
        pub const RGB_GRAY_MEDIUM:          [u8; 3] = [138, 138, 138];
        pub const RGB_GRAY_LIGHT:           [u8; 3] = [170, 170, 170];
    }
    #[cfg(not(target_os = "macos"))]
    pub use standard_components::*;

    #[cfg(target_os = "macos")]
    pub mod macos_components {
        // Gem colors
        pub const RGB_RED:                  u8 = 161;
        pub const RGB_CYAN_PASTEL:          u8 =  43;
        pub const RGB_BLUE:                 u8 =  31;
        pub const RGB_YELLOW:               u8 = 221;
        pub const RGB_PURPLE:               u8 =  93;
        pub const RGB_ORANGE:               u8 = 202;
        // Other colors
        pub const RGB_BLUE_DARK:            u8 = 232;   // TODO RGB(8,8,8) - change
        pub const RGB_GREEN:                u8 =  40;
        pub const RGB_GREEN_KHAKI:          u8 = 150;
        pub const RGB_ORANGE_BRIGHT:        u8 = 208;
        pub const RGB_CYAN_LIGHT:           u8 = 152;
        pub const RGB_GRAY_DARK:            u8 = 240;
        pub const RGB_GRAY_MEDIUM:          u8 = 245;
        pub const RGB_GRAY_LIGHT:           u8 = 248;
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
    use super::reusable_color_components::{RGB_GREEN, RGB_ORANGE_BRIGHT, RGB_GRAY_LIGHT, RGB_RED};

    pub const RGB_GET_READY:        [u8; 3] = RGB_GREEN;
    pub const RGB_LEVEL_UP:         [u8; 3] = RGB_ORANGE_BRIGHT;
    pub const RGB_PAUSED:           [u8; 3] = RGB_GRAY_LIGHT;
    pub const RGB_GAME_OVER:        [u8; 3] = RGB_RED;  // TODO make it more red
}

#[cfg(target_os = "macos")]
#[rustfmt::skip]
pub mod in_game_messages {
    use ratatui::style::Color;

    use super::{
        byte_to_color,
        reusable_color_components::{RGB_GREEN, RGB_ORANGE_BRIGHT, RGB_GRAY_LIGHT, RGB_RED},
    };

    pub const RGB_GET_READY:        Color = byte_to_color(RGB_GREEN);
    pub const RGB_LEVEL_UP:         Color = byte_to_color(RGB_ORANGE_BRIGHT);
    pub const RGB_PAUSED:           Color = byte_to_color(RGB_GRAY_LIGHT);
    pub const RGB_GAME_OVER:        Color = byte_to_color(RGB_RED);  // TODO make it more red
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

        pub const DEV_CONSOLE_BORDER: Color =       Color::Rgb(0, 255, 0);
        pub const DEV_CONSOLE_GRAY: Color =         Color::Rgb(210, 210, 215);
        pub const DEV_CONSOLE_CYAN: Color =         Color::Rgb(0, 230, 230);
        pub const DEV_CONSOLE_YELLOW: Color =       Color::Rgb(255, 215, 0);
        pub const DEV_CONSOLE_RED: Color =          Color::Rgb(255, 75, 75);
    }
    #[cfg(not(target_os = "macos"))]
    pub use standard::*;

    #[cfg(target_os = "macos")]
    mod macos {
        use ratatui::style::Color;

        pub const DEV_CONSOLE_BORDER: Color =       Color::Indexed(46);
        pub const DEV_CONSOLE_GRAY: Color =         Color::Indexed(248);
        pub const DEV_CONSOLE_CYAN: Color =         Color::Indexed(51);
        pub const DEV_CONSOLE_YELLOW: Color =       Color::Indexed(226);
        pub const DEV_CONSOLE_RED: Color =          Color::Indexed(196);
    }
    #[cfg(target_os = "macos")]
    pub use macos::*;
}
#[cfg(feature = "dev-console")]
pub use dev_console::*;
