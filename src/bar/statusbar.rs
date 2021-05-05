use std::io::{stdout, BufWriter, Write};

use crate::{bar::headerbar::*, colors::*, global::*, log::*, tab::*, util::*};
use crossterm::{cursor::*, terminal::*};
impl StatusBar {
    const CUR_AREA_BASE_WITH: usize = 32;

    pub fn new() -> Self {
        StatusBar { ..StatusBar::default() }
    }
    pub fn draw(str_vec: &mut Vec<String>, h_file: &HeaderFile, tab: &mut Tab) {
        Log::info_s("　　　　　　　StatusBar.draw");

        if tab.sbar.disp_row_num == 0 {
            return;
        }
        let cur_s = StatusBar::get_cur_str(tab);

        let enc = h_file.enc;

        Log::debug("enc", &enc);

        let (other_w, cur_w) = tab.sbar.get_areas_width(tab.sbar.disp_col_num, &get_str_width(&cur_s) + 1);

        tab.sbar.other_str = " ".repeat(other_w);
        // Adjusted by the difference between the character width and the number of characters
        tab.sbar.cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));

        let sber_str = format!("{}{}{}{}{}", MoveTo(0, (tab.sbar.disp_row_posi) as u16), Clear(ClearType::CurrentLine), Colors::get_sbar_fg_bg(), format!("{}{}", tab.sbar.other_str, tab.sbar.cur_str), Colors::get_default_fg(),);

        str_vec.push(sber_str);
        Colors::set_text_color(str_vec);

        let out = stdout();
        let mut out = BufWriter::new(out.lock());

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();

        str_vec.clear();
    }

    pub fn get_cur_str(tab: &mut Tab) -> String {
        let cur = tab.editor.cur.clone();
        let len_lines = tab.editor.buf.len_lines();
        let len_line_chars = tab.editor.buf.len_line_chars(tab.editor.cur.y);

        let row_str = format!("{}({}/{})", &LANG.row, (cur.y + 1).to_string(), len_lines.to_string());
        let len_line_chars = if len_line_chars == 0 { 0 } else { len_line_chars - 1 };
        let col_str = format!("{}({}/{})", &LANG.col, cur.x + 1, len_line_chars.to_string()).to_string();
        let cur_posi = format!("{rows} {cols}", rows = row_str, cols = col_str,);
        return cur_posi;
    }

    fn get_areas_width(&self, cols_w: usize, cur_str_w: usize) -> (usize, usize) {
        if cur_str_w < StatusBar::CUR_AREA_BASE_WITH {
            return (cols_w - StatusBar::CUR_AREA_BASE_WITH, StatusBar::CUR_AREA_BASE_WITH);
        } else {
            return (cols_w - cur_str_w, cur_str_w);
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBar {
    pub cur_str: String,
    pub other_str: String,

    // Position on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            cur_str: String::new(),
            other_str: String::new(),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
    }
}
