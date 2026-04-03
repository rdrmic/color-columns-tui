mod app;
mod logger;

use std::io;

use anyhow::Context;
use ratatui::crossterm;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    logger::init_logger()?;

    set_terminal_title().context("Failed to set terminal title")?;

    // TODO check if minimum num of (columns x rows) is satisfied
    let (columns, rows) = crossterm::terminal::size().context("Failed to get terminal size")?;
    log::info!("Terminal size (columns x rows): {columns} x {rows}");

    let terminal = ratatui::init();

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

    crossterm::execute!(io::stdout(), crossterm::terminal::SetTitle(title))?;
    Ok(())
}
