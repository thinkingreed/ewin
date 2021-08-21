use crate::{bar::headerbar::*, def::*, global::*, log::Log, model::*};
#[cfg(target_os = "linux")]
use anyhow::Context;
#[cfg(target_os = "linux")]
use std::io::Read;
use std::{fs, path, path::*, process::*, usize};
use unicode_width::*;

pub fn get_str_width(msg: &str) -> usize {
    let msg_chars = msg.chars();
    let mut width = 0;
    for c in msg_chars {
        // Because the width varies depending on the environment
        width += get_char_width_not_tab(&c);
    }
    return width;
}

/// Get cur_x of any x. If there are no characters, return None.
pub fn get_row_x(char_arr: &[char], disp_x: usize, offset_disp_x: usize, is_ctrlchar_incl: bool) -> Option<usize> {
    let (mut cur_x, mut width) = (0, 0);

    for c in char_arr {
        if c == &EOF_MARK || c == &NEW_LINE_LF || c == &NEW_LINE_CR {
            if is_ctrlchar_incl && (c == &NEW_LINE_LF || c == &NEW_LINE_CR) {
                width += 1;
                cur_x += 1;
            } else {
                if width >= disp_x {
                    return Some(cur_x);
                } else {
                    break;
                }
            }
        }
        let c_len = get_char_width(c, width + offset_disp_x);
        if width + c_len > disp_x {
            return Some(cur_x);
        }
        width += c_len;
        cur_x += 1;
    }
    return None;
}

pub fn get_row_width(char_arr: &[char], offset_disp_x: usize, is_ctrlchar_incl: bool) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);

    for c in char_arr {
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
pub fn get_until_x(char_vec: &Vec<char>, x: usize) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for c in char_vec {
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
        let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.size;
        return get_char_width_tab(c, width, cfg_tab_width);
    } else {
        return get_char_width_not_tab(c);
    }
}
// Everything including tab
pub fn get_char_width_tab(c: &char, width: usize, cfg_tab_width: usize) -> usize {
    if c == &TAB_CHAR {
        return cfg_tab_width - width % cfg_tab_width;
    } else {
        return get_char_width_not_tab(c);
    }
}

pub fn get_char_width_not_tab(c: &char) -> usize {
    if c == &NEW_LINE_LF {
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

    if limit_width > get_str_width(&str) {
        return str;
    } else {
        for i in 0..chars.len() {
            if let Some(c) = chars.get(i) {
                let w = get_char_width_not_tab(c);
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

pub fn ordinal_suffix(number: usize) -> &'static str {
    match (number % 10, number % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    }
}

pub fn change_regex(replace_str: String) -> String {
    let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;

    Log::debug("replace_strreplace_strreplace_strreplace_str", &replace_str);
    Log::debug("replace_str == ", &(replace_str == "\\\""));

    if cfg_search.regex {
        let replace_str = replace_str.replace("\\n", &'\n'.to_string());
        let replace_str = replace_str.replace("\\t", &'\t'.to_string());
        let replace_str = replace_str.replace("\\r", &'\r'.to_string());
        let replace_str = replace_str.replace("\\", &r"\".to_string());
        let replace_str = replace_str.replace("\\'", &"\'".to_string());
        let replace_str = replace_str.replace("\\\"", &"\"".to_string());
        return replace_str;
    }
    return replace_str;
}
pub fn get_tab_str() -> String {
    return CFG.get().unwrap().try_lock().unwrap().general.editor.tab.tab.clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_str_width_1() {
        assert_eq!(get_str_width("123"), 3);
    }
    #[test]
    fn test_get_str_width_2() {
        assert_eq!(get_str_width("あ亜ア"), 6);
    }
    #[test]
    fn test_get_str_width_3() {
        assert_eq!(get_str_width(""), 0);
    }
    #[test]
    // Case where characters exist
    // get_row_x(char_arr, disp_x, offset_disp_x, is_ctrlchar_incl)
    fn test_get_row_x_1() {
        let arr: [char; 2] = ['a', EOF_MARK];
        assert_eq!(get_row_x(&arr, 0, 0, false), Some(0));
    }
    #[test]
    // Case where characters exist
    fn test_get_row_x_2() {
        let arr: [char; 3] = ['a', 'b', EOF_MARK];
        assert_eq!(get_row_x(&arr, 2, 0, false), Some(2));
    }
    #[test]
    // Case where characters exist offset_disp_x set
    fn test_get_row_x_3() {
        let arr: [char; 6] = ['a', 'b', 'c', 'あ', '亜', EOF_MARK];
        assert_eq!(get_row_x(&arr, 5, 3, false), Some(4));
    }
}
