use crossterm::cursor::MoveTo;

use crate::{colors::*, global::*, log::*, model::*, util::*};
use std::{cmp::min, io::Write};

impl PromptCont {
    pub fn new_not_edit_type(disp_row_posi: u16) -> Self {
        PromptCont { disp_row_posi, ..PromptCont::default() }
    }

    pub fn new_edit_type(disp_row_posi: u16, prompt_cont_posi: PromptContPosi) -> Self {
        PromptCont { disp_row_posi, posi: prompt_cont_posi, ..PromptCont::default() }
    }

    pub fn get_draw_buf_str(&self) -> String {
        // Log::ep_s("                          Prompt.ctl_select_color");
        let ranges = self.sel.get_range();

        let mut str_vec: Vec<String> = vec![];
        for (i, c) in self.buf.iter().enumerate() {
            if ranges.sx <= i && i < ranges.ex {
                Colors::set_select_color(&mut str_vec);
            } else {
                Colors::set_text_color(&mut str_vec);
            }
            str_vec.push(c.to_string())
        }
        Colors::set_text_color(&mut str_vec);
        return str_vec.join("");
    }

    pub fn del_sel_range(&mut self) {
        let sel = self.sel.get_range();
        self.buf.drain(sel.sx..sel.ex);
        self.cur.disp_x = min(sel.disp_x_s, sel.disp_x_e);
        self.cur.x = min(sel.sx, sel.ex);
    }

    pub fn set_opt_case_sens(&mut self) {
        let key_case_sens = format!("{}{}:{}Alt + c", Colors::get_default_fg(), &LANG.case_sens, Colors::get_msg_warning_fg());
        let sx = get_str_width(&format!("{}:Alt + c", &LANG.case_sens)) as u16;
        let opt_case_sens = PromptContOpt {
            key: key_case_sens,
            is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens,
            mouse_area: (sx, sx + 3),
        };
        self.opt_1 = opt_case_sens;
    }

    pub fn set_opt_regex(&mut self) {
        let key_regex = format!("{}{}:{}Alt + r", Colors::get_default_fg(), &LANG.regex, Colors::get_msg_warning_fg());

        // +2 is the space between options
        let sx = self.opt_1.mouse_area.1 + 3 + get_str_width(&format!("{}:Alt + r", &LANG.regex)) as u16 + 1;

        let opt_regex = PromptContOpt {
            key: key_regex,
            is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex,
            mouse_area: (sx, sx + 3),
        };
        self.opt_2 = opt_regex;
    }

    pub fn change_opt_case_sens(&mut self) {
        self.opt_1.toggle_check();
        CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.case_sens = self.opt_1.is_check).unwrap();
    }

    pub fn change_opt_regex(&mut self) {
        self.opt_2.toggle_check();
        CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.regex = self.opt_2.is_check).unwrap();
    }

    pub fn ctrl_mouse(&mut self, x: u16, y: u16, is_mouse_left_down: bool) {
        Log::debug_s("              PromptCont.ctrl_mouse");
        if y != self.buf_row_posi {
            return;
        }
        let (cur_x, width) = get_until_x(&self.buf, x as usize);
        self.cur.x = cur_x;
        self.cur.disp_x = width;

        if is_mouse_left_down {
            self.sel.clear_prompt();
            self.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
            self.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
        } else {
            self.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
        }
    }
    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_s("              PromptCont.draw_only");
        let _ = out.write(&self.get_draw_buf_str().as_bytes());
        out.flush().unwrap();
    }
    /*
     * choice
     */
    pub fn left_down_choice(&mut self, y: u16, x: usize) {
        if y == self.buf_row_posi {
            let mut total_idx = 0;
            for vec in &self.choices.vec {
                for item in vec {
                    if item.area.0 <= x && x <= item.area.1 {
                        self.choices.idx = total_idx;
                    }
                    total_idx += 1;
                }
            }
        }
    }
    pub fn draw_cur_promcont(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_cur_promcont");

        let (mut y, mut x) = (0, 0);
        let mut total_idx = 0;
        for (row_idx, vec) in self.choices.vec.iter().enumerate() {
            for item in vec {
                if self.choices.idx == total_idx {
                    y = self.buf_row_posi + row_idx as u16;
                    x = item.area.0;
                    break;
                }
                total_idx += 1;
            }
        }
        Log::debug("x", &x);
        Log::debug("y", &y);

        str_vec.push(MoveTo(x as u16, y as u16).to_string());
    }
}

#[derive(Debug, Clone)]
pub struct PromptCont {
    pub disp_row_posi: u16,
    pub posi: PromptContPosi,
    pub guide_row_posi: u16,
    pub key_desc_row_posi: u16,
    pub opt_row_posi: u16,
    pub buf_desc_row_posi: u16,
    pub buf_row_posi: u16,
    pub cur: Cur,
    pub sel: SelRange,
    pub guide: String,
    pub opt_1: PromptContOpt,
    pub opt_2: PromptContOpt,
    pub key_desc: String,
    pub buf_desc: String,
    // For 1-line input
    pub buf: Vec<char>,
    pub updown_x: usize,
    // For list display
    pub file_list_vec: Vec<File>,
    // For multi-line selection items
    pub choices: Choices,
}

impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            disp_row_posi: 0,
            posi: PromptContPosi::First,
            guide_row_posi: 0,
            key_desc_row_posi: 0,
            opt_row_posi: 0,
            buf_desc_row_posi: 0,
            buf_row_posi: 0,
            guide: String::new(),
            key_desc: String::new(),
            opt_1: PromptContOpt::default(),
            opt_2: PromptContOpt::default(),
            buf_desc: String::new(),
            buf: vec![],
            cur: Cur::default(),
            updown_x: 0,
            sel: SelRange::default(),
            file_list_vec: vec![],
            choices: Choices::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PromptContOpt {
    pub key: String,
    pub is_check: bool,
    pub mouse_area: (u16, u16),
}
impl Default for PromptContOpt {
    fn default() -> Self {
        PromptContOpt { key: String::new(), is_check: false, mouse_area: (0, 0) }
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
#[derive(PartialEq, Debug, Clone)]
pub enum PromptContPosi {
    First,
    Second,
    Third,
    Fourth,
}
