use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*, util::*},
    model::*,
};

impl Editor {
    pub fn init_input_comple(&mut self, is_first: bool) {
        Log::debug_key("Editor.init_input_comple");
        let search_str = self.get_until_delim_str().0;
        let set = self.input_comple.search(&search_str);

        if !is_first && search_str.is_empty() {
            self.state.input_comple_mode = InputCompleMode::None;
            return;
        }
        if set.is_empty() {
            self.state.input_comple_mode = InputCompleMode::None;
            return;
        } else if set.len() == 1 {
            for s in set {
                let add_str = self.get_input_comple_addstr(&s);
                self.edit_proc(E_Cmd::InsertStr(add_str));
            }
        } else {
            self.state.input_comple_mode = InputCompleMode::WordComple;
            self.input_comple.set_disp_name(&search_str);
            self.input_comple.window.set_parent_disp_area(self.cur.y + self.row_posi - self.offset_y, self.cur.disp_x + self.get_rnw_and_margin() - 1);
            self.input_comple.window.set_init_menu();
        }
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

    pub fn get_input_comple_addstr(&mut self, candidate_str: &str) -> String {
        let ignore_str = self.get_until_delim_str().0;
        let ignore_idx = if let Some(i) = candidate_str.find(&ignore_str) { i } else { 0 };
        Log::debug("ignore_idx", &ignore_idx);
        let add_str = candidate_str[ignore_idx + ignore_str.chars().count()..].to_string();
        Log::debug("add_str", &add_str);

        return add_str;
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
