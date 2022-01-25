use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*},
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
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc_vec.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc_vec);
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn set_replace(&mut self) {
        if self.posi == PromptContPosi::First {
            self.guide_vec.push(format!("{}{}", Colors::get_msg_highlight_fg(), &Lang::get().set_replace));
            self.key_desc_vec.push(format!(
                "{}{}:{}{}  {}{}:{}Tab ↓↑  {}{}:{}{}",
                Colors::get_default_fg(),
                &Lang::get().all_replace,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
                Colors::get_default_fg(),
                &Lang::get().move_setting_location,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &Lang::get().close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            ));
            self.buf_desc_vec.push(format!("{}{}", Colors::get_default_fg(), &Lang::get().search_str,));
        } else {
            self.buf_desc_vec.push(format!("{}{}", Colors::get_default_fg(), &Lang::get().replace_char,));
        }
        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
