use crate::{ewin_core::clipboard::*, ewin_core::log::*, ewin_core::model::*, ewin_core::util::*, model::*};

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, 0);
        let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], 0, false);
        self.sel.set_e(self.buf.len_lines() - 1, cur_x, width);
        self.draw_range = EditorDrawRange::All;
    }

    pub fn cut(&mut self, ep: Proc) {
        Log::debug_key("cut");
        set_clipboard(&ep.str);
        self.draw_range = EditorDrawRange::All;
    }

    pub fn copy(&mut self) {
        Log::debug_key("copy");

        let copy_str;
        match self.sel.mode {
            SelMode::Normal => copy_str = self.buf.slice(self.sel.get_range()),
            SelMode::BoxSelect => {
                let (str, box_sel_vec) = self.slice_box_sel();
                copy_str = str.clone();
                self.box_insert.vec = box_sel_vec;
            }
        };
        set_clipboard(&copy_str);
        self.sel.clear();
    }
}
