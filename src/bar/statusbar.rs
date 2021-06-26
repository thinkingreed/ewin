use crate::{bar::headerbar::*, colors::*, def::*, global::*, log::*, tab::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::io::{stdout, BufWriter, Write};

impl StatusBar {
    pub fn new() -> Self {
        StatusBar { ..StatusBar::default() }
    }
    pub fn draw(str_vec: &mut Vec<String>, h_file: &HeaderFile, tab: &mut Tab) {
        Log::info_key("StatusBar.draw");

        if tab.sbar.disp_row_num == 0 {
            return;
        }
        let cur_s = StatusBar::get_cur_str(tab);

        let enc_nl = format!("{}({})", h_file.enc, h_file.nl);
        let (other_w, cur_w) = tab.sbar.get_areas_width(tab.sbar.disp_col_num, &enc_nl, &get_str_width(&cur_s) + 1);
        tab.sbar.cur_area = (other_w + 1, other_w + cur_w - 1);
        tab.sbar.enc_nl_area = (other_w + cur_w, other_w + cur_w + enc_nl.len() - 1);

        tab.sbar.other_str = " ".repeat(other_w);
        // Adjusted by the difference between the character width and the number of characters
        tab.sbar.cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));

        let sbar_ctr = format!("{}{}{}{}{}", tab.sbar.other_str, tab.sbar.cur_str, Colors::get_sbar_inversion_fg_bg(), &enc_nl, Colors::get_sbar_fg_bg());
        let sber_all_str = format!("{}{}{}{}{}", MoveTo(0, tab.sbar.disp_row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_sbar_fg_bg(), sbar_ctr, Colors::get_default_fg_bg(),);

        str_vec.push(sber_all_str);
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

    fn get_areas_width(&self, cols_w: usize, enc_nl: &String, cur_str_w: usize) -> (usize, usize) {
        return (cols_w - enc_nl.len() - cur_str_w, cur_str_w);
    }
}

#[derive(Debug, Clone)]
pub struct StatusBar {
    pub cur_str: String,
    pub other_str: String,
    // Position on the terminal
    pub disp_row_num: usize,
    // 0 index
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    pub cur_area: (usize, usize),
    pub enc_nl_area: (usize, usize),
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            cur_str: String::new(),
            other_str: String::new(),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
            cur_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            enc_nl_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
        }
    }
}
