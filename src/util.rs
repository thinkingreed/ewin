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
pub fn get_row_width(vec: &Vec<char>, sx: usize, ex: usize) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for i in sx..ex {
        if let Some(c) = vec.get(i) {
            // Log::ep("ccccc", c);
            let c_len = c.width().unwrap_or(0);
            width += c_len;
            cur_x += 1;
        } else {
            // 最終端の空白対応
            if i == ex - 1 {
                width += 1;
            }
        }
    }
    // Log::ep("cur_x", cur_x);
    // Log::ep("width", width);

    return (cur_x, width);
}

/// updown_xまでのwidthを加算してdisp_xとcursorx算出
pub fn get_until_updown_x(buf: &Vec<char>, x: usize) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for i in 0..buf.len() + 1 {
        if let Some(c) = buf.get(i) {
            let mut c_len = c.width().unwrap_or(0);
            if width + c_len >= x {
                if c_len > 1 {
                    c_len = 1;
                }
                width += c_len;
                break;
            } else {
                width += c_len;
            }
            cur_x += 1;
        // 最終端の空白の場合
        } else {
            width += 1;
        }
    }
    return (cur_x, width);
}

/// xまでのwidthを加算してdisp_xとcursorx算出
pub fn get_until_x(buf: &Vec<char>, x: usize) -> (usize, usize) {
    let (mut cur_x, mut sum_width) = (0, 0);
    for i in 0..buf.len() + 1 {
        if let Some(c) = buf.get(i) {
            let width = c.width().unwrap_or(0);
            if sum_width + width > x {
                break;
            } else {
                sum_width += width;
                cur_x += 1;
            }
        }
    }
    return (cur_x, sum_width);
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

pub fn get_cur_x_width(buf: &Vec<char>, x: usize) -> usize {
    if let Some(c) = buf.get(x) {
        return c.width().unwrap_or(0);
    }

    return 1;
}

impl Log {
    pub fn ep<T: Display>(m: &str, v: T) {
        if cfg!(debug_assertions) {
            eprintln!("{} {}", format!("{:?}", m), v);
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
            eprintln!("{}", m);
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
