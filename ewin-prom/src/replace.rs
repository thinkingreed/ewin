use crate::{
    ewin_core::{_cfg::key::keycmd::*, colors::*, global::*, log::*},
    model::*,
};

impl Prompt {
    pub fn replace(&mut self) {
        self.disp_row_num = 7;
        let mut cont_1 = PromptCont::new(Some(PromptContPosi::First));
        let mut cont_2 = PromptCont::new(Some(PromptContPosi::Second));
        cont_1.set_replace();
        cont_2.set_replace();
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
    }

    pub fn draw_replace(&self, str_vec: &mut Vec<String>) {
        Log::debug_s("111111111111111111111111");

        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn set_replace(&mut self) {
        if self.posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_replace);
            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}Tab ↓↑  {}{}:{}{}",
                Colors::get_default_fg(),
                &LANG.all_replace,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
                Colors::get_default_fg(),
                &LANG.move_setting_location,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            );
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.search_str,);
        } else {
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.replace_char,);
        }
        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
