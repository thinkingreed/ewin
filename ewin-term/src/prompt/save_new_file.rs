use crate::{
    ewin_core::{_cfg::keys::*, global::*, log::Log, model::*},
    model::*,
    tab::Tab,
    terminal::*,
};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(term: &mut Terminal) -> EvtActType {
        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_save_new_file();
                return EvtActType::Next;
            }
            KeyCmd::ConfirmPrompt => {
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
                        } else if term.state.is_all_save {
                            return EvtAct::check_exit_save(term);
                        }
                    }
                    term.enable_syntax_highlight(&Path::new(&filenm));
                }
                term.curt().editor.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
    pub fn check_exit_save(term: &mut Terminal) -> EvtActType {
        Log::debug_key("EvtAct.check_exit_save");
        if term.tabs.len() == 1 {
            return EvtActType::Exit;
        } else {
            if term.save_all_tab() {
                return EvtActType::Exit;
            }
            return EvtActType::DrawOnly;
        }
    }
}
