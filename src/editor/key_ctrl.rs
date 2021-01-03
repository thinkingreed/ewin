use crate::{def::*, global::*, model::*, util::*};
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;
use termion::cursor;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.sy = 0;
        self.sel.ey = self.buf.len_lines() - 1;
        self.sel.sx = 0;
        self.sel.s_disp_x = self.rnw + 1;
        let (cur_x, width) = get_row_width(&self.buf.char_vec(self.sel.ey)[..], false);
        self.sel.ex = cur_x;
        // +1 for EOF
        self.sel.e_disp_x = width + self.rnw + 1;
        self.d_range.d_type = DType::All;
    }

    pub fn cut(&mut self, term: &Terminal, mbar: &mut MsgBar) {
        Log::ep_s("　　　　　　　  cut");
        if !self.sel.is_selected() {
            mbar.set_err(&LANG.lock().unwrap().no_sel_range.to_string());
            return;
        }
        self.copy(term, mbar);
        self.set_sel_del_d_range();
        self.save_sel_del_evtproc(DoType::Cut);

        self.del_sel_range();
        self.sel.clear();
    }

    pub fn close<T: Write>(&mut self, out: &mut T, prompt: &mut Prompt) -> bool {
        Log::ep("is_change", prompt.is_change);

        if prompt.is_change == true {
            prompt.save_confirm_str();
            // self.draw_cursor(out, sbar).unwrap();
            prompt.is_close_confirm = true;
            return false;
        };
        write!(out, "{}", cursor::Hide.to_string()).unwrap();
        out.flush().unwrap();

        return true;
    }
    pub fn save(&mut self, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> bool {
        Log::ep_s("　　　　　　　  save");
        Log::ep("prom.cont_1.buf.len()", prom.cont_1.buf.len());

        if prom.cont_1.buf.len() > 0 {
            let s = prom.cont_1.buf.iter().collect::<String>();
            self.path = Some(Path::new(&s).into());
        }

        Log::ep("sbar.filenm", &sbar.filenm);
        Log::ep("prom.cont_1.buf", prom.cont_1.buf.iter().collect::<String>());

        if !Path::new(&sbar.filenm).exists() && prom.cont_1.buf.len() == 0 {
            Log::ep_s("!Path::new(&sbar.filenm).exists()");

            prom.is_save_new_file = true;
            prom.save_new_file();
            return false;
        } else {
            if let Some(path) = self.path.as_ref() {
                Log::ep("Some(path)", "");
                let result = self.buf.write_to(&path.to_string_lossy().to_string());

                match result {
                    Ok(()) => {
                        Log::ep_s("Ok(mut file)");

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

    pub fn copy(&mut self, term: &Terminal, mbar: &mut MsgBar) {
        Log::ep_s("　　　　　　　  copy");

        if !self.sel.is_selected() {
            mbar.set_err(&LANG.lock().unwrap().no_sel_range.to_string());
            return;
        }
        Log::ep("self.sel", self.sel);

        let mut str = self.buf.slice(self.sel.get_range());
        let copy_string = match term.env {
            Env::WSL => self.get_wsl_str(&mut str),
            _ => str,
        };

        Log::ep("copy_string", copy_string.clone());
        self.set_clipboard(&copy_string, &term);

        self.d_range = DRnage {
            sy: self.sel.sy,
            ey: self.sel.ey,
            d_type: DType::Target,
        };
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

    pub fn paste(&mut self, term: &Terminal) {
        Log::ep_s("　　　　　　　  paste");

        let y_offset_org = self.offset_y;
        let cur_y_org = self.cur.y;
        let rnw_org = self.rnw;

        let contexts = self.get_clipboard(&term).unwrap_or("".to_string());

        if contexts.len() == 0 {
            return;
        }
        Log::ep("clipboard str", &contexts);

        // EvtProcデータ設定
        let mut ep = EvtProc::new(DoType::Paste, self.cur, self.d_range);
        {
            ep.str_vec = vec![contexts.clone()];
            ep.sel.sy = self.cur.y;
            ep.sel.sx = self.cur.x - self.rnw;
            ep.sel.s_disp_x = self.cur.disp_x;
        }
        self.insert_str(&contexts);
        {
            ep.cur_e = Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x };
            ep.sel.ey = self.cur.y;
            ep.sel.ex = self.cur.x - self.rnw;
            ep.sel.e_disp_x = self.cur.disp_x;
        }
        self.undo_vec.push(ep);

        // d_range
        if contexts.match_indices(NEW_LINE).count() == 0 {
            self.d_range = DRnage { sy: cur_y_org, ey: self.cur.y, d_type: DType::Target };
        } else {
            if y_offset_org != self.offset_y || rnw_org != self.rnw {
                self.d_range.d_type = DType::All;
            } else {
                self.d_range = DRnage { sy: cur_y_org, ey: self.cur.y, d_type: DType::After };
            }
        }
    }

    pub fn insert_str(&mut self, str: &str) {
        Log::ep_s("        insert_str");
        Log::ep("contexts", str.clone());

        let i = self.buf.line_to_char(self.cur.y) + self.cur.x - self.rnw;
        self.buf.insert(i, str);
        let insert_strs: Vec<&str> = str.split(NEW_LINE).collect();

        let last_str_len = insert_strs.last().unwrap().chars().count();
        self.cur.y += insert_strs.len() - 1;
        self.rnw = self.buf.len_lines().to_string().len();
        let (cur_x, width) = get_row_width(&self.buf.char_vec(self.cur.y)[..last_str_len], false);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

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
        self.cur.y = self.buf.len_lines() - 1;
        self.set_cur_end_x(self.cur.y);
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
                if self.search.search_ranges.len() == 0 {
                    self.search.search_ranges = self.get_search_ranges(&self.search.str);
                }
                if self.search.search_ranges.len() > 0 {
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

            if self.search.search_ranges.len() == 0 {
                return;
            }
            if self.search.index != USIZE_UNDEFINED {
                let range = self.search.search_ranges[self.search.index];
                self.cur.y = range.y;
                self.cur.x = range.sx + self.rnw;
                let (_, width) = get_row_width(&self.buf.char_vec(range.y)[..range.sx], false);
                self.cur.disp_x = width + self.rnw + 1;
            }
            self.scroll();
            self.scroll_horizontal();
        }
    }
    pub fn get_search_ranges(&self, search_str: &String) -> Vec<SearchRange> {
        Log::ep_s("get_search_ranges get_search_ranges get_search_ranges get_search_ranges get_search_ranges");

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
            for (i, range) in self.search.search_ranges.iter().enumerate() {
                if self.cur.y < range.y || (self.cur.y == range.y && cur_x < range.sx) {
                    return i;
                }
            }
            // return 0 for circular search
            return 0;
        } else {
            let index = self.search.search_ranges.len() - 1;
            let mut ranges = self.search.search_ranges.clone();
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
        for (i, range) in self.search.search_ranges.iter().enumerate() {
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
        if let Some(ep) = self.undo_vec.pop() {
            // Log::ep("EvtProc", ep.clone());
            self.is_undo = true;
            if ep.str_vec.len() == 0 {
                // 行末でDelete
                if ep.do_type == DoType::Del {
                    self.set_evtproc(&ep, ep.cur_s);
                    self.enter();
                    self.set_evtproc(&ep, ep.cur_s);
                // 行頭でBS
                } else if ep.do_type == DoType::BS {
                    self.set_evtproc(&ep, ep.cur_e);
                    self.enter();
                } else if ep.do_type == DoType::Enter {
                    self.set_evtproc(&ep, ep.cur_e);
                    self.back_space();
                }
            // 行中
            } else {
                // initial cursor posi set
                if ep.do_type == DoType::Cut || ep.do_type == DoType::Del || ep.do_type == DoType::InsertChar || ep.do_type == DoType::Paste {
                    self.set_evtproc(&ep, ep.cur_s);
                } else if ep.do_type == DoType::BS {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep, ep.cur_s);
                    } else {
                        self.set_evtproc(&ep, ep.cur_e);
                    }
                }
                if ep.do_type == DoType::InsertChar {
                    self.delete();
                } else if ep.do_type == DoType::Paste {
                    self.sel.clear();
                    // paste対象をselで設定
                    self.sel = ep.sel;
                    self.set_sel_del_d_range();
                    self.del_sel_range();
                    self.sel.clear();
                } else {
                    self.insert_str(&ep.str_vec.join(""));
                }

                // last cursor posi set
                if ep.sel.is_selected() && ep.do_type != DoType::Paste {
                    self.set_evtproc(&ep, ep.cur_e);
                } else {
                    self.set_evtproc(&ep, ep.cur_s);
                }
            }
            self.scroll();
            self.scroll_horizontal();

            self.redo_vec.push(ep);
            self.is_undo = false;
        } else {
            Log::ep("undo_vec.len", self.undo_vec.len());
            mbar.set_err(&LANG.lock().unwrap().no_undo_operation.to_string());
        }
    }

    pub fn redo(&mut self, term: &Terminal, mbar: &mut MsgBar) {
        Log::ep_s("　　　　　　　　redo");
        if let Some(ep) = self.redo_vec.pop() {
            self.set_evtproc(&ep, ep.cur_s);
            self.sel = ep.sel;

            match ep.do_type {
                DoType::Del => self.delete(),
                DoType::BS => self.back_space(),
                DoType::Cut => self.cut(term, mbar),
                DoType::Enter => self.enter(),
                DoType::InsertChar => self.insert_char(ep.str_vec[0].chars().nth(0).unwrap_or(' ')),
                DoType::Paste => {
                    self.insert_str(&ep.str_vec[0]);
                    self.undo_vec.push(ep);
                    self.sel.clear();
                }
                _ => {}
            }
        } else {
            mbar.set_err(&LANG.lock().unwrap().no_operation_re_exec.to_string());
        }
    }
}

#[cfg(test)]
mod tests {}
