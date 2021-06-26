use crate::{_cfg::keys::KeyCmd, clipboard::*, def::*, global::*, log::*, model::*, prompt::prompt::prompt::*, tab::Tab, terminal::*, util::*};
use std::collections::BTreeSet;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, 0);
        let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], self.offset_disp_x, false);
        // -1 for EOF
        self.sel.set_e(self.buf.len_lines() - 1, cur_x, width);
        self.d_range.draw_type = DrawType::All;
    }

    pub fn cut(&mut self, cut_str: String) {
        Log::debug_key("cut");
        // self.sel = ep.sel.clone();
        set_clipboard(cut_str.clone());
        // self.sel.clear();
        self.d_range.draw_type = DrawType::All;
    }

    pub fn copy(&mut self) {
        Log::debug_key("copy");

        let str = self.buf.slice(self.sel.get_range());
        set_clipboard(str);
    }

    pub fn paste(&mut self, ep: &mut EvtProc) {
        Log::debug_key("paste");

        if self.is_enable_syntax_highlight {
            self.d_range.draw_type = DrawType::All;
        } else {
            self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::After);
        }

        // for Not Undo
        if self.keycmd == KeyCmd::Paste {
            let mut clipboard = get_clipboard().unwrap_or("".to_string());
            change_nl(&mut clipboard, &self.h_file);
            ep.str = clipboard;
        }
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
        self.insert_str(&ep.str);
        ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
    }

    pub fn insert_str(&mut self, str: &str) {
        Log::debug_key("insert_str");

        self.buf.insert(self.cur.y, self.cur.x, str);
        let insert_strs: Vec<&str> = str.split(NEW_LINE_LF).collect();

        let last_str_len = insert_strs.last().unwrap().chars().count();
        self.cur.y += insert_strs.len() - 1;

        let x = if insert_strs.len() == 1 { self.cur.x + last_str_len } else { last_str_len };
        self.set_cur_target(self.cur.y, x, false);

        self.scroll();
        self.scroll_horizontal();
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

    pub fn replace(&mut self, ep: &mut EvtProc) {
        let search_set = if self.keycmd == KeyCmd::InsertLine || self.keycmd == KeyCmd::Redo { REPLACE_SEARCH_RANGE.get().unwrap().try_lock().unwrap().pop().unwrap() } else { REPLACE_REPLACE_RANGE.get().unwrap().try_lock().unwrap().pop().unwrap() };
        let end_char_idx = self.buf.replace(&ep.str_replace, &search_set);

        let replace_set = self.get_replace_set(&ep.str, &ep.str_replace, &search_set);

        if self.keycmd == KeyCmd::InsertLine || self.keycmd == KeyCmd::Redo {
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
            match ep.evt_type {
                EvtType::Cut | EvtType::InsertChar | EvtType::Paste | EvtType::Replace => self.set_evtproc(&ep, &ep.cur_s),
                EvtType::Enter => self.set_evtproc(&ep, &ep.cur_s),
                EvtType::Del | EvtType::BS => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, if ep.cur_s.x > ep.cur_e.x { &ep.cur_e } else { &ep.cur_s });
                    } else {
                        if ep.evt_type == EvtType::Del {
                            self.set_evtproc(&ep, &ep.cur_s);
                        } else {
                            self.set_evtproc(&ep, &ep.cur_e);
                        }
                    }
                }
                _ => {}
            }
            // exec
            match ep.evt_type {
                EvtType::InsertChar => self.exec_edit_proc(EvtType::Del, "", ""),
                EvtType::Paste => {
                    // Set paste target with sel
                    self.sel = ep.sel;
                    self.exec_edit_proc(EvtType::Del, "", "");
                }
                EvtType::Enter => self.exec_edit_proc(EvtType::Del, "", ""),
                EvtType::Del | EvtType::BS | EvtType::Cut => self.exec_edit_proc(EvtType::Paste, &ep.str, ""),
                EvtType::Replace => {
                    self.exec_edit_proc(EvtType::Replace, &ep.str_replace, &ep.str);
                    // Return cursor position
                    self.set_evtproc(&ep, &ep.cur_s);
                }
                _ => {}
            }

            match ep.evt_type {
                EvtType::Del => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, if ep.cur_s.x > ep.cur_e.x { &ep.cur_s } else { &ep.cur_e });
                    } else {
                        self.set_evtproc(&ep, &ep.cur_s);
                    }
                }
                EvtType::BS => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, &ep.cur_e);
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
            self.sel = ep.sel;
            match ep.evt_type {
                EvtType::Del => self.exec_edit_proc(EvtType::Del, "", ""),
                EvtType::BS => self.exec_edit_proc(EvtType::BS, "", ""),
                EvtType::Cut => self.exec_edit_proc(EvtType::Cut, "", ""),
                EvtType::Enter => self.exec_edit_proc(EvtType::Enter, "", ""),
                EvtType::InsertChar => self.exec_edit_proc(EvtType::InsertChar, &ep.str, ""),
                EvtType::Paste => {
                    self.sel.clear();
                    self.exec_edit_proc(EvtType::Paste, &ep.str, "");
                }
                EvtType::Replace => self.exec_edit_proc(EvtType::Replace, &ep.str, &ep.str_replace),
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
