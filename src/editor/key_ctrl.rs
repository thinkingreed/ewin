use crate::model::{CopyRange, Editor, Log, MsgBar, Prompt, StatusBar};
use crate::util::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

impl Editor {
    pub fn all_select(&mut self) {
        self.sel.clear();
        self.sel.sy = 0;
        self.sel.ey = self.buf.len() - 1;
        self.sel.sx = 0;
        self.sel.s_disp_x = self.lnw + 1;
        let (cur_x, width) = self.get_row_width(self.sel.ey, 0, self.buf[self.buf.len() - 1].len());
        self.sel.ex = cur_x + self.lnw;
        self.sel.e_disp_x = width + self.lnw;
    }
    pub fn cut(&mut self) {
        Log::ep_s("★★  cut");
        if self.sel.is_unselected() {
            return;
        }

        self.copy();
        self.del_sel_range();
    }

    pub fn close(&mut self, prompt: &mut Prompt) -> bool {
        Log::ep("is_change", prompt.is_change);

        if prompt.is_change == true {
            prompt.save_confirm_str();
            // self.draw_cursor(out, sbar).unwrap();
            prompt.is_close_confirm = true;
            return false;
        };
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

    pub fn copy(&mut self) {
        Log::ep_s("★  copy");
        let copy_ranges: Vec<CopyRange> = self.get_copy_range();

        let mut vec: Vec<char> = vec![];
        for (i, copy_range) in copy_ranges.iter().enumerate() {
            // 行の終端までコピーの場合の改行対応

            if vec.len() > 0 {
                // コピー最終行がex == 0以外の場合
                if i == copy_ranges.len() - 1 && copy_range.ex == 0 {
                } else {
                    // WSL PowerShell用の','
                    vec.push(',');
                }
            }
            for j in copy_range.sx..copy_range.ex {
                if let Some(c) = self.buf[copy_range.y].get(j) {
                    Log::ep("ccc", c);
                    vec.push(c.clone());
                }
            }
        }

        let copy_string = vec.iter().collect::<String>().clone();
        Log::ep("&copy_string", &copy_string);

        self.set_clipboard(&copy_string);
    }

    pub fn paste(&mut self) {
        Log::ep_s("★★  paste");

        let contexts = self.get_clipboard().unwrap_or("".to_string());
        Log::ep("clipboard str", &contexts);

        if contexts.len() == 0 {
            return;
        }
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
                self.cursor_right();
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
        let (cur_x, width) = self.get_until_updown_x();
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
        let (cur_x, width) = self.get_until_updown_x();
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

    pub fn replace_prom(&mut self, prom: &mut Prompt) {
        Log::ep_s("★　replace_prom");
        prom.is_replace = true;
        prom.replace();
    }
}
