use chrono::{Datelike, Utc};
use log::{LevelFilter, Record};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::{Mutex, MutexGuard},
};

mod config;
pub use config::*;

struct LogMessage {
    timestamp: String,
    level: String,
    message: String,
}

struct Logger {
    config: config::Config,
    file: Option<Mutex<(File, u32)>>,
}

impl Logger {
    fn new(config: Config) -> Self {
        let file = match &config.output {
            Output::File(path) => {
                let file = Self::open_log_file(path);
                Some(Mutex::new(file))
            }
            _ => None,
        };

        Logger { config, file }
    }

    fn open_log_file(path: &str) -> (File, u32) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Create a new file
            .open(format!("{}_{}.log", path, Utc::now().format("%Y-%m-%d")))
            .unwrap();
        (file, Utc::now().day())
    }

    fn check_rollover(&self) -> Option<MutexGuard<(File, u32)>> {
        if let Some(file_mutex) = &self.file {
            let mut file_guard = file_mutex.lock().unwrap();
            let current_day = Utc::now().day();
            if file_guard.1 != current_day {
                let path = match &self.config.output {
                    Output::File(path) => path,
                    _ => unreachable!(),
                };
                *file_guard = Self::open_log_file(path);
            }
            Some(file_guard)
        } else {
            None
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Utc::now();
            let log_message = LogMessage {
                timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                level: record.level().to_string(),
                message: record.args().to_string(),
            };

            match &self.config.output {
                Output::Stdout => {
                    println!(
                        "{} [{}] - {}",
                        log_message.timestamp, log_message.level, log_message.message
                    );
                }
                Output::File(_) => {
                    if let Some(mut file_guard) = self.check_rollover() {
                        writeln!(
                            file_guard.0,
                            "{} [{}] - {}",
                            log_message.timestamp, log_message.level, log_message.message
                        )
                        .unwrap();
                    }
                }
            }
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.file {
            let mut file = file.lock().unwrap();
            file.0.flush().unwrap();
        }
    }
}

pub fn initialize(level: LevelFilter, output: Output) {
    let config = Config::new(level, output);
    let logger = Logger::new(config);
    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("could not initialize logging system");

    log::info!("initialized logging system successfully")
}
