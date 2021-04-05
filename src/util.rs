use crate::{def::*, global::*, model::*};
use anyhow::Context;
use std::io::Read;
use std::process::{Command, Stdio};
use unicode_width::UnicodeWidthChar;

pub fn get_str_width(msg: &str) -> usize {
    let msg_chars = msg.chars().collect::<Vec<char>>();
    let mut width = 0;
    for i in 0..msg_chars.len() {
        // Because the width varies depending on the environment

        width += &msg_chars[i].width().unwrap_or(0);
    }
    return width;
}

pub fn get_row_width(vec: &[char], is_ctrlchar_incl: bool) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);

    for c in vec {
        // Log::ep("ccccc", c);
        if c == &EOF_MARK || c == &NEW_LINE || c == &NEW_LINE_CR {
            if is_ctrlchar_incl && (c == &NEW_LINE || c == &NEW_LINE_CR) {
                width += 1;
                cur_x += 1;
            }
            break;
        }
        let c_len = c.width().unwrap_or(0);
        width += c_len;
        cur_x += 1;
    }
    return (cur_x, width);
}

/// Calculate disp_x and cursor_x by adding the width up to updown_x.
pub fn get_until_updown_x(buf: &Vec<char>, x: usize) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for i in 0..buf.len() + 1 {
        if let Some(c) = buf.get(i) {
            if c == &EOF_MARK || c == &NEW_LINE || c == &NEW_LINE_CR {
                width += 1;
                break;
            }
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
        // White space at the end of Prompt
        } else {
            width += 1;
        }
    }
    return (cur_x, width);
}

/// Calculate disp_x and cursor_x by adding the widths up to x.
pub fn get_until_x(buf: &Vec<char>, x: usize) -> (usize, usize) {
    let (mut cur_x, mut sum_width) = (0, 0);
    for i in 0..=buf.len() {
        if let Some(c) = buf.get(i) {
            if c == &NEW_LINE || c == &EOF_MARK || c == &NEW_LINE_CR {
                break;
            }
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

// Get the number of characters from the beginning of the string to the specified width.
pub fn get_char_count(vec: &Vec<char>, width: usize) -> usize {
    let (mut x, mut sum_width) = (0, 0);
    for c in vec {
        if width > sum_width {
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
        if c == &NEW_LINE {
            return 1;
        }
        return c.width().unwrap_or(0);
    }
    return 1;
}

pub fn get_char_width(c: char) -> usize {
    if c == NEW_LINE {
        return 1;
    }
    return c.width().unwrap_or(0);
}

pub fn split_inclusive(target: &str, split_char: char) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let mut string = String::new();

    let chars: Vec<char> = target.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        string.push(*c);
        if *c == split_char {
            vec.push(string.clone());
            string.clear();
        }
        if i == chars.len() - 1 {
            vec.push(string.clone());
        }
    }
    return vec;
}
#[cfg(target_os = "linux")]
pub fn get_env() -> Env {
    let child = Command::new("uname").arg("-r").stdout(Stdio::piped()).spawn().unwrap();
    let mut stdout = child.stdout.context("take stdout").unwrap();
    let mut buf = String::new();
    stdout.read_to_string(&mut buf).unwrap();

    if buf.to_ascii_lowercase().contains("microsoft") {
        return Env::WSL;
    } else {
        return Env::Linux;
    }
}
#[cfg(target_os = "windows")]
pub fn get_env() -> Env {
    return Env::Windows;
}

pub fn is_powershell_enable() -> bool {
    let mut rtn = false;
    if *ENV == Env::WSL {
        let result = Command::new("powershell.exe").stdout(Stdio::null()).stdin(Stdio::null()).stderr(Stdio::null()).spawn();
        rtn = match result {
            Ok(_) => true,
            Err(_) => false,
        };
    }
    return rtn;
}

pub fn is_line_end(c: char) -> bool {
    ['\u{000a}', '\u{000d}'].contains(&c)
}

pub fn is_enable_syntax_highlight(ext: &str) -> bool {
    if ext.len() == 0 || ext == "txt" || ext == "log" {
        return false;
    } else {
        return true;
    }
}

pub fn get_char_type(c: char) -> CharType {
    if DELIM_STR.contains(c) {
        return CharType::Delim;
    } else if HALF_SPACE.contains(c) {
        return CharType::HalfSpace;
    } else if FULL_SPACE.contains(c) {
        return CharType::FullSpace;
    } else {
        return CharType::Nomal;
    }
}

pub fn cut_str(str: String, limit_width: usize, is_from_before: bool) -> String {
    let mut chars: Vec<char> = if is_from_before { str.chars().rev().collect() } else { str.chars().collect() };
    let mut width = 0;
    for i in 0..chars.len() {
        if let Some(c) = chars.get(i) {
            let w = c.width().unwrap_or(0);
            if width + w > limit_width {
                if is_from_before {
                    return chars.drain(0..i).rev().collect();
                } else {
                    return chars.drain(0..i).collect();
                }
            }
            width += w;
        }
    }
    return str;
}
