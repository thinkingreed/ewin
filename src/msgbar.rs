use crate::_cfg::lang::cfg::LangCfg;
use crate::model::*;
use crate::util::*;
use std::io::Write;
use termion::{clear, cursor};

impl MsgBar {
    pub fn new(lang_cfg: LangCfg) -> Self {
        MsgBar { lang: lang_cfg, ..MsgBar::default() }
    }

    pub fn clear(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear");
        self.msg_disp = String::new();
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        // Log::ep_s("　　　　　　　　MsgBar.draw");
        if self.msg_disp.len() > 0 {
            let msg = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.msg_disp.clone());
            str_vec.push(msg);
        }
    }
    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        // Log::ep_s("　　　　　　　　MsgBar.draw");
        if self.msg_disp.len() > 0 {
            let msg = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.msg_disp.clone());
            write!(out, "{}", msg).unwrap();
            out.flush().unwrap();
        }
    }
    pub fn set_info(&mut self, msg: String) {
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        self.msg_disp = format!("{}{}", Colors::get_msg_fg(), msg_str,);
    }

    pub fn set_err(&mut self, msg: String) {
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        self.msg_disp = format!("{}{}", Colors::get_msg_err_fg(), msg_str,);
    }
}
