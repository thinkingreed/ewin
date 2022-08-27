use crate::{ewin_key::clipboard::*, ewin_key::model::*, model::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_key::sel_range::*;
use ewin_utils::char_edit::*;

impl Editor {
    pub fn all_select(&mut self) {
        self.win_mgr.curt().sel.clear();
        self.win_mgr.curt().sel.set_s(0, 0, 0);
        let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.buf.len_rows() - 1)[..], 0, false);
        self.win_mgr.curt().sel.set_e(self.buf.len_rows() - 1, cur_x, width);
    }

    pub fn cut(&mut self, ep: Proc) -> ActType {
        Log::debug_key("cut");
        Log::debug("self.sel.is_selected()", &self.win_mgr.curt().sel.is_selected());
        if !self.win_mgr.curt().sel.is_selected() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_sel_range.to_string()));
        }
        set_clipboard(&ep.str);
        self.win_mgr.curt().sel.clear();
        return ActType::Next;
    }

    pub fn copy(&mut self) -> ActType {
        Log::debug_key("copy");
        if !self.win_mgr.curt().sel.is_selected() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_sel_range.to_string()));
        }
        let copy_str = match self.win_mgr.curt().sel.mode {
            SelMode::Normal => self.buf.slice(self.win_mgr.curt().sel.get_range()),
            //
            SelMode::BoxSelect => {
                let (str, box_sel_vec) = self.slice_box_sel();
                self.box_insert.vec = box_sel_vec;
                str
            }
        };
        Log::debug("self.box_insert.vec", &self.box_insert.vec);
        Log::debug("copy_str", &copy_str);

        set_clipboard(&copy_str);
        self.win_mgr.curt().sel.clear();

        return ActType::Next;
    }
}
