use crate::cont::promptcont::PromptCont;
use ewin_core::{_cfg::keys::*, colors::*, global::*};

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.save_confirmation_to_close);
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &LANG.yes,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.no,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.cancel,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::EscPrompt),
            Colors::get_default_fg(),
        );
    }
}
