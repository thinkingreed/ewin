use crate::model::*;
use std::fmt::Display;

impl Log {
    pub fn ep<T: Display>(m: &str, v: T) {
        eprintln!("{}{} {}", Colors::get_default_fg(), format!("{:?}", m), v);
        if cfg!(debug_assertions) {
            eprintln!("{}{} {}", Colors::get_default_fg(), format!("{:?}", m), v);
        } else {

            /*
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
            }
            */
        }
    }
    pub fn ep_s(m: &str) {
        eprintln!("{}{}", Colors::get_default_fg(), m);
        if cfg!(debug_assertions) {
            eprintln!("{}{}", Colors::get_default_fg(), m);
        } else {

            /*
             let debug_mode: &str = ARGS.get("debug_mode").unwrap();
             if debug_mode == "true" {
            }
             */
        }
    }
}
