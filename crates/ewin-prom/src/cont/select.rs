use crate::{
    ewin_com::{clipboard::*, log::Log, model::*},
    model::*,
};

impl PromptCont {
    pub fn copy(&mut self) {
        Log::debug_key("copy");
        let sel = self.sel.get_range();
        let str = self.buf[sel.sx..sel.ex].iter().collect::<String>();
        set_clipboard(&str);
    }
    pub fn cut(&mut self, cut_str: String) {
        Log::debug_key("cut");
        set_clipboard(&cut_str);
    }

    pub fn set_evtproc(&mut self, cur: &Cur) {
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
    }
}
