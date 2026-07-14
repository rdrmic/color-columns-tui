use ratatui::{Terminal, backend::CrosstermBackend, crossterm};

use crate::{
    errors::{self, Context},
    rendering,
};

const TERMINAL_TITLE: &str = concat!(env!("CARGO_PKG_DESCRIPTION"), " v", env!("CARGO_PKG_VERSION"));

pub fn check_size() -> Result<(), errors::Error> {
    let (columns, rows) = crossterm::terminal::size().context("Failed to get terminal size")?;
    log::info!("Terminal size (columns x rows): {columns} x {rows}");

    // An effective terminal window size check (with a message to the user) is being dynamically performed in src/rendering/mod.rs
    if columns < rendering::MIN_WINDOW_WIDTH {
        log::warn!("Terminal width must be at least {} columns (current: {})", rendering::MIN_WINDOW_WIDTH, columns);
    }
    if rows < rendering::MIN_WINDOW_HEIGHT {
        log::warn!("Terminal height must be at least {} rows (current: {})", rendering::MIN_WINDOW_HEIGHT, rows);
    }

    Ok(())
}

pub fn set_title() {
    let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::SetTitle(&TERMINAL_TITLE))
        .inspect(|()| log::info!("Terminal title '{TERMINAL_TITLE}' set"))
        .inspect_err(|e| log::warn!("Settting terminal title ({TERMINAL_TITLE}) failed: {e}"));
}

pub fn init() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    #[cfg(feature = "dev-console")]
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
        .inspect(|()| log::debug!("Mouse event capturing enabled"))
        .inspect_err(|e| log::warn!("Mouse event capturing enabling failed: {e}"));

    ratatui::init()
}

pub fn restore() {
    #[cfg(feature = "dev-console")]
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)
        .inspect(|()| log::debug!("Mouse event capturing disabled"))
        .inspect_err(|e| log::warn!("Mouse event capturing disabling failed: {e}"));

    ratatui::restore();
}

pub fn get_key_press() {
    let _ = crossterm::terminal::enable_raw_mode();
    let _ = crossterm::event::read();
    let _ = crossterm::terminal::disable_raw_mode();
}

pub fn has_emoji_support() -> bool {
    #[cfg(target_os = "windows")]
    {
        // Modern Windows Terminal
        if has_env(c"WT_SESSION") {
            return true;
        }
        // "Legacy" Windows cmd and PowerShell
        false
    }
    #[cfg(target_os = "linux")]
    {
        // If WSL is running inside Modern Windows Terminal, it inherits WT_SESSION
        if has_env(c"WT_SESSION") {
            return true;
        }
        // WSL is running inside "legacy" Windows cmd or PowerShell
        if has_env(c"WSL_DISTRO_NAME") {
            return false;
        }
        // Native Linux terminal emulators
        true
    }

    #[cfg(target_os = "macos")]
    {
        // macOS terminals
        true
    }
}

// =============================================================================
// Minimal environment variable check via C runtime `getenv`
// =============================================================================
/// Checks if an environment variable is present.
///
/// Uses the C runtime `getenv()` instead of `std::env` to keep small binaries
/// lean. The unsafe FFI boundary is contained here; the C string input is
/// guaranteed to be NUL-terminated and the returned pointer is only compared
/// against null, never dereferenced.
#[cfg(any(target_os = "windows", target_os = "linux"))]
#[allow(unsafe_code)]
fn has_env(name: &'static std::ffi::CStr) -> bool {
    unsafe extern "C" {
        fn getenv(name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    }

    unsafe { !getenv(name.as_ptr()).is_null() }
}
