use crate::char_edit::*;
use ewin_cfg::log::*;
use ewin_const::{def::*, models::view::*};
use std::mem;
use unicode_width::UnicodeWidthStr;

pub fn adjust_str_len(str: &str, limit_width: usize, is_front: bool, is_add_continue_str: bool) -> String {
    let diff: isize = limit_width as isize - str.width() as isize;
    return if diff < 0 { cut_str(str, limit_width, is_front, is_add_continue_str) } else { format!("{}{}", str, get_space(diff as usize)) };
}

pub fn cut_str(str: &str, limit_width: usize, is_front: bool, is_add_continue_str: bool) -> String {
    let mut chars: Vec<char> = if is_front { str.chars().rev().collect() } else { str.chars().collect() };
    let mut width = 0;

    let str_width = get_str_width(str);
    if limit_width >= str_width {
        return str.to_string();
    } else {
        let limit_width = if is_add_continue_str { limit_width - get_str_width(CONTINUE_STR) } else { limit_width };
        for i in 0..chars.len() {
            if let Some(c) = chars.get(i) {
                let w = get_char_width_not_tab(c);
                if width + w > limit_width {
                    let mut rtn_str: String = if is_front { chars.drain(0..i).rev().collect() } else { chars.drain(0..i).collect() };
                    if is_add_continue_str {
                        if is_front {
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
    str.to_string()
}

pub fn get_str_width(msg: &str) -> usize {
    let msg_chars = msg.chars();
    let mut width = 0;
    for c in msg_chars {
        // Because the width varies depending on the environment
        width += get_char_width_not_tab(&c);
    }
    width
}

pub fn is_contain_row_end_str(s: &str) -> bool {
    for c in s.chars().rev() {
        if is_nl_char(c) {
            return true;
        }
    }
    return false;
}

pub fn split_tgt_str_width(target: &str, split_char: &[char], max_width: usize) -> Vec<String> {
    let mut rtn_vec = vec![];
    let vec = split_chars(target, true, true, split_char);

    let mut width = 0;
    let mut add_str = String::new();
    for (i, s) in vec.iter().enumerate() {
        let len = get_str_width(s);
        if width + len > max_width {
            rtn_vec.push(add_str.clone());
            add_str.clear();
            width = 0;
        }
        add_str.push_str(s);
        width += len;

        if i == vec.len() - 1 && !add_str.is_empty() {
            rtn_vec.push(add_str.clone());
        }
    }
    return rtn_vec;
}

pub fn get_strs_max_width(vec: &mut Vec<String>) -> usize {
    let mut max_width = 0;
    for s in vec {
        let len = get_str_width(s);
        max_width = if max_width >= len { max_width } else { len };
    }
    return max_width;
}

pub fn adjust_width_str(vec: &mut [String], max_width: usize) {
    Log::debug_key("adjust_width_str");
    for s in vec.iter_mut() {
        let len = get_str_width(s);
        if len != max_width {
            let _ = mem::replace(s, format!("{}{}", &s, get_space(max_width - len)));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_str_width() {
        assert_eq!(get_str_width("123"), 3);
        assert_eq!(get_str_width("あ亜ア"), 6);
        assert_eq!(get_str_width(""), 0);
    }

    #[test]
    fn test_cut_str() {
        assert_eq!(cut_str("abc", 0, false, false), "".to_string());
        assert_eq!(cut_str("abc", 4, false, false), "abc".to_string());
        assert_eq!(cut_str("abc", 2, false, false), "ab".to_string());
        assert_eq!(cut_str("abc", 2, true, false), "bc".to_string());
        assert_eq!(cut_str("abc", 3, false, true), "abc".to_string());
        assert_eq!(cut_str("aあbcd", 5, true, false), "あbcd".to_string());
        assert_eq!(cut_str("aあbcd", 5, true, true), "..bcd".to_string());
        assert_eq!(cut_str("aあbcd", 5, false, false), "aあbc".to_string());
        assert_eq!(cut_str("aあbcd", 5, false, true), "aあ..".to_string());
        assert_eq!(cut_str("aあbcd", 6, false, true), "aあbcd".to_string());
    }

    #[test]
    fn test_split_inclusive() {
        assert_eq!(split_chars("a,b:c", true, true, &[',', ':']), vec!["a".to_string(), ",".to_string(), "b".to_string(), ":".to_string(), "c".to_string(),]);
        assert_eq!(split_chars(",ab", true, true, &[',']), vec![",".to_string(), "ab".to_string()]);
        assert_eq!(split_chars("ab,", true, true, &[',']), vec!["ab".to_string(), ",".to_string()]);
    }
}
