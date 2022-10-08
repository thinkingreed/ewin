use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*};
use ewin_utils::str_edit::*;
use ewin_view::view::*;

impl MsgBar {
    pub fn clear_mag(&mut self) {
        Log::debug_key("MsgBar.clear_mag");
        self.msg_org = self.msg.clone();
        self.msg = Msg::default();
    }

    pub fn clear(&mut self) {
        Log::debug_key("MsgBar.clear");
        self.msg = Msg::default();
    }

    pub fn get_disp_msg(&self) -> String {
        let fg_color = match self.msg.msg_type {
            MsgType::Info => Colors::get_msg_highlight_fg(),
            MsgType::Error => Colors::get_msg_err_fg(),
        };
        let cut_str = cut_str(&self.msg.str, self.view.width, false, true);
        let msg_str = format!("{msg:^width$}", msg = cut_str, width = self.view.width - (get_str_width(&self.msg.str) - self.msg.str.chars().count()));
        return format!("{}{}{}{}{}", MoveTo(0, self.view.y as u16), Clear(ClearType::CurrentLine), fg_color, Colors::get_msg_bg(), msg_str);
    }

    pub fn set_info(&mut self, msg: &str) {
        self.msg.str = msg.to_string();
        self.msg.msg_type = MsgType::Info;
    }

    pub fn set_err(&mut self, msg: &str) {
        self.msg.str = msg.to_string();
        self.msg.msg_type = MsgType::Error;
    }

    pub fn is_msg_changed(&self) -> bool {
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
    pub view: View,
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
