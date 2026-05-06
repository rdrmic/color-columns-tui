use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
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

pub fn init_logger(app_state_dir_path: Option<&Path>) -> Result<Option<PathBuf>, std::io::Error> {
    let Some(dir_path) = app_state_dir_path else {
        return Ok(None);
    };

    let log_file_path = dir_path.join("last_run.log");
    let file = File::create(&log_file_path)?;

    let logger = Box::leak(Box::new(FileLogger { file: Mutex::new(file) }));
    log::set_logger(logger).map_err(|_| std::io::Error::other("Logger already set"))?;
    log::set_max_level(log::LevelFilter::Debug);

    Ok(Some(log_file_path))
}
