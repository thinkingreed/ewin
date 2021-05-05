use crate::{colors::*, global::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(term: &mut Terminal) -> EvtActType {
        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    if term.tabs[term.idx].prom.cont_1.buf.len() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_filenm);
                    } else {
                        let filenm = term.tabs[term.idx].prom.cont_1.buf.iter().collect::<String>();
                        if Path::new(&filenm).exists() {
                            term.tabs[term.idx].mbar.set_err(&LANG.file_already_exists);
                            return EvtActType::Hold;
                        }
                        term.hbar.file_vec[term.idx].filenm = filenm.clone();
                        Tab::save(term);
                        if term.tabs[term.idx].state.is_close_confirm {
                            return EvtAct::check_exit_close(term);
                        }
                    }
                    // Normal save
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn save_new_file(term: &mut Terminal) {
        term.tabs[term.idx].state.is_save_new_file = true;
        term.tabs[term.idx].prom.disp_row_num = 3;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.tabs[term.idx].prom.disp_row_posi as u16);
        cont.set_new_file_name();
        term.tabs[term.idx].prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_new_filenm);
        self.key_desc = format!("{}{}:{}Enter  {}{}:{}Esc{}", Colors::get_default_fg(), &LANG.fixed, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(), Colors::get_default_fg(),);
        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.buf_row_posi = base_posi + 2;
    }
}
