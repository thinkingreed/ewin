use crate::{
    ewin_core::{log::*, model::*, util::*},
    model::PromptCont,
};

impl PromptCont {
    pub fn ctrl_mouse(&mut self, x: usize, y: u16, is_left_down: bool) {
        Log::debug_key("PromptCont.ctrl_mouse");
        if y != self.buf_row_posi {
            return;
        }
        let (cur_x, width) = get_until_x(&self.buf, x as usize);
        self.cur.x = cur_x;
        self.cur.disp_x = width;
        self.history.set_sel_multi_click(if is_left_down { MouseProc::DownLeft } else { MouseProc::DragLeft }, &mut self.sel, &self.cur, &self.buf, &self.keys);
    }
}
