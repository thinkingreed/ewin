use crate::{
    ewin_com::{_cfg::key::keycmd::*, def::*, log::*, model::*},
    model::*,
};

impl EvtAct {
    pub fn ctrl_headerbar(term: &mut Terminal) -> ActType {
        Log::debug_key("ctrl_headerbar");

        match &term.keycmd {
            KeyCmd::HeaderBar(h_cmd) => match &h_cmd {
                H_Cmd::MouseDownLeft(y, x) => {
                    let (x, _) = (*x as usize, *y as usize);
                    term.hbar.state.clear();
                    if term.hbar.all_filenm_space_w >= x {
                        for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                            if !h_file.is_disp {
                                continue;
                            }
                            Log::debug("h_file.close_area", &h_file.close_area);

                            if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                                if term.curt().editor.state.is_changed {
                                    term.tab_idx = idx;
                                    term.set_keys(&Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::CloseFile)));

                                    return ActType::Next;
                                } else if term.tabs.len() == 1 {
                                    return ActType::Exit;
                                } else {
                                    term.tab_idx = if idx == term.hbar.file_vec.len() - 1 { idx - 1 } else { idx };
                                    term.del_tab(idx);
                                    return ActType::Render(RParts::All);
                                }
                            }
                            if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                                term.change_tab(idx);
                                term.curt().editor.set_cmd(KeyCmd::Null);
                                return ActType::Render(RParts::All);
                            }
                        }
                        if term.hbar.all_filenm_rest_area.0 <= x && x <= term.hbar.all_filenm_rest_area.1 {
                            let keys = &term.keys.clone();
                            if term.hbar.history.count_multi_click(keys) == 2 {
                                term.new_tab();
                                return ActType::Render(RParts::All);
                            }
                        }
                    }
                    if is_on_left_arraw(term, x) {
                        term.hbar.disp_base_idx -= 1;
                        return ActType::Render(RParts::All);
                    }
                    if is_on_right_arraw(term, x) {
                        term.hbar.disp_base_idx += 1;
                        return ActType::Render(RParts::All);
                    }

                    if term.hbar.plus_btn_area.0 <= x && x <= term.hbar.plus_btn_area.1 {
                        if term.curt().state.is_open_file {
                            term.clear_curt_tab(true, true);
                        } else {
                            term.curt().prom_open_file(OpenFileType::Normal);
                        }
                        return ActType::Render(RParts::All);
                    } else if term.hbar.menu_btn_area.0 <= x && x <= term.hbar.menu_btn_area.1 {
                        if term.curt().state.is_menu {
                            term.clear_curt_tab(true, true);
                        } else {
                            term.curt().prom_menu();
                        }
                        return ActType::Render(RParts::All);
                    } else if term.hbar.close_btn_area.0 <= x && x <= term.hbar.close_btn_area.1 {
                        return if term.close_tabs(USIZE_UNDEFINED) { ActType::Exit } else { ActType::Render(RParts::All) };
                    }
                    return ActType::Cancel;
                }
                H_Cmd::MouseDragLeftRight(y, x) | H_Cmd::MouseDragLeftLeft(y, x) => {
                    let (x, _) = (*x as usize, *y as usize);
                    Log::debug("term.hbar.all_filenm_space_w", &term.hbar.all_filenm_space_w);
                    if term.hbar.all_filenm_space_w >= x {
                        term.hbar.state.is_dragging = true;
                        let mut inset_idx = USIZE_UNDEFINED;
                        for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                            if !h_file.is_disp {
                                continue;
                            }
                            Log::debug("xxx", &x);

                            if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                                Log::debug("h_file.filenm_area.0", &h_file.filenm_area.0);
                                Log::debug("h_file.filenm_area.1", &h_file.filenm_area.1);
                                let change_range = ((h_file.filenm_area.1 as f64 - h_file.filenm_area.0 as f64) as f64 / 2_f64).floor() as usize;
                                if matches!(h_cmd, H_Cmd::MouseDragLeftLeft(_, _)) {
                                    Log::debug("MouseDragLeftLeft change_range", &change_range);
                                    if h_file.filenm_area.0 + change_range >= x {
                                        inset_idx = idx;
                                    }
                                } else if matches!(h_cmd, &H_Cmd::MouseDragLeftRight(_, _)) {
                                    Log::debug("MouseDragLeftRight change_range", &change_range);
                                    if h_file.filenm_area.0 + change_range <= x {
                                        inset_idx = idx;
                                    }
                                }
                                Log::debug("inset_idx", &inset_idx);
                            }

                            if is_on_left_arraw(term, x) || is_on_right_arraw(term, x) {
                                term.hbar.disp_base_idx = if is_on_left_arraw(term, x) { term.hbar.disp_base_idx - 1 } else { term.hbar.disp_base_idx + 1 };
                                inset_idx = term.hbar.disp_base_idx;
                            }
                        }
                        if inset_idx != USIZE_UNDEFINED {
                            Log::debug_s("swap_tabswap_tabswap_tabswap_tabswap_tabswap_tab");
                            Log::debug("term.idx", &term.tab_idx);
                            Log::debug("inset_idx", &inset_idx);

                            term.swap_tab(term.tab_idx, inset_idx);
                        }
                    }
                    return ActType::Render(RParts::HeaderBar);
                }
                H_Cmd::MouseUpLeft(_, _) => {
                    term.hbar.state.is_dragging = false;
                    return ActType::Render(RParts::HeaderBar);
                }
                H_Cmd::MouseDragLeftDown(_, _) | H_Cmd::MouseDragLeftUp(_, _) => return ActType::Cancel,
            },
            _ => return ActType::Next,
        }
    }
}

pub fn is_on_left_arraw(term: &Terminal, x: usize) -> bool {
    return term.hbar.is_left_arrow_disp && term.hbar.left_arrow_area.0 <= x && x <= term.hbar.left_arrow_area.1 && term.hbar.disp_base_idx > 0;
}
pub fn is_on_right_arraw(term: &Terminal, x: usize) -> bool {
    return term.hbar.is_right_arrow_disp && term.hbar.right_arrow_area.0 <= x && x <= term.hbar.right_arrow_area.1;
}
