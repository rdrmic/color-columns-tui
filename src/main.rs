mod app;
mod blocks;
mod errors;
mod game;
mod logging;
mod messages;
mod rendering;
mod scoring;
mod stage_handlers;

use std::path::{Path, PathBuf};

use ratatui::{Terminal, backend::CrosstermBackend, crossterm};

use crate::errors::Context;

const TERMINAL_TITLE: &str = concat!(env!("CARGO_PKG_DESCRIPTION"), " v", env!("CARGO_PKG_VERSION"));

fn main() {
    let app_state_dir_path = create_app_state_dir().context("Failed to create application state directory").map_err(stderr_warning).ok();

    let log_file_path =
        logging::file_logger::init_logger(app_state_dir_path.as_deref()).context("Failed to setup application logging").map_err(stderr_warning).ok().flatten();

    let original_hook = std::panic::take_hook();
    let is_logging_initialized = log_file_path.is_some();
    std::panic::set_hook(Box::new(move |panic_info| {
        if is_logging_initialized {
            log::error!("FATAL: Application panicked:\n{panic_info}");
        }
        restore_terminal();
        original_hook(panic_info);
    }));

    if let Err(e) = run_app(app_state_dir_path.as_deref()) {
        eprintln!("Error: {e}");
        if let Some(log_file_path) = log_file_path {
            eprintln!("Check '{}' for full details.", log_file_path.display());
        }
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn create_app_state_dir() -> Result<PathBuf, std::io::Error> {
    let mut dir_path = std::env::var_os("XDG_STATE_HOME").map_or_else(
        || {
            let home = std::env::var_os("HOME").expect("HOME env var not set");
            PathBuf::from(home).join(".local").join("state")
        },
        PathBuf::from,
    );
    dir_path.push(env!("CARGO_PKG_NAME"));

    std::fs::create_dir_all(&dir_path).map_err(|_| std::io::Error::other("Failed to create app state directory"))?;
    Ok(dir_path)
}

fn run_app(app_state_dir_path: Option<&Path>) -> Result<(), errors::Error> {
    check_terminal_size()?;

    set_terminal_title();
    let terminal = init_terminal();

    let exit_result = app::App::new(app_state_dir_path)
        .context("Failed to initialize the app")
        .and_then(|app| app.run(terminal))
        .inspect(|()| log::info!("App exited normally"))
        .inspect_err(|e| log::error!("Fatal error: {e}"));

    restore_terminal();
    exit_result
}

fn check_terminal_size() -> Result<(), errors::Error> {
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

fn set_terminal_title() {
    let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::SetTitle(&TERMINAL_TITLE))
        .inspect(|()| log::info!("Terminal title '{TERMINAL_TITLE}' set"))
        .inspect_err(|e| log::warn!("Settting terminal title ({TERMINAL_TITLE}) failed: {e}"));
}

fn init_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    #[cfg(feature = "dev-console")]
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
        .inspect(|()| log::debug!("Mouse event capturing enabled"))
        .inspect_err(|e| log::warn!("Mouse event capturing enabling failed: {e}"));

    ratatui::init()
}

fn restore_terminal() {
    #[cfg(feature = "dev-console")]
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)
        .inspect(|()| log::debug!("Mouse event capturing disabled"))
        .inspect_err(|e| log::warn!("Mouse event capturing disabling failed: {e}"));

    ratatui::restore();
}

fn stderr_warning(e: impl std::fmt::Display) {
    eprintln!("Warning: {e}");
}
