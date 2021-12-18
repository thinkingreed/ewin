use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*},
    model::*,
};
use std::env;

impl Prompt {
    pub fn grep(&mut self) {
        self.disp_row_num = 9;
        self.cont_1 = PromptCont::new(Some(PromptContPosi::First)).get_grep(self);
        self.cont_2 = PromptCont::new(Some(PromptContPosi::Second)).get_grep(self);
        self.cont_3 = PromptCont::new(Some(PromptContPosi::Third)).get_grep(self);
    }

    pub fn draw_grep(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &self.cont_3.buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_row_posi, &self.cont_3.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn get_grep(&mut self, prom: &Prompt) -> PromptCont {
        if self.posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &Lang::get().set_grep);
            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}↓↑  {}{}:{}{}  {}{}:{}Tab {}({})",
                Colors::get_default_fg(),
                &Lang::get().search,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
                Colors::get_default_fg(),
                &Lang::get().move_setting_location,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &Lang::get().close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
                Colors::get_default_fg(),
                &Lang::get().complement,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &Lang::get().search_folder,
            );
            self.set_opt_case_sens();
            self.set_opt_regex();

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &Lang::get().search_str, Colors::get_default_fg());
        } else if self.posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &Lang::get().search_file, Colors::get_default_fg());

            if !prom.prom_grep.cache_search_filenm.is_empty() {
                self.buf = prom.prom_grep.cache_search_filenm.chars().collect();
            } else {
                self.buf = "*.*".chars().collect();
            }
        } else {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &Lang::get().search_folder, Colors::get_default_fg());
            if !prom.prom_grep.cache_search_folder.is_empty() {
                self.buf = prom.prom_grep.cache_search_folder.chars().collect();
            } else if let Ok(path) = env::current_dir() {
                self.buf = path.to_string_lossy().to_string().chars().collect();
            };
        }
        self.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PromGrep {
    pub cache_search_filenm: String,
    pub cache_search_folder: String,
    pub tab_comp: TabComp,
}
