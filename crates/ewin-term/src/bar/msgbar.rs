use crate::{
    ewin_com::{colors::*, log::*, util::*},
    model::MsgType,
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;

impl MsgBar {
    pub fn clear_mag(&mut self) {
        Log::debug_key("MsgBar.clear_mag");
        self.msg = Msg::default();
    }

    pub fn clear(&mut self) {
        Log::debug_key("MsgBar.clear");
        self.msg = Msg::default();
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
    }

    pub fn clear_key_record(&mut self) {
        Log::debug_key("MsgBar.clear_macro");
        self.msg_keyrecord = String::new();
    }

    pub fn render(&mut self, str_vec: &mut Vec<String>) {
        Log::info_key("MsgBar.draw");

        if !self.msg_readonly.is_empty() || !self.msg_keyrecord.is_empty() || !self.msg.str.is_empty() {
            str_vec.push(Colors::get_default_bg());

            if !self.msg_readonly.is_empty() {
                str_vec.push(self.get_disp_readonly_msg());
            }
            if !self.msg_keyrecord.is_empty() {
                str_vec.push(self.get_disp_keyrecord_msg());
            }
            if self.is_msg_changed() {
                str_vec.push(self.get_disp_msg());
            }
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("MsgBar.draw_only");

        let mut v: Vec<String> = vec![];
        self.render(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn get_disp_readonly_msg(&mut self) -> String {
        let msg_str = format!("{msg:^width$}", msg = self.msg_readonly, width = self.disp_col_num - (get_str_width(&self.msg_readonly) - self.msg_readonly.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, self.disp_readonly_row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_msg_err_fg(), msg_str);
    }
    pub fn get_disp_keyrecord_msg(&mut self) -> String {
        let msg_str = format!("{msg:^width$}", msg = self.msg_keyrecord, width = self.disp_col_num - (get_str_width(&self.msg_keyrecord) - self.msg_keyrecord.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, self.disp_keyrecord_row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_msg_warning_fg(), msg_str);
    }
    pub fn get_disp_msg(&mut self) -> String {
        let color = match self.msg.msg_type {
            MsgType::Info => Colors::get_msg_highlight_fg(),
            MsgType::Error => Colors::get_msg_err_fg(),
        };
        let msg_str = format!("{msg:^width$}", msg = self.msg.str, width = self.disp_col_num - (get_str_width(&self.msg.str) - self.msg.str.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, self.disp_row_posi as u16), Clear(ClearType::CurrentLine), color, msg_str);
    }

    pub fn set_info(&mut self, msg: &str) {
        self.msg.str = msg.to_string();
        self.msg.msg_type = MsgType::Info;
    }

    pub fn set_err(&mut self, msg: &str) {
        self.msg.str = msg.to_string();
        self.msg.msg_type = MsgType::Error;
    }

    pub fn set_keyrecord(&mut self, msg: &str) {
        self.msg_keyrecord = msg.to_string();
    }

    pub fn set_readonly(&mut self, msg: &str) {
        self.msg_readonly = msg.to_string();
    }

    pub fn is_msg_changed(&mut self) -> bool {
        return !(self.msg_org == self.msg || self.msg.str.is_empty());
    }
    pub fn is_exsist_msg(&mut self) -> bool {
        return !(self.msg.str.is_empty() && self.msg_keyrecord.is_empty());
    }

    pub fn is_msg_keyrecord_changed(&mut self) -> bool {
        return self.msg_keyrecord_org != self.msg_keyrecord;
    }

    pub fn new() -> Self {
        MsgBar { ..MsgBar::default() }
    }
}
