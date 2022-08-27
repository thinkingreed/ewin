use crate::tabs::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, evt::*, file::*};
use ewin_key::{key::cmd::*, model::*};
use ewin_prom::cont::parts::pulldown::*;
use ewin_state::term::*;
use ewin_utils::global::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

impl Tabs {
    pub fn save_new_filenm(&mut self) -> ActType {
        Log::debug_key("EvtAct.save_new_filenm");

        match self.curt().prom.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                if let Ok(cont) = self.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.base.row_posi_range.end == y {
                        cont.pulldown.is_show = !cont.pulldown.is_show;
                    } else if cont.pulldown.menulist.is_mouse_within_area(y, x) {
                        if let Some((_, _)) = cont.pulldown.menulist.get_curt_parent() {
                            cont.pulldown.sel_idx = cont.pulldown.menulist.parent_sel_y;
                            cont.pulldown.set_sel_name();
                            if cont.pulldown.is_show {
                                cont.pulldown.is_show = false;
                            }
                        }
                    } else {
                        cont.pulldown.is_show = false;
                    }
                }
                return ActType::Draw(DParts::All);
            }
            CmdType::MouseMove(y, x) => {
                if let Ok(cont) = self.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
                    if cont.pulldown.menulist.is_mouse_within_area(y, x) {
                        cont.pulldown.menulist.ctrl_mouse_move(y, x);
                    }
                }
                return ActType::Draw(DParts::All);
            }
            CmdType::Confirm => {
                let filenm_input = self.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);
                if filenm_input.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else {
                    let filenm = if Path::new(&filenm_input).extension().is_some() {
                        filenm_input
                    } else {
                        format! {"{}{}",filenm_input,self.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>().unwrap().pulldown.get_sel_name() }
                    };

                    let filenm_path = Path::new(&filenm);

                    let absolute_path = if Path::new(&filenm).is_absolute() { PathBuf::from(&filenm) } else { Path::new(&*CURT_DIR).join(&filenm) };
                    if Path::new(&absolute_path).exists() {
                        return ActType::Draw(DParts::MsgBar(Lang::get().file_already_exists.to_string()));
                    }

                    if filenm_path.is_absolute() {
                        State::get().curt_mut_state().file.name = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string();
                        State::get().curt_mut_state().file.fullpath = filenm.clone();
                    } else {
                        State::get().curt_mut_state().file.name = filenm.clone();
                        State::get().curt_mut_state().file.fullpath = absolute_path.to_string_lossy().to_string();
                    }
                    let ext = filenm_path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
                    State::get().curt_mut_state().file.ext = ext;

                    let act_type = self.curt().save(&SaveFileType::NewFile);
                    Log::debug_s("save act_type");
                    if let ActType::Draw(_) = act_type {
                        return act_type;
                    } else if State::get().curt_state().prom == PromState::SaveConfirm {
                        return self.check_exit_close();
                    } else if self.state.is_all_save {
                        return self.check_exit_save();
                    }
                    self.curt().enable_syntax_highlight();
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn check_exit_save(&mut self) -> ActType {
        Log::debug_key("EvtAct.check_exit_save");
        if self.vec.len() == 1 {
            return ActType::Exit;
        } else {
            let act_type = self.save_all_tab();
            if let ActType::Draw(_) = act_type {
                return act_type;
            } else {
                return ActType::Exit;
            }
        }
    }
}
