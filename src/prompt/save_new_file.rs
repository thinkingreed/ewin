use crate::{
    _cfg::keys::{KeyCmd, Keybind},
    colors::*,
    global::*,
    model::*,
    prompt::cont::promptcont::*,
    prompt::prompt::prompt::*,
    tab::Tab,
    terminal::Terminal,
};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(term: &mut Terminal) -> EvtActType {
        match term.curt().editor.keycmd {
            KeyCmd::InsertLine => {
                if term.curt().prom.cont_1.buf.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_filenm);
                } else {
                    let filenm = &term.curt().prom.cont_1.buf.iter().collect::<String>();
                    if Path::new(&filenm).exists() {
                        term.curt().mbar.set_err(&LANG.file_already_exists);
                        return EvtActType::Hold;
                    }
                    if Path::new(&filenm).is_absolute() {
                        term.hbar.file_vec[term.idx].filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string().clone();
                        term.hbar.file_vec[term.idx].fullpath = filenm.clone();
                    } else {
                        term.hbar.file_vec[term.idx].filenm = filenm.clone();
                        let absolute_path = Path::new(&*CURT_DIR).join(filenm);
                        term.hbar.file_vec[term.idx].fullpath = absolute_path.to_string_lossy().to_string();
                    }
                    if Tab::save(term) {
                        if term.curt().state.is_close_confirm {
                            return EvtAct::check_exit_close(term);
                        }
                    }
                    Terminal::enable_syntax_highlight(&Path::new(&filenm), term.curt());
                }
                term.curt().editor.d_range.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn save_new_file(term: &mut Terminal) {
        term.curt().state.is_save_new_file = true;
        term.curt().prom.disp_row_num = 3;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.curt());
        cont.set_new_file_name();
        term.curt().prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_new_filenm);
        self.key_desc = format!("{}{}:{}{}  {}{}:{}{}{}", Colors::get_default_fg(), &LANG.fixed, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::ConfirmPrompt), Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::EscPrompt), Colors::get_default_fg(),);
        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.buf_row_posi = base_posi + 2;
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
