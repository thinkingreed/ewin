use crate::{
    ewin_core::{_cfg::key::keycmd::*, colors::*, global::*},
    model::*,
};

impl Prompt {
    pub fn set_grep_working(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(None);
        cont.set_grep_working();
        self.cont_1 = cont;
    }

    pub fn set_grep_result(&mut self, is_grep_result_vec_empty: bool, is_cancel: bool) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(None);
        cont.set_grep_result(is_grep_result_vec_empty, is_cancel);

        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_grep_working(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.long_time_to_search);
        self.key_desc = format!("{}{}:{}{}", Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),);

        /*
           let base_posi = self.disp_row_posi;
           self.guide_row_posi = base_posi;
           self.key_desc_row_posi = base_posi + 1;
        */
    }

    pub fn set_grep_result(&mut self, is_grep_result_empty: bool, is_cancel: bool) {
        let cancel_str = if is_cancel { &LANG.processing_canceled } else { "" };

        if is_grep_result_empty {
            self.guide = format!("{}{} {}", Colors::get_msg_highlight_fg(), &LANG.show_search_no_result, cancel_str,);
            self.key_desc = format!("{}{}:{}Ctrl + w", Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(),);
        } else {
            self.guide = format!("{}{} {}", Colors::get_msg_highlight_fg(), &LANG.show_search_result, cancel_str);
            self.key_desc = format!("{}{}:{}Enter  {}{}:{}Ctrl + f", Colors::get_default_fg(), &LANG.open_target_file, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.search, Colors::get_msg_highlight_fg(),);
        }
        /*
        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
         */
    }
}
