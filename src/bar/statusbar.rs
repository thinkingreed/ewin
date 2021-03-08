use std::io::{stdout, BufWriter, Write};

use crate::{colors::*, global::*, log::*, model::*, util::*};
use crossterm::{cursor::*, terminal::*};
#[derive(Debug, Clone)]
pub struct StatusBar {
    pub cur_str: String,
    pub changed_str: String,

    // Position on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            cur_str: String::new(),
            changed_str: String::new(),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
    }
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar { ..StatusBar::default() }
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) {
        Log::ep_s("　　　　　　　　StatusBar.draw");

        if self.disp_row_num == 0 {
            return;
        }
        let cur_s = self.get_cur_str(editor);
        let (changed_w, cur_w) = self.get_areas_width(self.disp_col_num, &get_str_width(&cur_s) + 1);

        let is_changed = FILE.get().unwrap().try_lock().unwrap().is_changed;
        if is_changed {
            self.changed_str = LANG.changed.clone();
        }
        Log::ep("is_changed", &is_changed);
        Log::ep("self.changed_str", &self.changed_str);

        self.changed_str = format!("{changed:^w$}", changed = self.changed_str, w = changed_w - (get_str_width(&self.changed_str) - self.changed_str.chars().count()));
        // Adjusted by the difference between the character width and the number of characters
        self.cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));

        let sber_str = format!(
            "{}{}{}{}{}{}",
            MoveTo(0, (self.disp_row_posi) as u16),
            Clear(ClearType::CurrentLine),
            Colors::get_sber_bg(),
            Colors::get_sber_fg(),
            format!("{}{}", self.changed_str, self.cur_str),
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
        let col_str = format!("{}({}/{})", &LANG.col, editor.buf.len_line_chars(editor.cur.y).to_string(), editor.cur.x + 1 - editor.rnw).to_string();
        let cur_posi = format!("{rows} {cols}", rows = row_str, cols = col_str,);
        return cur_posi;
    }

    fn get_areas_width(&self, cols_w: usize, cur_str_w: usize) -> (usize, usize) {
        return (cols_w - cur_str_w, cur_str_w);
    }
}
