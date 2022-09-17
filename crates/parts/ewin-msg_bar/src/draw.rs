use crate::msgbar::*;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use ewin_cfg::{colors::*, log::*};

impl MsgBar {
    pub fn draw(&self, str_vec: &mut Vec<String>, is_forced: bool) {
        if self.is_msg_changed() || is_forced {
            Log::debug_key("MsgBar.draw");
            Log::debug("self.view.width", &self.view.width);
            Log::debug("self.get_disp_msg()", &self.get_disp_msg());

            str_vec.push(format!("{}{}", MoveTo(0, self.view.y as u16), Clear(ClearType::CurrentLine),));
            str_vec.push(Colors::get_default_bg());
            // let msg = cut_str(&self.get_disp_msg(), self.col_num, false, true);
            str_vec.push(self.get_disp_msg());
        }
    }

    pub fn draw_only<T: std::io::Write>(&self, out: &mut T, is_forced: bool) {
        Log::debug_key("MsgBar.draw_only");

        let mut v: Vec<String> = vec![];
        self.draw(&mut v, is_forced);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
