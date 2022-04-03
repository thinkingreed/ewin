use ewin_com::_cfg::lang::lang_cfg::Lang;

use crate::{ewin_com::clipboard::*, ewin_com::log::*, ewin_com::model::*, ewin_com::util::*, model::*};

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, 0);
        let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.buf.len_rows() - 1)[..], 0, false);
        self.sel.set_e(self.buf.len_rows() - 1, cur_x, width);
    }

    pub fn cut(&mut self, ep: Proc) -> ActType {
        if !self.sel.is_selected() {
            return ActType::Render(RParts::MsgBar(Lang::get().no_sel_range.to_string()));
        }
        Log::debug_key("cut");
        set_clipboard(&ep.str);
        return ActType::Next;
    }

    pub fn copy(&mut self) -> ActType {
        Log::debug_key("copy");
        if !self.sel.is_selected() {
            return ActType::Render(RParts::MsgBar(Lang::get().no_sel_range.to_string()));
        }
        let copy_str = match self.sel.mode {
            SelMode::Normal => self.buf.slice(self.sel.get_range()),
            //
            SelMode::BoxSelect => {
                let (str, box_sel_vec) = self.slice_box_sel();

                Log::debug("str", &str);
                Log::debug("box_sel_vec", &box_sel_vec);

                self.box_insert.vec = box_sel_vec;
                str
            }
        };
        Log::debug("self.box_insert.vec", &self.box_insert.vec);
        Log::debug("copy_str", &copy_str);

        set_clipboard(&copy_str);
        self.sel.clear();

        return ActType::Next;
    }
}
