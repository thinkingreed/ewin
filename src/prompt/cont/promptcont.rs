use crate::{
    _cfg::keys::{KeyCmd, Keybind},
    colors::*,
    global::*,
    log::*,
    model::*,
    prompt::choice::{Choice, Choices},
    tab::Tab,
    util::*,
};
use crossterm::cursor::MoveTo;
use std::{cmp::min, collections::HashMap, io::Write, usize};

impl PromptCont {
    pub fn new_not_edit_type(tab: &mut Tab) -> Self {
        PromptCont { disp_row_posi: tab.prom.disp_row_posi, keycmd: tab.editor.keycmd, ..PromptCont::default() }
    }

    pub fn new_edit_type(tab: &mut Tab, prompt_cont_posi: PromptContPosi) -> Self {
        PromptCont { disp_row_posi: tab.prom.disp_row_posi, keycmd: tab.editor.keycmd, posi: prompt_cont_posi, ..PromptCont::default() }
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
        let key_str = Keybind::get_key_str(KeyCmd::FindCaseSensitive);
        let key_case_sens = format!("{}{}:{}{}", Colors::get_default_fg(), &LANG.case_sens, Colors::get_msg_warning_fg(), key_str);
        let sx = get_str_width(&format!("{}:{}", &LANG.case_sens, key_str)) as u16;
        let opt_case_sens = PromptContOpt { key: key_case_sens, is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens, mouse_area: (sx, sx + 3) };
        self.opt_1 = opt_case_sens;
    }

    pub fn set_opt_regex(&mut self) {
        let key_str = Keybind::get_key_str(KeyCmd::FindRegex);
        let key_regex = format!("{}{}:{}{}", Colors::get_default_fg(), &LANG.regex, Colors::get_msg_warning_fg(), key_str);

        // +2 is the space between options
        let sx = self.opt_1.mouse_area.1 + 3 + get_str_width(&format!("{}:{}", &LANG.regex, key_str)) as u16 + 1;

        let opt_regex = PromptContOpt { key: key_regex, is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex, mouse_area: (sx, sx + 3) };
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

    pub fn ctrl_mouse(&mut self, x: usize, y: usize, is_left_down: bool) {
        Log::debug_key("PromptCont.ctrl_mouse");

        if y as u16 != self.buf_row_posi {
            return;
        }
        let (cur_x, width) = get_until_x(&self.buf, x as usize);
        self.cur.x = cur_x;
        self.cur.disp_x = width;

        if is_left_down {
            self.sel.clear_prompt();
            self.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
            self.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
        } else {
            self.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
        }
    }
    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("PromptCont.draw_only");
        let _ = out.write(&self.get_draw_buf_str().as_bytes());
        out.flush().unwrap();
    }
    /*
     * choice
     */
    pub fn left_down_choice(&mut self, y: u16, x: u16) {
        let (y, x) = (y as usize, x as usize);
        for (_, choices) in self.choices_map.iter_mut() {
            if choices.is_show {
                for (y_idx, vec) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in vec.iter().enumerate() {
                        if item.area.0 == y && item.area.1 <= x && x <= item.area.2 {
                            choices.vec_y = y_idx;
                            choices.vec_x = x_idx;
                        }
                    }
                }
            }
        }
    }
    pub fn draw_choice_cur(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_cur_promcont");

        let (mut y, mut x) = (0, 0);
        'outer: for (_, choices) in self.choices_map.iter() {
            if choices.is_show {
                for (y_idx, vec) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in vec.iter().enumerate() {
                        if choices.vec_y == y_idx && choices.vec_x == x_idx {
                            y = self.buf_row_posi + y_idx as u16;
                            x = item.area.1;
                            break 'outer;
                        }
                    }
                }
            }
        }
        Log::debug("x", &x);
        Log::debug("y", &y);

        str_vec.push(MoveTo(x as u16, y as u16).to_string());
    }

    pub fn get_choices(&mut self) -> Option<&mut Choices> {
        for (_, choices) in self.choices_map.iter_mut() {
            if choices.is_show {
                return Some(choices);
            }
        }
        // dummy
        return None;
    }

    pub fn get_choice(&self) -> Choice {
        let dummy = Choice::new(&"".to_string());
        for (_, choices) in self.choices_map.iter() {
            if choices.is_show {
                for (y_idx, v) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in v.iter().enumerate() {
                        if choices.vec_y == y_idx && choices.vec_x == x_idx {
                            return item.clone();
                        }
                    }
                }
            }
        }
        return dummy;
    }
    pub fn set_cur_target(&mut self, x: usize) {
        let (cur_x, width) = get_row_width(&self.buf[..x], 0, false);
        self.cur.x = cur_x;
        self.cur.disp_x = width;
    }
}

#[derive(Debug, Clone)]
pub struct PromptCont {
    pub keycmd: KeyCmd,
    pub disp_row_posi: u16,
    pub row_len: u16,
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
    pub history: History,
    // For list display
    pub file_list_vec: Vec<File>,
    // <(Parent choices posi y, Parent choices posi y), Self Choices>
    pub choices_map: HashMap<(usize, usize), Choices>,
}

impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            keycmd: KeyCmd::Null,
            disp_row_posi: 0,
            row_len: 0,
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
            history: History::default(),
            sel: SelRange::default(),
            file_list_vec: vec![],
            choices_map: HashMap::new(),
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
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Debug, Clone)]
pub enum PromptContPosi {
    First,
    Second,
    Third,
    Fourth,
}
