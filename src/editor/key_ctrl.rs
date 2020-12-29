use crate::{def::*, global::*, model::*, util::*};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use termion::cursor;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.sy = 0;
        self.sel.ey = self.buf.len() - 1;
        self.sel.sx = 0;
        self.sel.s_disp_x = self.rnw + 1;
        let (cur_x, width) = get_row_width(&self.buf[self.sel.ey], 0, self.buf[self.sel.ey].len(), false);
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
                let result = File::create(path);

                match result {
                    Ok(mut file) => {
                        Log::ep_s("Ok(mut file)");

                        for (i, line) in self.buf.iter().enumerate() {
                            let mut line_str: String = line.iter().collect();
                            if i == self.buf.len() - 1 {
                                line_str = line_str.replace(EOF, "");
                                write!(file, "{}", line_str).unwrap();
                            } else {
                                if &line_str.chars().last().unwrap_or(' ') == &NEW_LINE_MARK {
                                    line_str = line_str.chars().take(line_str.chars().count() - 1).collect::<String>();
                                }
                                writeln!(file, "{}", line_str).unwrap();
                            }
                        }
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

        let mut copy_string;
        Log::ep("self.sel", self.sel);
        let sel_vec = get_sel_range_str(&mut self.buf, &mut self.sel);

        if term.env == Env::WSL {
            copy_string = self.get_wsl_str(sel_vec);
        } else {
            copy_string = sel_vec.join("");
            copy_string = copy_string.replace(NEW_LINE_MARK, NEW_LINE.to_string().as_str());
        }

        Log::ep("copy_string", copy_string.clone());
        self.set_clipboard(&copy_string, &term);

        self.d_range = DRnage {
            sy: self.sel.sy,
            ey: self.sel.ey,
            d_type: DType::Target,
        };
    }

    // WSL:powershell.clipboard対応で"’"で文字列を囲み、改行は","
    fn get_wsl_str(&mut self, sel_vec: Vec<String>) -> String {
        let mut copy_str: String = String::new();

        for (i, s) in sel_vec.iter().enumerate() {
            let mut str = format!("'{}'", s);
            if i == sel_vec.len() - 1 {
                if s.chars().last().unwrap_or(' ') == NEW_LINE_MARK {
                    str.push_str(",");
                    str.push_str("''");
                }
            } else {
                str.push_str(",");
            }
            str = str.replace(NEW_LINE_MARK, "");
            copy_str.push_str(&str);
        }

        return copy_str;
    }

    pub fn paste(&mut self, term: &Terminal) {
        Log::ep_s("　　　　　　　  paste");

        let y_offset_org = self.y_offset;
        let cur_y_org = self.cur.y;
        let rnw_org = self.rnw;

        let mut contexts = self.get_clipboard(&term).unwrap_or("".to_string());

        if contexts.len() == 0 {
            return;
        }
        Log::ep("clipboard str", &contexts);

        // EvtProcデータ設定
        let mut ep = EvtProc::new(DoType::Paste, &self);
        {
            ep.str_vec = vec![contexts.clone()];
            ep.sel.sy = self.cur.y;
            ep.sel.sx = self.cur.x - self.rnw;
            ep.sel.s_disp_x = self.cur.disp_x;
        }
        contexts = contexts.replace(NEW_LINE, NEW_LINE_MARK.to_string().as_str());
        self.insert_str(&mut contexts);
        {
            ep.cur_e = Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x };
            ep.sel.ey = self.cur.y;
            ep.sel.ex = self.cur.x - self.rnw;
            ep.sel.e_disp_x = self.cur.disp_x;
        }
        self.undo_vec.push(ep);

        // d_range
        if contexts.match_indices(NEW_LINE_MARK).count() == 0 {
            self.d_range = DRnage { sy: cur_y_org, ey: self.cur.y, d_type: DType::Target };
        } else {
            if y_offset_org != self.y_offset || rnw_org != self.rnw {
                self.d_range.d_type = DType::All;
            } else {
                self.d_range = DRnage { sy: cur_y_org, ey: self.cur.y, d_type: DType::After };
            }
        }
    }

    fn insert_str(&mut self, contexts: &mut String) {
        Log::ep_s("        insert_str");

        Log::ep("contexts", contexts.clone());
        let insert_strs: Vec<String> = split_inclusive(contexts, NEW_LINE_MARK);
        let insert_s_y = self.cur.y;

        // rnw increase
        if insert_strs.len() > 1 {
            let diff = (self.buf.len() + insert_strs.len() - 1).to_string().len() - self.rnw;
            self.rnw += diff;
            self.cur.x += diff;
            self.cur.disp_x += diff;
        }

        if insert_strs.len() > 1 {
            // rest char from the cursor to the end of the line when inserting multi line
            let rest_char_vec: Vec<char> = self.buf[self.cur.y].drain(self.cur.x - self.rnw..).collect();
            // add line
            for i in 1..insert_strs.len() {
                self.buf.insert(insert_s_y + i, vec![]);
            }
            self.buf[insert_s_y + insert_strs.len() - 1] = rest_char_vec;
        }

        for (i, copy_str) in insert_strs.iter().enumerate() {
            Log::ep("copy_str", copy_str);
            if i != 0 {
                self.cur.x = self.rnw;
            }
            let chars: Vec<char> = copy_str.chars().collect();
            for c in chars {
                // Log::ep("ccc", c);
                self.buf[insert_s_y + i].insert(self.cur.x - self.rnw, c.clone());
                self.cursor_right();
            }
        }

        // cursor posi adjustment
        if insert_strs.len() > 1 {
            let last_line_str = insert_strs.get(insert_strs.len() - 1).unwrap().to_string();
            if last_line_str.chars().last().unwrap_or(' ') == NEW_LINE_MARK {
                self.cur.x = self.rnw;
                self.cur.disp_x = 1 + self.rnw;
            } else {
                self.cur.x = last_line_str.chars().count() + self.rnw;
                self.cur.disp_x = get_str_width(&last_line_str) + 1 + self.rnw;
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctl_home(&mut self) {
        Log::ep_s("ctl_home");
        self.updown_x = 0;
        self.set_cur_default();
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctl_end(&mut self) {
        Log::ep_s("　　　　　　　　ctl_end");
        self.cur.y = self.buf.len() - 1;
        self.cur.x = self.buf[self.buf.len() - 1].len() - 1 + self.rnw;
        let (_, width) = get_row_width(&self.buf[self.buf.len() - 1], 0, self.buf[self.buf.len() - 1].len(), false);
        self.cur.disp_x = width + self.rnw + 1;
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
        self.scroll();
        self.scroll_horizontal();
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
                self.search.search_ranges = self.get_search_ranges(self.search.str.clone());

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
                let (_, width) = get_row_width(&self.buf[self.cur.y], 0, range.sx, false);
                self.cur.disp_x = width + self.rnw + 1;
            }
            self.scroll();
            self.scroll_horizontal();
        }
    }
    pub fn get_search_ranges(&mut self, search_str: String) -> Vec<SearchRange> {
        let mut vec = vec![];

        for (i, chars) in self.buf.iter().enumerate() {
            let row_str = chars.iter().collect::<String>();
            let v: Vec<(usize, &str)> = row_str.match_indices(&search_str).collect();
            if v.len() == 0 {
                continue;
            }
            for (index, _) in v {
                let x = get_char_count(&chars, index);
                vec.push(SearchRange { y: i, sx: x, ex: x + search_str.chars().count() - 1 });
            }
        }
        return vec;
    }

    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x - self.rnw;
        if is_asc {
            for (i, range) in self.search.search_ranges.iter().enumerate() {
                if self.cur.y < range.y || (self.cur.y == range.y && cur_x < range.sx) {
                    return i;
                }
            }
            // 循環検索の為に0返却
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
            // 循環検索の為にindex返却
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

        let search_str = prom.cont_1.buf.iter().collect::<String>();
        let replace_str = prom.cont_2.buf.iter().collect::<String>();
        for i in 0..self.buf.len() {
            let row_str = &self.buf[i].iter().collect::<String>();
            let row_str = row_str.replace(&search_str, &replace_str);
            self.buf[i] = row_str.chars().collect::<Vec<char>>();
        }
    }
    pub fn grep_prom(&mut self, prom: &mut Prompt) {
        Log::ep_s("　　　　　　　　grep_prom");
        prom.is_grep = true;
        prom.grep();
    }

    pub fn undo(&mut self, mbar: &mut MsgBar) {
        Log::ep_s("　　　　　　　　undo");
        if let Some(ep) = self.undo_vec.pop() {
            Log::ep("EvtProc", ep.clone());
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
                    self.insert_str(&mut ep.str_vec.join(""));
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
        if let Some(mut ep) = self.redo_vec.pop() {
            self.set_evtproc(&ep, ep.cur_s);
            self.sel = ep.sel;

            match ep.do_type {
                DoType::Del => self.delete(),
                DoType::BS => self.back_space(),
                DoType::Cut => self.cut(term, mbar),
                DoType::Enter => self.enter(),
                DoType::InsertChar => self.insert_char(ep.str_vec[0].chars().nth(0).unwrap_or(' ')),
                DoType::Paste => {
                    self.insert_str(&mut ep.str_vec[0]);
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
