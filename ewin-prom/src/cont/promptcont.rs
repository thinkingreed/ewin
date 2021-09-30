use crate::{
    ewin_core::{_cfg::key::keycmd::*, colors::*, global::*, log::*, util::*},
    model::*,
    prompt::choice::*,
};
use crossterm::cursor::MoveTo;
use std::{cmp::min, io::Write, usize};

impl PromptCont {
    pub fn new(cont_posi: Option<PromptContPosi>) -> Self {
        if let Some(prompt_cont_posi) = cont_posi {
            return PromptCont::new_edit_type(prompt_cont_posi);
        } else {
            return PromptCont::new_not_edit_type();
        }
    }
    fn new_not_edit_type() -> Self {
        PromptCont { ..PromptCont::default() }
    }

    fn new_edit_type(prompt_cont_posi: PromptContPosi) -> Self {
        PromptCont { posi: prompt_cont_posi, ..PromptCont::default() }
    }

    /*
    pub fn new(keycmd: KeyCmd, cont_posi: Option<PromptContPosi>) -> Self {
        let p_cmd = match &keycmd {
            KeyCmd::Prom(p_keycmd) => p_keycmd.clone(),
            _ => P_Cmd::Null,
        };
        if let Some(prompt_cont_posi) = cont_posi {
            return PromptCont::new_edit_type(keycmd, p_cmd, prompt_cont_posi);
        } else {
            return PromptCont::new_not_edit_type(keycmd, p_cmd);
        }
    }

    fn new_not_edit_type(keycmd: KeyCmd, p_cmd: P_Cmd) -> Self {
        PromptCont { keycmd, p_cmd, ..PromptCont::default() }
    }

    fn new_edit_type(keycmd: KeyCmd, p_cmd: P_Cmd, prompt_cont_posi: PromptContPosi) -> Self {
        PromptCont { keycmd, p_cmd, posi: prompt_cont_posi, ..PromptCont::default() }
    }
     */

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
        self.cur.disp_x = min(sel.s_disp_x, sel.e_disp_x);
        self.cur.x = min(sel.sx, sel.ex);
    }

    pub fn set_opt_case_sens(&mut self) {
        let key_str = Keybind::get_key_str(KeyCmd::Prom(P_Cmd::FindCaseSensitive));
        let key_case_sens = format!("{}{}:{}{}", Colors::get_default_fg(), &LANG.case_sens, Colors::get_msg_warning_fg(), key_str);
        let sx = get_str_width(&format!("{}:{}", &LANG.case_sens, key_str)) as u16;
        let opt_case_sens = PromptContOpt { key: key_case_sens, is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens, mouse_area: (sx, sx + 2) };
        self.opt_1 = opt_case_sens;
    }

    pub fn set_opt_regex(&mut self) {
        let key_str = Keybind::get_key_str(KeyCmd::Prom(P_Cmd::FindRegex));
        let key_regex = format!("{}{}:{}{}", Colors::get_default_fg(), &LANG.regex, Colors::get_msg_warning_fg(), key_str);

        // +2 is the space between options
        let sx = self.opt_1.mouse_area.1 + 2 + get_str_width(&format!("{}:{}", &LANG.regex, key_str)) as u16 + 1;

        let opt_regex = PromptContOpt { key: key_regex, is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex, mouse_area: (sx, sx + 2) };
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
    pub fn left_down_choice(&mut self, y: u16, x: u16) -> bool {
        Log::debug_key("left_down_choice_menu");

        let (y, x) = (y as usize, x as usize);
        for (_, choices) in self.choices_map.iter_mut() {
            if choices.is_show {
                for (y_idx, vec) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in vec.iter().enumerate() {
                        Log::debug("item", &item);
                        if item.area.0 == y && item.area.1 <= x && x <= item.area.2 {
                            Log::debug_key("item.area.0");
                            choices.vec_y = y_idx;
                            choices.vec_x = x_idx;
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
    pub fn draw_choice_cur(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_choice_cur");

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
