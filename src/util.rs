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

// 特定の文字列の先頭から指定バイト数となる文字数取得
pub fn get_char_count(vec: &Vec<char>, byte_nm: usize) -> usize {
    let (mut x, mut sum_width) = (0, 0);
    for c in vec {
        if byte_nm > sum_width {
            sum_width += c.to_string().len();
            x += 1;
        } else {
            break;
        }
    }
    return x;
}

impl Log {
    pub fn ep<T: Display>(m: &str, v: T) {
        if cfg!(debug_assertions) {
            eprintln!("{} {}", format!("{:?}", m), v);
        } else {
            eprintln!("{} {}", format!("{:?}", m), v);

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
            eprintln!("{}", m);
        } else {
            eprintln!("{}", m);
            /*
            let debug_mode: &str = ARGS.get("debug_mode").unwrap();
            if debug_mode == "true" {
                eprintln!("{}", m);
            }
            */
        }
    }
}
