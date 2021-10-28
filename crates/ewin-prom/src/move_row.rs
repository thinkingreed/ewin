use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*},
    model::*,
};

impl Prompt {
    pub fn move_row(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(Some(PromptContPosi::First));
        cont.set_move_row();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_move_row(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), Lang::get().set_move_row);
        self.key_desc = format!(
            "{}{}:{}{}  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &Lang::get().move_to_specified_row,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
            Colors::get_default_fg(),
            &Lang::get().close,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            Colors::get_default_fg(),
        );
    }
}
