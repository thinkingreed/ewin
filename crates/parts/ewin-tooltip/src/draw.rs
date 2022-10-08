use crate::tooltip::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};

impl ToolTip {
    pub fn draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("ToolTip.draw");

        Log::debug("self", &self);
        Log::debug("self.view", &self);

        if self.is_show {
            str_vec.push(Colors::get_tooltip_fg_bg());
            for (i, s) in self.vec.iter().enumerate() {
                str_vec.push(MoveTo(self.view.x as u16, (self.view.y + i) as u16).to_string());
                str_vec.push(format!(" {} ", s));
            }
        }
    }

    pub fn draw_only<T: std::io::Write>(&self, out: &mut T) {
        Log::debug_key("ToolTip.draw_only");

        let mut v: Vec<String> = vec![];
        self.draw(&mut v);

        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
