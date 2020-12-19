use crate::model::*;
use std::fmt::Display;

impl Log {
    pub fn ep<T: Display>(m: &str, v: T) {
        if cfg!(debug_assertions) {
            eprintln!("{}{} {}", Colors::get_default_fg(), format!("{:?}", m), v);
        } else {
            // eprintln!("{} {}", format!("{:?}", m), v);

            /*
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
                eprintln!("{} {}", format!("{:?}", m), v);
            }
            */
        }
    }
    pub fn ep_s(m: &str) {
        if cfg!(debug_assertions) {
            eprintln!("{}{}", Colors::get_default_fg(), m);
        } else {
            // eprintln!("{}", m);

            /*
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
                eprintln!("{}", m);
            }
            */
        }
    }
}
