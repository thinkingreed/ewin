use crate::{
    ewin_com::{_cfg::key::keycmd::*, global::*, log::Log, model::*},
    model::*,
    tab::Tab,
    terminal::*,
};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.save_new_filenm");

        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_save_new_file();
                return ActType::Draw(DParts::All);
            }
            KeyCmd::Prom(p_keycmd) => match p_keycmd {
                P_Cmd::ConfirmPrompt => {
                    if term.curt().prom.cont_1.buf.len() == 0 {
                        return ActType::Draw(DParts::MsgBar(LANG.not_entered_filenm.to_string()));
                    } else {
                        let filenm = &term.curt().prom.cont_1.buf.iter().collect::<String>();
                        if Path::new(&filenm).exists() {
                            return ActType::Draw(DParts::MsgBar(LANG.file_already_exists.to_string()));
                        }
                        if Path::new(&filenm).is_absolute() {
                            term.hbar.file_vec[term.idx].filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string().clone();
                            term.hbar.file_vec[term.idx].fullpath = filenm.clone();
                        } else {
                            term.hbar.file_vec[term.idx].filenm = filenm.clone();
                            let absolute_path = Path::new(&*CURT_DIR).join(filenm);
                            term.hbar.file_vec[term.idx].fullpath = absolute_path.to_string_lossy().to_string();
                        }
                        let act_type = Tab::save(term);
                        if let ActType::Draw(_) = act_type {
                            return act_type;
                        } else {
                            if term.curt().state.is_close_confirm {
                                return EvtAct::check_exit_close(term);
                            } else if term.state.is_all_save {
                                return EvtAct::check_exit_save(term);
                            }
                        }
                        term.enable_syntax_highlight(&Path::new(&filenm));
                    }
                    return ActType::Draw(DParts::All);
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }
    pub fn check_exit_save(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.check_exit_save");
        if term.tabs.len() == 1 {
            return ActType::Exit;
        } else {
            let act_type = term.save_all_tab();
            if let ActType::Draw(_) = act_type {
                return act_type;
            } else {
                return ActType::Exit;
            }
        }
    }
}
