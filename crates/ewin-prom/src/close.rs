use crate::model::*;
use ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*};

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &Lang::get().save_confirmation_to_close);
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &Lang::get().yes,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &Lang::get().no,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &Lang::get().cancel,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            Colors::get_default_fg(),
        );
    }
}
