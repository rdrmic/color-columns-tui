use crate::errors::Context;

mod app;
mod blocks;
mod color_palettes;
mod data_dir;
mod errors;
mod game_state;
mod logging;
mod messages;
mod rendering;
mod scoring;
mod stage_handlers;
mod terminal;
mod visual_effects;

use color_palettes as palette;

fn main() {
    let app_data_dir_path = data_dir::get_app_data_dir_path();

    let log_file_path = logging::file_logger::init_logger(app_data_dir_path.as_deref())
        .context("Failed to setup application logging")
        .map_err(|e| eprintln!("Warning: {e}"))
        .ok()
        .flatten();

    let original_hook = std::panic::take_hook();
    let is_logging_initialized = log_file_path.is_some();
    std::panic::set_hook(Box::new(move |panic_info| {
        if is_logging_initialized {
            log::error!("FATAL: Application panicked:\n{panic_info}");
        }
        terminal::restore();
        original_hook(panic_info);
    }));

    if let Err(e) = run_app(app_data_dir_path.as_deref()) {
        eprintln!("Error: {e}");
        if let Some(log_file_path) = log_file_path {
            eprintln!("Check '{}' for full details.", log_file_path.display());
        }
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn run_app(app_data_dir_path: Option<&std::path::Path>) -> Result<(), errors::Error> {
    terminal::check_size()?;

    terminal::set_title();
    let terminal = terminal::init();

    let exit_result = app::App::new(app_data_dir_path)
        .context("Failed to initialize the app")
        .and_then(|app| app.run(terminal))
        .inspect(|()| log::info!("App exited normally"))
        .inspect_err(|e| log::error!("Fatal error: {e}"));

    terminal::restore();
    exit_result
}
