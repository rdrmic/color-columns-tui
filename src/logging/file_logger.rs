use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, PoisonError},
};

pub struct FileLogger {
    file: Mutex<File>,
}

impl FileLogger {
    fn lock_file(&self) -> MutexGuard<'_, File> {
        self.file.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let _ = writeln!(
            &mut *self.lock_file(),
            "{} [{}:{}] {}",
            padded_level(record.level()),
            source_path(record.file()),
            record.line().unwrap_or(0),
            record.args()
        );
    }

    fn flush(&self) {}
}

fn source_path(file: Option<&str>) -> &str {
    let Some(path) = file else {
        return "<unknown>";
    };

    let Some(index) = path.as_bytes().windows(4).position(|window| window == b"src/") else {
        return "<unknown>";
    };

    &path[index + 4..]
}

pub fn init_logger(app_data_dir_path: Option<&Path>) -> Result<Option<PathBuf>, std::io::Error> {
    let Some(dir_path) = app_data_dir_path else {
        return Ok(None);
    };

    let log_file_path = dir_path.join("last_run.log");
    let file = File::create(&log_file_path)?;

    let logger = Box::leak(Box::new(FileLogger { file: Mutex::new(file) }));
    log::set_logger(logger).map_err(|_| std::io::Error::other("Logger already set"))?;
    log::set_max_level(log::LevelFilter::Debug);

    Ok(Some(log_file_path))
}

#[rustfmt::skip]
const fn padded_level(level: log::Level) -> &'static str {
    match level {
        log::Level::Error => "ERROR",
        log::Level::Warn  => "WARN ",
        log::Level::Info  => "INFO ",
        log::Level::Debug => "DEBUG",
        log::Level::Trace => "TRACE",
    }
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::source_path;

    #[test]
    fn source_path_matches_the_previous_split_once_behavior() {
        let cases = [
            None,
            Some("src/main.rs"),
            Some("/workspace/project/src/main.rs"),
            Some("prefix/src/first/src/second.rs"),
            Some("résumé/src/file.rs"),
            Some("main.rs"),
            Some("src/"),
        ];

        for file in cases {
            let expected = file.and_then(|path| path.split_once("src/")).map_or("<unknown>", |(_, suffix)| suffix);
            assert_eq!(source_path(file), expected, "file: {file:?}");
        }
    }
}
