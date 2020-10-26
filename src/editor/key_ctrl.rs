use crate::model::*;
use crate::util::*;
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
        self.sel.s_disp_x = self.lnw + 1;
        let (cur_x, width) = get_row_width(&self.buf[self.sel.ey], 0, self.buf[self.sel.ey].len());
        self.sel.ex = cur_x + self.lnw;
        self.sel.e_disp_x = width + self.lnw + 1;
    }

    pub fn cut(&mut self, term: &Terminal) {
        Log::ep_s("★  cut");
        if !self.sel.is_selected() {
            return;
        }
        self.copy(term);
        self.save_sel_del_evtproc();
        self.set_sel_del_d_range();

        self.del_sel_range();
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
        Log::ep_s("★  save");
        Log::ep("prom.cont.buf.len()", prom.cont.buf.len());

        if prom.cont.buf.len() > 0 {
            let s = prom.cont.buf.iter().collect::<String>();
            self.path = Some(Path::new(&s).into());
        }

        eprintln!("self.path {:?}", self.path);

        Log::ep("sbar.filenm", &sbar.filenm);
        Log::ep("prom.cont.buf", prom.cont.buf.iter().collect::<String>());
        // Log::ep("path", self.path.to_owned());

        if !Path::new(&sbar.filenm).exists() && prom.cont.buf.len() == 0 {
            prom.is_save_new_file = true;
            prom.save_new_file();
            return false;
        } else {
            if let Some(path) = self.path.as_ref() {
                let result = File::create(path);

                match result {
                    Ok(mut file) => {
                        for line in &self.buf {
                            for &c in line {
                                Log::ep("save c", c);
                                write!(file, "{}", c).unwrap();
                            }
                            writeln!(file).unwrap();
                        }
                        prom.is_change = false;
                        prom.clear();
                        mbar.clear();
                        return true;
                    }
                    Err(err) => {
                        Log::ep("err", err.to_string());
                        // TODO 新規ファイル時はフォルダの権限をmainで先に確認が必要
                        // eprintln!("err.kind() {:?}", err.kind()); // 権限が無い場合のout PermissionDenied
                    }
                }
            }
        }
        return false;
    }

    pub fn copy(&mut self, term: &Terminal) {
        Log::ep_s("★  copy");
        let copy_ranges: Vec<CopyRange> = self.get_copy_range();
        if copy_ranges.len() == 0 {
            return;
        };

        let copy_string;
        let sel_vec = self.get_sel_range_str();
        if term.env == Env::WSL {
            copy_string = self.set_wsl_vec(sel_vec).join("\n");
        } else {
            copy_string = sel_vec.join("\n");
        }

        //  let copy_string = vec.iter().collect::<String>().clone();
        Log::ep("copy_string", copy_string.clone());
        self.set_clipboard(&copy_string);
    }

    // WSL:powershell.clipboard対応で"’"で文字列を囲み、改行は","
    fn set_wsl_vec(&mut self, sel_vec: Vec<String>) -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        for (i, s) in sel_vec.iter().enumerate() {
            let mut str = format!("{}{}", "'", s);
            str.push_str("'");
            if i != sel_vec.len() - 1 {
                str.push_str(",");
            }
            vec.push(str);
        }
        return vec;
    }

    pub fn paste(&mut self) {
        Log::ep_s("★★  paste");

        let mut contexts = self.get_clipboard().unwrap_or("".to_string());
        Log::ep("clipboard str", &contexts);
        if contexts.len() == 0 {
            return;
        }

        self.insert_str(&mut contexts);
    }

    fn insert_str(&mut self, contexts: &mut String) {
        let mut copy_strings: Vec<&str> = contexts.split('\n').collect();

        let mut add_line_count = 0;
        for str in &copy_strings {
            if str.len() > 0 {
                add_line_count += 1;
            }
        }

        // self.lnwの増加対応
        if copy_strings.len() > 1 {
            let diff = (self.buf.len() + add_line_count).to_string().len() - self.lnw;
            self.lnw += diff;
            self.cur.x += diff;
            self.cur.disp_x += diff;
        }

        let mut last_line_str = copy_strings.get(copy_strings.len() - 1).unwrap().to_string();
        let last_line_str_org = last_line_str.clone();

        // 複数行のペーストでカーソル以降の行末までの残りの文字
        let line_rest: Vec<char> = self.buf[self.cur.y].drain(self.cur.x - self.lnw..).collect();

        let line_rest_string: String = line_rest.iter().collect();

        // ペーストが複数行の場合にカーソル行のカーソル以降の文字列をペースト文字列最終行に追加
        if copy_strings.len() > 0 {
            for c in line_rest {
                last_line_str.push(c);
            }
            copy_strings.pop();
            copy_strings.push((&*last_line_str).clone());
        }

        for (i, copy_str) in copy_strings.iter().enumerate() {
            // ペーストが複数行の場合にcursorを進める
            if i != 0 {
                self.cur.y += 1;
                self.cur.x = self.lnw;
                self.cur.disp_x = self.lnw;
            }
            if copy_str.len() == 0 {
                continue;
            }

            let chars: Vec<char> = copy_str.chars().collect();
            for (j, c) in chars.iter().enumerate() {
                Log::ep("ccc", c);

                // 複数行のコピペで既存行で不足の場合
                if self.cur.y == self.buf.len() {
                    self.buf.push(vec![]);
                }
                if i != 0 && j == 0 {
                    self.buf.insert(self.cur.y, vec![]);
                }
                self.buf[self.cur.y].insert(self.cur.x - self.lnw, c.clone());
                // 元々のコピペ文字分は移動

                Log::ep("cur.disp_x 111", self.cur.disp_x);
                self.cursor_right();
                Log::ep("cur.disp_x 222", self.cur.disp_x);
            }
        }
        // 複数行の場合はカーソル位置調整
        if copy_strings.len() > 1 {
            self.cur.x = last_line_str_org.chars().count() + self.lnw;
            self.cur.disp_x = get_str_width(&last_line_str_org) + 1 + self.lnw;
        } else {
            if line_rest_string.len() > 0 {
                self.cur.x -= line_rest_string.chars().count();
                self.cur.disp_x -= get_str_width(&line_rest_string);
            }
        }
    }
    pub fn ctl_home(&mut self) {
        Log::ep_s("ctl_home");
        if self.cur.updown_x == 0 {
            self.cur.updown_x = self.cur.disp_x;
        }
        self.cur.y = 0;
        let (cur_x, width) = get_until_updown_x(self.lnw, &self.buf[self.cur.y], self.cur.updown_x);
        self.cur.disp_x = width;
        self.cur.x = cur_x;
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctl_end(&mut self) {
        Log::ep_s("★　ctl_end");
        if self.cur.updown_x == 0 {
            self.cur.updown_x = self.cur.disp_x;
        }
        self.cur.y = self.buf.len() - 1;
        let (cur_x, width) = get_until_updown_x(self.lnw, &self.buf[self.cur.y], self.cur.updown_x);
        self.cur.disp_x = width;
        self.cur.x = cur_x;
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn search_prom(&mut self, prom: &mut Prompt) {
        Log::ep_s("★　search_prom");
        prom.is_search = true;
        prom.search();
    }
    pub fn search_str(&mut self, is_asc: bool) {
        Log::ep_s("★　search_str");

        if self.search.str.len() > 0 {
            // 初回検索
            if self.search.index == Search::INDEX_UNDEFINED {
                self.search.search_ranges = self.get_search_ranges(self.search.str.clone());

                eprintln!("self.search.search_ranges {:?}", self.search.search_ranges);

                if self.search.search_ranges.len() > 0 {
                    self.search.index = 0;
                }
            } else {
                self.search.index = self.get_search_str_index(is_asc);
            }

            if self.search.search_ranges.len() == 0 {
                return;
            }
            if self.search.index != Search::INDEX_UNDEFINED {
                let range = self.search.search_ranges[self.search.index];
                self.cur.y = range.y;
                self.cur.x = range.sx + self.lnw;
                let (_, width) = get_row_width(&self.buf[self.cur.y], 0, range.sx);
                self.cur.disp_x = width + self.lnw + 1;

                self.scroll();
                self.scroll_horizontal();
            }
        }
    }
    pub fn get_search_ranges(&mut self, search_str: String) -> Vec<SearchRange> {
        let mut vec = vec![];
        let len = search_str.chars().count() - 1;

        for (i, chars) in self.buf.iter().enumerate() {
            let row_str = chars.iter().collect::<String>();
            let v: Vec<(usize, &str)> = row_str.match_indices(&search_str).collect();
            if v.len() == 0 {
                continue;
            }
            for (index, _) in v {
                let x = get_char_count(&chars, index);
                vec.push(SearchRange { y: i, sx: x, ex: x + len });
            }
        }
        return vec;
    }
    pub fn get_search_str_index(&mut self, is_asc: bool) -> usize {
        let cur_x = self.cur.x - self.lnw;
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

    pub fn replace_prom(&mut self, prom: &mut Prompt) {
        Log::ep_s("★　replace_prom");
        prom.is_replace = true;
        prom.replace();
    }

    pub fn replace(&mut self, prom: &mut Prompt) {
        Log::ep_s("★　replace");

        let search_str = prom.cont.buf.iter().collect::<String>();
        let replace_str = prom.cont_sub.buf.iter().collect::<String>();
        for i in 0..self.buf.len() {
            //let buf = &self.buf[i];
            let row_str = &self.buf[i].iter().collect::<String>();
            let row_str = row_str.replace(&search_str, &replace_str);
            self.buf[i] = row_str.chars().collect::<Vec<char>>();
            //  std::mem::replace(&mut self.buf[i], row_str.chars().collect::<Vec<char>>());
        }
    }
    pub fn undo(&mut self) {
        Log::ep_s("★　undo");
        eprintln!("EvtProcess {:?}", self.undo_vec);
        if let Some(ep) = self.undo_vec.pop() {
            if ep.e_type == EvtType::Del {
                if ep.str_vec.len() == 0 {
                } else {
                    self.cur.y = ep.sy;
                    self.cur.x = ep.x + self.lnw;
                    self.cur.disp_x = ep.disp_x;
                    self.insert_str(&mut ep.str_vec.join("\n"));
                }
            }
            self.redo_vec.push(ep);
        };
    }
}
