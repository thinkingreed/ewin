use std::io::{stdout, BufWriter, Write};

use crate::{colors::*, global::*, log::*, model::*, util::*};
use crossterm::{cursor::*, terminal::*};
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

impl StatusBar {
    const CUR_AREA_BASE_WITH: usize = 32;

    pub fn new() -> Self {
        StatusBar { ..StatusBar::default() }
    }
    pub fn draw(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) {
        Log::ep_s("　　　　　　　　StatusBar.draw");

        if self.disp_row_num == 0 {
            return;
        }
        let cur_s = self.get_cur_str(editor);
        let (other_w, cur_w) = self.get_areas_width(self.disp_col_num, &get_str_width(&cur_s) + 1);

        self.other_str = " ".repeat(other_w);
        // Adjusted by the difference between the character width and the number of characters
        self.cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));

        let sber_str = format!(
            "{}{}{}{}{}{}",
            MoveTo(0, (self.disp_row_posi) as u16),
            Clear(ClearType::CurrentLine),
            Colors::get_sber_bg(),
            Colors::get_sber_fg(),
            format!("{}{}", self.other_str, self.cur_str),
            Colors::get_default_fg(),
        );

        str_vec.push(sber_str);
        Colors::set_text_color(str_vec);

        let out = stdout();
        let mut out = BufWriter::new(out.lock());

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();

        str_vec.clear();
    }

    pub fn get_cur_str(&mut self, editor: &mut Editor) -> String {
        let row_str = format!("{}({}/{})", &LANG.row, (editor.cur.y + 1).to_string(), editor.buf.len_lines().to_string());
        let col_str = format!("{}({}/{})", &LANG.col, editor.cur.x + 1 - editor.rnw, (editor.buf.len_line_chars(editor.cur.y) - 1).to_string()).to_string();
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
