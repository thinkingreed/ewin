use crate::{
    ewin_core::{_cfg::cfg::*, def::*, global::*, log::*, model::*},
    model::*,
};
use std::{cmp::min, collections::BTreeMap};

impl Editor {
    pub fn exec_search_incremental(&mut self, search_str: String) {
        Log::debug_s("exec_search_incremental");
        self.search.str = search_str;
        let regex = CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex;

        let s_row_idx = if regex { self.buf.line_to_byte(self.offset_y) } else { self.buf.line_to_char(self.offset_y) };
        let ey = min(self.offset_y + self.disp_row_num, self.buf.len_lines());
        let e_row_idx = if regex { self.buf.line_to_byte(ey) } else { self.buf.line_to_char(ey) };
        let search_org = self.search.clone();

        self.search.ranges = if self.search.str.len() == 0 { vec![] } else { self.get_search_ranges(&self.search.str, s_row_idx, e_row_idx, 0, &CFG.get().unwrap().try_lock().unwrap().general.editor.search) };
        if !search_org.ranges.is_empty() || !self.search.ranges.is_empty() {
            // Search in advance for drawing
            if !self.search.ranges.is_empty() {
                self.search_str(true, true);
            }
            self.draw_range = EditorDrawRange::After(self.offset_y);
        }
    }
    pub fn exec_search_confirm(&mut self, search_str: String) -> Option<String> {
        Log::debug_s("exec_search_confirm");
        if &search_str.len() == &0 {
            return Some(LANG.not_entered_search_str.clone());
        }
        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
        let search_vec = self.search(&search_str, cfg_search);

        if search_vec.len() == 0 {
            return Some(LANG.cannot_find_char_search_for.clone());
        } else {
            self.search.clear();
            self.search.ranges = search_vec;
            self.search.str = search_str;

            // Set index to initial value
            self.search.idx = USIZE_UNDEFINED;
            self.search_str(true, false);
            return None;
        }
    }

    pub fn search(&mut self, search_str: &String, cfg_search: &CfgSearch) -> Vec<SearchRange> {
        Log::debug_key("search");

        let search_vec = self.get_search_ranges(search_str, 0, self.buf.len_chars(), 0, cfg_search);
        if search_vec.len() == 0 {
            return search_vec;
        } else {
            self.search.clear();
            self.search.ranges = search_vec.clone();
            self.search.str = search_str.clone();
            // Set index to initial value
            self.search.idx = USIZE_UNDEFINED;
        }
        return search_vec;
    }

    pub fn search_str(&mut self, is_asc: bool, is_incremental: bool) {
        Log::debug_key("search_str");

        if self.search.str.len() > 0 {
            if self.search.ranges.len() == 0 {
                let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
                self.search.ranges = self.get_search_ranges(&self.search.str, 0, self.buf.len_chars(), 0, cfg_search);
            }
            if self.search.ranges.len() == 0 {
                return;
            }

            if self.search.row_num == USIZE_UNDEFINED {
                self.search.idx = self.get_search_str_index(is_asc);
            } else {
                self.search.idx = self.get_search_row_no_index(self.search.row_num);
                self.search.row_num = USIZE_UNDEFINED;
            }

            if !is_incremental {
                let range = self.search.ranges[self.search.idx];
                self.set_cur_target(range.y, range.sx, false);
            }

            self.scroll();
            self.scroll_horizontal();
        }
    }

    pub fn get_search_ranges(&self, search_str: &String, s_idx: usize, e_idx: usize, ignore_prefix_len: usize, cfg_search: &CfgSearch) -> Vec<SearchRange> {
        let search_map = self.buf.search(&search_str, s_idx, e_idx, cfg_search);
        let mut rtn_vec = vec![];

        // Case regex: Use the number of bytes
        //      normal: Use the number of characters
        for ((sx, ex), _) in search_map {
            // Ignore file name and line number match when grep
            if ignore_prefix_len != 0 {
                let line_s_idx = if cfg_search.regex { self.buf.line_to_byte(self.buf.byte_to_line(sx)) } else { self.buf.line_to_char(self.buf.char_to_line(sx)) };
                if sx - line_s_idx < ignore_prefix_len {
                    continue;
                }
            }
            let y = if cfg_search.regex { self.buf.byte_to_line(sx) } else { self.buf.char_to_line(sx) };
            let sx = if cfg_search.regex { self.buf.byte_to_line_char_idx(sx) } else { self.buf.char_to_line_char_idx(sx) };
            let ex = if cfg_search.regex { self.buf.byte_to_line_char_idx(ex) } else { self.buf.char_to_line_char_idx(ex) };

            rtn_vec.push(SearchRange { y, sx, ex });
        }

        return rtn_vec;
    }

    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x;

        if is_asc {
            for (i, range) in self.search.ranges.iter().enumerate() {
                // When the cursor position is the target in the first search
                if self.search.idx == USIZE_UNDEFINED && (self.cur.y <= range.y || (self.cur.y == range.y && cur_x <= range.sx)) {
                    return i;
                }
                if self.cur.y < range.y || (self.cur.y == range.y && cur_x < range.sx) {
                    return i;
                }
            }
            // return 0 for circular search
            return 0;
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
            return max_index;
        }
    }
    pub fn get_search_row_no_index(&self, row_num: usize) -> usize {
        // let row_num: usize = row_num.parse().unwrap();
        let index = 0;
        for (i, range) in self.search.ranges.iter().enumerate() {
            if row_num == range.y {
                return i;
            }
        }
        return index;
    }

    pub fn replace(&mut self, ep: &mut Proc, is_regex: bool, replace_str: String, replace_map: BTreeMap<(usize, usize), String>) {
        let ((s_idx, _), _) = replace_map.iter().min().unwrap();
        let (y, x) = if is_regex { (self.buf.byte_to_line(*s_idx), self.buf.byte_to_line_char_idx(*s_idx)) } else { (self.buf.char_to_line(*s_idx), self.buf.char_to_line_char_idx(*s_idx)) };
        self.set_cur_target(y, x, false);
        ep.cur_s = self.cur;

        Log::debug("replace is_regex", &is_regex);
        Log::debug("replace replace_str", &replace_str);
        Log::debug("replace search_map", &replace_map);

        let end_char_idx = self.buf.replace(is_regex, &replace_str, &replace_map);

        self.set_draw_range_each_process(EditorDrawRange::After(self.offset_y));

        let y = self.buf.char_to_line(end_char_idx);
        let x = end_char_idx - self.buf.line_to_char(y);
        self.set_cur_target(y, x, false);
        ep.cur_e = self.cur;
    }

    pub fn get_replace_map(&mut self, is_regex: bool, replace_str: &String, org_map: &BTreeMap<(usize, usize), String>) -> BTreeMap<(usize, usize), String> {
        let mut replace_map: BTreeMap<(usize, usize), String> = BTreeMap::new();
        let mut total = 0;

        for (i, ((sx, _), search_str)) in org_map.iter().enumerate() {
            let replace_str_len = if is_regex { replace_str.len() } else { replace_str.chars().count() };
            let diff: isize = if is_regex { replace_str.len() as isize - search_str.len() as isize } else { replace_str.chars().count() as isize - search_str.chars().count() as isize };
            let sx = if i == 0 {
                *sx
            } else if is_regex {
                *sx
            } else {
                (*sx as isize + total) as usize
            };
            replace_map.insert((sx as usize, sx as usize + replace_str_len), search_str.clone());
            total += diff;
        }
        return replace_map;
    }
}
