use std::{env, fs::File, io::Write, path::PathBuf};

use anyhow::Context;
use time::{OffsetDateTime, format_description::FormatItem, macros::format_description};

static TIME_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

pub fn init_logger() -> anyhow::Result<PathBuf> {
    let log_file_path = assemble_log_file_path()?;
    let log_file = File::create(&log_file_path).context("Failed to create log file")?;

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Never)
        .format(move |buf, record| {
            let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

            writeln!(
                buf,
                "{} {:5} [{}:{}] - {}",
                now.format(&TIME_FORMAT).unwrap_or_else(|_| "<invalid time>".to_string()),
                record.level(),
                record.file().unwrap_or("<unknown>").split_once("src/").map_or("", |(_, path)| path),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    log::info!("Logger initialized");

    Ok(log_file_path)
}

fn assemble_log_file_path() -> anyhow::Result<PathBuf> {
    let mut log_dir = env::var_os("XDG_STATE_HOME").map_or_else(
        || {
            let home = env::var_os("HOME").expect("HOME env var not set");
            PathBuf::from(home).join(".local").join("state")
        },
        PathBuf::from,
    );
    log_dir.push(env!("CARGO_PKG_NAME"));

    std::fs::create_dir_all(&log_dir).context("Failed to create log file directory")?;

    let log_file_path = log_dir.join("last_run.log");

    Ok(log_file_path)
}
