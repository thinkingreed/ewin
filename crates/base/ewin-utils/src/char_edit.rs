use ewin_cfg::{log::*, model::default::*};
use ewin_const::{def::*, models::model::CharType};
use unicode_width::UnicodeWidthChar;

/// Get cur_x of any disp_x. If there are no characters, return None.
pub fn get_row_x_opt(char_arr: &[char], disp_x: usize, is_ctrlchar_incl: bool, is_return_on_the_way: bool) -> Option<usize> {
    Log::debug_key("get_row_x_opt");
    let (mut cur_x, mut width) = (0, 0);

    for c in char_arr {
        if c == &NEW_LINE_LF || c == &NEW_LINE_CR {
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
        if c == &NEW_LINE_LF || c == &NEW_LINE_CR {
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
pub fn get_until_disp_x(char_vec: &[char], disp_x: usize, is_ctrlchar_incl: bool) -> (usize, usize) {
    let (mut cur_x, mut width) = (0, 0);
    for c in char_vec {
        if (c == &NEW_LINE_LF || c == &NEW_LINE_CR) && !is_ctrlchar_incl {
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
pub fn get_x_offset(chars: &[char], col_len: usize) -> usize {
    Log::debug_key("get_x_offset");
    let (mut cur_x, mut width) = (0, 0);

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
        let cfg_tab_width = Cfg::get().general.editor.tab.size;
        get_tab_width(width, cfg_tab_width)
    } else {
        get_char_width_not_tab(c)
    };
}

pub fn get_tab_width(width: usize, cfg_tab_width: usize) -> usize {
    return cfg_tab_width - width % cfg_tab_width;
}

pub fn get_c_width(c: &char) -> usize {
    let width = if let Some(ambiguous_width) = Cfg::get().general.font.ambiguous_width {
        if ambiguous_width == 2 {
            c.width_cjk().unwrap_or(0)
        } else {
            // ambiguous_width == 1
            c.width().unwrap_or(0)
        }
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        c.width().unwrap_or(0)
    } else {
        // Windows
        c.width_cjk().unwrap_or(0)
    };
    return width;
}

pub fn get_char_width_not_tab(c: &char) -> usize {
    if c == &NEW_LINE_LF {
        return 1;
    }
    return get_c_width(c);
}

pub fn is_nl_char(c: char) -> bool {
    // LF, CR
    [NEW_LINE_LF, NEW_LINE_CR].contains(&c)
}

pub fn get_char_type(c: char) -> CharType {
    if Cfg::get().general.editor.word.word_delimiter.contains(c) {
        CharType::Delim
    } else if HALF_SPACE.contains(c) {
        CharType::HalfSpace
    } else if FULL_SPACE == c {
        CharType::FullSpace
    } else if is_nl_char(c) {
        CharType::NewLineCode
    } else {
        CharType::Nomal
    }
}

pub fn split_chars(target: &str, is_inclusive: bool, is_new_line_code_ignore: bool, split_char: &[char]) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let mut string = String::new();

    let chars: Vec<char> = target.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if split_char.contains(c) {
            if !string.is_empty() {
                vec.push(string.clone());
            }
            if is_inclusive {
                vec.push(c.to_string());
            }
            string.clear();
        } else if !(is_new_line_code_ignore && [NEW_LINE_LF, NEW_LINE_CR].contains(c)) {
            string.push(*c);
        }
        if i == chars.len() - 1 && !string.is_empty() {
            vec.push(string.clone());
        }
    }
    return vec;
}

pub fn get_delim_x(row: &[char], x: usize) -> (usize, usize) {
    Log::debug_key("get_delim_x");

    let (mut sx, mut ex) = (0, 0);
    if row.len() > x {
        let mut forward = row[..=x].to_vec();
        forward.reverse();
        sx = get_delim(&forward, x, true);
        let backward = row[x..].to_vec();
        ex = get_delim(&backward, x, false);
    }
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
            rtn_x = if is_forward {
                x - i
            } else {
                // +1 is last row
                x + i + 1
            };
            break;
        }
        char_type_org = char_type;
    }
    rtn_x
}

pub fn get_until_delim_sx(rows: &[char]) -> usize {
    let mut rows = rows.to_vec();
    rows.reverse();
    let row_len = rows.len();
    for (i, c) in (0_usize..).zip(rows) {
        if Cfg::get().general.editor.input_comple.word_delimiter.contains(c) {
            return row_len - i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use ewin_cfg::model::{default::Cfg, modal::AppArgs};
    use ewin_const::def::*;

    use super::*;

    #[test]
    fn test_get_row_x() {
        Cfg::init(&AppArgs { ..AppArgs::default() });

        assert_eq!(get_row_x_opt(&['a'], 0, false, false,), Some(0));
        assert_eq!(get_row_x_opt(&['a', 'b'], 1, false, false,), Some(1));
        assert_eq!(get_row_x_opt(&['a', 'あ', NEW_LINE_LF], 3, false, false,), Some(2));
        assert_eq!(get_row_x_opt(&[TAB_CHAR, 'a'], 5, false, false,), None);
        assert_eq!(get_row_x_opt(&[TAB_CHAR, 'a', NEW_LINE_LF], 5, true, false,), Some(2));
        assert_eq!(get_row_x_opt(&[' ', TAB_CHAR, 'a'], 2, false, false,), Some(1));
        assert_eq!(get_row_x_opt(&['a', NEW_LINE_LF, 'b'], 2, false, false,), None);
        assert_eq!(get_row_x_opt(&['a', NEW_LINE_LF, 'b'], 2, true, false,), Some(2));
        assert_eq!(get_row_x_opt(&['a', 'b'], 2, true, false,), None);
    }

    #[test]
    fn test_get_row_width() {
        Cfg::init(&AppArgs { ..AppArgs::default() });

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
    }

    #[test]
    fn test_get_until_x() {
        Cfg::init(&AppArgs { ..AppArgs::default() });
        assert_eq!(get_until_disp_x(&['a'], 0, false), (0, 0));
        assert_eq!(get_until_disp_x(&['a', 'あ',], 2, false), (1, 1));
        assert_eq!(get_until_disp_x(&['a', 'あ',], 2, false), (1, 1));
        assert_eq!(get_until_disp_x(&['a', 'あ',], 3, false), (2, 3));
        assert_eq!(get_until_disp_x(&['a', TAB_CHAR,], 3, false), (1, 1));
        assert_eq!(get_until_disp_x(&['a', TAB_CHAR,], 4, false), (2, 4));
        assert_eq!(get_until_disp_x(&['a', 'あ', NEW_LINE_LF], 4, false), (2, 3));
    }
    #[test]
    fn test_get_char_width() {
        Cfg::init(&AppArgs { ..AppArgs::default() });
        assert_eq!(get_char_width(&'a', 0), 1);
        assert_eq!(get_char_width(&'あ', 0), 2);
        assert_eq!(get_char_width(&TAB_CHAR, 1), 3);
        assert_eq!(get_char_width(&TAB_CHAR, 2), 2);
        assert_eq!(get_char_width(&TAB_CHAR, 0), 4);
        assert_eq!(get_char_width(&NEW_LINE_LF, 0), 1);
        assert_eq!(get_char_width(&NEW_LINE_CR, 0), 0);
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
    #[test]
    fn test_get_until_delim_sx() {
        assert_eq!(get_until_delim_sx(&"123".chars().collect::<Vec<char>>()), 0);
        assert_eq!(get_until_delim_sx(&"1:23".chars().collect::<Vec<char>>()), 2);
        assert_eq!(get_until_delim_sx(&"123:".chars().collect::<Vec<char>>()), 4);
    }
}
