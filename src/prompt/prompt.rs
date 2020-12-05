use crate::_cfg::lang::cfg::LangCfg;
use crate::model::*;
use crate::util::*;
use crossterm::event::KeyCode;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent};
use std::cmp::{max, min};
use std::io::Write;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Prompt {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        // Log::ep_s("　　　　　　　　Prompt draw");
        if self.cont_1.guide.len() > 0 {
            let cont_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.cont_1.guide.clone());
            str_vec.push(cont_desc);

            let key_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 1) as u16), clear::CurrentLine, self.cont_1.key_desc.clone());
            str_vec.push(key_desc);

            if self.is_save_new_file || self.is_search {
                let buf = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont_1.buf.iter().collect::<String>());
                str_vec.push(buf);
            }

            if self.is_replace || self.is_grep {
                let buf_desc_1 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont_1.buf_desc.clone());
                str_vec.push(buf_desc_1);
                let buf_1 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 3) as u16), clear::CurrentLine, self.cont_1.buf.iter().collect::<String>());
                str_vec.push(buf_1);
                let buf_desc_2 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 4) as u16), clear::CurrentLine, self.cont_2.buf_desc.clone());
                str_vec.push(buf_desc_2);
                let buf_2 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 5) as u16), clear::CurrentLine, self.cont_2.buf.iter().collect::<String>());
                str_vec.push(buf_2);
            }
            if self.is_grep {
                let buf_desc_3 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 6) as u16), clear::CurrentLine, self.cont_3.buf_desc.clone());
                str_vec.push(buf_desc_3);
                let buf_3 = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 7) as u16), clear::CurrentLine, self.cont_3.buf.iter().collect::<String>());
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
            if self.buf_posi == PromptBufPosi::First {
                self.cursor_down();
            } else {
                self.cursor_up();
            }
        } else if self.is_grep {
            if is_asc {
                if self.buf_posi == PromptBufPosi::First || self.buf_posi == PromptBufPosi::Second {
                    self.cursor_down();
                // PromptBufPosi::Third
                } else {
                    self.buf_posi = PromptBufPosi::First;
                    Prompt::set_cur(&self.cont_3, &mut self.cont_1);
                }
            } else {
                if self.buf_posi == PromptBufPosi::Second || self.buf_posi == PromptBufPosi::Third {
                    self.cursor_up();
                // PromptBufPosi::First
                } else {
                    self.buf_posi = PromptBufPosi::Third;
                    Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                }
            }
        }
    }
    fn set_cur(cont_org: &PromptCont, cont: &mut PromptCont) {
        cont.updown_x = cont_org.cur.disp_x;
        let (cur_x, width) = get_until_updown_x(&cont.buf, cont.updown_x);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }
}

impl PromptCont {
    pub fn new(lang_cfg: LangCfg) -> Self {
        PromptCont { lang: lang_cfg, ..PromptCont::default() }
    }

    pub fn insert_char(&mut self, c: char) {
        self.buf.insert(self.cur.x, c);
        self.cur.disp_x += c.width().unwrap_or(0);
        self.cur.x += 1;
    }

    pub fn cursor_left(&mut self) {
        if self.cur.x != 0 {
            self.cur.x = max(self.cur.x - 1, 0);
            self.cur.disp_x -= get_cur_x_width(&self.buf, self.cur.x);
        }
    }
    pub fn cursor_right(&mut self) {
        if self.cur.x < self.buf.len() {
            self.cur.disp_x += get_cur_x_width(&self.buf, self.cur.x);
            self.cur.x = min(self.cur.x + 1, self.buf.len());
        }
    }
    pub fn delete(&mut self) {
        if self.cur.x < self.buf.len() {
            self.buf.remove(self.cur.x);
        }
    }
    pub fn backspace(&mut self) {
        if self.cur.x > 0 {
            self.cur.x -= 1;
            self.cur.disp_x -= self.buf[self.cur.x].width().unwrap_or(0);
            self.buf.remove(self.cur.x);
        }
    }

    pub fn edit(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Delete => self.delete(),
            KeyCode::Backspace => self.backspace(),
            _ => {}
        }
    }
}
