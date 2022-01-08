use crate::{def::*, file::*, global::*, log::Log, model::*};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use anyhow::Context;
use crossterm::terminal::size;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::io::Read;
use std::{self, fs, path::*, process::*, *};
use unicode_width::*;

pub fn get_str_width(msg: &str) -> usize {
    let msg_chars = msg.chars();
    let mut width = 0;
    for c in msg_chars {
        // Because the width varies depending on the environment
        width += get_char_width_not_tab(&c);
    }
    width
}

/// Get cur_x of any disp_x. If there are no characters, return None.
pub fn get_row_x(char_arr: &[char], disp_x: usize, is_ctrlchar_incl: bool, is_return_on_the_way: bool) -> Option<usize> {
    let (mut cur_x, mut width) = (0, 0);

    for c in char_arr {
        if c == &EOF_MARK || c == &NEW_LINE_LF || c == &NEW_LINE_CR {
            if is_ctrlchar_incl && (c == &NEW_LINE_LF || c == &NEW_LINE_CR) {
            } else if width >= disp_x {
                return Some(cur_x);
            } else {
                break;
            }
        }
        let c_len = get_char_width(c, width);
        if width + c_len > disp_x {
            return Some(cur_x);
        }
        width += c_len;
        cur_x += 1;
    }
    if is_return_on_the_way && cur_x > 0 {
        return Some(cur_x);
    }
    None
}

/// Get cur_x and disp_x of the target string
pub fn get_row_cur_x_disp_x(char_arr: &[char], offset_disp_x: usize, is_ctrlchar_incl: bool) -> (usize, usize) {
    let (mut cur_x, mut disp_x) = (0, 0);

    for c in char_arr {
        if c == &EOF_MARK || c == &NEW_LINE_LF || c == &NEW_LINE_CR {
            if is_ctrlchar_incl && (c == &NEW_LINE_LF || c == &NEW_LINE_CR) {
                disp_x += 1;
                cur_x += 1;
            }
            break;
        }

        let c_len = get_char_width(c, disp_x + offset_disp_x);
        disp_x += c_len;
        cur_x += 1;
    }
    (cur_x, disp_x)
}

/// Calculate disp_x and cursor_x by adding the widths up to x.
pub fn get_until_disp_x(char_vec: &[char], disp_x: usize) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for c in char_vec {
        if c == &NEW_LINE_LF || c == &EOF_MARK || c == &NEW_LINE_CR {
            break;
        }
        let c_len = get_char_width(c, width);
        if width + c_len > disp_x {
            break;
        } else {
            width += c_len;
            cur_x += 1;
        }
    }
    (cur_x, width)
}

/// Get x_offset from the specified cur_x
pub fn get_x_offset_by_cur_x(chars: &[char], col_len: usize) -> usize {
    let (mut cur_x, mut width) = (0, 0);

    if chars.len() < col_len {
        return 0;
    }
    for c in chars.iter().rev() {
        width += get_char_width(c, width);
        if width > col_len {
            break;
        }
        cur_x += 1;
    }
    chars.len() - cur_x
}

// Everything including tab
pub fn get_char_width(c: &char, width: usize) -> usize {
    return if c == &TAB_CHAR {
        let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.size;
        get_tab_width(width, cfg_tab_width)
    } else {
        get_char_width_not_tab(c)
    };
}
pub fn get_tab_width(width: usize, cfg_tab_width: usize) -> usize {
    return cfg_tab_width - width % cfg_tab_width;
}

pub fn get_char_width_not_tab(c: &char) -> usize {
    if c == &NEW_LINE_LF {
        return 1;
    }
    // Correspondence for each OS of characters whose judgment is wrong in unicode_width
    if let Some(width) = get_char_width_tgt_os(c) {
        return width;
    };
    return c.width().unwrap_or(0);
}

#[cfg(target_os = "windows")]
pub fn get_char_width_tgt_os(c: &char) -> Option<usize> {
    if [NEW_LINE_LF, '■', '●', '◆', '□', '○', '◇', '→', '←', '↑', '↓', '⇒', '⇔', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬', '⑭', '⑮', '⑯', '⑰', '⑱', '⑲', '⑳'].contains(c) {
        return Some(2);
    }
    return None;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_char_width_tgt_os(_c: &char) -> Option<usize> {
    return None;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_env_platform() -> Env {
    let child = Command::new("uname").arg("-r").stdout(Stdio::piped()).spawn().unwrap();
    let mut stdout = child.stdout.context("take stdout").unwrap();
    let mut buf = String::new();
    stdout.read_to_string(&mut buf).unwrap();

    if buf.to_ascii_lowercase().contains("microsoft") {
        Env::WSL
    } else {
        Env::Linux
    }
}

#[cfg(target_os = "windows")]
pub fn get_env_platform() -> Env {
    return Env::Windows;
}

pub fn is_wsl_powershell_enable() -> bool {
    let mut rtn = false;
    if *ENV == Env::WSL {
        let result = Command::new("powershell.exe").stdout(Stdio::null()).stdin(Stdio::null()).stderr(Stdio::null()).spawn();
        rtn = result.is_ok();
    }
    rtn
}

pub fn change_output_encoding() {
    let result = Command::new("powershell.exe").arg("chcp 65001").stdout(Stdio::null()).stdin(Stdio::null()).stderr(Stdio::null()).spawn();
    Log::debug("change output encoding chcp 65001 ", &result.is_ok());
}

pub fn is_row_end_char(c: char) -> bool {
    // LF, CR
    [NEW_LINE_LF, NEW_LINE_CR].contains(&c)
}

pub fn is_row_end_str(s: &str) -> bool {
    for c in s.chars() {
        if is_row_end_char(c) {
            return true;
        }
    }
    return false;
}
pub fn is_ctrl_char(c: char) -> bool {
    // LF, CR, EOF
    [NEW_LINE_LF, NEW_LINE_CR, EOF_MARK].contains(&c)
}

pub fn is_enable_syntax_highlight(ext: &str) -> bool {
    let disable_syntax_highlight_ext_vec = &CFG.get().unwrap().try_lock().unwrap().colors.theme.disable_syntax_highlight_ext;

    !(ext.is_empty() || disable_syntax_highlight_ext_vec.contains(&ext.to_string()))
}

pub fn get_char_type(c: char) -> CharType {
    if DELIM_STR.contains(c) {
        CharType::Delim
    } else if HALF_SPACE.contains(c) {
        CharType::HalfSpace
    } else if FULL_SPACE.contains(c) {
        CharType::FullSpace
    } else {
        CharType::Nomal
    }
}

pub fn cut_str(str: String, limit_width: usize, is_from_before: bool, is_add_continue_str: bool) -> String {
    let mut chars: Vec<char> = if is_from_before { str.chars().rev().collect() } else { str.chars().collect() };
    let mut width = 0;

    let str_width = get_str_width(&str);
    if limit_width >= str_width {
        return str;
    } else {
        let limit_width = if is_add_continue_str { limit_width - get_str_width(CONTINUE_STR) } else { limit_width };
        for i in 0..chars.len() {
            if let Some(c) = chars.get(i) {
                let w = get_char_width_not_tab(c);
                if width + w > limit_width {
                    let mut rtn_str: String = if is_from_before { chars.drain(0..i).rev().collect() } else { chars.drain(0..i).collect() };
                    if is_add_continue_str {
                        if is_from_before {
                            rtn_str = format!("{}{}", &CONTINUE_STR, rtn_str);
                        } else {
                            rtn_str.push_str(CONTINUE_STR);
                        }
                    }
                    return rtn_str;
                }
                width += w;
            }
        }
    }
    str
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
    vec
}
pub fn get_tab_comp_files(target_path: String, is_dir_only: bool, is_full_path_filenm: bool) -> Vec<File> {
    Log::debug_key("get_tab_comp_files");

    // Search target dir
    let mut base_dir = ".".to_string();
    let vec: Vec<(usize, &str)> = target_path.match_indices(path::MAIN_SEPARATOR).collect();
    // "/" exist
    if !vec.is_empty() {
        let (base, _) = target_path.split_at(vec[vec.len() - 1].0 + 1);
        base_dir = base.to_string();
    }

    let mut rtn_vec: Vec<File> = vec![];

    if let Ok(mut read_dir) = fs::read_dir(&base_dir) {
        while let Some(Ok(path)) = read_dir.next() {
            if !is_dir_only || path.path().is_dir() {
                let mut filenm = path.path().display().to_string();

                if filenm.match_indices(target_path.as_str()).next().is_some() {
                    // Replace "./" for display
                    if &base_dir == "." {
                        filenm = filenm.replace("./", "");
                    }
                    if !is_full_path_filenm {
                        filenm = path.path().file_name().unwrap().to_string_lossy().to_string();
                    }
                    let is_dir = if path.metadata().is_ok() { path.metadata().unwrap().is_dir() } else { true };
                    rtn_vec.push(File { name: filenm, is_dir });
                }
            }
        }
    }
    rtn_vec.sort_by_key(|file| file.name.clone());
    rtn_vec
}
pub fn get_dir_path(path_str: &str) -> String {
    let mut vec = split_inclusive(path_str, MAIN_SEPARATOR);
    // Deleted when characters were entered

    if !vec.is_empty() && vec.last().unwrap() != &MAIN_SEPARATOR.to_string() {
        vec.pop();
    }
    vec.join("")
}

pub fn change_nl(string: &mut String, to_nl: &str) {
    // Since it is not possible to replace only LF from a character string containing CRLF,
    // convert it to LF and then convert it to CRLF.
    *string = string.replace(NEW_LINE_CRLF, &NEW_LINE_LF.to_string());
    if to_nl == NEW_LINE_CRLF_STR {
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

pub fn change_regex(string: String) -> String {
    let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;

    if cfg_search.regex {
        let string = string.replace("\\n", &'\n'.to_string());
        let string = string.replace("\\t", &'\t'.to_string());
        let string = string.replace("\\r", &'\r'.to_string());
        let string = string.replace("\\", &r"\".to_string());
        let string = string.replace("\\'", &"\'".to_string());
        let string = string.replace("\\\"", &"\"".to_string());
        return string;
    }
    string
}
pub fn get_tab_str() -> String {
    CFG.get().unwrap().try_lock().unwrap().general.editor.tab.tab.clone()
}

pub fn is_include_path(src: &str, dst: &str) -> bool {
    let src_vec: Vec<&str> = src.split(MAIN_SEPARATOR).collect();
    let dst_vec: Vec<&str> = dst.split(MAIN_SEPARATOR).collect();

    let mut is_include = false;
    for (i, src) in src_vec.iter().enumerate() {
        if let Some(dst) = dst_vec.get(i) {
            is_include = src == dst;
        } else {
            is_include = false;
        }
    }
    is_include
}

pub fn get_term_size() -> (u16, u16) {
    Log::debug("get_term_size", &size());

    let (columns, rows) = size().unwrap_or((TERM_MINIMUM_WIDTH as u16, TERM_MINIMUM_HEIGHT as u16));

    // (1, 1) is judged as test
    if (columns, rows) == (1, 1) {
        (TERM_MINIMUM_WIDTH as u16, TERM_MINIMUM_HEIGHT as u16)
    } else {
        (columns, rows)
    }
}
pub fn get_delim_x(row: &[char], x: usize) -> (usize, usize) {
    let mut forward = row[..x + 1].to_vec();
    forward.reverse();
    let sx = get_delim(&forward, x, true);
    let backward = row[x..].to_vec();
    let ex = get_delim(&backward, x, false);
    (sx, ex)
}

fn get_delim(target: &[char], x: usize, is_forward: bool) -> usize {
    let mut rtn_x = 0;

    let mut char_type_org = CharType::Nomal;
    for (i, c) in (0_usize..).zip(target) {
        let char_type = get_char_type(*c);
        if i == 0 {
            char_type_org = char_type;
        }
        if char_type != char_type_org {
            rtn_x = if is_forward { x - i + 1 } else { x + i };
            break;
        } else if i == target.len() - 1 {
            rtn_x = if is_forward { x - i } else { x + i + 1 };
        }
        char_type_org = char_type;
    }
    rtn_x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{_cfg::cfg::Cfg, model::Args};

    #[test]
    fn test_get_str_width() {
        assert_eq!(get_str_width("123"), 3);
        assert_eq!(get_str_width("あ亜ア"), 6);
        assert_eq!(get_str_width(""), 0);
    }
    #[test]
    fn test_get_row_x() {
        Cfg::init(&Args { ..Args::default() }, include_str!("../../../setting.toml"));

        assert_eq!(get_row_x(&['a'], 0, false, false,), Some(0));
        assert_eq!(get_row_x(&['a', 'b'], 1, false, false,), Some(1));
        assert_eq!(get_row_x(&['a', 'あ', NEW_LINE_LF], 3, false, false,), Some(2));
        assert_eq!(get_row_x(&[TAB_CHAR, 'a'], 5, false, false,), None);
        assert_eq!(get_row_x(&[TAB_CHAR, 'a', NEW_LINE_LF], 5, true, false,), Some(2));
        assert_eq!(get_row_x(&[' ', TAB_CHAR, 'a'], 2, false, false,), Some(1));
        assert_eq!(get_row_x(&['a', NEW_LINE_LF, EOF_MARK], 2, false, false,), None);
        assert_eq!(get_row_x(&['a', NEW_LINE_LF, EOF_MARK], 2, true, false,), Some(2));
        assert_eq!(get_row_x(&['a', EOF_MARK], 2, true, false,), None);
    }

    #[test]
    fn test_get_row_width() {
        Cfg::init(&Args { ..Args::default() }, include_str!("../../../setting.toml"));

        assert_eq!(get_row_cur_x_disp_x(&['a'], 0, false), (1, 1));
        assert_eq!(get_row_cur_x_disp_x(&['あ'], 0, false), (1, 2));
        assert_eq!(get_row_cur_x_disp_x(&['a', NEW_LINE_LF], 0, false), (1, 1));
        assert_eq!(get_row_cur_x_disp_x(&['a', NEW_LINE_LF], 0, true), (2, 2));
        assert_eq!(get_row_cur_x_disp_x(&[TAB_CHAR, 'a'], 0, false), (2, 5));
        assert_eq!(get_row_cur_x_disp_x(&[TAB_CHAR, 'a'], 1, false), (2, 4));
        assert_eq!(get_row_cur_x_disp_x(&['a', NEW_LINE_LF], 0, false), (1, 1));
        assert_eq!(get_row_cur_x_disp_x(&['a', NEW_LINE_LF], 0, true), (2, 2));
        assert_eq!(get_row_cur_x_disp_x(&['a', NEW_LINE_CR], 0, false), (1, 1));
        assert_eq!(get_row_cur_x_disp_x(&['a', NEW_LINE_CR], 0, true), (2, 2));
        assert_eq!(get_row_cur_x_disp_x(&['a', EOF_MARK], 0, false), (1, 1));
        assert_eq!(get_row_cur_x_disp_x(&['a', EOF_MARK], 0, true), (1, 1));
    }

    #[test]
    fn test_get_until_x() {
        Cfg::init(&Args { ..Args::default() }, include_str!("../../../setting.toml"));
        assert_eq!(get_until_disp_x(&['a'], 0), (0, 0));
        assert_eq!(get_until_disp_x(&['a', 'あ',], 2), (1, 1));
        assert_eq!(get_until_disp_x(&['a', 'あ',], 2), (1, 1));
        assert_eq!(get_until_disp_x(&['a', 'あ',], 3), (2, 3));
        assert_eq!(get_until_disp_x(&['a', TAB_CHAR,], 3), (1, 1));
        assert_eq!(get_until_disp_x(&['a', TAB_CHAR,], 4), (2, 4));
        assert_eq!(get_until_disp_x(&['a', 'あ', NEW_LINE_LF], 4), (2, 3));
        assert_eq!(get_until_disp_x(&['a', 'あ', EOF_MARK], 4), (2, 3));
    }
    #[test]
    fn test_get_char_width() {
        Cfg::init(&Args { ..Args::default() }, include_str!("../../../setting.toml"));
        assert_eq!(get_char_width(&'a', 0), 1);
        assert_eq!(get_char_width(&'あ', 0), 2);
        assert_eq!(get_char_width(&TAB_CHAR, 1), 3);
        assert_eq!(get_char_width(&TAB_CHAR, 2), 2);
        assert_eq!(get_char_width(&TAB_CHAR, 0), 4);
        assert_eq!(get_char_width(&NEW_LINE_LF, 0), 1);
        assert_eq!(get_char_width(&NEW_LINE_CR, 0), 0);
        assert_eq!(get_char_width(&EOF_MARK, 0), 1);
    }

    #[test]
    fn test_get_env_platform() {
        match *ENV {
            Env::WSL => assert_eq!(get_env_platform(), Env::WSL),
            Env::Linux => assert_eq!(get_env_platform(), Env::Linux),
            Env::Windows => assert_eq!(get_env_platform(), Env::Windows),
        };
    }

    #[test]
    fn test_is_enable_syntax_highlight() {
        Cfg::init(&Args { ..Args::default() }, include_str!("../../../setting.toml"));

        assert!(!is_enable_syntax_highlight("txt"));
        assert!(is_enable_syntax_highlight("rs"));
        assert!(!is_enable_syntax_highlight(""));
    }

    #[test]
    fn test_get_char_type() {
        assert_eq!(get_char_type('"'), CharType::Delim);
        assert_eq!(get_char_type('!'), CharType::Delim);
        assert_eq!(get_char_type(' '), CharType::HalfSpace);
        assert_eq!(get_char_type('　'), CharType::FullSpace);
        assert_eq!(get_char_type('a'), CharType::Nomal);
        assert_eq!(get_char_type('z'), CharType::Nomal);
    }
    #[test]
    fn test_cut_str() {
        assert_eq!(cut_str("abc".to_string(), 0, false, false), "".to_string());
        assert_eq!(cut_str("abc".to_string(), 4, false, false), "abc".to_string());
        assert_eq!(cut_str("abc".to_string(), 2, false, false), "ab".to_string());
        assert_eq!(cut_str("abc".to_string(), 2, true, false), "bc".to_string());
        assert_eq!(cut_str("abc".to_string(), 3, false, true), "abc".to_string());
        assert_eq!(cut_str("aあbcd".to_string(), 5, true, false), "あbcd".to_string());
        assert_eq!(cut_str("aあbcd".to_string(), 5, true, true), "..bcd".to_string());
        assert_eq!(cut_str("aあbcd".to_string(), 5, false, false), "aあbc".to_string());
        assert_eq!(cut_str("aあbcd".to_string(), 5, false, true), "aあ..".to_string());
        assert_eq!(cut_str("aあbcd".to_string(), 6, false, true), "aあbcd".to_string());
    }
    #[test]
    fn test_split_inclusive() {
        assert_eq!(split_inclusive("a,b", ','), vec!["a".to_string(), ",".to_string(), "b".to_string()]);
        assert_eq!(split_inclusive(",ab", ','), vec![",".to_string(), "ab".to_string()]);
        assert_eq!(split_inclusive("ab,", ','), vec!["ab".to_string(), ",".to_string()]);
    }
    #[test]
    fn test_get_dir_path() {
        assert_eq!(get_dir_path(&"/home/".to_string()), "/home/".to_string());
        assert_eq!(get_dir_path(&"/home/ewin".to_string()), "/home/".to_string());
        assert_eq!(get_dir_path(&"".to_string()), "".to_string());
    }
    #[test]
    fn test_is_include_path() {
        assert!(is_include_path("/home", "/home/ewin"));
        assert!(!is_include_path("/hoge", "/home/ewin"));
    }

    #[test]
    fn test_get_delim() {
        //  let vec: Vec<char> = ;

        assert_eq!(get_delim_x(&"<12345>".chars().collect::<Vec<char>>(), 0), (0, 1));
        assert_eq!(get_delim_x(&"<12345>".chars().collect::<Vec<char>>(), 1), (1, 6));
        assert_eq!(get_delim_x(&"<12345>".chars().collect::<Vec<char>>(), 6), (6, 7));
        assert_eq!(get_delim_x(&"  12345>".chars().collect::<Vec<char>>(), 0), (0, 2));
        assert_eq!(get_delim_x(&"  　　12345>".chars().collect::<Vec<char>>(), 1), (0, 2));
        assert_eq!(get_delim_x(&"<12345>>".chars().collect::<Vec<char>>(), 6), (6, 8));
        assert_eq!(get_delim_x(&"<12345  ".chars().collect::<Vec<char>>(), 6), (6, 8));
        assert_eq!(get_delim_x(&"<12345　　".chars().collect::<Vec<char>>(), 6), (6, 8));
        assert_eq!(get_delim_x(&"<12345".chars().collect::<Vec<char>>(), 1), (1, 6));
        assert_eq!(get_delim_x(&"<12<<345>".chars().collect::<Vec<char>>(), 4), (3, 5));
        assert_eq!(get_delim_x(&"<12  345>".chars().collect::<Vec<char>>(), 4), (3, 5));
        assert_eq!(get_delim_x(&"<12　　345>".chars().collect::<Vec<char>>(), 4), (3, 5));
    }
}
