use crate::model::PromptCont;
use ewin_com::{clipboard::*, log::*, model::*};

impl PromptCont {
    pub fn insert_str(&mut self, ep: &mut Proc) {
        //  let chars: Vec<char> = ep.str.chars().collect();
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);

        Log::debug("ep.strep.strep.str", &ep.str);

        for c in ep.str.chars() {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
    }
    pub fn paste(&mut self, ep: &mut Proc) {
        // for Not Undo
        ep.str = get_clipboard().unwrap_or("".to_string());

        let chars: Vec<char> = ep.str.chars().collect();
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
        for c in chars {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
    }
}
