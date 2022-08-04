use crate::{model::*, terms::term::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::model::*;
use ewin_key::{global::*, key::cmd::*, model::*};
use ewin_prom::cont::parts::pulldown::*;
use ewin_state::tabs::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

impl EvtAct {
    pub fn save_new_filenm(term: &mut Term) -> ActType {
        Log::debug_key("EvtAct.save_new_filenm");

        match term.curt().prom.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                if let Ok(cont) = term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.base.row_posi_range.end == y {
                        cont.pulldown.is_disp = !cont.pulldown.is_disp;
                    } else if cont.pulldown.menulist.is_mouse_within_area(y, x) {
                        if let Some((_, _)) = cont.pulldown.menulist.get_curt_parent() {
                            cont.pulldown.sel_idx = cont.pulldown.menulist.parent_sel_y;
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
            CmdType::MouseMove(y, x) => {
                if let Ok(cont) = term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.pulldown.menulist.is_mouse_within_area(y, x) {
                        cont.pulldown.menulist.ctrl_mouse_move(y, x);
                    }
                }
                return ActType::Draw(DParts::All);
            }
            CmdType::Confirm => {
                let filenm_input = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);
                if filenm_input.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else {
                    let filenm = if Path::new(&filenm_input).extension().is_some() {
                        filenm_input
                    } else {
                        format! {"{}{}",filenm_input,term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>().unwrap().pulldown.get_sel_name() }
                    };

                    let filenm_path = Path::new(&filenm);

                    let absolute_path = if Path::new(&filenm).is_absolute() { PathBuf::from(&filenm) } else { Path::new(&*CURT_DIR).join(&filenm) };
                    if Path::new(&absolute_path).exists() {
                        return ActType::Draw(DParts::MsgBar(Lang::get().file_already_exists.to_string()));
                    }

                    if filenm_path.is_absolute() {
                        Tabs::get().curt_mut_h_file().file.name = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string();
                        Tabs::get().curt_mut_h_file().file.fullpath = filenm.clone();
                    } else {
                        Tabs::get().curt_mut_h_file().file.name = filenm.clone();
                        Tabs::get().curt_mut_h_file().file.fullpath = absolute_path.to_string_lossy().to_string();
                    }
                    let ext = filenm_path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
                    Tabs::get().curt_mut_h_file().file.ext = ext;

                    let act_type = term.curt().save(SaveType::NewName);
                    Log::debug_s("save act_type");
                    if let ActType::Draw(_) = act_type {
                        return act_type;
                    } else if term.curt().state.prom == PromState::SaveConfirm {
                        return EvtAct::check_exit_close(term);
                    } else if term.state.is_all_save {
                        return EvtAct::check_exit_save(term);
                    }
                    term.curt().enable_syntax_highlight();
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn check_exit_save(term: &mut Term) -> ActType {
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
