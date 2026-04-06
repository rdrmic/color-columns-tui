pub mod file;

#[cfg(feature = "dev-console")]
pub mod dev_console;

// ============================================================================
// FEATURE ENABLED: Working macros
// ============================================================================
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_gray { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Gray) }; }
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_cyan { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Cyan) }; }
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_yellow { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Yellow) }; }
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_red { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Red) }; }

// ============================================================================
// FEATURE DISABLED: Dummy macros and functions
// ============================================================================
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_gray {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_cyan {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_yellow {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_red {
    ($($arg:tt)*) => {};
}

// #[cfg(not(feature = "dev-console"))]
// pub mod dev_console {
//     use ratatui::{
//         Frame,
//         crossterm::event::{KeyEvent, MouseEvent},
//         layout::Rect,
//     };

//     pub fn handle_key_pressed_event(_key_event: KeyEvent) -> bool {
//         false
//     }
//     pub fn handle_mouse_scroll_event(_mouse_event: MouseEvent) {}
//     pub fn draw(_frame: &mut Frame, _area: Rect) {}
// }
