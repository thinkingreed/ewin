use crate::{
    ewin_com::{_cfg::key::keycmd::*, global::*, model::*},
    model::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_prom::cont::parts::pulldown::*;
use std::path::{Path, PathBuf};

impl EvtAct {
    pub fn save_new_filenm(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.save_new_filenm");

        match term.curt().prom.p_cmd {
            P_Cmd::MouseDownLeft(y, x) => {
                if let Ok(cont) = term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.base.row_posi_range.end == y {
                        cont.pulldown.is_disp = !cont.pulldown.is_disp;
                    } else if cont.pulldown.widget.is_mouse_within_area(y, x) {
                        if let Some((_, _)) = cont.pulldown.widget.get_curt_parent() {
                            cont.pulldown.sel_idx = cont.pulldown.widget.parent_sel_y;
                            cont.pulldown.set_sel_name();
                            if cont.pulldown.is_disp {
                                cont.pulldown.is_disp = false;
                            }
                        }
                    } else {
                        cont.pulldown.is_disp = false;
                    }
                }
                return ActType::Draw(DParts::All);
            }
            P_Cmd::MouseMove(y, x) => {
                if let Ok(cont) = term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.pulldown.widget.is_mouse_within_area(y, x) {
                        cont.pulldown.widget.ctrl_mouse_move(y, x);
                    }
                }
                return ActType::Draw(DParts::All);
            }
            P_Cmd::Confirm => {
                let filenm_input = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);
                if filenm_input.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else {
                    let filenm = format! {"{}{}",filenm_input,term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>().unwrap().pulldown.get_sel_name() };
                    let absolute_path = if Path::new(&filenm).is_absolute() { PathBuf::from(&filenm) } else { Path::new(&*CURT_DIR).join(&filenm) };
                    if Path::new(&absolute_path).exists() {
                        return ActType::Draw(DParts::MsgBar(Lang::get().file_already_exists.to_string()));
                    }
                    if Path::new(&filenm).is_absolute() {
                        term.curt_h_file().filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string();
                        term.curt_h_file().fullpath = filenm.clone();
                    } else {
                        term.curt_h_file().filenm = filenm.clone();
                        term.curt_h_file().fullpath = absolute_path.to_string_lossy().to_string();
                    }
                    let act_type = Tab::save(term, SaveType::NewName);
                    Log::debug_s("save act_type");
                    if let ActType::Draw(_) = act_type {
                        return act_type;
                    } else if term.curt().state.prom == PromState::SaveConfirm {
                        return EvtAct::check_exit_close(term);
                    } else if term.state.is_all_save {
                        return EvtAct::check_exit_save(term);
                    }
                    term.enable_syntax_highlight(Path::new(&filenm));
                }
                return ActType::Draw(DParts::All);
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
            if let ActType::Draw(_) = act_type {
                return act_type;
            } else {
                return ActType::Exit;
            }
        }
    }
}
