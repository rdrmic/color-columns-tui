use std::{fs::File, io::Write};

use env_logger::Builder;
use time::{OffsetDateTime, format_description::FormatItem, macros::format_description};

static TIME_FORMAT: &[FormatItem] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

pub fn init_logger() -> anyhow::Result<()> {
    let log_file = File::create("_last_run.log")?;

    Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
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
    Ok(())
}
