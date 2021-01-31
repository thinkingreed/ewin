use crate::{model::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;

impl MsgBar {
    pub fn new() -> Self {
        MsgBar { ..MsgBar::default() }
    }

    pub fn clear_mag(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear_mag");
        self.msg = String::new();
    }

    pub fn clear(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear");
        self.msg = String::new();
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
    }

    pub fn clear_macro(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear_macro");
        self.msg_keyrecord = String::new();
        self.disp_keyrecord_row_posi = 0;
        self.disp_keyrecord_row_num = 0;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        // Log::ep_s("　　　　　　　　MsgBar.draw");
        if self.msg_readonly.len() > 0 {
            str_vec.push(self.get_disp_readonly_msg());
        }
        if self.msg_keyrecord.len() > 0 {
            str_vec.push(self.get_disp_macro_msg());
        }
        if self.msg.len() > 0 {
            str_vec.push(self.get_disp_msg());
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T, editor: &mut Core, prom: &mut Prompt, sbar: &mut StatusBar) {
        // Log::ep_s("　　　　　　　　MsgBar.draw");
        Terminal::set_disp_size(editor, self, prom, sbar);

        if self.msg_readonly.len() > 0 {
            write!(out, "{}", self.get_disp_readonly_msg()).unwrap();
            out.flush().unwrap();
        }
        if self.msg_keyrecord.len() > 0 {
            write!(out, "{}", self.get_disp_macro_msg()).unwrap();
            out.flush().unwrap();
        }
        if self.msg.len() > 0 {
            write!(out, "{}", self.get_disp_msg()).unwrap();
            out.flush().unwrap();
        }
    }

    pub fn get_disp_readonly_msg(&mut self) -> String {
        return format!("{}{}{}", MoveTo(0, (self.disp_readonly_row_posi - 1) as u16), Clear(ClearType::CurrentLine), self.msg_readonly.clone());
    }
    pub fn get_disp_macro_msg(&mut self) -> String {
        return format!("{}{}{}", MoveTo(0, (self.disp_keyrecord_row_posi - 1) as u16), Clear(ClearType::CurrentLine), self.msg_keyrecord.clone());
    }

    pub fn get_disp_msg(&mut self) -> String {
        return format!("{}{}{}", MoveTo(0, (self.disp_row_posi - 1) as u16), Clear(ClearType::CurrentLine), self.msg.clone());
    }

    pub fn set_info(&mut self, msg: &str) {
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        self.msg = format!("{}{}{}", Colors::get_msg_fg(), Colors::get_default_bg(), msg_str,);
    }

    pub fn set_err(&mut self, msg: &str) {
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        self.msg = format!("{}{}{}{}", Colors::get_msg_err_fg(), Colors::get_default_bg(), msg_str, Colors::get_default_fg());
    }

    pub fn set_keyrecord(&mut self, msg: &str) {
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        self.msg_keyrecord = format!("{}{}{}", Colors::get_msg_warning_fg(), Colors::get_default_bg(), msg_str,);
    }

    pub fn set_readonly(&mut self, msg: &str) {
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num - (get_str_width(&msg) - msg.chars().count()));
        self.msg_readonly = format!("{}{}{}", Colors::get_msg_err_fg(), Colors::get_default_bg(), msg_str,);
    }
}
