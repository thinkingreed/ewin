use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, StatusBar};
use crate::util::*;
use termion::color;
use unicode_width::UnicodeWidthChar;

impl StatusBar {
    pub fn new(lang_cfg: LangCfg) -> Self {
        StatusBar { lang: lang_cfg, ..StatusBar::default() }
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) {
        //let mut str_vec: Vec<String> = vec![];
        if self.disp_row_num == 0 {
            return;
        }

        let (filenm_w, cur_w) = self.get_areas_width(self.disp_col_num);

        let mut file_str = self.filenm.clone();
        if file_str.len() == 0 {
            file_str = self.filenm_tmp.clone();
        }

        let filenm = self.cut_str(file_str.clone(), filenm_w);

        let filenm_disp = format!("{fnm:^width$}", fnm = filenm, width = filenm_w - (get_str_width(&filenm) - filenm.chars().count()));

        // 文字横幅と文字数の差分で調整
        let cur_s = self.get_cur_str(editor);
        let cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));
        str_vec.push("\r\n".to_string());
        self.set_color(str_vec);
        str_vec.push(filenm_disp.clone());
        str_vec.push(cur_str.clone());
        editor.set_textarea_color(str_vec);
        self.filenm_disp = filenm_disp;
        self.cur_str = cur_str;
    }
    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) {
        let rows = self.disp_row_posi;

        // statusber表示領域がない場合
        if rows == 0 {
            return;
        }
        let cur_str = format!("{cur:>w$}", cur = self.get_cur_str(editor), w = self.cur_str.chars().count());
        let all_str = format!("{}{}{}", color::Fg(color::Rgb(221, 72, 20)).to_string(), self.filenm_disp, cur_str);
        let sber_str = format!("{}{}{}{}", termion::cursor::Goto(1, rows as u16), termion::clear::CurrentLine, all_str, color::Fg(color::White).to_string());

        str_vec.push(sber_str);
    }
    pub fn get_cur_str(&mut self, editor: &mut Editor) -> String {
        let mut row_vec: Vec<&str> = vec![];
        row_vec.push(&self.lang.row);
        row_vec.push("(");
        let row = (editor.cur.y + 1).to_string();
        row_vec.push(&row);
        row_vec.push("/");
        let rows = (editor.buf.len()).to_string();
        row_vec.push(&rows);
        row_vec.push(")");

        let mut col_vec: Vec<&str> = vec![];
        col_vec.push(&self.lang.col);
        col_vec.push("(");

        let (cols, col) = (editor.buf[editor.cur.y].len().to_string(), (editor.cur.x + 1 - editor.lnw).to_string());
        col_vec.push(&col);
        col_vec.push("/");
        col_vec.push(&cols);
        col_vec.push(")");

        let cur_posi = format!("{rows} {cols}", rows = row_vec.concat(), cols = col_vec.concat(),);
        return cur_posi;
    }

    fn get_areas_width(&self, cols: usize) -> (usize, usize) {
        let filenm_w_max = 16;
        let cur_w_max = 28;

        if cols < cur_w_max {
            return (0, cols);
        } else if cols < cur_w_max + filenm_w_max {
            return (cols - cur_w_max, cur_w_max);
        } else {
            let (area_w, rest) = (cols / 6, cols % 6);
            let (filenm_w, cur_w) = (area_w * 3, (area_w * 3) + rest);
            return (filenm_w, cur_w);
        }
    }
    fn cut_str(&mut self, string: String, limit_width: usize) -> String {
        let mut chars: Vec<char> = string.chars().collect();
        let mut width = 0;
        for i in 0..chars.len() {
            if let Some(c) = chars.get(i) {
                let w = c.width().unwrap_or(0);
                if width + w > limit_width {
                    return chars.drain(0..i).collect();
                }
                width += w;
            }
        }
        return string;
    }
}
