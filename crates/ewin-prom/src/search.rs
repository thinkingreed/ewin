use crate::{
    ewin_com::{_cfg::key::keycmd::*, colors::*, global::*},
    model::*,
};

impl Prompt {
    pub fn search(&mut self) {
        self.disp_row_num = 4;
        let mut cont = PromptCont::new(Some(PromptContPosi::First));
        cont.set_search();
        self.cont_1 = cont;
    }

    pub fn draw_search(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_search);
        self.key_desc = format!(
            "{}{}:{}{}  {}{}:{}{}  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &LANG.search_bottom,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::FindNext)),
            Colors::get_default_fg(),
            &LANG.search_top,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::FindBack)),
            Colors::get_default_fg(),
            &LANG.close,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            Colors::get_default_fg(),
        );

        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
