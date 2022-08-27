use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*};
use ewin_utils::str_edit::*;
use std::io::Write;

impl MsgBar {
    pub fn clear_mag(&mut self) {
        Log::debug_key("MsgBar.clear_mag");
        self.msg_org = self.msg.clone();
        self.msg = Msg::default();
    }

    pub fn clear(&mut self) {
        Log::debug_key("MsgBar.clear");
        self.msg = Msg::default();
        self.row_posi = 0;
        self.col_num = 0;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, is_forced: bool) {
        if self.is_msg_changed() || is_forced {
            Log::debug_key("MsgBar.draw");
            str_vec.push(format!("{}{}", MoveTo(0, self.row_posi as u16), Clear(ClearType::CurrentLine),));
            str_vec.push(Colors::get_default_bg());
            str_vec.push(self.get_disp_msg());
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("MsgBar.draw_only");

        let mut v: Vec<String> = vec![];
        self.draw(&mut v, true);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn get_disp_msg(&mut self) -> String {
        let color = match self.msg.msg_type {
            MsgType::Info => Colors::get_msg_highlight_fg(),
            MsgType::Error => Colors::get_msg_err_fg(),
        };
        let msg_str = format!("{msg:^width$}", msg = self.msg.str, width = self.col_num - (get_str_width(&self.msg.str) - self.msg.str.chars().count()));
        return format!("{}{}{}{}", MoveTo(0, self.row_posi as u16), Clear(ClearType::CurrentLine), color, msg_str);
    }

    pub fn set_info(&mut self, msg: &str) {
        self.msg.str = msg.to_string();
        self.msg.msg_type = MsgType::Info;
    }

    pub fn set_err(&mut self, msg: &str) {
        self.msg.str = msg.to_string();
        self.msg.msg_type = MsgType::Error;
    }

    pub fn is_msg_changed(&mut self) -> bool {
        return self.msg_org != self.msg;
    }

    pub fn is_exsist_msg(&mut self) -> bool {
        return !self.msg.str.is_empty();
    }

    pub fn set_org_state(&mut self) {
        self.msg_org = self.msg.clone();
    }
}

#[derive(Debug, Default, Clone)]
pub struct MsgBar {
    pub msg: Msg,
    pub msg_org: Msg,
    // 0 indexed
    pub row_posi: usize,
    pub row_num: usize,
    pub col_num: usize,
}
impl MsgBar {
    pub fn new() -> Self {
        MsgBar { ..MsgBar::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
