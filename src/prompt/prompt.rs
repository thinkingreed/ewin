use crate::model::*;
use crate::util::*;
use std::fs;
use std::io::Write;
use termion::{clear, cursor};

impl Prompt {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        // Log::ep_s("　　　　　　　　Prompt draw");
        if self.cont_1.guide.len() > 0 {
            let cont_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.cont_1.guide.clone());
            str_vec.push(cont_desc);

            let key_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 1) as u16), clear::CurrentLine, self.cont_1.key_desc.clone());
            str_vec.push(key_desc);

            if self.is_save_new_file || self.is_search {
                let buf = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont_1.ctl_select_color());
                str_vec.push(buf);
            }

            if self.is_replace || self.is_grep {
                let buf_desc_1 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont_1.buf_desc.clone());
                str_vec.push(buf_desc_1);
                let buf_1 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 3) as u16), clear::CurrentLine, self.cont_1.ctl_select_color());
                str_vec.push(buf_1);
                let buf_desc_2 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 4) as u16), clear::CurrentLine, self.cont_2.buf_desc.clone());
                str_vec.push(buf_desc_2);
                let buf_2 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 5) as u16), clear::CurrentLine, self.cont_2.ctl_select_color());
                str_vec.push(buf_2);
            }
            if self.is_grep {
                let buf_desc_3 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 6) as u16), clear::CurrentLine, self.cont_3.buf_desc.clone());
                str_vec.push(buf_desc_3);
                let buf_3 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 7) as u16), clear::CurrentLine, self.cont_3.ctl_select_color());
                str_vec.push(buf_3);
            }
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("　　　　　　　　Prompt draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        self.draw_cur(&mut v);
        write!(out, "{}", v.concat()).unwrap();
        out.flush().unwrap();
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>) {
        if self.is_replace || self.is_grep {
            if self.buf_posi == PromptBufPosi::First {
                Log::ep("prom.cont_1.cur.disp_x", self.cont_1.cur.disp_x);
                str_vec.push(cursor::Goto(self.cont_1.cur.disp_x as u16, (self.disp_row_posi + 3) as u16).to_string());
            } else if self.buf_posi == PromptBufPosi::Second {
                Log::ep("prom.cont_2.cur.disp_x", self.cont_2.cur.disp_x);
                str_vec.push(cursor::Goto(self.cont_2.cur.disp_x as u16, (self.disp_row_posi + 5) as u16).to_string());
            } else if self.buf_posi == PromptBufPosi::Third {
                str_vec.push(cursor::Goto(self.cont_3.cur.disp_x as u16, (self.disp_row_posi + 7) as u16).to_string());
            }
        } else {
            str_vec.push(cursor::Goto(self.cont_1.cur.disp_x as u16, (self.disp_row_posi + self.disp_row_num - 1) as u16).to_string());
        }
    }

    pub fn cursor_down(&mut self) {
        Log::ep_s("◆　cursor_down");
        if self.is_replace {
            if self.buf_posi == PromptBufPosi::First {
                self.buf_posi = PromptBufPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            }
        } else if self.is_grep {
            if self.buf_posi == PromptBufPosi::First {
                self.buf_posi = PromptBufPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            } else if self.buf_posi == PromptBufPosi::Second {
                self.buf_posi = PromptBufPosi::Third;
                Prompt::set_cur(&self.cont_2, &mut self.cont_3)
            }
        }
    }

    pub fn cursor_up(&mut self) {
        Log::ep_s("cursor_up");

        if self.is_replace {
            if self.buf_posi == PromptBufPosi::Second {
                self.buf_posi = PromptBufPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            }
        } else if self.is_grep {
            if self.buf_posi == PromptBufPosi::Second {
                self.buf_posi = PromptBufPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            } else if self.buf_posi == PromptBufPosi::Third {
                self.buf_posi = PromptBufPosi::Second;
                Prompt::set_cur(&self.cont_3, &mut self.cont_2)
            }
        }
    }

    pub fn tab(&mut self, is_asc: bool) {
        Log::ep_s("tab");
        Log::ep("is_asc ", is_asc);

        if self.is_replace {
            match self.buf_posi {
                PromptBufPosi::First => self.cursor_down(),
                PromptBufPosi::Second => self.cursor_up(),
                _ => {}
            }
        } else if self.is_grep {
            if is_asc {
                match self.buf_posi {
                    PromptBufPosi::First | PromptBufPosi::Second => self.cursor_down(),
                    PromptBufPosi::Third => {
                        self.set_path();
                    }
                }
            } else {
                match self.buf_posi {
                    PromptBufPosi::First => {
                        self.buf_posi = PromptBufPosi::Third;
                        Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                    }
                    PromptBufPosi::Second => self.cursor_down(),
                    PromptBufPosi::Third => {}
                }
            }
        }
    }

    fn set_path(&mut self) {
        let mut dirs: Vec<String> = Vec::new();

        let input_path = self.cont_3.buf.iter().collect::<String>();
        Log::ep("             self.cont_3.buf", input_path.clone());

        if input_path.len() == 0 || self.cont_3.buf[self.cont_3.buf.len() - 1] == '/' {
            let mut prefix = "".to_string();
            self.tab_comp.index = 0;
            if self.cont_3.buf.len() == 0 {
                prefix = "/".to_string();
            } else {
                prefix = input_path.clone();
            }

            if let Ok(mut read_dir) = fs::read_dir(prefix) {
                while let Some(Ok(path)) = read_dir.next() {
                    if path.path().is_dir() {
                        dirs.push(path.path().display().to_string());
                    }
                }
            }
            dirs.sort();
            Log::ep("read_dir", dirs.clone().join(""));
            self.tab_comp.dirs = dirs;
        } else {
        }
        for (i, dir_str) in self.tab_comp.dirs.iter().enumerate() {
            Log::ep("dir_str", dir_str.clone());

            let v: Vec<(usize, &str)> = dir_str.match_indices(&input_path).collect();
            let mut char_vec: Vec<char> = vec![];

            // input_path 完全一致 or 入力無し
            if input_path == dir_str.to_string() || input_path.len() == 0 {
                Log::ep_s("input_path 完全一致");
                if !self.tab_comp.is_end {
                    char_vec = self.tab_comp.dirs[self.tab_comp.index].chars().collect();

                    if self.tab_comp.index + 1 >= self.tab_comp.dirs.len() {
                        self.tab_comp.index = 0;
                    } else {
                        self.tab_comp.index += 1;
                    }
                    break;
                }

            // input_path 部分一致
            } else if input_path.len() > 0 && v.len() > 0 {
                Log::ep_s("input_path 部分一致");
                char_vec = self.tab_comp.dirs[i].chars().collect();

            // 一致しない場合
            } else {
                Log::ep_s("一致しない");
                // pass
            }
            let (cur_x, width) = get_row_width(&char_vec, 0, char_vec.len());

            self.cont_3.buf = char_vec;
            self.cont_3.cur.x = cur_x;
            self.cont_3.cur.disp_x = width + 1;

            Log::ep("cont_3.buf", self.cont_3.buf.clone().iter().collect::<String>());
            Log::ep("cont_3.cur.x", self.cont_3.cur.x);
            Log::ep("cont_3.cur.disp_x", width + 1);
        }

        Log::ep("             set_path", &self.tab_comp);
    }

    fn set_cur(cont_org: &PromptCont, cont: &mut PromptCont) {
        cont.updown_x = cont_org.cur.disp_x;
        let (cur_x, width) = get_until_updown_x(&cont.buf, cont.updown_x);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }

    pub fn clear_sels(&mut self) {
        self.cont_1.sel.clear();
        self.cont_2.sel.clear();
        self.cont_3.sel.clear();
    }
}
