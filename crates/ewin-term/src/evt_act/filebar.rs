use crate::{model::*, terms::term::*};
use ewin_cfg::log::*;
use ewin_const::{def::*, model::*};
use ewin_key::key::cmd::*;
use ewin_state::tabs::*;

impl EvtAct {
    pub fn ctrl_filebar(term: &mut Term) -> ActType {
        Log::debug_key("ctrl_filebar");

        match &term.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                let (x, _) = (*x as usize, *y as usize);
                term.fbar.state.clear();
                if term.fbar.all_filenm_space_w >= x {
                    let file_vec = &Tabs::get().h_file_vec.clone();
                    Log::debug("file_vec", &file_vec);

                    for (idx, h_file) in file_vec.iter().enumerate() {
                        if !h_file.is_disp {
                            continue;
                        }
                        if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                            if term.curt().editor.state.is_changed {
                                term.tab_idx = idx;
                                term.set_keys(&Cmd::cmd_to_keys(CmdType::CloseFile));

                                return ActType::Next;
                            } else if term.tabs.len() == 1 {
                                return ActType::Exit;
                            } else {
                                term.tab_idx = if idx == Tabs::get().h_file_vec.len() - 1 { idx - 1 } else { idx };
                                term.del_tab(idx);
                                return ActType::Draw(DParts::All);
                            }
                        }
                        if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                            term.change_tab(idx);
                            term.curt().editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                            return ActType::Draw(DParts::All);
                        }
                    }
                    if term.fbar.all_filenm_rest_area.0 <= x && x <= term.fbar.all_filenm_rest_area.1 {
                        let keys = &term.keys.clone();
                        if term.fbar.history.count_multi_click(keys) == 2 {
                            term.new_tab();
                            return ActType::Draw(DParts::All);
                        }
                    }
                }
                if is_on_left_arraw(term, x) {
                    term.fbar.disp_base_idx -= 1;
                    return ActType::Draw(DParts::All);
                }
                if is_on_right_arraw(term, x) {
                    term.fbar.disp_base_idx += 1;
                    return ActType::Draw(DParts::All);
                }

                if term.fbar.menu_btn_area.0 <= x && x <= term.fbar.menu_btn_area.1 {
                    // if term.curt().state.is_menu {

                    // TODO
                    term.curt().clear_curt_tab(true);
                    // } else {
                    //  term.curt().prom_menu();
                    // }
                    return ActType::Draw(DParts::All);
                } else if term.fbar.close_btn_area.0 <= x && x <= term.fbar.close_btn_area.1 {
                    return term.close_tabs(USIZE_UNDEFINED);
                }
                return ActType::Cancel;
            }
            CmdType::MouseDragLeftRight(y, x) | CmdType::MouseDragLeftLeft(y, x) => {
                let (x, _) = (*x as usize, *y as usize);
                Log::debug("term.hbar.all_filenm_space_w", &term.fbar.all_filenm_space_w);
                if term.fbar.all_filenm_space_w >= x {
                    term.fbar.state.is_dragging = true;
                    let mut inset_idx = USIZE_UNDEFINED;
                    for (idx, h_file) in Tabs::get().h_file_vec.iter().enumerate() {
                        if !h_file.is_disp {
                            continue;
                        }
                        Log::debug("xxx", &x);
                        Log::debug("term.tabs.len()", &term.tabs.len());

                        if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                            let change_range = ((h_file.filenm_area.1 as f64 - h_file.filenm_area.0 as f64) as f64 / 2_f64).floor() as usize;
                            if matches!(&term.cmd.cmd_type, &CmdType::MouseDragLeftLeft(_, _)) {
                                if h_file.filenm_area.0 + change_range >= x {
                                    inset_idx = idx;
                                }
                            } else if matches!(&term.cmd.cmd_type, &CmdType::MouseDragLeftRight(_, _)) && h_file.filenm_area.0 + change_range <= x {
                                inset_idx = idx;
                            }
                        }

                        if is_on_left_arraw(term, x) || is_on_right_arraw(term, x) {
                            term.fbar.disp_base_idx = if is_on_left_arraw(term, x) { term.fbar.disp_base_idx - 1 } else { term.fbar.disp_base_idx + 1 };
                            inset_idx = term.fbar.disp_base_idx;
                        }
                    }
                    if inset_idx != USIZE_UNDEFINED {
                        Log::debug("term.tabs.len()", &term.tabs.len());

                        term.swap_tab(term.tab_idx, inset_idx);
                    }
                }
                return ActType::Draw(DParts::FileBar);
            }
            CmdType::MouseUpLeft(_, _) => {
                term.fbar.state.is_dragging = false;
                return ActType::Draw(DParts::FileBar);
            }
            CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) => return ActType::Cancel,

            _ => return ActType::Next,
        }
    }
}

pub fn is_on_left_arraw(term: &Term, x: usize) -> bool {
    return term.fbar.is_left_arrow_disp && term.fbar.left_arrow_area.0 <= x && x <= term.fbar.left_arrow_area.1 && term.fbar.disp_base_idx > 0;
}
pub fn is_on_right_arraw(term: &Term, x: usize) -> bool {
    return term.fbar.is_right_arrow_disp && term.fbar.right_arrow_area.0 <= x && x <= term.fbar.right_arrow_area.1;
}
