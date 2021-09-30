use crate::{
    ewin_core::{_cfg::key::keycmd::*, colors::*, global::*},
    model::*,
};

impl Prompt {
    pub fn save_new_file(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(None);
        cont.set_new_file_name();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_new_filenm);
        self.key_desc = format!(
            "{}{}:{}{}  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &LANG.fixed,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
            Colors::get_default_fg(),
            &LANG.cancel,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            Colors::get_default_fg(),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromSaveNewFile {
    pub tab_comp: TabComp,
}

impl Default for PromSaveNewFile {
    fn default() -> Self {
        PromSaveNewFile { tab_comp: TabComp::default() }
    }
}
