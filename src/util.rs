use crate::{bar::headerbar::HeaderFile, def::*, global::*, log::Log, model::*};
#[cfg(target_os = "linux")]
use anyhow::Context;
#[cfg(target_os = "linux")]
use std::io::Read;
use std::{
    fs, path,
    path::MAIN_SEPARATOR,
    process::{Command, Stdio},
    usize,
};
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
pub fn get_row_width(vec: &[char], offset_disp_x: usize, is_ctrlchar_incl: bool) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);

    for c in vec {
        if c == &EOF_MARK || c == &NEW_LINE_LF || c == &NEW_LINE_CR {
            if is_ctrlchar_incl && (c == &NEW_LINE_LF || c == &NEW_LINE_CR) {
                width += 1;
                cur_x += 1;
            }
            break;
        }

        let c_len = get_char_width(c, width + offset_disp_x);
        width += c_len;
        cur_x += 1;
    }
    return (cur_x, width);
}

/// Calculate disp_x and cursor_x by adding the widths up to x.
pub fn get_until_x(buf: &Vec<char>, x: usize) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for c in buf {
        if c == &NEW_LINE_LF || c == &EOF_MARK || c == &NEW_LINE_CR {
            break;
        }
        let c_len = get_char_width(c, width);
        if width + c_len > x {
            break;
        } else {
            width += c_len;
            cur_x += 1;
        }
    }
    return (cur_x, width);
}

// Everything including tab
pub fn get_char_width(c: &char, width: usize) -> usize {
    if c == &TAB_CHAR {
        let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.width;
        return get_char_width_exec(c, width, cfg_tab_width);
    } else {
        return c.width().unwrap_or(0);
    }
}
// Everything including tab
pub fn get_char_width_exec(c: &char, width: usize, cfg_tab_width: usize) -> usize {
    if c == &TAB_CHAR {
        return cfg_tab_width - width % cfg_tab_width;
    } else {
        return c.width().unwrap_or(0);
    }
}

pub fn get_char_width_not_tab(c: char) -> usize {
    if c == NEW_LINE_LF {
        return 1;
    }
    return c.width().unwrap_or(0);
}

#[cfg(target_os = "linux")]
pub fn get_env_platform() -> Env {
    let child = Command::new("uname").arg("-r").stdout(Stdio::piped()).spawn().unwrap();
    let mut stdout = child.stdout.context("take stdout").unwrap();
    let mut buf = String::new();
    stdout.read_to_string(&mut buf).unwrap();

    let env = if buf.to_ascii_lowercase().contains("microsoft") { Env::WSL } else { Env::Linux };
    return env;
}
#[cfg(target_os = "windows")]
pub fn get_env_platform() -> Env {
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

pub fn change_output_encoding() {
    let result = Command::new("powershell.exe").arg("chcp 65001").stdout(Stdio::null()).stdin(Stdio::null()).stderr(Stdio::null()).spawn();
    Log::debug("change_output_encoding result", &result);
}

pub fn is_line_end(c: char) -> bool {
    // LF, CR
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

pub fn cut_str(str: String, limit_width: usize, is_from_before: bool, is_add_continue_str: bool) -> String {
    let mut chars: Vec<char> = if is_from_before { str.chars().rev().collect() } else { str.chars().collect() };
    let mut width = 0;
    let limit_width = if is_add_continue_str { limit_width - get_str_width(CONTINUE_STR) } else { limit_width };
    for i in 0..chars.len() {
        if let Some(c) = chars.get(i) {
            let w = c.width().unwrap_or(0);
            if width + w > limit_width {
                let mut rtn_str: String = if is_from_before { chars.drain(0..i).rev().collect() } else { chars.drain(0..i).collect() };
                if is_add_continue_str {
                    if is_from_before {
                        rtn_str = format!("{}{}", &CONTINUE_STR, rtn_str);
                    } else {
                        rtn_str.push_str(&CONTINUE_STR);
                    }
                }
                return rtn_str;
            }
            width += w;
        }
    }
    return str;
}

pub fn split_inclusive(target: &str, split_char: char) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let mut string = String::new();

    let chars: Vec<char> = target.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c == split_char {
            if !string.is_empty() {
                vec.push(string.clone());
            }
            vec.push(split_char.to_string());
            string.clear();
        } else {
            string.push(*c);
        }
        if i == chars.len() - 1 && !string.is_empty() {
            vec.push(string.clone());
        }
    }
    return vec;
}
pub fn get_tab_comp_files(target_path: String, is_dir_only: bool, is_full_path_filenm: bool) -> Vec<File> {
    Log::debug_key("get_tab_comp_files");

    // Search target dir
    let mut base_dir = ".".to_string();
    let vec: Vec<(usize, &str)> = target_path.match_indices(path::MAIN_SEPARATOR).collect();
    // "/" exist
    if vec.len() > 0 {
        let (base, _) = target_path.split_at(vec[vec.len() - 1].0 + 1);
        base_dir = base.to_string();
    }

    let mut rtn_vec: Vec<File> = vec![];

    if let Ok(mut read_dir) = fs::read_dir(&base_dir) {
        while let Some(Ok(path)) = read_dir.next() {
            if !is_dir_only || (is_dir_only && path.path().is_dir()) {
                let mut filenm = path.path().display().to_string();
                let v: Vec<(usize, &str)> = filenm.match_indices(target_path.as_str()).collect();

                if v.len() > 0 {
                    // Replace "./" for display
                    if &base_dir == "." {
                        filenm = filenm.replace("./", "");
                    }
                    if !is_full_path_filenm {
                        filenm = path.path().file_name().unwrap().to_string_lossy().to_string();
                    }
                    let is_dir = if path.metadata().is_ok() { path.metadata().unwrap().is_dir() } else { true };
                    rtn_vec.push(File { name: filenm, is_dir: is_dir });
                }
            }
        }
    }
    rtn_vec.sort_by_key(|file| file.name.clone());
    return rtn_vec;
}
pub fn get_dir_path(path_str: &String) -> String {
    let mut vec = split_inclusive(path_str, MAIN_SEPARATOR);
    // Deleted when characters were entered

    if !vec.is_empty() && vec.last().unwrap() != &MAIN_SEPARATOR.to_string() {
        vec.pop();
    }
    return vec.join("");
}

pub fn change_nl(string: &mut String, h_file: &HeaderFile) {
    if h_file.nl == NEW_LINE_LF_STR {
        *string = string.replace(NEW_LINE_CRLF, &NEW_LINE_LF.to_string());
        // CRLF
    } else {
        //Since it is not possible to replace only LF from a character string containing CRLF,
        // convert it to LF and then convert it to CRLF.
        *string = string.replace(NEW_LINE_CRLF, &NEW_LINE_LF.to_string());
        *string = string.replace(&NEW_LINE_LF.to_string(), NEW_LINE_CRLF);
    }
}
