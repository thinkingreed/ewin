use crate::{colors::*, global::*, log::*, model::*};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct PromptCont {
    pub guide: String,
    pub opt_1: PromptContOpt,
    pub opt_2: PromptContOpt,
    pub key_desc: String,
    pub buf_desc: String,
    pub buf: Vec<char>,
    pub cur: Cur,
    pub updown_x: usize,
    pub sel: SelRange,
}
impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            guide: String::new(),
            key_desc: String::new(),
            opt_1: PromptContOpt::default(),
            opt_2: PromptContOpt::default(),
            buf_desc: String::new(),
            buf: vec![],
            cur: Cur::default(),
            updown_x: 0,
            sel: SelRange::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PromptContOpt {
    pub key: String,
    pub is_check: bool,
}
impl Default for PromptContOpt {
    fn default() -> Self {
        PromptContOpt { key: String::new(), is_check: false }
    }
}
impl PromptContOpt {
    pub fn get_check_str(&self) -> String {
        let str = if self.is_check { "[*]" } else { "[ ]" };
        return str.to_string();
    }
    pub fn toggle_check(&mut self) {
        match self.is_check {
            true => self.is_check = false,
            false => self.is_check = true,
        }
    }
}

#[derive(PartialEq)]
pub enum PromptBufPosi {
    First,
    Second,
    Third,
}

impl PromptCont {
    pub fn new() -> Self {
        PromptCont { ..PromptCont::default() }
    }
    pub fn ctl_select_color(&mut self) -> String {
        // Log::ep_s("                          Prompt.ctl_select_color");
        let ranges = self.sel.get_range();

        let mut str_vec: Vec<String> = vec![];
        for (i, c) in self.buf.iter().enumerate() {
            if ranges.sx <= i && i < ranges.ex {
                Colors::set_select_color(&mut str_vec);
                str_vec.push(c.to_string())
            } else {
                Colors::set_text_color(&mut str_vec);
                str_vec.push(c.to_string())
            }
        }
        Colors::set_text_color(&mut str_vec);

        return str_vec.join("");
    }

    pub fn del_sel_range(&mut self) {
        Log::ep_s("　　　　　　　  del_sel_range");
        let sel = self.sel.get_range();
        Log::ep("sel", &sel);
        self.buf.drain(sel.sx..sel.ex);
        self.cur.disp_x = min(sel.s_disp_x, sel.e_disp_x);
        self.cur.x = min(sel.sx, sel.ex);
    }

    pub fn set_opt_case_sens(&mut self) {
        let key_case_sens = format!("{}{}:{}Alt + c{}", Colors::get_default_fg(), &LANG.case_sens, Colors::get_msg_warning_fg(), Colors::get_default_fg(),);
        let opt_case_sens = PromptContOpt {
            key: key_case_sens,
            is_check: CFG.get().unwrap().lock().unwrap().general.editor.search.case_sens,
        };
        self.opt_1 = opt_case_sens;
    }

    pub fn set_opt_regex(&mut self) {
        let key_regex = format!("{}{}:{}Alt + r{}", Colors::get_default_fg(), &LANG.regex, Colors::get_msg_warning_fg(), Colors::get_default_fg(),);
        let opt_regex = PromptContOpt {
            key: key_regex,
            is_check: CFG.get().unwrap().lock().unwrap().general.editor.search.regex,
        };
        self.opt_2 = opt_regex;
    }
}
