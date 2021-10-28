use crate::{
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*},
        _cfg::lang::lang_cfg::*,
        colors::*,
        global::*,
        log::*,
        util::*,
    },
    model::*,
};
use std::{cmp::min, usize};

impl PromptCont {
    pub fn new(cont_posi: Option<PromptContPosi>) -> Self {
        if let Some(prompt_cont_posi) = cont_posi {
            PromptCont::new_edit_type(prompt_cont_posi)
        } else {
            PromptCont::new_not_edit_type()
        }
    }
    fn new_not_edit_type() -> Self {
        PromptCont { ..PromptCont::default() }
    }

    fn new_edit_type(prompt_cont_posi: PromptContPosi) -> Self {
        PromptCont { posi: prompt_cont_posi, ..PromptCont::default() }
    }

    pub fn set_key_info(&mut self, keycmd: KeyCmd, keys: Keys, p_cmd: P_Cmd) {
        self.keycmd = keycmd;
        self.keys = keys;
        self.p_cmd = p_cmd;
    }

    pub fn get_draw_buf_str(&self) -> String {
        Log::debug_key("PromptCont.get_draw_buf_str");
        let ranges = self.sel.get_range();

        Log::debug("ranges", &ranges);

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
        str_vec.join("")
    }

    pub fn del_sel_range(&mut self) {
        let sel = self.sel.get_range();
        self.buf.drain(sel.sx..sel.ex);
        self.cur.disp_x = min(sel.s_disp_x, sel.e_disp_x);
        self.cur.x = min(sel.sx, sel.ex);
    }

    pub fn set_opt_case_sens(&mut self) {
        let key_str = Keybind::get_key_str(KeyCmd::Prom(P_Cmd::FindCaseSensitive));
        let key_case_sens = format!("{}{}:{}{}", Colors::get_default_fg(), &Lang::get().case_sens, Colors::get_msg_warning_fg(), key_str);
        let sx = get_str_width(&format!("{}:{}", &Lang::get().case_sens, key_str)) as u16;
        let opt_case_sens = PromptContOpt { key: key_case_sens, is_check: CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens, mouse_area: (sx, sx + 2) };
        self.opt_1 = opt_case_sens;
    }

    pub fn set_opt_regex(&mut self) {
        let key_str = Keybind::get_key_str(KeyCmd::Prom(P_Cmd::FindRegex));
        let key_regex = format!("{}{}:{}{}", Colors::get_default_fg(), &Lang::get().regex, Colors::get_msg_warning_fg(), key_str);

        // +2 is the space between options
        let sx = self.opt_1.mouse_area.1 + 2 + get_str_width(&format!("{}:{}", &Lang::get().regex, key_str)) as u16 + 1;

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

    pub fn set_cur_target(&mut self, x: usize) {
        let (cur_x, width) = get_row_width(&self.buf[..x], 0, false);
        self.cur.x = cur_x;
        self.cur.disp_x = width;
    }
}
