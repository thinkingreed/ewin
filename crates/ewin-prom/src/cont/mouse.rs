use crate::{
    ewin_com::{
        _cfg::key::{keycmd::*, keys::Keys},
        log::*,
        util::*,
    },
    model::PromptCont,
};

impl PromptCont {
    pub fn ctrl_mouse(&mut self, x: usize, y: u16) {
        Log::debug_key("PromptCont.ctrl_mouse");
        if y != self.buf_row_posi {
            return;
        }
        let (cur_x, width) = get_until_x(&self.buf, x as usize);
        self.cur.x = cur_x;
        self.cur.disp_x = width;

        let keys = match self.p_cmd {
            P_Cmd::MouseDownLeft(y, x) => Keys::MouseDownLeft(y as u16, x as u16),
            _ => Keys::MouseDownLeft(y as u16, x as u16),
        };

        self.history.set_sel_multi_click(&keys, &mut self.sel, &self.cur, &self.buf);
    }
}
