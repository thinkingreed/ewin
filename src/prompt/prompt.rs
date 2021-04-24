use crate::{colors::*, def::*, log::*, prompt::promptcont::promptcont::PromptContPosi::*, prompt::promptcont::promptcont::*, tab::TabState, util::*};
use crossterm::{cursor::*, event::*, terminal::ClearType::*, terminal::*};

use std::{
    fmt,
    io::{stdout, BufWriter, Write},
};

impl Prompt {
    pub fn new() -> Self {
        Prompt { ..Prompt::default() }
    }
    pub fn clear(&mut self) {
        //  self = &mut Prompt { disp_row_num: 0, ..Prompt::default() };
        Log::ep_s("　　　　　　　　Prompt clear");
        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.cont_1 = PromptCont::default();
        self.cont_2 = PromptCont::default();
        self.cont_3 = PromptCont::default();
        self.buf_posi = PromptContPosi::First;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, tab_state: &TabState) {
        Log::ep_s("　　　　　　　　Prompt draw");

        if self.cont_1.guide.len() > 0 {
            Prompt::set_draw_vec(str_vec, self.cont_1.guide_row_posi, &self.cont_1.guide);
            Prompt::set_draw_vec(str_vec, self.cont_1.key_desc_row_posi, &self.cont_1.key_desc);

            if tab_state.is_save_new_file || tab_state.is_move_line {
                Log::ep_s("is_move_line is_move_line is_move_line is_move_line is_move_line");
                Log::ep("self.cont_1.guide", &self.cont_1.guide);
                Log::ep("get_draw_buf_str", &self.cont_1.get_draw_buf_str());

                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            } else if tab_state.is_search {
                let o1 = &self.cont_1.opt_1;
                let o2 = &self.cont_1.opt_2;
                let opt_str = format!("{} {}{}{}  {} {}{}{}", o1.key, Colors::get_msg_warning_fg(), o1.get_check_str(), Colors::get_default_fg(), o2.key, Colors::get_msg_warning_fg(), o2.get_check_str(), Colors::get_default_fg(),);
                Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &opt_str);
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            }
            if tab_state.is_replace || tab_state.grep_info.is_grep {
                let o1 = &self.cont_1.opt_1;
                let o2 = &self.cont_1.opt_2;
                let opt_str = format!("{}{}  {}{}", o1.key, o1.get_check_str(), o2.key, o2.get_check_str());
                Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &opt_str);
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());

                if tab_state.grep_info.is_grep {
                    Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &self.cont_3.buf_desc);
                    Prompt::set_draw_vec(str_vec, self.cont_3.buf_row_posi, &self.cont_3.get_draw_buf_str());
                }
            }

            let out = stdout();
            let mut out = BufWriter::new(out.lock());
            let _ = out.write(&str_vec.concat().as_bytes());
            out.flush().unwrap();
            str_vec.clear();
        }
    }
    pub fn set_draw_vec(str_vec: &mut Vec<String>, posi: u16, cont: &String) {
        str_vec.push(format!("{}{}{}", MoveTo(0, posi), Clear(CurrentLine), cont));
    }
    pub fn draw_only<T: Write>(&mut self, out: &mut T, tab_state: &TabState) {
        Log::ep_s("　　　　　　　　Prompt draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v, tab_state);
        self.draw_cur(&mut v);
        let _ = out.write(&v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_cur_only<T: Write>(&mut self, out: &mut T) {
        let mut v: Vec<String> = vec![];
        self.draw_cur(&mut v);
        write!(out, "{}", v.concat()).unwrap();
        out.flush().unwrap();
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>) {
        let mut x = 0;
        let mut y = 0;

        Log::ep("self.buf_posi", &self.buf_posi);

        if self.buf_posi == PromptContPosi::First {
            x = self.cont_1.cur.disp_x;
            y = self.cont_1.buf_row_posi;
        } else if self.buf_posi == PromptContPosi::Second {
            x = self.cont_2.cur.disp_x;
            y = self.cont_2.buf_row_posi;
        } else if self.buf_posi == PromptContPosi::Third {
            x = self.cont_3.cur.disp_x;
            y = self.cont_3.buf_row_posi;
        }
        str_vec.push(MoveTo(x as u16, y as u16).to_string());
    }

    pub fn cursor_down(&mut self, tab_state: &TabState) {
        Log::ep_s("　cursor_down");
        if tab_state.is_replace {
            if self.buf_posi == PromptContPosi::First {
                self.buf_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            }
        } else if tab_state.grep_info.is_grep {
            if self.buf_posi == PromptContPosi::First {
                self.buf_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            } else if self.buf_posi == PromptContPosi::Second {
                self.buf_posi = PromptContPosi::Third;
                Prompt::set_cur(&self.cont_2, &mut self.cont_3)
            }
        }
    }

    pub fn cursor_up(&mut self, tab_state: &TabState) {
        Log::ep_s("cursor_up");

        if tab_state.is_replace {
            if self.buf_posi == PromptContPosi::Second {
                self.buf_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            }
        } else if tab_state.grep_info.is_grep {
            if self.buf_posi == PromptContPosi::Second {
                self.buf_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            } else if self.buf_posi == PromptContPosi::Third {
                self.buf_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_3, &mut self.cont_2)
            }
        }
    }

    pub fn set_cur(cont_org: &PromptCont, cont: &mut PromptCont) {
        cont.updown_x = cont_org.cur.disp_x;
        let (cur_x, width) = get_until_x(&cont.buf, cont.updown_x);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }

    pub fn clear_sels(&mut self) {
        self.cont_1.sel.clear();
        self.cont_2.sel.clear();
        self.cont_3.sel.clear();
    }

    pub fn ctrl_mouse(&mut self, x: u16, y: u16, is_mouse_left_down: bool) {
        Log::ep_s("　　　　　　　  PromptCont.ctrl_mouse");

        if y == self.cont_1.buf_row_posi {
            self.buf_posi = PromptContPosi::First;
            self.cont_1.ctrl_mouse(x, y, is_mouse_left_down);
        } else if y == self.cont_2.buf_row_posi {
            self.buf_posi = PromptContPosi::Second;
            self.cont_2.ctrl_mouse(x, y, is_mouse_left_down);
        } else if y == self.cont_3.buf_row_posi {
            self.buf_posi = PromptContPosi::Third;
            self.cont_3.ctrl_mouse(x, y, is_mouse_left_down);
        }
    }

    pub fn shift_right(&mut self) {
        match &self.buf_posi {
            First => self.cont_1.shift_right(),
            Second => self.cont_2.shift_right(),
            Third => self.cont_3.shift_right(),
        }
    }
    pub fn shift_left(&mut self) {
        match &self.buf_posi {
            First => self.cont_1.shift_left(),
            Second => self.cont_2.shift_left(),
            Third => self.cont_3.shift_left(),
        }
    }
    pub fn shift_home(&mut self) {
        match &self.buf_posi {
            First => self.cont_1.shift_home(),
            Second => self.cont_2.shift_home(),
            Third => self.cont_3.shift_home(),
        }
    }
    pub fn shift_end(&mut self) {
        match &self.buf_posi {
            First => self.cont_1.shift_end(),
            Second => self.cont_2.shift_end(),
            Third => self.cont_3.shift_end(),
        }
    }
    pub fn insert_char(&mut self, c: char, rnw: usize, state: &TabState) {
        match self.buf_posi {
            First => self.cont_1.insert_char(c, state.is_move_line, rnw),
            Second => self.cont_2.insert_char(c, state.is_move_line, rnw),
            Third => self.cont_3.insert_char(c, state.is_move_line, rnw),
        }
    }
    pub fn paste(&mut self, clipboard: &String) -> bool {
        match &self.buf_posi {
            First => self.cont_1.paste(clipboard),
            Second => self.cont_2.paste(clipboard),
            Third => self.cont_3.paste(clipboard),
        }
    }

    pub fn operation(&mut self, code: KeyCode) {
        match &self.buf_posi {
            First => self.cont_1.operation(code),
            Second => self.cont_2.operation(code),
            Third => self.cont_3.operation(code),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    // Prompt Content_Sequence number
    pub cont_1: PromptCont,
    pub cont_2: PromptCont,
    pub cont_3: PromptCont,
    pub buf_posi: PromptContPosi,
    pub tab_comp: TabComp,
    // cache
    pub cache_search_filenm: String,
    pub cache_search_folder: String,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            disp_row_num: 0,
            disp_row_posi: 0,
            disp_col_num: 0,
            //  is_grep_result: false,
            //  is_grep_result_cancel: false,
            cont_1: PromptCont::default(),
            cont_2: PromptCont::default(),
            cont_3: PromptCont::default(),
            buf_posi: PromptContPosi::First,
            tab_comp: TabComp::default(),
            cache_search_filenm: String::new(),
            cache_search_folder: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabComp {
    // List of complementary candidates
    pub dirs: Vec<String>,
    // List of complementary candidates index
    pub index: usize,
}
impl TabComp {}
impl Default for TabComp {
    fn default() -> Self {
        TabComp { index: USIZE_UNDEFINED, dirs: vec![] }
    }
}
impl fmt::Display for TabComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabComp index:{}, dirs:{},", self.index, self.dirs.join(" "),)
    }
}
