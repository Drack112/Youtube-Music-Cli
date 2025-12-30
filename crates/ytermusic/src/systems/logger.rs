use flume::Sender;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError, info};
use once_cell::sync::Lazy;

use std::{io::Write, path::PathBuf};

use crate::utils::get_project_dirs;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LEVEL.0))?;
    info!("Logger mode {}", LEVEL.1);
    Ok(())
}

static LOG: Lazy<Sender<String>> = Lazy::new(|| {
    let (tx, rx) = flume::unbounded::<String>();
    std::thread::spawn(move || {
        let mut buffer = String::new();
        let filepath = get_log_file_path();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .unwrap();

        while let Ok(e) = rx.recv() {
            buffer.clear();
            buffer.push_str(&(e + "\n"));
            while let Ok(e) = rx.try_recv() {
                buffer.push_str(&(e + "\n"));
            }
            file.write_all(buffer.as_bytes()).unwrap();
        }
    });
    tx
});

struct SimpleLogger;
static FILTER: &[&str] = &["rustls", "tokio-util", "want-", "mio-"];

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LEVEL.1
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if FILTER.iter().any(|x| record.file().unwrap().contains(x)) {
                return;
            }
            LOG.send(format!(
                "{} - {} [{}]",
                record.level(),
                record.args(),
                record.file().unwrap_or_default(),
            ))
            .unwrap();
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;
static LEVEL: Lazy<(LevelFilter, Level)> = Lazy::new(|| {
    let logger_env = std::env::var("YTMUSIC_LOG");
    if let Ok(logger_env) = logger_env {
        if logger_env == "true" {
            (LevelFilter::Trace, Level::Trace)
        } else {
            (LevelFilter::Info, Level::Info)
        }
    } else {
        (LevelFilter::Info, Level::Info)
    }
});

pub fn get_log_file_path() -> PathBuf {
    if let Some(val) = get_project_dirs() {
        if let Err(e) = std::fs::create_dir_all(val.cache_dir()) {
            panic!("Failed to create cache dir: {}", e);
        }
        val.cache_dir().join("log.txt")
    } else {
        PathBuf::from("log.txt")
    }
}
