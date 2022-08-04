use super::parts::input_area::*;
use crate::ewin_key::clipboard::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::model::*;
use ewin_view::cur::Cur;

impl PromContInputArea {
    pub fn copy(&mut self) -> ActType {
        Log::debug_key("PromContInputArea.copy");
        if !self.sel.is_selected() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_sel_range.to_string()));
        }
        let sel = self.sel.get_range();
        let str = self.buf[sel.sx..sel.ex].iter().collect::<String>();
        set_clipboard(&str);
        return ActType::Next;
    }
    pub fn cut(&mut self, cut_str: String) -> ActType {
        Log::debug_key("copy");
        if self.sel.is_selected() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_sel_range.to_string()));
        }
        Log::debug_key("cut");
        set_clipboard(&cut_str);
        return ActType::Next;
    }

    pub fn set_evtproc(&mut self, cur: &Cur) {
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
    }
}
