use crate::{
    cont::promptcont::*,
    ewin_core::{_cfg::keys::*, colors::*, global::*},
    prompt::prompt::*,
};

impl Prompt {
    pub fn move_row(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::First);
        cont.set_move_row();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_move_row(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_move_row);
        self.key_desc = format!("{}{}:{}{}  {}{}:{}{}{}", Colors::get_default_fg(), &LANG.move_to_specified_row, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::ConfirmPrompt), Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::EscPrompt), Colors::get_default_fg(),);
    }
}
