use crate::{
    ewin_com::{
        _cfg::{key::keycmd::*, lang::lang_cfg::*, model::default::*},
        def::*,
        log::*,
        model::*,
    },
    model::*,
};
use std::{cmp::min, collections::BTreeSet};

impl Editor {
    pub fn exec_search_incremental(&mut self, search_str: String) {
        Log::debug_s("exec_search_incremental");
        self.search.str = search_str;

        let regex = Cfg::get().general.editor.search.regex;

        let s_row_idx = if regex { self.buf.row_to_byte(self.offset_y) } else { self.buf.row_to_char(self.offset_y) };
        let ey = min(self.offset_y + self.row_disp_len, self.buf.len_rows());
        let e_row_idx = if regex { self.buf.row_to_byte(ey) } else { self.buf.row_to_char(ey) };

        let cfg_search = Cfg::get_edit_search();

        // self.search_org = self.search.clone();
        self.search.ranges = if self.search.str.is_empty() { vec![] } else { self.get_search_ranges(&cfg_search, &self.search.str, s_row_idx, e_row_idx, 0) };

        // self.set_search_org_and_raneg();

        Log::debug("self.search.ranges", &self.search.ranges);

        if !self.search_org.ranges.is_empty() || !self.search.ranges.is_empty() {
            // Search in advance for drawing
            if !self.search.ranges.is_empty() {
                self.search_str(true, true);
            }
            for s in &self.search.ranges {
                self.change_info.restayle_row_set.insert(s.y);
            }
            for s in &self.search_org.ranges {
                self.change_info.restayle_row_set.insert(s.y);
            }
            self.draw_range = E_DrawRange::Targetpoint;
        }
    }
    pub fn exec_search_confirm(&mut self, search_str: String) -> Option<String> {
        Log::debug_s("exec_search_confirm");
        if search_str.is_empty() {
            return Some(Lang::get().not_set_search_str.clone());
        }
        let cfg_search = &Cfg::get_edit_search();

        if self.search(&search_str, cfg_search) {
            return Some(Lang::get().cannot_find_search_char.clone());
        } else {
            self.search_str(true, false);
            None
        }
    }

    pub fn search(&mut self, search_str: &str, cfg_search: &CfgSearch) -> bool {
        Log::debug_key("search");

        let search_vec = self.get_search_ranges(cfg_search, search_str, 0, self.buf.len_chars(), 0);
        if search_vec.is_empty() {
            return search_vec.is_empty();
        } else {
            self.search.clear();
            self.search.ranges = search_vec.clone();
            // self.set_search_org_and_raneg(search_vec.clone());
            //  self.search.ranges = search_vec.clone();
            self.search.str = search_str.to_string();
            // Set index to initial value
            self.search.idx = USIZE_UNDEFINED;
        }
        return search_vec.is_empty();
    }

    pub fn search_str(&mut self, is_asc: bool, is_incremental: bool) {
        Log::debug_key("search_str");

        if self.search.ranges.is_empty() {
            let cfg_search = &Cfg::get_edit_search();
            // self.set_search_org_and_raneg()
            self.search.ranges = self.get_search_ranges(cfg_search, &self.search.str, 0, self.buf.len_chars(), 0);
        }
        if self.search.ranges.is_empty() {
            return;
        }
        if self.search.row_num == USIZE_UNDEFINED {
            Log::debug("self.search.idx 111", &self.search.idx);

            self.search.idx = self.get_search_str_index(is_asc);
            Log::debug("self.search.idx 222", &self.search.idx);
        } else {
            self.search.idx = self.get_search_row_no_index(self.search.row_num);
            self.search.row_num = USIZE_UNDEFINED;
        }

        if !is_incremental {
            let range = self.search.ranges[self.search.idx];
            self.set_cur_target_by_x(range.y, range.sx, false);
        }

        self.scroll();
        self.scroll_horizontal();
    }

    pub fn get_search_ranges(&self, cfg_search: &CfgSearch, search_str: &str, s_idx: usize, e_idx: usize, ignore_prefix_len: usize) -> Vec<SearchRange> {
        let search_set = self.buf.search(search_str, s_idx, e_idx, cfg_search);
        let mut rtn_vec = vec![];

        // Case regex: Use the number of bytes
        //      normal: Use the number of characters
        let search_str_byte_len = search_str.len();
        let search_str_chars_len = search_str.chars().count();
        for s_idx in search_set {
            // Ignore file name and line number match when grep
            if ignore_prefix_len != 0 {
                let line_s_idx = if cfg_search.regex { self.buf.row_to_byte(self.buf.byte_to_line(s_idx)) } else { self.buf.row_to_char(self.buf.char_to_row(s_idx)) };
                if s_idx - line_s_idx < ignore_prefix_len {
                    continue;
                }
            }
            let y = if cfg_search.regex { self.buf.byte_to_line(s_idx) } else { self.buf.char_to_row(s_idx) };
            let sx = if cfg_search.regex { self.buf.byte_to_line_char_idx(s_idx) } else { self.buf.char_to_line_char_idx(s_idx) };
            let ex = if cfg_search.regex { self.buf.byte_to_line_char_idx(s_idx + search_str_byte_len) } else { self.buf.char_to_line_char_idx(s_idx + search_str_chars_len) };

            rtn_vec.push(SearchRange { y, sx, ex });
        }

        rtn_vec
    }

    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x;

        if is_asc {
            for (i, range) in self.search.ranges.iter().enumerate() {
                // When the cursor position is the target in the first search
                if self.search.idx == USIZE_UNDEFINED && (self.cur.y < range.y || (self.cur.y == range.y && cur_x <= range.sx)) {
                    return i;
                }
                if self.cur.y < range.y || (self.cur.y == range.y && cur_x < range.sx) {
                    return i;
                }
            }
            // return 0 for circular search
            0
        } else {
            let max_index = self.search.ranges.len() - 1;

            let mut ranges = self.search.ranges.clone();
            ranges.reverse();
            for (i, range) in ranges.iter().enumerate() {
                // Log::ep("iii ", &i);
                if self.cur.y > range.y || (self.cur.y == range.y && cur_x > range.sx) {
                    return max_index - i;
                }
            }
            // return index for circular search
            max_index
        }
    }
    pub fn get_search_row_no_index(&self, row_num: usize) -> usize {
        // let row_num: usize = row_num.parse().unwrap();
        for (i, range) in self.search.ranges.iter().enumerate() {
            if row_num == range.y {
                return i;
            }
        }
        0
    }

    pub fn replace(&mut self, proc: &mut Proc, search_str: &str, replace_str: &str, replace_set: &BTreeSet<usize>) {
        let s_idx = replace_set.iter().min().unwrap();
        let cfg_search = Cfg::get_edit_search();

        let (y, x) = if cfg_search.regex { (self.buf.byte_to_line(*s_idx), self.buf.byte_to_line_char_idx(*s_idx)) } else { (self.buf.char_to_row(*s_idx), self.buf.char_to_line_char_idx(*s_idx)) };
        self.set_cur_target_by_x(y, x, false);

        Log::debug("replace replace_str", &replace_str);
        Log::debug("replace search_map", &replace_set);

        let end_char_idx = self.buf.replace(search_str, replace_str, replace_set);

        let y = self.buf.char_to_row(end_char_idx);
        let x = end_char_idx - self.buf.row_to_char(y);
        self.set_cur_target_by_x(y, x, false);
        proc.cur_e = self.cur;
    }

    pub fn get_idx_set(&mut self, search_str: &str, replace_str: &str, org_set: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut replace_set: BTreeSet<usize> = BTreeSet::new();
        let mut total = 0;
        let cfg_search = Cfg::get_edit_search();

        for (i, sx) in org_set.iter().enumerate() {
            // let replace_str_len = if is_regex { replace_str.len() } else { replace_str.chars().count() };
            let diff: isize = if cfg_search.regex { replace_str.len() as isize - search_str.len() as isize } else { replace_str.chars().count() as isize - search_str.chars().count() as isize };
            let sx = if i == 0 || cfg_search.regex { *sx } else { (*sx as isize + total) as usize };
            replace_set.insert(sx);
            total += diff;
        }
        return replace_set;
    }

    pub fn find_next_back(&mut self) -> ActType {
        Log::debug_key("Editor.find_next_back");
        // Quick search
        Log::debug("self.search.ranges.is_empty()", &self.search.ranges.is_empty());
        Log::debug("self.search.str", &self.search.str);
        Log::debug("self.search_org.str", &self.search_org.str);

        let sel_str = self.buf.slice_string(self.sel);
        if self.search.ranges.is_empty() || (!sel_str.is_empty() && self.search.str != sel_str) {
            Log::debug_key("11111111111111111111111111111");
            //  self.search.clear();
            if sel_str.is_empty() {
                return ActType::Render(RParts::MsgBar(Lang::get().not_set_search_str.to_string()));
            }
            self.search.str = sel_str.clone();
            self.search(&sel_str, &Cfg::get_edit_search());
        }
        Log::debug_key("2222222222222222222222222222");

        self.search_str(self.e_cmd == E_Cmd::FindNext, false);
        return ActType::Next;
    }
}
