use crate::{def::*, global::*, model::*, util::*};
use std::iter::FromIterator;
use std::path::Path;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, self.rnw + 1);
        let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], false);
        // e_disp_x +1 for EOF
        self.sel.set_e(self.buf.len_lines() - 1, cur_x, width + self.rnw + 1);
        self.d_range.d_type = DrawType::All;
    }

    pub fn cut(&mut self) {
        Log::ep_s("　　　　　　　  cut");
        self.copy();
        self.d_range.d_type = DrawType::All;
    }

    pub fn close(&mut self, prom: &mut Prompt) -> bool {
        Log::ep("is_change", prom.is_change);

        if prom.is_change == true {
            prom.save_confirm_str();
            prom.is_close_confirm = true;
            return false;
        };
        return true;
    }
    pub fn save(&mut self, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> bool {
        Log::ep_s("　　　　　　　  save");

        if prom.cont_1.buf.len() > 0 {
            let s = prom.cont_1.buf.iter().collect::<String>();
            self.path = Some(Path::new(&s).into());
        }

        if !Path::new(&sbar.filenm).exists() && prom.cont_1.buf.len() == 0 {
            Log::ep_s("!Path::new(&sbar.filenm).exists()");
            prom.is_save_new_file = true;
            prom.save_new_file();
            return false;
        } else {
            if let Some(path) = self.path.as_ref() {
                let result = self.buf.write_to(&path.to_string_lossy().to_string());
                match result {
                    Ok(()) => {
                        prom.is_change = false;
                        prom.clear();
                        mbar.clear();
                        return true;
                    }
                    Err(err) => {
                        Log::ep("err", err.to_string());
                    }
                }
            }
        }
        return false;
    }

    pub fn copy(&mut self) {
        Log::ep_s("　　　　　　　  copy");

        let mut str = self.buf.slice(self.sel.get_range());
        let copy_string = match *ENV {
            Env::WSL => self.get_wsl_str(&mut str),
            _ => str,
        };
        self.set_clipboard(&copy_string);
    }

    // WSL:powershell.clipboard
    // enclose the string in "’ "
    // new line are ","
    // Empty line is an empty string
    fn get_wsl_str(&mut self, str: &mut String) -> String {
        let mut copy_str: String = String::new();
        let str = str.replace(NEW_LINE_CRLF, ",").replace(NEW_LINE, ",");
        let vec = Vec::from_iter(str.split(",").map(String::from));

        for (i, s) in vec.iter().enumerate() {
            let ss = if *s == "" { "''".to_string() } else { format!("'{}'", s) };
            copy_str.push_str(ss.as_str());
            if i != vec.len() - 1 {
                copy_str.push_str(",");
            }
        }
        Log::ep("copy_str", copy_str.clone());
        copy_str
    }

    pub fn paste(&mut self, ep: &mut EvtProc) {
        Log::ep_s("　　　　　　　  paste");
        self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::After);
        Log::ep("clipboard str", &self.clipboard);
        if self.evt == PASTE {
            ep.str = self.clipboard.clone();
        }
        ep.sel.set_s(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x);
        self.insert_str(&ep.str);
        ep.sel.set_e(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x);
    }

    pub fn insert_str(&mut self, str: &str) {
        Log::ep_s("        insert_str");

        let i = self.buf.line_to_char(self.cur.y) + self.cur.x - self.rnw;
        self.buf.insert(i, str);
        let insert_strs: Vec<&str> = str.split(NEW_LINE).collect();

        let last_str_len = insert_strs.last().unwrap().chars().count();
        self.cur.y += insert_strs.len() - 1;

        let x = if insert_strs.len() == 1 { self.cur.x - self.rnw + last_str_len } else { last_str_len };
        self.set_cur_target(self.cur.y, x);

        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctrl_home(&mut self) {
        Log::ep_s("ctl_home");
        self.updown_x = 0;
        self.set_cur_default();
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctrl_end(&mut self) {
        Log::ep_s("　　　　　　　　ctl_end");
        let y = self.buf.len_lines() - 1;
        self.set_cur_target(y, self.buf.len_line_chars(y));
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
    }

    pub fn search(&mut self, prom: &mut Prompt) {
        Log::ep_s("　　　　　　　　search_prom");
        prom.is_search = true;
        prom.search();
    }

    pub fn search_str(&mut self, is_asc: bool) {
        Log::ep_s("　　　　　　　　search_str");

        if self.search.str.len() > 0 {
            // 初回検索
            Log::ep("search.index", self.search.index);
            if self.search.index == USIZE_UNDEFINED {
                if self.search.ranges.len() == 0 {
                    self.search.ranges = self.get_search_ranges(&self.search.str);
                }
                if self.search.ranges.len() > 0 {
                    if self.search.row_num.len() == 0 {
                        self.search.index = 0;
                    } else {
                        self.search.index = self.get_search_row_no_index(&self.search.row_num);
                    }
                }
            } else {
                self.search.index = self.get_search_str_index(is_asc);
                Log::ep("search.index", self.search.index);
            }

            if self.search.ranges.len() == 0 {
                return;
            }
            if self.search.index != USIZE_UNDEFINED {
                let range = self.search.ranges[self.search.index];

                self.set_cur_target(range.y, range.sx);
            }
            self.scroll();
            self.scroll_horizontal();
        }
    }
    pub fn get_search_ranges(&self, search_str: &String) -> Vec<SearchRange> {
        let mut vec = vec![];

        let search_vec = self.buf.search(&search_str);
        for (sx, ex) in search_vec {
            vec.push(SearchRange {
                y: self.buf.char_to_line(sx),
                sx: self.buf.char_to_line_idx(sx),
                ex: self.buf.char_to_line_idx(ex),
            });
        }

        for s in &vec {
            Log::ep("SearchRange {:?}", s);
        }
        vec
    }

    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x - self.rnw;
        if is_asc {
            for (i, range) in self.search.ranges.iter().enumerate() {
                if self.cur.y < range.y || (self.cur.y == range.y && cur_x < range.sx) {
                    return i;
                }
            }
            // return 0 for circular search
            return 0;
        } else {
            let index = self.search.ranges.len() - 1;
            let mut ranges = self.search.ranges.clone();
            ranges.reverse();
            for (i, range) in ranges.iter().enumerate() {
                if self.cur.y > range.y || (self.cur.y == range.y && cur_x > range.sx) {
                    return index - i;
                }
            }
            // return index for circular search
            return index;
        }
    }
    pub fn get_search_row_no_index(&self, row_num: &String) -> usize {
        let row_num: usize = row_num.parse().unwrap();
        let index = 0;
        for (i, range) in self.search.ranges.iter().enumerate() {
            if row_num == range.y + 1 {
                return i;
            }
        }
        return index;
    }
    pub fn replace_prom(&mut self, prom: &mut Prompt) {
        Log::ep_s("　　　　　　　　replace_prom");
        prom.is_replace = true;
        prom.replace();
    }

    pub fn replace(&mut self, prom: &mut Prompt) {
        Log::ep_s("　　　　　　　　replace");
        let search_str: String = prom.cont_1.buf.iter().collect();
        let replace_str: String = prom.cont_2.buf.iter().collect();
        self.buf.search_and_replace(&search_str, &replace_str);
    }

    pub fn grep_prom(&mut self, prom: &mut Prompt) {
        Log::ep_s("　　　　　　　　grep_prom");
        prom.is_grep = true;
        prom.grep();
    }

    pub fn undo(&mut self, mbar: &mut MsgBar) {
        Log::ep_s("　　　　　　　　undo");
        if let Some(ep) = self.history.get_undo_last() {
            Log::ep("EvtProc", ep.clone());
            // initial cursor posi set
            match ep.evt_type {
                EvtType::Cut | EvtType::Del | EvtType::InsertChar | EvtType::Paste => self.set_evtproc(&ep, &ep.cur_s),
                EvtType::BS => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, &ep.cur_s);
                    } else {
                        self.set_evtproc(&ep, &ep.cur_e);
                    }
                }
                EvtType::Enter => self.set_evtproc(&ep, &ep.cur_e),
                _ => {}
            }
            // exec
            match ep.evt_type {
                EvtType::InsertChar => self.exec_edit_proc(EvtType::Del, ""),
                EvtType::Paste => {
                    // Set paste target with sel
                    self.sel = ep.sel;
                    self.exec_edit_proc(EvtType::Del, "");
                }
                EvtType::Enter => self.exec_edit_proc(EvtType::BS, ""),
                EvtType::Del | EvtType::BS | EvtType::Cut => self.exec_edit_proc(EvtType::Paste, &ep.str),
                _ => {}
            }

            self.scroll();
            self.scroll_horizontal();
        } else {
            mbar.set_err(&LANG.no_undo_operation.to_string());
        }
    }

    pub fn redo(&mut self) {
        Log::ep_s("　　　　　　　　redo");
        if let Some(ep) = self.history.get_redo_last() {
            Log::ep("EvtProc", ep.clone());
            self.set_evtproc(&ep, &ep.cur_s);
            self.sel = ep.sel;
            match ep.evt_type {
                EvtType::Del => self.exec_edit_proc(EvtType::Del, ""),
                EvtType::BS => self.exec_edit_proc(EvtType::BS, ""),
                EvtType::Cut => self.exec_edit_proc(EvtType::Cut, ""),
                EvtType::Enter => self.exec_edit_proc(EvtType::Enter, ""),
                EvtType::InsertChar => self.exec_edit_proc(EvtType::InsertChar, &ep.str),
                EvtType::Paste => {
                    self.sel.clear();
                    self.exec_edit_proc(EvtType::Paste, &ep.str);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {}
