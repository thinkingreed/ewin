use crate::ewin_key::key::{cmd::*, keys::*};
use ewin_cfg::log::*;

use super::parts::input_area::*;

impl PromContInputArea {
    pub fn ctrl_mouse(&mut self, y: usize, x: usize) {
        Log::debug_key("PromptCont.ctrl_mouse");
        Log::debug("yyy", &y);
        Log::debug("xxx", &x);

        self.set_cur_target(x);

        let keys = match self.base.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => Keys::MouseDownLeft(y as u16, x as u16),
            _ => Keys::MouseDragLeft(y as u16, x as u16),
        };

        // Second &self.cur is dummy
        Log::debug("self.sel 111", &self.sel);

        self.history.set_sel_multi_click(&keys, &mut self.sel, &self.cur, &self.cur, &self.buf);
        Log::debug("self.sel 222", &self.sel);
    }
}
