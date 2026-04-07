mod app;
mod blocks;
mod game;
mod logging;
mod renderer;

use anyhow::Context;
use ratatui::{Terminal, backend::CrosstermBackend, crossterm};

fn main() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_terminal();
        original_hook(panic_info);
    }));

    let log_file_path = logging::file::init_logger()
        .context("Failed to setup application logging")
        .inspect_err(|err| eprintln!("Warning: {err:?}"))
        .ok();

    if let Err(err) = run_app() {
        eprintln!("Error: {err}");
        if let Some(log_file_path) = log_file_path {
            eprintln!("Check '{}' for full details.", log_file_path.display());
        }
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn run_app() -> anyhow::Result<()> {
    check_terminal_size()?;

    set_terminal_title();
    let terminal = init_terminal();

    let exit_result = app::App::new()
        .context("Failed to initialize the app")
        .and_then(|app| app.run(terminal))
        .inspect(|()| log::info!("App exited normally"))
        .inspect_err(|err| log::error!("Fatal error: {err:#?}"));

    restore_terminal();
    exit_result
}

fn check_terminal_size() -> anyhow::Result<()> {
    let (columns, rows) = crossterm::terminal::size().context("Failed to get terminal size")?;
    log::info!("Terminal size (columns x rows): {columns} x {rows}");

    // TODO check if minimum num of (columns x rows) is satisfied
    let min_columns = 20;
    if columns < min_columns {
        anyhow::bail!("Terminal width must be at least {min_columns} columns (current: {columns})");
    }

    let min_rows = 40;
    if rows < min_rows {
        anyhow::bail!("Terminal height must be at least {min_rows} rows (current: {rows})");
    }
    Ok(())
}

fn set_terminal_title() {
    let title = [env!("CARGO_PKG_DESCRIPTION"), " v", env!("CARGO_PKG_VERSION")].concat();
    let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::SetTitle(&title))
        .inspect(|()| log::info!("Terminal title '{title}' set"))
        .inspect_err(|err| log::warn!("Settting terminal title ({title}) failed: {err:?}"));
}

fn init_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    #[cfg(feature = "dev-console")]
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
        .inspect(|()| log::debug!("Mouse event capturing enabled"))
        .inspect_err(|err| log::warn!("Mouse event capturing enabling failed: {err:?}"));

    ratatui::init()
}

fn restore_terminal() {
    #[cfg(feature = "dev-console")]
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)
        .inspect(|()| log::debug!("Mouse event capturing disabled"))
        .inspect_err(|err| log::warn!("Mouse event capturing disabling failed: {err:?}"));

    ratatui::restore();
}
