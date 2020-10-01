use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, Log, StatusBar};
use crate::terminal::*;
use crate::util::*;
use std::io::{self};
use termion::color;
use unicode_width::UnicodeWidthChar;

impl StatusBar {
    // StatusBarを表示する行数
    pub const NUM_LINES_TO_DISP: usize = 1;
    // StatusBarを表示するterminal sizeの最低行数
    pub const MIN_NUM_LINES_TO_DISP: usize = 5;
    pub fn new(filenm: &str, lang_cfg: LangCfg) -> Self {
        StatusBar {
            lang: lang_cfg,
            filenm: String::from(filenm),
            ..StatusBar::default()
        }
    }
    pub fn draw(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) -> Result<(), io::Error> {
        str_vec.push(self.get_file_cur_str(editor));
        editor.set_cur_str(str_vec);
        return Ok(());
    }
    pub fn draw_statusber(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) {
        let (rows, _) = get_term_disp_size(TermDispType::StatusBar);

        // statusber表示領域がない場合
        if rows == 0 {
            return;
        }
        let cur_str = format!("{cur:>w$}", cur = self.get_cur_str(editor), w = self.cur_str.chars().count());
        let mut all_str = format!("{}{}{}", color::Fg(color::Rgb(221, 72, 20)).to_string(), self.filenm_str, cur_str);
        if self.msg_str.len() > 0 {
            all_str = format!("{}", self.get_msg_str());
        }

        let sber_str = format!("{}{}{}{}", termion::cursor::Goto(1, rows as u16), termion::clear::CurrentLine, all_str, color::Fg(color::White).to_string());

        str_vec.push(sber_str);
    }

    pub fn get_msg_str(&mut self) -> String {
        let (rows, cols) = get_term_disp_size(TermDispType::StatusBar);
        if rows == 0 {
            return "".to_string();
        }
        return format!("{msg:width$}", msg = self.msg_str, width = cols);
    }
    pub fn get_file_cur_str(&mut self, editor: &mut Editor) -> String {
        let mut str_vec: Vec<String> = vec![];
        let (rows, cols) = get_term_disp_size(TermDispType::StatusBar);
        if rows == 0 {
            return "".to_string();
        }

        let (filenm_w, cur_w) = self.get_areas_width(cols);
        let filenm = self.cut_str(self.filenm.clone(), filenm_w);

        let filenm_str = format!("{fnm:^width$}", fnm = filenm, width = filenm_w - (get_str_width(&filenm) - filenm.chars().count()));

        // 文字横幅と文字数の差分で調整
        let cur_s = self.get_cur_str(editor);
        let cur_str = format!("{cur:>w$}", cur = cur_s, w = cur_w - (get_str_width(&cur_s) - cur_s.chars().count()));
        str_vec.push("\r\n".to_string());
        self.set_color(&mut str_vec);
        str_vec.push(filenm_str.clone());
        str_vec.push(cur_str.clone());
        editor.set_textarea_color(&mut str_vec);
        self.filenm_str = filenm_str;
        self.cur_str = cur_str;

        return str_vec.concat();
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

        Log::ep("editor.cur.x", editor.cur.x);
        Log::ep("editor.lnw", editor.lnw);

        let (cols, col) = (editor.buf[editor.cur.y].len().to_string(), (editor.cur.x + 1 - editor.lnw).to_string());
        col_vec.push(&col);
        col_vec.push("/");
        col_vec.push(&cols);
        col_vec.push(")");

        let cur_posi = format!("{rows} {cols}", rows = row_vec.concat(), cols = col_vec.concat(),);
        return cur_posi;
    }

    pub fn get_save_confirm_str(&mut self) -> String {
        //
        //
        // Msg領域への作成
        //
        //
        let msg = format!(
            "{}{} {}{}:{}Y{} {}:{}N",
            &color::Fg(color::LightGreen).to_string(),
            self.lang.save_confirmation_to_close.clone(),
            &color::Fg(color::White).to_string(),
            self.lang.yes.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.no.clone(),
            &color::Fg(color::LightGreen).to_string(),
        );

        return msg;
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
