use crate::model::*;
use crossterm::style::ResetColor;
use std::fmt::Debug;

impl Log {
    pub fn ep<T: Debug>(m: &str, v: &T) {
        if cfg!(debug_assertions) {
            eprintln!("{}{} {:?}", ResetColor, format!("{:?}", m), v);
        } else {

            /*
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
            }
            */
        }
    }
    pub fn ep_s(m: &str) {
        if cfg!(debug_assertions) {
            eprintln!("{}{}", ResetColor, m);
        } else {

            /*
             let debug_mode: &str = ARGS.get("debug_mode").unwrap();
             if debug_mode == "true" {
            }
             */
        }
    }
}
