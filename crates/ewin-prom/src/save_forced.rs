use crate::model::*;
use chrono::{prelude::DateTime, Local};
use ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*};
use std::time::SystemTime;

impl Prompt {
    pub fn save_forced(&mut self, open_modified_time: SystemTime, last_modified_time: SystemTime) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(None);
        cont.set_save_forced(open_modified_time, last_modified_time);
        self.cont_1 = cont;
    }
    pub fn draw_save_forced(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
    }
}
impl PromptCont {
    pub fn set_save_forced(&mut self, open_modified_time: SystemTime, last_modified_time: SystemTime) {
        let open_datetime = DateTime::<Local>::from(open_modified_time);
        let open_str = open_datetime.format("%m-%d %H:%M:%S.%3f").to_string();

        let last_datetime = DateTime::<Local>::from(last_modified_time);
        let last_str = last_datetime.format("%m-%d %H:%M:%S.%3f").to_string();

        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &Lang::get().file_has_been_modified_by_other_app);
        self.key_desc = format!("{}{}:{} {}:{}", Colors::get_default_fg(), &Lang::get().open_modified_time, open_str, &Lang::get().last_modified_time, last_str);

        self.buf_desc = format!(
            "{}{}:{}Y  {}{}:{}R  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &Lang::get().save_forced,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            format!("{}({})", &Lang::get().reopen, &Lang::get().edit_discard),
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &Lang::get().cancel,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
            Colors::get_default_fg(),
        );
    }
}
