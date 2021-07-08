use crate::{_cfg::keys::KeyCmd, clipboard::*, def::*, global::*, log::*, model::*, prompt::prompt::prompt::*, sel_range::SelMode, tab::Tab, terminal::*, util::*};
use std::collections::BTreeSet;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, 0);
        let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], self.offset_disp_x, false);
        self.sel.set_e(self.buf.len_lines() - 1, cur_x, width);
        self.d_range.draw_type = DrawType::All;
    }

    pub fn cut(&mut self, ep: EvtProc) {
        Log::debug_key("cut");
        // self.sel = ep.sel.clone();
        set_clipboard(&ep.str);
        // self.sel.clear();
        self.d_range.draw_type = DrawType::All;
    }

    pub fn copy(&mut self) {
        Log::debug_key("copy");

        let copy_str;
        match self.sel.mode {
            SelMode::Normal => copy_str = self.buf.slice(self.sel.get_range()),
            SelMode::BoxSelect => {
                let (str, box_sel_vec) = self.slice_box_sel();
                copy_str = str.clone();
                self.box_sel.clipboard_str = str;
                self.box_sel.clipboard_box_sel_vec = box_sel_vec;
            }
        };
        set_clipboard(&copy_str);
    }

    /*
    pub fn insert_char(&mut self, c: char) {
        self.buf.insert_char(self.cur.y, self.cur.x, c);
        self.cur_right();
        if self.is_enable_syntax_highlight {
            self.d_range.draw_type = DrawType::All;
        } else {
            self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::After);
        }
    } */

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

    pub fn search_str(&mut self, is_asc: bool, is_incremental: bool) {
        if self.search.str.len() > 0 {
            if self.search.ranges.len() == 0 {
                self.search.ranges = self.get_search_ranges(&self.search.str, 0, self.buf.len_chars(), 0);
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

    pub fn get_search_ranges(&self, search_str: &String, start_idx: usize, end_idx: usize, ignore_prefix_len: usize) -> Vec<SearchRange> {
        let regex = CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex;
        let search_set = self.buf.search(&search_str, start_idx, end_idx);
        let mut rtn_vec = vec![];

        // Case regex: Use the number of bytes
        //      normal: Use the number of characters
        for (sx, ex) in search_set {
            // Ignore file name and line number match when grep
            if ignore_prefix_len != 0 {
                let line_s_idx = if regex { self.buf.line_to_byte(self.buf.byte_to_line(sx)) } else { self.buf.line_to_char(self.buf.char_to_line(sx)) };
                if sx - line_s_idx < ignore_prefix_len {
                    continue;
                }
            }
            let y = if regex { self.buf.byte_to_line(sx) } else { self.buf.char_to_line(sx) };
            let sx = if regex { self.buf.byte_to_line_char_idx(sx) } else { self.buf.char_to_line_char_idx(sx) };
            let ex = if regex { self.buf.byte_to_line_char_idx(ex) } else { self.buf.char_to_line_char_idx(ex) };

            rtn_vec.push(SearchRange { y: y, sx: sx, ex: ex });
        }

        return rtn_vec;
    }

    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x;

        if is_asc {
            for (i, range) in self.search.ranges.iter().enumerate() {
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

    pub fn replace(&mut self, ep: &mut EvtProc, search_str: &String, replace_str: &String) {
        let search_set = match self.keycmd {
            KeyCmd::ConfirmPrompt | KeyCmd::Redo => REPLACE_SEARCH_RANGE.get().unwrap().try_lock().unwrap().pop().unwrap(),
            KeyCmd::Undo => REPLACE_REPLACE_RANGE.get().unwrap().try_lock().unwrap().pop().unwrap(),
            _ => BTreeSet::new(),
        };
        let (s_idx, _) = *search_set.iter().min().unwrap();
        let (y, x) = (self.buf.char_to_line(s_idx), self.buf.char_to_line_char_idx(s_idx));
        self.set_cur_target(y, x, false);
        ep.cur_s = self.cur;

        let end_char_idx = self.buf.replace(replace_str, &search_set);
        let replace_set = self.get_replace_set(search_str, replace_str, &search_set);

        if self.keycmd == KeyCmd::ConfirmPrompt || self.keycmd == KeyCmd::Redo {
            REPLACE_REPLACE_RANGE.get().unwrap().try_lock().unwrap().push(replace_set);
        } else if self.keycmd == KeyCmd::Undo {
            REPLACE_SEARCH_RANGE.get().unwrap().try_lock().unwrap().push(replace_set);
        }

        if self.is_enable_syntax_highlight {
            self.d_range.draw_type = DrawType::All;
        } else {
            self.d_range = DRange::new(self.offset_y, self.offset_y, DrawType::After);
        }
        let y = self.buf.char_to_line(end_char_idx);
        let x = end_char_idx - self.buf.line_to_char(y) + 1;
        self.set_cur_target(y, x, false);
        ep.cur_e = self.cur;

        self.scroll();
        self.scroll_horizontal();
    }

    pub fn get_replace_set(&mut self, search_str: &String, replace_str: &String, org_set: &BTreeSet<(usize, usize)>) -> BTreeSet<(usize, usize)> {
        let diff: isize = replace_str.chars().count() as isize - search_str.chars().count() as isize;
        let replace_str_len = replace_str.chars().count();
        let mut replace_set: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut total = 0;
        for (i, (sx, _)) in org_set.iter().enumerate() {
            let sx = if i == 0 { *sx as isize } else { *sx as isize + total };
            replace_set.insert((sx as usize, sx as usize + replace_str_len));
            total += diff;
        }
        return replace_set;
    }

    pub fn undo(&mut self) {
        Log::debug_key("undo");

        if let Some(ep) = self.history.get_undo_last() {
            Log::debug("EvtProc", &ep);

            // initial cursor posi set
            match &ep.keycmd {
                KeyCmd::InsertLine | KeyCmd::CutSelect | KeyCmd::InsertStr(_) | KeyCmd::ReplaceExec(_, _) => self.set_evtproc(&ep, &ep.cur_s),
                KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, if ep.cur_s.x > ep.cur_e.x { &ep.cur_e } else { &ep.cur_s });
                    } else {
                        if ep.keycmd == KeyCmd::DeleteNextChar {
                            self.set_evtproc(&ep, &ep.cur_s);
                        } else {
                            self.set_evtproc(&ep, &ep.cur_e);
                        }
                    }
                }
                _ => {}
            }
            // exec
            match &ep.keycmd {
                // KeyCmd::InsertChar(_) => self.edit_proc(KeyCmd::DeleteNextChar),
                KeyCmd::InsertLine => self.edit_proc(KeyCmd::DeleteNextChar),
                KeyCmd::InsertStr(_) => {
                    if ep.box_sel_vec.is_empty() {
                        // Set paste target with sel
                        self.sel = ep.sel;
                        self.edit_proc(KeyCmd::DeleteNextChar);
                    } else {
                        self.edit_proc(KeyCmd::DelBox(ep.box_sel_vec.clone()));
                    }
                }
                KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                    if ep.box_sel_vec.is_empty() {
                        self.edit_proc(KeyCmd::InsertStr(ep.str.clone()));
                    } else {
                        self.edit_proc(KeyCmd::InsertBox(ep.box_sel_vec.clone()));
                    }
                }
                KeyCmd::ReplaceExec(search_str, replace_str) => {
                    self.edit_proc(KeyCmd::ReplaceExec(replace_str.clone(), search_str.clone()));
                    // Return cursor position
                    self.set_evtproc(&ep, &ep.cur_s);
                }

                _ => {}
            }
            // last cursor posi set
            match &ep.keycmd {
                KeyCmd::DeleteNextChar => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, if ep.cur_s.x > ep.cur_e.x { &ep.cur_s } else { &ep.cur_e });
                    } else {
                        self.set_evtproc(&ep, &ep.cur_s);
                    }
                }
                KeyCmd::DeletePrevChar => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, &ep.cur_e);
                    } else if !ep.box_sel_vec.is_empty() {
                        self.set_evtproc(&ep, &ep.cur_s);
                    }
                }
                _ => {}
            }

            self.scroll();
            self.scroll_horizontal();
        }
    }

    pub fn redo(&mut self) {
        Log::debug_key("　redo");

        if let Some(ep) = self.history.get_redo_last() {
            Log::debug("EvtProc", &ep);
            self.set_evtproc(&ep, &ep.cur_s);

            match ep.keycmd {
                KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::CutSelect => self.sel = ep.sel,
                _ => {}
            }
            match ep.keycmd {
                KeyCmd::DeleteNextChar => self.edit_proc(KeyCmd::DeleteNextChar),
                KeyCmd::DeletePrevChar => self.edit_proc(KeyCmd::DeletePrevChar),
                KeyCmd::CutSelect => self.edit_proc(KeyCmd::CutSelect),
                KeyCmd::InsertLine => self.edit_proc(KeyCmd::InsertLine),
                KeyCmd::InsertStr(_) => {
                    if ep.box_sel_vec.is_empty() {
                        self.edit_proc(KeyCmd::InsertStr(ep.str.clone()));
                    } else {
                        self.edit_proc(KeyCmd::InsertBox(ep.box_sel_redo_vec.clone()));
                    }
                }
                KeyCmd::ReplaceExec(search_str, replace_str) => self.edit_proc(KeyCmd::ReplaceExec(search_str, replace_str)),
                _ => {}
            }
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
                        term.hbar.file_vec[term.idx].is_changed = false;
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
