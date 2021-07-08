use crate::{bar::headerbar::*, colors::*, def::*, global::*, log::*, model::BoxInsertMode, sel_range::SelMode, tab::*, util::*};
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
        let (other_w, box_sel_mode_w, select_mode_w, cur_w) = StatusBar::get_areas_width(tab, &enc_nl, get_str_width(&cur_s));
        tab.sbar.select_mode_area = (other_w + 1, other_w + select_mode_w - 1);
        tab.sbar.cur_area = (other_w + select_mode_w, other_w + select_mode_w - 1);
        tab.sbar.enc_nl_area = (other_w + select_mode_w + cur_w, other_w + select_mode_w + cur_w + enc_nl.len() - 1);

        tab.sbar.other_str = " ".repeat(other_w);
        let select_mode_str = tab.editor.sel.mode.to_string();
        let select_mode_disp_str = format!("{select:>w$}", select = select_mode_str, w = select_mode_w - (get_str_width(&select_mode_str) - select_mode_str.chars().count()));

        let box_mode_str = tab.editor.box_sel.mode.to_string();
        let box_mode_disp_str = format!("{select:>w$}", select = box_mode_str, w = box_sel_mode_w - (get_str_width(&box_mode_str) - box_mode_str.chars().count()));

        // Adjusted by the difference between the character width and the number of characters
        tab.sbar.cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));

        let sbar_ctr = format!("{}{}{}{}{}{}{}{}{}{}{}", tab.sbar.other_str, Colors::get_sbar_inversion_fg_bg(), &box_mode_disp_str, Colors::get_sbar_fg_bg(), Colors::get_sbar_inversion_fg_bg(), &select_mode_disp_str, Colors::get_sbar_fg_bg(), tab.sbar.cur_str, Colors::get_sbar_inversion_fg_bg(), &enc_nl, Colors::get_sbar_fg_bg());
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
        let cur_posi = format!("{} {}", row_str, col_str,);
        return cur_posi;
    }

    fn get_areas_width(tab: &mut Tab, enc_nl: &String, cur_str_w: usize) -> (usize, usize, usize, usize) {
        let cols_w = tab.sbar.disp_col_num;

        let select_mode_w = match tab.editor.sel.mode {
            SelMode::Normal => 0,
            SelMode::BoxSelect => get_str_width(&LANG.box_select),
        };
        let box_sel_mode_w = match tab.editor.box_sel.mode {
            BoxInsertMode::Nomal => 0,
            BoxInsertMode::Insert => get_str_width(&LANG.box_insert),
        };

        return (cols_w - enc_nl.len() - box_sel_mode_w - select_mode_w - cur_str_w, box_sel_mode_w, select_mode_w, cur_str_w);
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
    pub select_mode_area: (usize, usize),
    pub cur_area: (usize, usize),
    pub enc_nl_area: (usize, usize),
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar { cur_str: String::new(), other_str: String::new(), disp_row_num: 1, disp_row_posi: 0, disp_col_num: 0, select_mode_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), cur_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), enc_nl_area: (USIZE_UNDEFINED, USIZE_UNDEFINED) }
    }
}
