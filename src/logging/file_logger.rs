use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Mutex, PoisonError},
};

pub struct FileLogger {
    file: Mutex<File>,
}

impl log::Log for FileLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let _ = writeln!(
            &mut *self.file.lock().unwrap_or_else(PoisonError::into_inner),
            "{:5} [{}:{}] {}",
            record.level(),
            record.file().and_then(|f| f.split_once("src/")).map_or("<unknown>", |(_, p)| p),
            record.line().unwrap_or(0),
            record.args()
        );
    }

    fn flush(&self) {
        let _ = self.file.lock().unwrap_or_else(PoisonError::into_inner).flush();
    }
}

pub fn init_logger() -> Result<PathBuf, std::io::Error> {
    let log_file_path = assemble_log_file_path()?;
    let file = File::create(&log_file_path)?;

    let logger = Box::leak(Box::new(FileLogger { file: Mutex::new(file) }));
    log::set_logger(logger).map_err(|_| std::io::Error::other("Logger already set"))?;
    log::set_max_level(log::LevelFilter::Debug);

    Ok(log_file_path)
}

fn assemble_log_file_path() -> Result<PathBuf, std::io::Error> {
    let mut log_dir = std::env::var_os("XDG_STATE_HOME").map_or_else(
        || {
            let home = std::env::var_os("HOME").expect("HOME env var not set");
            PathBuf::from(home).join(".local").join("state")
        },
        PathBuf::from,
    );
    log_dir.push(env!("CARGO_PKG_NAME"));

    std::fs::create_dir_all(&log_dir).map_err(|_| std::io::Error::other("Failed to create log file directory"))?;

    let log_file_path = log_dir.join("last_run.log");
    Ok(log_file_path)
}
