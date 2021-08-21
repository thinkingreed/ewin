use crate::{
    _cfg::{cfg::*, keys::*},
    clipboard::*,
    def::*,
    global::*,
    log::*,
    model::*,
    prompt::prompt::prompt::*,
    sel_range::*,
    tab::Tab,
    terminal::*,
    util::*,
};
use std::collections::BTreeMap;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, 0);
        let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], self.offset_disp_x, false);
        self.sel.set_e(self.buf.len_lines() - 1, cur_x, width);
        self.draw_type = DrawType::All;
    }

    pub fn cut(&mut self, ep: Proc) {
        Log::debug_key("cut");
        set_clipboard(&ep.str);
        self.draw_type = DrawType::All;
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

    pub fn ctrl_home(&mut self) {
        self.updown_x = 0;
        self.set_cur_default();
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctrl_end(&mut self) {
        let y = self.buf.len_lines() - 1;
        let len_line_chars = self.buf.len_line_chars(y);
        self.set_cur_target(y, len_line_chars, false);
        self.scroll();
        self.scroll_horizontal();
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
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

            rtn_vec.push(SearchRange { y: y, sx: sx, ex: ex });
        }

        return rtn_vec;
    }

    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x;

        if is_asc {
            for (i, range) in self.search.ranges.iter().enumerate() {
                // When the cursor position is the target in the first search
                if self.search.idx == USIZE_UNDEFINED {
                    if self.cur.y <= range.y || (self.cur.y == range.y && cur_x <= range.sx) {
                        return i;
                    }
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

        self.set_draw_range_each_process(DrawType::After(self.offset_y));

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
            } else {
                if is_regex {
                    *sx
                } else {
                    (*sx as isize + total) as usize
                }
            };
            replace_map.insert((sx as usize, sx as usize + replace_str_len), search_str.clone());
            total += diff;
        }
        return replace_map;
    }

    pub fn undo(&mut self) {
        Log::debug_key("undo");

        if let Some(evt_proc) = self.history.get_undo_last() {
            Log::debug("evt_proc", &evt_proc);

            if let Some(ep) = evt_proc.evt_proc {
                self.undo_init(&ep);
                self.undo_exec(&ep);
                self.undo_finalize(&ep);
            }
            if let Some(sp) = evt_proc.sel_proc {
                self.undo_init(&sp);
                self.undo_exec(&sp);
                self.undo_finalize(&sp);
            }
            self.scroll();
            self.scroll_horizontal();

            if let Some(undo_ep) = self.history.pop_undo() {
                self.history.redo_vec.push(undo_ep);
            }
        }
    }
    // initial cursor posi set
    pub fn undo_init(&mut self, proc: &Proc) {
        match &proc.keycmd {
            KeyCmd::InsertStr(_) | KeyCmd::InsertLine | KeyCmd::Cut | KeyCmd::ReplaceExec(_, _, _) => self.set_evtproc(&proc, &proc.cur_s),
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc, if proc.cur_s.x > proc.cur_e.x { &proc.cur_e } else { &proc.cur_s });
                } else {
                    if proc.keycmd == KeyCmd::DeleteNextChar {
                        self.set_evtproc(&proc, &proc.cur_s);
                    } else {
                        self.set_evtproc(&proc, &proc.cur_e);
                    }
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match &proc.keycmd {
            KeyCmd::InsertLine => self.edit_proc(KeyCmd::DeleteNextChar),
            KeyCmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    // Set paste target with sel
                    self.sel = proc.sel;
                    self.edit_proc(KeyCmd::DeleteNextChar);
                } else {
                    self.edit_proc(KeyCmd::DelBox(proc.box_sel_vec.clone()));
                }
            }
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(KeyCmd::InsertStr(proc.str.clone()));
                } else {
                    self.edit_proc(KeyCmd::InsertBox(proc.box_sel_vec.clone()));
                }
            }
            KeyCmd::ReplaceExec(is_regex, replace_str, search_map) => {
                let replace_map = self.get_replace_map(*is_regex, replace_str, &search_map);

                if *is_regex {
                    for ((s, e), org_str) in replace_map {
                        let mut map = BTreeMap::new();
                        map.insert((s, e), "".to_string());
                        self.edit_proc(KeyCmd::ReplaceExec(*is_regex, org_str.clone(), map));
                    }
                } else {
                    let search_str = search_map.iter().min().unwrap().1;
                    self.edit_proc(KeyCmd::ReplaceExec(*is_regex, search_str.clone(), replace_map.clone()));
                }
            }

            _ => {}
        }
    }
    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match &proc.keycmd {
            KeyCmd::DeleteNextChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc, if proc.cur_s.x > proc.cur_e.x { &proc.cur_s } else { &proc.cur_e });
                } else {
                    self.set_evtproc(&proc, &proc.cur_s);
                }
            }
            KeyCmd::DeletePrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc, &proc.cur_e);
                } else if !proc.box_sel_vec.is_empty() {
                    self.set_evtproc(&proc, &proc.cur_s);
                }
            }
            KeyCmd::ReplaceExec(_, _, _) => {
                // Return cursor position
                self.set_evtproc(&proc, &proc.cur_s);
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) {
        Log::debug_key("　redo");

        if let Some(evt_proc) = self.history.get_redo_last() {
            Log::debug("evt_proc", &evt_proc);
            if let Some(sp) = evt_proc.sel_proc {
                self.redo_exec(sp);
            }
            if let Some(ep) = evt_proc.evt_proc {
                self.redo_exec(ep);
            }
            if let Some(redo_ep) = self.history.pop_redo() {
                self.history.undo_vec.push(redo_ep);
            }
        }
    }
    pub fn redo_exec(&mut self, proc: Proc) {
        self.set_evtproc(&proc, &proc.cur_s);

        match &proc.keycmd {
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Cut => self.sel = proc.sel,
            _ => {}
        }
        match &proc.keycmd {
            KeyCmd::DeleteNextChar => self.edit_proc(KeyCmd::DeleteNextChar),
            KeyCmd::DeletePrevChar => self.edit_proc(KeyCmd::DeletePrevChar),
            KeyCmd::Cut => self.edit_proc(KeyCmd::Cut),
            KeyCmd::InsertLine => self.edit_proc(KeyCmd::InsertLine),
            KeyCmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(KeyCmd::InsertStr(proc.str.clone()));
                } else {
                    self.edit_proc(KeyCmd::InsertBox(proc.box_sel_redo_vec.clone()));
                }
            }
            KeyCmd::ReplaceExec(is_regex, replace_str, search_map) => self.edit_proc(KeyCmd::ReplaceExec(*is_regex, replace_str.clone(), search_map.clone())),
            _ => {}
        }
    }
}

impl Tab {
    pub fn save(term: &mut Terminal) -> bool {
        let filenm = term.hbar.file_vec[term.idx].filenm.clone();
        if filenm == LANG.new_file {
            Prompt::save_new_file(term);
            return false;
        } else {
            let h_file = &term.hbar.file_vec[term.idx];
            Log::info_s(&format!("Save {}, file info {:?}", &filenm, &h_file));
            let result = term.tabs[term.idx].editor.buf.write_to(&h_file.fullpath, &h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        term.curt().mbar.set_err(&LANG.cannot_convert_encoding);
                    } else {
                        term.tabs[term.idx].editor.is_changed = false;
                        term.curt().prom.clear();
                        term.curt().mbar.clear();
                        if !term.curt().state.is_close_confirm {
                            term.curt().state.clear();
                        }
                        Log::info("Saved file", &filenm.as_str());
                        return true;
                    }
                }
                Err(err) => {
                    term.curt().mbar.set_err(&format!("{} {:?}", LANG.file_saving_problem, err.kind()));
                    Log::error("err", &err.to_string());
                }
            }
        }
        return false;
    }
}
