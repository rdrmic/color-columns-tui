mod app;
mod logger;

use anyhow::Context;
use ratatui::{Terminal, crossterm, prelude::CrosstermBackend};

use crate::app::App;

fn main() {
    let log_file_path = logger::init_logger()
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
    set_terminal_title().context("Failed to set terminal title")?;

    let terminal = init_terminal().context("Failed to initialize terminal")?;

    let exit_result = App::new()
        .context("Failed to initialize the app")
        .and_then(|app| app.run(terminal))
        .inspect(|()| log::info!("App exited normally"))
        .inspect_err(|err| log::error!("Fatal error: {err:#?}"));

    ratatui::restore();

    exit_result
}

fn set_terminal_title() -> anyhow::Result<()> {
    let title = [env!("CARGO_PKG_DESCRIPTION"), " v", env!("CARGO_PKG_VERSION")].concat();
    log::info!("{title}");

    crossterm::execute!(std::io::stdout(), crossterm::terminal::SetTitle(title))?;

    Ok(())
}

fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
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

    Ok(ratatui::init())
}
