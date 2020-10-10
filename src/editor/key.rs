use crate::model::{CopyRange, Editor, Log, MsgBar, Prompt, StatusBar};
use crate::util::*;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode::{Left, Right};
use std::cmp::{max, min};
use std::fs::File;
use std::io::Write;
use std::path::Path;

impl Editor {
    pub fn cursor_up(&mut self) {
        Log::ep_s("★ c_u start");
        if self.cur.y > 0 {
            self.cur.y -= 1;

            self.cursor_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn cursor_down(&mut self) {
        Log::ep_s("★ c_d start");
        if self.cur.y + 1 < self.buf.len() {
            self.cur.y += 1;

            self.cursor_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cursor_updown_com(&mut self) {
        if self.cur.updown_x == 0 {
            self.cur.updown_x = self.cur.disp_x;
        }
        let (cursorx, line_width) = self.get_row_width(self.cur.y, 0, self.buf[self.cur.y].len());

        // Left,Rightの場合は設定しない
        if self.curt_evt == Key(Left.into()) || self.curt_evt == Key(Right.into()) {
        } else if line_width < self.x_offset_disp {
            Log::ep_s("c_d x_offset 切り替わる");
            self.cur.disp_x = line_width + self.lnw + 1;
            self.cur.x = cursorx + self.lnw;
        } else {
            Log::ep_s("c_d x_offset 切り替わっていない");
            let (cursorx, disp_x) = self.get_until_updown_x();
            self.cur.disp_x = disp_x;
            self.cur.x = cursorx;
        }
    }

    pub fn cursor_left(&mut self) {
        Log::ep_s("★  c_l start");
        // 0, 0の位置の場合
        if self.cur.y == 0 && self.cur.x == self.lnw {
            return;
        // 行頭の場合
        } else if self.cur.x == self.lnw {
            let rowlen = self.buf[self.cur.y - 1].len();
            self.cur.x = rowlen + self.lnw;
            let (_, width) = self.get_row_width(self.cur.y - 1, 0, rowlen);
            self.cur.disp_x = width + self.lnw + 1;

            self.cursor_up();
        } else {
            self.cur.x = max(self.cur.x - 1, self.lnw);
            self.cur.disp_x -= self.get_cur_x_width(self.cur.y);
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn cursor_right(&mut self) {
        Log::ep_s("★  c_r start");
        // 行末
        if self.cur.x == self.buf[self.cur.y].len() + self.lnw {
            // 最終行の行末
            if self.cur.y == self.buf.len() - 1 {
                return;
            // その他の行末
            } else {
                self.cur.updown_x = self.lnw;
                self.cur.disp_x = self.lnw + 1;
                self.cur.x = self.lnw;
                self.cursor_down();
            }
        } else {
            self.cur.disp_x += self.get_cur_x_width(self.cur.y);
            self.cur.x = min(self.cur.x + 1, self.buf[self.cur.y].len() + self.lnw);
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn enter(&mut self) {
        Log::ep_s("★  enter");
        let rest: Vec<char> = self.buf[self.cur.y].drain(self.cur.x - self.lnw..).collect();
        self.buf.insert(self.cur.y + 1, rest);
        self.cur.y += 1;
        self.lnw = self.buf.len().to_string().len();
        self.cur.x = self.lnw;
        self.cur.disp_x = self.lnw + 1;
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn insert_char(&mut self, c: char) {
        //  if !c.is_control() {
        self.buf[self.cur.y].insert(self.cur.x - self.lnw, c);
        self.cursor_right();
        //}
    }

    pub fn back_space(&mut self) {
        Log::ep_s("★  back_space");
        if self.sel.is_unselected() {
            // 0,0の位置の場合
            if self.cur.y == 0 && self.cur.x == self.lnw {
                return;
            }
            if self.cur.x == self.lnw {
                let row_len_org = self.buf.len().to_string().len();

                // 行の先頭
                let line = self.buf.remove(self.cur.y);
                self.cur.y -= 1;
                self.cur.x = self.buf[self.cur.y].len() + self.lnw;
                let (_, width) = self.get_row_width(self.cur.y, 0, self.buf[self.cur.y].len());
                self.cur.disp_x = self.lnw + width + 1;
                self.buf[self.cur.y].extend(line.into_iter());

                let row_len_curt = self.buf.len().to_string().len();
                // 行番号の桁数が減った場合
                if row_len_org != row_len_curt {
                    self.lnw -= 1;
                    self.cur.x -= 1;
                    self.cur.disp_x -= 1;
                }
            } else {
                self.cursor_left();
                self.buf[self.cur.y].remove(self.cur.x - self.lnw);
            }
        } else {
            self.del_sel_range();
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn delete(&mut self) {
        // 最終行の終端
        if self.cur.y == self.buf.len() - 1 && self.cur.x == self.buf[self.cur.y].len() + self.lnw {
            return;
        }
        if self.sel.is_unselected() {
            if self.cur.x == self.buf[self.cur.y].len() + self.lnw {
                // 行末
                let line = self.buf.remove(self.cur.y + 1);
                self.buf[self.cur.y].extend(line.into_iter());
            } else {
                self.buf[self.cur.y].remove(self.cur.x - self.lnw);
            }
        } else {
            self.del_sel_range();
        }
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
            prompt.is_save_confirm = true;
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
            // Log::ep("copy_str", copy_str);
            // Log::ep("copy_str.len", copy_str.len());

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

    pub fn home(&mut self) {
        self.cur.x = self.lnw;
        self.cur.disp_x = self.get_cur_x_width(self.cur.y) + self.lnw;
        self.scroll_horizontal();
    }
    pub fn end(&mut self) {
        self.cur.x = self.buf[self.cur.y].len() + self.lnw;
        let (_, disp_x) = self.get_row_width(self.cur.y, 0, self.cur.x + 1);
        self.cur.disp_x = disp_x;
        self.scroll_horizontal();
    }
    pub fn page_down(&mut self) {
        self.cur.y = min(self.cur.y + self.disp_row_num, self.buf.len() - 1);
        self.cur.x = self.lnw;
        self.scroll();
    }
    pub fn page_up(&mut self) {
        if self.cur.y > self.disp_row_num {
            self.cur.y = self.cur.y - self.disp_row_num;
        } else {
            self.cur.y = 0
        }
        self.cur.x = self.lnw;
        self.scroll();
    }

    pub fn shift_right(&mut self) {
        Log::ep_s("★  shift_right");

        let is_unselected_org = self.sel.is_unselected();
        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if self.sel.is_unselected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_right();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;
        // shift_leftからのshift_right
        if e_disp_x_org == disp_x_org {
            self.sel.ey = self.cur.y;
            self.sel.ex = self.cur.x - self.lnw;
            self.sel.e_disp_x = self.cur.disp_x;
        }
        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org
        // 行の終端から次行に移る場合の不具合対応でcur.yと比較
            && self.sel.sy == self.cur.y
            // sel.s_disp_x == sel.e_disp_x + 1文字でclear
            && self.sel.s_disp_x + self.get_char_width(self.cur.y, self.cur.x - 1) == self.cur.disp_x
        {
            self.sel.clear();
        }
    }
    pub fn shift_left(&mut self) {
        Log::ep_s("★  shift_left");

        let is_unselected_org = self.sel.is_unselected();
        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if self.sel.is_unselected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_left();

        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;

        // shift_rightからのshift_left
        if e_disp_x_org != 0 && e_disp_x_org < disp_x_org {
            self.sel.e_disp_x -= 1;
        }
        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org && self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
    }
    pub fn shift_down(&mut self) {
        Log::ep_s("★　shift_down");
        let is_unselected_org = self.sel.is_unselected();

        if self.sel.is_unselected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_down();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;

        Log::ep("sel.ex", self.sel.ex);
        Log::ep("sel.e_disp_x ", self.sel.e_disp_x);

        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org && self.sel.s_disp_x == self.sel.e_disp_x && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
    }
    pub fn shift_up(&mut self) {
        Log::ep_s("★　shift_up");

        Log::ep("cur.x", self.cur.x);

        let is_unselected_org = self.sel.is_unselected();

        if self.cur.y == 0 {
            return;
        } else {
            if self.sel.is_unselected() {
                self.sel.sy = self.cur.y;
                self.sel.sx = self.cur.x - 1;
                self.sel.s_disp_x = self.cur.disp_x;
                // 行頭の場合に先頭文字を含めない
                if self.cur.x == self.lnw {
                    self.sel.s_disp_x = self.cur.disp_x - 1;
                }
            }
            self.cursor_up();
            self.sel.ey = self.cur.y;
            self.sel.ex = self.cur.x - self.lnw;
            self.sel.e_disp_x = self.cur.disp_x;
        }
        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org && self.sel.s_disp_x == self.sel.e_disp_x && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
    }
    pub fn shift_home(&mut self) {
        Log::ep_s("★　shift_home");
        self.sel.sy = self.cur.y;
        self.sel.sx = self.cur.x - self.lnw;
        self.sel.s_disp_x = self.cur.disp_x;
        self.sel.ey = self.cur.y;
        self.sel.ex = self.lnw;
        self.sel.e_disp_x = self.lnw;
        self.cur.x = self.lnw;
        self.cur.disp_x = self.lnw + 1;
    }
    pub fn shift_end(&mut self) {
        Log::ep_s("★  shift_end");

        self.sel.sy = self.cur.y;
        self.sel.sx = self.cur.x - self.lnw;
        self.sel.s_disp_x = self.cur.disp_x;
        self.sel.ey = self.cur.y;
        self.sel.ex = self.buf[self.cur.y].len();
        let (_, width) = self.get_row_width(self.cur.y, self.cur.x - self.lnw, self.buf[self.cur.y].len());
        self.sel.e_disp_x = self.cur.disp_x + width;

        self.cur.disp_x = self.sel.e_disp_x;
        self.cur.x = self.buf[self.cur.y].len() + self.lnw;
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

    pub fn search(&mut self, prom: &mut Prompt) {
        Log::ep_s("★　search");
        prom.is_search = true;
        prom.search();
    }
}
