use ewin_com::_cfg::model::default::Cfg;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*},
    model::*,
};

impl Prompt {
    pub fn save_new_file(&mut self, candidate_filenm: String) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(None);
        cont.set_new_file_name(candidate_filenm);
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self, candidate_filenm: String) {
        self.guide_vec.push(format!("{}{}. {} .{}", Colors::get_msg_highlight_fg(), &Lang::get().set_new_filenm, &Lang::get().auto_assigned_extension, Cfg::get().general.editor.save.extension_when_saving_new_file));
        self.key_desc_vec.push(format!(
            "{}{}:{}{}  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &Lang::get().fixed,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
            Colors::get_default_fg(),
            &Lang::get().cancel,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            Colors::get_default_fg(),
        ));
        self.buf = candidate_filenm.chars().collect();
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PromSaveNewFile {
    pub tab_comp: TabComp,
}
