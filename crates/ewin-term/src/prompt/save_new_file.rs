use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, global::*, log::Log, model::*},
    model::*,
    tab::Tab,
};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.save_new_filenm");

        match &term.curt().prom.keycmd {
            KeyCmd::Prom(P_Cmd::Resize(_, _)) => {
                term.curt().prom_save_new_file();
                return ActType::Render(RParts::All);
            }
            KeyCmd::Prom(P_Cmd::ConfirmPrompt) => {
                if term.curt().prom.cont_1.buf.is_empty() {
                    return ActType::Render(RParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else {
                    let filenm = &term.curt().prom.cont_1.buf.iter().collect::<String>();
                    if Path::new(&filenm).exists() {
                        return ActType::Render(RParts::MsgBar(Lang::get().file_already_exists.to_string()));
                    }
                    if Path::new(&filenm).is_absolute() {
                        term.hbar.file_vec[term.tab_idx].filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string();
                        term.hbar.file_vec[term.tab_idx].fullpath = filenm.clone();
                    } else {
                        term.hbar.file_vec[term.tab_idx].filenm = filenm.clone();
                        let absolute_path = Path::new(&*CURT_DIR).join(filenm);
                        term.hbar.file_vec[term.tab_idx].fullpath = absolute_path.to_string_lossy().to_string();
                    }
                    let act_type = Tab::save(term, SaveType::NewName);
                    Log::debug_s("save act_type");
                    if let ActType::Render(_) = act_type {
                        return act_type;
                    } else if term.curt().state.is_save_confirm {
                        return EvtAct::check_exit_close(term);
                    } else if term.state.is_all_save {
                        return EvtAct::check_exit_save(term);
                    }
                    term.enable_syntax_highlight(Path::new(&filenm));
                }
                return ActType::Render(RParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
    pub fn check_exit_save(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.check_exit_save");
        if term.tabs.len() == 1 {
            return ActType::Exit;
        } else {
            let act_type = term.save_all_tab();
            if let ActType::Render(_) = act_type {
                return act_type;
            } else {
                return ActType::Exit;
            }
        }
    }
}
