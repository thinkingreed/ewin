use std::collections::BTreeSet;

use ewin_com::_cfg::lang::lang_cfg::Lang;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*, util::*},
    model::*,
};

impl Editor {
    pub fn init_input_comple(&mut self, is_first: bool) -> ActType {
        Log::debug_key("Editor.init_input_comple");

        let search_str = self.get_until_delim_str().0;
        if self.cur.x != 0 && search_str.is_empty() {
            return ActType::Render(RParts::MsgBar(Lang::get().no_input_comple_candidates.to_string()));
        }
        self.input_comple.search_set = self.input_comple.search(&search_str);
        if self.input_comple.search_set.is_empty() {
            return ActType::Render(RParts::MsgBar(Lang::get().no_input_comple_candidates.to_string()));
        }

        self.input_comple.window.clear();

        let search_str = self.get_until_delim_str().0;

        Log::debug("self.input_comple.search_set", &self.input_comple.search_set);
        let set = if is_first { self.input_comple.search_set.clone() } else { self.input_comple.search(&search_str) };

        if !is_first && search_str.is_empty() {
            self.state.input_comple_mode = InputCompleMode::None;
            return ActType::Cancel;
        }
        if set.is_empty() {
            self.state.input_comple_mode = InputCompleMode::None;
            return ActType::Render(RParts::MsgBar(Lang::get().no_input_comple_candidates.to_string()));
        } else if set.len() == 1 {
            for s in set {
                self.replace_input_comple(s);
                self.clear_input_comple();
            }
        } else {
            self.state.input_comple_mode = InputCompleMode::WordComple;
            self.input_comple.set_disp_name(&search_str);
            self.input_comple.window.set_parent_disp_area(self.cur.y + self.row_posi - self.offset_y, self.cur.disp_x + self.get_rnw_and_margin() - 1);
            self.input_comple.window.set_init_menu();
        }

        return ActType::Next;
    }

    pub fn replace_input_comple(&mut self, replace_str: String) {
        let (search_str, str_sx) = self.get_until_delim_str();
        let s_idx = self.buf.row_to_char(self.cur.y) + str_sx;

        self.edit_proc(E_Cmd::ReplaceExec(search_str, replace_str, BTreeSet::from([s_idx])));
    }

    pub fn clear_input_comple(&mut self) {
        self.input_comple.clear();
        self.state.input_comple_mode = InputCompleMode::None;
    }

    pub fn get_until_delim_str(&self) -> (String, usize) {
        let y = self.cur.y;
        let sx = get_until_delim_sx(&self.buf.char_vec_row(y)[..self.cur.x]);
        return (self.buf.char_vec_row(y)[sx..self.cur.x].iter().collect::<String>(), sx);
    }

    pub fn is_input_imple_mode(&self, is_curt: bool) -> bool {
        if is_curt {
            self.state.input_comple_mode == InputCompleMode::WordComple
        // org
        } else {
            self.state.input_comple_mode_org == InputCompleMode::WordComple
        }
    }
}
