use super::parts::input_area::*;
use ewin_cfg::log::*;
use ewin_const::models::event::*;
use ewin_key::{clipboard::*, model::*};
use ewin_utils::char_edit::*;

impl PromContInputArea {
    pub fn insert_str(&mut self, ep: &mut Proc) {
        //  let chars: Vec<char> = ep.str.chars().collect();
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);

        Log::debug("ep.strep.strep.str", &ep.str);

        for c in ep.str.chars() {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        Log::debug(" self.buf", &self.buf);

        ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
    }

    pub fn backspace(&mut self, ep: &mut Proc) -> ActType {
        if !self.sel.is_selected_width() && self.cur.x == 0 {
            return ActType::Cancel;
        }
        if self.cur.x > 0 {
            self.cur.x -= 1;
            self.cur.disp_x -= get_char_width_not_tab(&self.buf[self.cur.x]);
            ep.str = self.buf[self.cur.x].to_string();
            self.buf.remove(self.cur.x);
        }
        return ActType::Next;
    }
    pub fn delete(&mut self, ep: &mut Proc) -> ActType {
        if !self.sel.is_selected_width() && self.cur.x == self.buf.len() {
            return ActType::Cancel;
        }
        if self.cur.x < self.buf.len() {
            ep.str = self.buf[self.cur.x].to_string();
            self.buf.remove(self.cur.x);
        }
        return ActType::Next;
    }

    pub fn paste(&mut self, ep: &mut Proc) -> ActType {
        let act_type = check_clipboard(true);
        if act_type != ActType::Next {
            return act_type;
        }
        // for Not Undo
        ep.str = get_clipboard().unwrap_or_else(|_| "".to_string());

        let chars: Vec<char> = ep.str.chars().collect();
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
        for c in chars {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
        return ActType::Next;
    }
}
