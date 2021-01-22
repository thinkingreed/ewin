use crate::{def::*, model::*, util::*};
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
                Log::ep("prom.cont_1.cur.x", self.cont_1.cur.x);
                str_vec.push(cursor::Goto(self.cont_1.cur.disp_x as u16, (self.disp_row_posi + 3) as u16).to_string());
            } else if self.buf_posi == PromptBufPosi::Second {
                Log::ep("prom.cont_2.cur.disp_x", self.cont_2.cur.disp_x);
                Log::ep("prom.cont_2.cur.x", self.cont_2.cur.x);
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
            match self.buf_posi {
                PromptBufPosi::First => {
                    if is_asc {
                        self.cursor_down();
                    } else {
                        self.buf_posi = PromptBufPosi::Third;
                        Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                    }
                }
                PromptBufPosi::Second => {
                    if is_asc {
                        self.cursor_down();
                    } else {
                        self.cursor_up();
                    }
                }
                PromptBufPosi::Third => {
                    self.cont_3.buf = self.get_tab_candidate(is_asc).chars().collect();
                    let (cur_x, width) = get_row_width(&self.cont_3.buf[..], false);
                    self.cont_3.cur.x = cur_x;
                    self.cont_3.cur.disp_x = width + 1;
                }
            }
        }
    }

    fn get_tab_candidate(&mut self, is_asc: bool) -> String {
        Log::ep_s("set_path");
        let mut target_path = self.cont_3.buf.iter().collect::<String>();

        // Search target dir
        let mut base_dir = ".".to_string();
        // cur.xまでの文字列対象
        let _ = target_path.split_off(self.cont_3.cur.x);
        let vec: Vec<(usize, &str)> = target_path.match_indices("/").collect();
        // "/"有り
        if vec.len() > 0 {
            let (base, _) = target_path.split_at(vec[vec.len() - 1].0 + 1);
            base_dir = base.to_string();
        }

        if self.tab_comp.dirs.len() == 0 {
            if let Ok(mut read_dir) = fs::read_dir(&base_dir) {
                while let Some(Ok(path)) = read_dir.next() {
                    if path.path().is_dir() {
                        let mut dir_str = path.path().display().to_string();
                        // Log::ep("dir_str", dir_str.clone());
                        let v: Vec<(usize, &str)> = dir_str.match_indices(target_path.as_str()).collect();
                        if v.len() > 0 {
                            // 表示用に"./"置換
                            if &base_dir == "." {
                                dir_str = dir_str.replace("./", "");
                            }
                            self.tab_comp.dirs.push(dir_str);
                        }
                    }
                }
            }
            self.tab_comp.dirs.sort();
        }

        Log::ep("read_dir", self.tab_comp.dirs.clone().join(" "));

        let mut cont_3_str: String = self.cont_3.buf.iter().collect::<String>();
        for candidate in &self.tab_comp.dirs {
            // One candidate
            if self.tab_comp.dirs.len() == 1 {
                Log::ep_s("　　One candidate");
                cont_3_str = format!("{}{}", candidate.to_string(), "/");
                self.clear_tab_comp();
                break;

            // Multiple candidates
            } else if self.tab_comp.dirs.len() > 1 {
                Log::ep_s("  Multi candidates");
                Log::ep("self.tab_comp.index", self.tab_comp.index);
                if is_asc && self.tab_comp.index >= self.tab_comp.dirs.len() - 1 || self.tab_comp.index == USIZE_UNDEFINED {
                    self.tab_comp.index = 0;
                } else if !is_asc && self.tab_comp.index == 0 {
                    self.tab_comp.index = self.tab_comp.dirs.len() - 1;
                } else {
                    self.tab_comp.index = if is_asc { self.tab_comp.index + 1 } else { self.tab_comp.index - 1 };
                }
                cont_3_str = self.tab_comp.dirs[self.tab_comp.index].clone();
                break;
            }
        }

        return cont_3_str;
    }

    pub fn clear_tab_comp(&mut self) {
        Log::ep_s("                  clear_tab_comp ");
        self.tab_comp.index = USIZE_UNDEFINED;
        self.tab_comp.dirs.clear();
    }

    fn set_cur(cont_org: &PromptCont, cont: &mut PromptCont) {
        cont.updown_x = cont_org.cur.disp_x;
        Log::ep("cont.updown_x", cont.updown_x);
        let (cur_x, width) = get_until_updown_x(&cont.buf, cont.updown_x);
        Log::ep("cur_x", cur_x);
        Log::ep("width", width);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }

    pub fn clear_sels(&mut self) {
        self.cont_1.sel.clear();
        self.cont_2.sel.clear();
        self.cont_3.sel.clear();
    }
}
