use crate::model::*;
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

pub fn get_sel_range_str(buf: &mut Vec<Vec<char>>, sel: &mut SelRange) -> Vec<String> {
    let mut all_vec: Vec<String> = vec![];
    let copy_ranges: Vec<CopyRange> = get_copy_range(buf, sel);

    for copy_range in copy_ranges {
        let mut vec: Vec<String> = vec![];

        for j in copy_range.sx..copy_range.ex {
            if let Some(c) = buf[copy_range.y].get(j) {
                vec.insert(vec.len(), c.to_string());
            }
        }

        // 空行
        if copy_range.sx == 0 && copy_range.ex == 0 {
            vec.push("".to_string());
        }

        if vec.len() > 0 {
            all_vec.push(vec.join(""));
        }
    }
    return all_vec;
}

pub fn get_copy_range(buf: &mut Vec<Vec<char>>, sel: &mut SelRange) -> Vec<CopyRange> {
    let copy_posi = sel.get_range();

    let mut copy_ranges: Vec<CopyRange> = vec![];
    if copy_posi.sy == 0 && copy_posi.ey == 0 && copy_posi.ex == 0 {
        return copy_ranges;
    }

    Log::ep("copy_posi.sy", copy_posi.sy);
    Log::ep("copy_posi.ey", copy_posi.ey);
    Log::ep("copy_posi.sx", copy_posi.sx);
    Log::ep("copy_posi.ex", copy_posi.ex);

    for i in copy_posi.sy..=copy_posi.ey {
        /* if copy_posi.sy != copy_posi.ey && copy_posi.ex == 0 {
            continue;
        }*/
        Log::ep("iii", i);
        // 開始行==終了行
        if copy_posi.sy == copy_posi.ey {
            copy_ranges.push(CopyRange { y: i, sx: copy_posi.sx, ex: copy_posi.ex });
        // 開始行
        } else if i == copy_posi.sy {
            Log::ep("i == copy_posi.sy", i == copy_posi.sy);
            copy_ranges.push(CopyRange { y: i, sx: copy_posi.sx, ex: buf[i].len() });
        // 終了行
        } else if i == copy_posi.ey {
            // カーソルが行頭の対応
            copy_ranges.push(CopyRange { y: i, sx: 0, ex: copy_posi.ex });
        // 中間行 全て対象
        } else {
            copy_ranges.push(CopyRange { y: i, sx: 0, ex: buf[i].len() });
        }
    }

    return copy_ranges;
}
