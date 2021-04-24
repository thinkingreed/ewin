use crate::{colors::*, log::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::io::{stdout, BufWriter, Write};

#[derive(Debug, Clone)]
pub struct MsgBar {
    pub msg_readonly: String,
    pub msg_keyrecord: String,
    pub msg: Msg,
    pub msg_org: Msg,
    pub disp_readonly_row_posi: usize,
    pub disp_keyrecord_row_posi: usize,
    pub disp_row_posi: usize,
    pub disp_readonly_row_num: usize,
    pub disp_keyrecord_row_num: usize,
    pub disp_row_num: usize,
    pub disp_col_num: usize,
}

impl Default for MsgBar {
    fn default() -> Self {
        MsgBar {
            msg_readonly: String::new(),
            msg_keyrecord: String::new(),
            msg: Msg::default(),
            msg_org: Msg::default(),
            disp_readonly_row_posi: 0,
            disp_keyrecord_row_posi: 0,
            disp_row_posi: 0,
            disp_readonly_row_num: 0,
            disp_keyrecord_row_num: 0,
            disp_row_num: 0,
            disp_col_num: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Msg {
    pub str: String,
    pub msg_type: MsgType,
}

impl Default for Msg {
    fn default() -> Self {
        Msg { str: String::new(), msg_type: MsgType::Info }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgType {
    Info,
    Error,
}
impl MsgBar {
    pub fn new() -> Self {
        MsgBar { ..MsgBar::default() }
    }

    pub fn clear_mag(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear_mag");
        self.msg = Msg::default();
    }

    pub fn clear(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear");
        self.msg = Msg::default();
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
    }

    pub fn clear_keyrecord(&mut self) {
        Log::ep_s("　　　　　　　　MsgBar.clear_macro");
        self.msg_keyrecord = String::new();
        self.disp_keyrecord_row_posi = 0;
        self.disp_keyrecord_row_num = 0;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("　　　　　　　　MsgBar.draw");

        if self.msg_readonly.is_empty() || self.msg_keyrecord.is_empty() || !self.msg.str.is_empty() {
            str_vec.push(Colors::get_default_bg());

            if !self.msg_readonly.is_empty() {
                str_vec.push(self.get_disp_readonly_msg());
            }
            if !self.msg_keyrecord.is_empty() {
                str_vec.push(self.get_disp_keyrecord_msg());
            }
            if !self.msg.str.is_empty() {
                str_vec.push(self.get_disp_msg());
            }

            let out = stdout();
            let mut out = BufWriter::new(out.lock());

            let _ = out.write(&str_vec.concat().as_bytes());
            out.flush().unwrap();

            str_vec.clear();
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        if !self.msg_readonly.is_empty() {
            write!(out, "{}", self.get_disp_readonly_msg()).unwrap();
            out.flush().unwrap();
        }
        if !self.msg_keyrecord.is_empty() {
            write!(out, "{}", self.get_disp_keyrecord_msg()).unwrap();
            out.flush().unwrap();
        }
        if self.is_msg_changed() {
            write!(out, "{}", self.get_disp_msg()).unwrap();
            out.flush().unwrap();
        }
    }

    pub fn get_disp_readonly_msg(&mut self) -> String {
        let msg_str = format!("{msg:^width$}", msg = self.msg_readonly, width = self.disp_col_num - (get_str_width(&self.msg_readonly) - self.msg_readonly.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, (self.disp_readonly_row_posi - 1) as u16), Clear(ClearType::CurrentLine), Colors::get_msg_err_fg(), msg_str);
    }
    pub fn get_disp_keyrecord_msg(&mut self) -> String {
        let msg_str = format!("{msg:^width$}", msg = self.msg_keyrecord, width = self.disp_col_num - (get_str_width(&self.msg_keyrecord) - self.msg_keyrecord.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, (self.disp_keyrecord_row_posi - 1) as u16), Clear(ClearType::CurrentLine), Colors::get_msg_warning_fg(), msg_str);
    }
    pub fn get_disp_msg(&mut self) -> String {
        let color = match self.msg.msg_type {
            MsgType::Info => Colors::get_msg_highlight_fg(),
            MsgType::Error => Colors::get_msg_err_fg(),
        };
        let msg_str = format!("{msg:^width$}", msg = self.msg.str, width = self.disp_col_num - (get_str_width(&self.msg.str) - self.msg.str.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, (self.disp_row_posi - 1) as u16), Clear(ClearType::CurrentLine), color, msg_str);
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
        if self.msg_org == self.msg {
            return false;
        } else {
            return true;
        }
    }
}
