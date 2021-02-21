use std::io::{stdout, BufWriter, Write};

use crate::{colors::*, def::*, global::*, log::*, model::*, util::*};
use crossterm::{cursor::*, terminal::*};
#[derive(Debug, Clone)]
pub struct StatusBar {
    pub filenm: String,
    pub filenm_disp: String,
    pub filenm_disp_flg: bool,
    pub cur_str: String,
    // Number displayed on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            filenm: String::new(),
            filenm_disp: String::new(),
            filenm_disp_flg: false,
            cur_str: String::new(),
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
        let (help_w, filenm_w, cur_w) = self.get_areas_width(self.disp_col_num, &get_str_width(&cur_s) + 1);

        let mut file_str = self.filenm.clone();
        if file_str.len() == 0 {
            file_str = LANG.new_file.clone();
        }

        let help = format!("{}:{}", KEY_HELP, LANG.help);
        let help_disp = format!("{h:^width$}", h = help, width = help_w);

        let filenm = cut_str(file_str.clone(), filenm_w, true);
        self.filenm_disp = format!("{fnm:^width$}", fnm = filenm, width = filenm_w - (get_str_width(&filenm) - filenm.chars().count()));

        // Adjusted by the difference between the character width and the number of characters
        self.cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));

        let sber_str = format!(
            "{}{}{}{}{}{}",
            MoveTo(0, (self.disp_row_posi - 1) as u16),
            Clear(ClearType::CurrentLine),
            Colors::get_sber_bg(),
            Colors::get_sber_fg(),
            format!("{}{}{}", help_disp, self.filenm_disp, self.cur_str),
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
    /*
    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) {
        Log::ep_s("               StatusBar.draw_cur");
        let rows = self.disp_row_posi;
        // no display area
        if rows == 0 {
            return;
        }
        let cur_str = format!("{cur:>w$}", cur = self.get_cur_str(editor), w = self.cur_str.chars().count());
        let all_str = format!("{}{}{}{}", Colors::get_sber_bg(), Colors::get_sber_fg(), self.filenm_disp, cur_str);
        let sber_str = format!("{}{}{}{}", MoveTo(0, (rows - 1) as u16), Clear(ClearType::CurrentLine), all_str, Colors::get_default_fg());

        str_vec.push(sber_str);
    }
    */
    pub fn get_cur_str(&mut self, editor: &mut Editor) -> String {
        let mut row_vec: Vec<&str> = vec![];
        row_vec.push(&LANG.row);
        row_vec.push("(");
        let row = (editor.cur.y + 1).to_string();
        row_vec.push(&row);
        row_vec.push("/");
        let rows = editor.buf.len_lines().to_string();
        row_vec.push(&rows);
        row_vec.push(")");

        let mut col_vec: Vec<&str> = vec![];
        col_vec.push(&LANG.col);
        col_vec.push("(");

        let (cols, col) = (editor.buf.len_line_chars(editor.cur.y).to_string(), (editor.cur.x + 1 - editor.rnw).to_string());
        col_vec.push(&col);
        col_vec.push("/");
        col_vec.push(&cols);
        col_vec.push(")");

        let cur_posi = format!("{rows} {cols}", rows = row_vec.concat(), cols = col_vec.concat(),);
        return cur_posi;
    }

    fn get_areas_width(&self, cols_w: usize, cur_str_w: usize) -> (usize, usize, usize) {
        // "f1:help "
        let help_w_max = 8;
        if cur_str_w > cols_w {
            return (0, 0, cols_w);
        } else if cur_str_w + help_w_max > cols_w {
            return (cols_w - cur_str_w, 0, cur_str_w);
        } else {
            return (help_w_max, cols_w - help_w_max - cur_str_w, cur_str_w);
        }
    }
}
