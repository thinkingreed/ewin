use crate::{def::*, model::*};
use anyhow::Context;
use std::io::Read;
use std::process;
use std::process::Command;
use unicode_width::UnicodeWidthChar;

pub fn get_str_width(msg: &str) -> usize {
    let msg_chars = msg.chars().collect::<Vec<char>>();
    let mut width = 0;
    for i in 0..msg_chars.len() {
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
pub fn get_env() -> Env {
    let child = Command::new("uname").arg("-r").stdout(process::Stdio::piped()).spawn().unwrap();
    let mut stdout = child.stdout.context("take stdout").unwrap();
    let mut buf = String::new();
    stdout.read_to_string(&mut buf).unwrap();

    if buf.to_ascii_lowercase().contains("microsoft") {
        return Env::WSL;
    } else {
        return Env::Linux;
    }
}

pub fn is_line_end(c: char) -> bool {
    ['\u{000a}', '\u{000d}'].contains(&c)
}

pub fn is_enable_syntax(ext: &str) -> bool {
    if ext.len() == 0 || ext == "txt" || ext == "log" || ext == "ini" {
        return false;
    } else {
        return true;
    }
}
