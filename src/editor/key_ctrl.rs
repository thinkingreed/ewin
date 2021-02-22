use crate::{def::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, statusbar::*, util::*};
use std::{collections::BTreeSet, iter::FromIterator, path::Path};

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.set_s(0, 0, self.rnw + 1);
        let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], false);
        // e_disp_x +1 for EOF
        self.sel.set_e(self.buf.len_lines() - 1, cur_x, width + self.rnw + 1);
        self.d_range.draw_type = DrawType::All;
    }

    pub fn cut(&mut self) {
        Log::ep_s("　　　　　　　  cut");
        self.copy();
        self.d_range.draw_type = DrawType::All;
    }

    pub fn save(&mut self, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> bool {
        Log::ep_s("　　　　　　　  save");

        if prom.cont_1.buf.len() > 0 {
            let s = prom.cont_1.buf.iter().collect::<String>();
            self.file.path = Some(Path::new(&s).into());
        }

        if !Path::new(&sbar.filenm).exists() && prom.cont_1.buf.len() == 0 {
            Log::ep_s("!Path::new(&sbar.filenm).exists()");
            prom.is_save_new_file = true;
            prom.save_new_file(self, mbar, help, sbar);
            return false;
        } else {
            if let Some(path) = self.file.path.as_ref() {
                let result = self.buf.write_to(&path.to_string_lossy().to_string());
                match result {
                    Ok(()) => {
                        prom.is_change = false;
                        prom.clear();
                        mbar.clear();
                        return true;
                    }
                    Err(err) => {
                        Log::ep("err", &err.to_string());
                    }
                }
            }
        }
        return false;
    }

    pub fn copy(&mut self) {
        Log::ep_s("　　　　　　　  copy");

        let mut str = self.buf.slice(self.sel.get_range());

        Log::ep("str", &str);

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
        let str = str.replace(NEW_LINE_CRLF, &NEW_LINE.to_string());
        let vec = Vec::from_iter(str.split(NEW_LINE).map(String::from));
        for (i, s) in vec.iter().enumerate() {
            let ss = if *s == "" { "''".to_string() } else { format!("'{}'", s) };
            copy_str.push_str(ss.as_str());
            if i != vec.len() - 1 {
                copy_str.push_str(",");
            }
        }
        Log::ep("copy_str", &copy_str);
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
        self.scroll();
        self.scroll_horizontal();
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
    }

    pub fn search_str(&mut self, is_asc: bool, is_incremental: bool) {
        Log::ep_s("　　　　　　　　search_str");

        if self.search.str.len() > 0 {
            if self.search.ranges.len() == 0 {
                self.search.ranges = self.get_search_ranges(&self.search.str, 0, self.buf.len_chars(), 0);
            }
            if self.search.ranges.len() == 0 {
                return;
            }
            if self.search.row_num.len() == 0 {
                self.search.idx = self.get_search_str_index(is_asc);
            } else {
                self.search.idx = self.get_search_row_no_index(&self.search.row_num);
            }

            if !is_incremental {
                let range = self.search.ranges[self.search.idx];

                Log::ep("range", &range);

                self.set_cur_target(range.y, range.sx);
            }

            self.scroll();
            self.scroll_horizontal();
        }
    }

    pub fn get_search_ranges(&self, search_str: &String, start_idx: usize, end_idx: usize, ignore_prefix_len: usize) -> Vec<SearchRange> {
        Log::ep_s("              get_search_ranges");

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
        let cur_x = self.cur.x - self.rnw;

        if is_asc {
            if self.search.idx == USIZE_UNDEFINED {
                return 0;
            }
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

    pub fn replace(&mut self, prom: &mut Prompt, search_set: BTreeSet<(usize, usize)>) {
        Log::ep_s("　　　　　　　　replace");
        let replace_str: String = prom.cont_2.buf.iter().collect();
        let end_char_idx = self.buf.replace(&replace_str, search_set);

        let y = self.buf.char_to_line(end_char_idx);
        let x = end_char_idx - self.buf.line_to_char(y) + 1;
        self.set_cur_target(y, x);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_grep_result(&mut self) {
        Log::ep_s("set_grep_result");

        self.rnw = self.buf.len_lines().to_string().len();
        self.cur = Cur { y: self.buf.len_lines() - 1, x: self.rnw, disp_x: 0 };
        self.cur.disp_x = self.rnw + get_char_width(self.buf.char(self.cur.y, self.cur.x - self.rnw));
        self.scroll();

        // -2 is a line break for each line
        let line_str = self.buf.char_vec_line(self.buf.len_lines() - 2).iter().collect::<String>();

        // Pattern
        //   text.txt:100:string
        //   grep:text.txt:No permission
        let vec: Vec<&str> = line_str.splitn(3, ":").collect();

        if vec.len() > 2 && vec[0] != "grep" {
            let ignore_prefix_str = format!("{}:{}:", vec[0], vec[1]);

            let mut regex = false;
            {
                regex = CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex;
            }
            let row = self.buf.len_lines() - 2;

            let mut start_idx = 0;
            let mut end_idx = 0;
            let mut ignore_prefix_len = 0;
            if regex {
                start_idx = self.buf.line_to_byte(row);
                end_idx = self.buf.len_bytes();
                ignore_prefix_len = ignore_prefix_str.len();
            } else {
                start_idx = self.buf.line_to_char(row);
                end_idx = self.buf.len_chars();
                ignore_prefix_len = ignore_prefix_str.chars().count();
            }

            Log::ep("ignore_prefix_str", &ignore_prefix_str);
            Log::ep("start_idx", &start_idx);
            Log::ep("end_idx", &end_idx);
            Log::ep("ignore_prefix_len", &ignore_prefix_len);

            let mut search_vec: Vec<SearchRange> = self.get_search_ranges(&self.search.str, start_idx, end_idx, ignore_prefix_len);

            self.search.ranges.append(&mut search_vec);
        }

        if vec.len() > 1 {
            let result: Result<usize, _> = vec[1].parse();

            let grep_result = match result {
                // text.txt:100:string
                Ok(row_num) => GrepResult::new(vec[0].to_string(), row_num),
                // grep:text.txt:No permission
                Err(_) => GrepResult::new(vec[1].to_string().as_str().trim().to_string(), USIZE_UNDEFINED),
            };
            self.grep_result_vec.push(grep_result);
        }
    }

    pub fn undo(&mut self, mbar: &mut MsgBar) {
        Log::ep_s("　　　　　　　　undo");
        if let Some(ep) = self.history.get_undo_last() {
            Log::ep("EvtProc", &ep);
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
            Log::ep("EvtProc", &ep);
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
