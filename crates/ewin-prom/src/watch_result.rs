use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*},
    model::*,
};

impl Prompt {
    pub fn watch_result(&mut self) {
        self.disp_row_num = 5;
        let mut cont = PromptCont::new(None);
        cont.set_watch_result();
        self.cont_1 = cont;
    }

    pub fn draw_watch_result(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc_vec.clone());
    }
}
impl PromptCont {
    pub fn set_watch_result(&mut self) {
        self.guide_vec.push(format!("{}{}", Colors::get_msg_highlight_fg(), &Lang::get().file_has_been_modified_by_other_app));
        self.key_desc_vec.push(format!("{}{}({}):{}R", Colors::get_default_fg(), &Lang::get().reopen, &Lang::get().edit_discard, Colors::get_msg_highlight_fg()));
        self.key_desc_vec.push(format!("{}{}:{}{}{}", Colors::get_default_fg(), &Lang::get().cancel, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)), Colors::get_default_fg()));
        self.key_desc_vec.push(format!("{}{}:{}L", Colors::get_default_fg(), &Lang::get().not_edited_will_reloaded_auto, Colors::get_msg_highlight_fg()));
        self.key_desc_vec.push(format!("{}{}:{}N", Colors::get_default_fg(), &Lang::get().no_further_monitoring, Colors::get_msg_highlight_fg()));
    }
}
