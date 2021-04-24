use chrono::Local;
use crossterm::style::ResetColor;
use std::{env, fmt::Debug, fs::OpenOptions, io::Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {}
impl Log {
    pub fn ep<T: Debug>(m: &str, v: &T) {
        if cfg!(debug_assertions) {
            eprintln!("{}{} {:?}", ResetColor, format!("{:?}", m), v);
        } else {
            // eprintln!("{}{} {:?}", ResetColor, format!("{:?}", m), v);
            /*
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
            }
            */
        }
    }

    pub fn ep_tmp<T: Debug>(m: &str, v: &T) {
        let mut path = env::temp_dir();
        path.push(format!("{}{}", env!("CARGO_PKG_NAME"), ".log"));
        let file = OpenOptions::new().create(true).append(true).open(path);

        if let Ok(mut log) = file {
            let t = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            writeln!(log, "{} {}: {:?}", t, m, v).unwrap();
        } else {
            panic!("{:?}", file);
        }
    }
    pub fn ep_s(m: &str) {
        if cfg!(debug_assertions) {
            eprintln!("{}{}", ResetColor, m);
        } else {
            // eprintln!("{}{}", ResetColor, m);

            /*
             let debug_mode: &str = ARGS.get("debug_mode").unwrap();
             if debug_mode == "true" {
            }
             */
        }
    }
}
