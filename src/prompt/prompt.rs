use crate::{def::*, log::*, prompt::promptcont::promptcont::*, util::*};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::{
    fmt,
    io::{stdout, BufWriter, Write},
};
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
    // fn clear not clear
    pub is_change: bool,
    pub is_grep_result: bool,
    pub is_grep_result_cancel: bool,
    // *************
    pub is_close_confirm: bool,
    pub is_save_new_file: bool,
    pub is_search: bool,
    pub is_replace: bool,
    pub is_grep: bool,
    // grep result stdout/stderr output complete flg
    pub is_grep_stdout: bool,
    pub is_grep_stderr: bool,
    pub is_key_record: bool,
    pub is_key_record_exec: bool,
    pub is_key_record_exec_draw: bool,
    pub is_move_line: bool,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            disp_row_num: 0,
            disp_row_posi: 0,
            disp_col_num: 0,
            is_change: false,
            is_grep_result: false,
            is_grep_result_cancel: false,
            cont_1: PromptCont::default(),
            cont_2: PromptCont::default(),
            cont_3: PromptCont::default(),
            buf_posi: PromptContPosi::First,
            tab_comp: TabComp::default(),
            cache_search_filenm: String::new(),
            cache_search_folder: String::new(),
            is_close_confirm: false,
            is_save_new_file: false,
            is_search: false,
            is_replace: false,
            is_grep: false,
            is_grep_stdout: false,
            is_grep_stderr: false,
            is_key_record: false,
            is_key_record_exec: false,
            is_key_record_exec_draw: false,
            is_move_line: false,
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
        self.is_close_confirm = false;
        self.is_save_new_file = false;
        self.is_search = false;
        self.is_replace = false;
        self.is_grep = false;
        self.is_grep_stdout = false;
        self.is_grep_stderr = false;
        self.is_move_line = false;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("　　　　　　　　Prompt draw");

        if self.cont_1.guide.len() > 0 {
            Prompt::set_draw_vec(str_vec, self.cont_1.guide_row_posi, &self.cont_1.guide);
            Prompt::set_draw_vec(str_vec, self.cont_1.key_desc_row_posi, &self.cont_1.key_desc);

            if self.is_save_new_file || self.is_move_line {
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            } else if self.is_search {
                let opt_1 = &self.cont_1.opt_1;
                let opt_2 = &self.cont_1.opt_2;
                let opt_str = format!("{}{}  {}{}", opt_1.key, opt_1.get_check_str(), opt_2.key, opt_2.get_check_str());
                Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &opt_str);
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            }
            if self.is_replace || self.is_grep {
                let opt_1 = &self.cont_1.opt_1;
                let opt_2 = &self.cont_1.opt_2;
                let opt_str = format!("{}{}  {}{}", opt_1.key, opt_1.get_check_str(), opt_2.key, opt_2.get_check_str());
                Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &opt_str);
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());

                if self.is_grep {
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
    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("　　　　　　　　Prompt draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
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
        if self.is_replace || self.is_grep {
            if self.buf_posi == PromptContPosi::First {
                str_vec.push(MoveTo((self.cont_1.cur.disp_x - 1) as u16, (self.disp_row_posi + 3) as u16).to_string());
            } else if self.buf_posi == PromptContPosi::Second {
                str_vec.push(MoveTo((self.cont_2.cur.disp_x - 1) as u16, (self.disp_row_posi + 5) as u16).to_string());
            } else if self.buf_posi == PromptContPosi::Third {
                str_vec.push(MoveTo((self.cont_3.cur.disp_x - 1) as u16, (self.disp_row_posi + 7) as u16).to_string());
            }
        } else {
            str_vec.push(MoveTo((self.cont_1.cur.disp_x - 1) as u16, (self.disp_row_posi + self.disp_row_num - 2) as u16).to_string());
        }
    }

    pub fn cursor_down(&mut self) {
        Log::ep_s("◆　cursor_down");
        if self.is_replace {
            if self.buf_posi == PromptContPosi::First {
                self.buf_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            }
        } else if self.is_grep {
            if self.buf_posi == PromptContPosi::First {
                self.buf_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            } else if self.buf_posi == PromptContPosi::Second {
                self.buf_posi = PromptContPosi::Third;
                Prompt::set_cur(&self.cont_2, &mut self.cont_3)
            }
        }
    }

    pub fn cursor_up(&mut self) {
        Log::ep_s("cursor_up");

        if self.is_replace {
            if self.buf_posi == PromptContPosi::Second {
                self.buf_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            }
        } else if self.is_grep {
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
        Log::ep("cont.updown_x", &cont.updown_x);
        let (cur_x, width) = get_until_updown_x(&cont.buf, cont.updown_x);
        Log::ep("cur_x", &cur_x);
        Log::ep("width", &width);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }

    pub fn clear_sels(&mut self) {
        self.cont_1.sel.clear();
        self.cont_2.sel.clear();
        self.cont_3.sel.clear();
    }

    pub fn ctrl_mouse<T: Write>(&mut self, x: u16, y: u16, is_mouse_left_down: bool, out: &mut T) {
        Log::ep_s("　　　　　　　  PromptCont.ctrl_mouse");

        if y == self.cont_1.buf_row_posi {
            self.buf_posi = PromptContPosi::First;
            self.cont_1.ctrl_mouse(x, y, is_mouse_left_down, out);
        } else if y == self.cont_2.buf_row_posi {
            self.buf_posi = PromptContPosi::Second;
            self.cont_2.ctrl_mouse(x, y, is_mouse_left_down, out);
        } else if y == self.cont_3.buf_row_posi {
            self.buf_posi = PromptContPosi::Third;
            self.cont_3.ctrl_mouse(x, y, is_mouse_left_down, out);
        }
    }
}
