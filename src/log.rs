use crate::{_cfg::cfg::CfgLog, global::*};
use chrono::{DateTime, Local};
use std::{
    env,
    fmt::{self, Debug},
    fs,
    fs::{File, OpenOptions},
    io::Write,
    panic::Location,
    path,
};

impl Log {
    fn set_log_level(log_level: &str) -> LogLevel {
        match log_level {
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            _ => LogLevel::Info,
        }
    }
    pub fn set_logger(cfg_log: &Option<CfgLog>) {
        let dt: DateTime<Local> = Local::now();
        let mut path = env::temp_dir();
        path.push(env!("CARGO_PKG_NAME"));

        // Ignore the error
        let _ = fs::create_dir(&path);
        path.push(format!("{}_{}{}", env!("CARGO_PKG_NAME"), &dt.format("%Y_%m%d").to_string(), ".log"));

        if let Ok(file) = OpenOptions::new().create(true).append(true).open(path) {
            // let file = OpenOptions::new().create(true).append(true).open(path).unwrap();

            let log_level = match cfg_log {
                Some(cfg_log) => match &cfg_log.level {
                    Some(level) => level,
                    _ => "info",
                },
                _ => "info",
            };

            let log = Log { file, level: Log::set_log_level(&log_level) };

            let _ = LOG.set(log);
        } else {
            eprintln!("{}", &LANG.log_file_create_failed);
        }
    }

    #[track_caller]
    pub fn write(s: &str, log_level: LogLevel) {
        if let Some(log) = LOG.get() {
            if log.level > log_level {
                return;
            }

            let t = Local::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string();
            let caller = Location::caller().to_string();
            let vec: Vec<&str> = caller.split(path::MAIN_SEPARATOR).collect();
            let file_info = vec.last().unwrap();
            let s = if cfg!(debug_assertions) { format!("{} {}", log_level, s) } else { format!("{} {} {} :: {}", t, log_level, s, file_info) };

            writeln!(&log.file, "{}", s).unwrap();
        } else {
            eprintln!("{}", s);
        }
        if cfg!(debug_assertions) {
            eprintln!("{}", s);
        }
    }

    #[track_caller]
    pub fn info<T: Debug>(m: &str, v: &T) {
        let s = &format!("{}: {:?}", m, v);
        Log::write(s, LogLevel::Info);
    }
    #[track_caller]
    pub fn info_s(m: &str) {
        Log::write(m, LogLevel::Info);
    }
    #[track_caller]
    pub fn info_key(m: &str) {
        Log::write(&format!("     {}", m), LogLevel::Info);
    }
    #[track_caller]
    pub fn debug<T: Debug>(m: &str, v: &T) {
        let s = &format!("{}: {:?}", m, v);
        Log::write(s, LogLevel::Debug);
    }
    #[track_caller]
    pub fn debug_s(m: &str) {
        Log::write(m, LogLevel::Debug);
    }
    #[track_caller]
    pub fn debug_key(m: &str) {
        Log::write(&format!("     {}", m), LogLevel::Debug);
    }
    #[track_caller]
    pub fn warn<T: Debug>(m: &str, v: &T) {
        let s = &format!("{}: {:?}", m, v);
        Log::write(s, LogLevel::Warn);
    }
    #[track_caller]
    pub fn error<T: Debug>(m: &str, v: &T) {
        let s = &format!("{}: {:?}", m, v);
        Log::write(s, LogLevel::Error);
    }
    #[track_caller]
    pub fn error_s(m: &str) {
        Log::write(m, LogLevel::Error);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum LogLevel {
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO "),
            LogLevel::Warn => write!(f, "WARN "),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}
#[derive(Debug)]
pub struct Log {
    pub level: LogLevel,
    pub file: File,
}
