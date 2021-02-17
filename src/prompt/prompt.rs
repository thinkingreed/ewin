use crate::{def::*, log::*, prompt::promptcont::promptcont::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::{fmt, io::Write};
pub struct Prompt {
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    // Prompt Content_Sequence number
    pub cont_1: PromptCont,
    pub cont_2: PromptCont,
    pub cont_3: PromptCont,
    pub buf_posi: PromptBufPosi,
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
            buf_posi: PromptBufPosi::First,
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
        self.buf_posi = PromptBufPosi::First;
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
            let cont_desc = format!("{}{}{}", MoveTo(0, (self.disp_row_posi - 1) as u16), Clear(ClearType::CurrentLine), self.cont_1.guide.clone());
            str_vec.push(cont_desc);

            let key_desc = format!("{}{}{}", MoveTo(0, (self.disp_row_posi) as u16), Clear(ClearType::CurrentLine), self.cont_1.key_desc.clone());
            str_vec.push(key_desc);

            if self.is_search {
                let cont_opt = &self.cont_1.opt_1;
                let str = format!("{}{}{}{}", MoveTo(0, (self.disp_row_posi + 1) as u16), Clear(ClearType::CurrentLine), cont_opt.key, cont_opt.get_check_str());
                str_vec.push(str);
                let buf = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 2) as u16), Clear(ClearType::CurrentLine), self.cont_1.ctl_select_color());
                str_vec.push(buf);
            }

            if self.is_save_new_file || self.is_move_line {
                let buf = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 1) as u16), Clear(ClearType::CurrentLine), self.cont_1.ctl_select_color());
                str_vec.push(buf);
            }
            if self.is_replace || self.is_grep {
                let buf_desc_1 = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 1) as u16), Clear(ClearType::CurrentLine), self.cont_1.buf_desc.clone());
                str_vec.push(buf_desc_1);
                let buf_1 = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 2) as u16), Clear(ClearType::CurrentLine), self.cont_1.ctl_select_color());
                str_vec.push(buf_1);
                let buf_desc_2 = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 3) as u16), Clear(ClearType::CurrentLine), self.cont_2.buf_desc.clone());
                str_vec.push(buf_desc_2);
                let buf_2 = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 4) as u16), Clear(ClearType::CurrentLine), self.cont_2.ctl_select_color());
                str_vec.push(buf_2);
            }
            if self.is_grep {
                let buf_desc_3 = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 5) as u16), Clear(ClearType::CurrentLine), self.cont_3.buf_desc.clone());
                str_vec.push(buf_desc_3);
                let buf_3 = format!("{}{}{}", MoveTo(0, (self.disp_row_posi + 6) as u16), Clear(ClearType::CurrentLine), self.cont_3.ctl_select_color());
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

    pub fn draw_cur_only<T: Write>(&mut self, out: &mut T) {
        let mut v: Vec<String> = vec![];
        self.draw_cur(&mut v);
        write!(out, "{}", v.concat()).unwrap();
        out.flush().unwrap();
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>) {
        if self.is_replace || self.is_grep {
            if self.buf_posi == PromptBufPosi::First {
                str_vec.push(MoveTo((self.cont_1.cur.disp_x - 1) as u16, (self.disp_row_posi + 2) as u16).to_string());
            } else if self.buf_posi == PromptBufPosi::Second {
                str_vec.push(MoveTo((self.cont_2.cur.disp_x - 1) as u16, (self.disp_row_posi + 4) as u16).to_string());
            } else if self.buf_posi == PromptBufPosi::Third {
                str_vec.push(MoveTo((self.cont_3.cur.disp_x - 1) as u16, (self.disp_row_posi + 6) as u16).to_string());
            }
        } else {
            str_vec.push(MoveTo((self.cont_1.cur.disp_x - 1) as u16, (self.disp_row_posi + self.disp_row_num - 2) as u16).to_string());
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
}
