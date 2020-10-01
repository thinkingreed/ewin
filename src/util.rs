use crate::_cfg::args::ARGS;
use crate::model::Log;
use std::fmt::Display;
use unicode_width::UnicodeWidthChar;

pub fn get_str_width(msg: &str) -> usize {
    let msg_chars = msg.chars().collect::<Vec<char>>();
    let mut width = 0;
    for i in 0..msg_chars.len() {
        width += &msg_chars[i].width().unwrap_or(0);
    }
    return width;
}

impl Log {
    pub fn ep<T: Display>(m: &str, v: T) {
        if cfg!(debug_assertions) {
            eprintln!("{} {}", format!("{m:?}", m = m), v);
        } else {
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
                eprintln!("{} {}", format!("{m:?}", m = m), v);
            }
        }
    }
    pub fn ep_s(m: &str) {
        if cfg!(debug_assertions) {
            eprintln!("{}", m);
        }
    }
}
