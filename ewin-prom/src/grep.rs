use crate::{
    cont::promptcont::*,
    ewin_core::{_cfg::keys::*, colors::*, global::*},
    prompt::prompt::*,
};
use std::env;

impl Prompt {
    pub fn grep(&mut self) {
        self.disp_row_num = 9;
        self.cont_1 = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::First).get_grep(&self);
        self.cont_2 = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::Second).get_grep(&self);
        self.cont_3 = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::Third).get_grep(&self);
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
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_grep);
            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}↓↑  {}{}:{}{}  {}{}:{}Tab {}({})",
                Colors::get_default_fg(),
                &LANG.search,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::ConfirmPrompt),
                Colors::get_default_fg(),
                &LANG.move_setting_location,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::EscPrompt),
                Colors::get_default_fg(),
                &LANG.complement,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.search_folder,
            );
            self.set_opt_case_sens();
            self.set_opt_regex();

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_str, Colors::get_default_fg());
        } else if self.posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_file, Colors::get_default_fg());

            if prom.prom_grep.cache_search_filenm.len() > 0 {
                self.buf = prom.prom_grep.cache_search_filenm.chars().collect();
            } else {
                self.buf = "*.*".chars().collect();
            }
        } else {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_folder, Colors::get_default_fg());
            if prom.prom_grep.cache_search_folder.len() > 0 {
                self.buf = prom.prom_grep.cache_search_folder.chars().collect();
            } else if let Ok(path) = env::current_dir() {
                self.buf = path.to_string_lossy().to_string().chars().collect();
            };
        }
        return self.clone();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromGrep {
    pub cache_search_filenm: String,
    pub cache_search_folder: String,
    pub tab_comp: TabComp,
}

impl Default for PromGrep {
    fn default() -> Self {
        PromGrep { cache_search_filenm: String::new(), cache_search_folder: String::new(), tab_comp: TabComp::default() }
    }
}
