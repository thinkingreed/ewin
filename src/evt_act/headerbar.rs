use crate::{
    _cfg::keys::{KeyCmd, Keybind, Keys},
    log::*,
    model::*,
    prompt::prompt::prompt::*,
    terminal::Terminal,
};

impl EvtAct {
    pub fn check_headerbar(term: &mut Terminal) -> EvtActType {
        Log::debug_key("check_headerbar");

        match term.curt().editor.keycmd {
            KeyCmd::Resize => return EvtActType::Hold,
            KeyCmd::MouseDownLeft(y, x) => {
                let (x, y) = (x as usize, y as usize);

                if y != term.hbar.disp_row_posi {
                    return EvtActType::Hold;
                }
                if term.hbar.all_filenm_space_w >= x {
                    for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                        if !h_file.is_disp {
                            continue;
                        }
                        if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                            if term.hbar.file_vec[idx].is_changed {
                                term.idx = idx;
                                term.curt().editor.keys = Keybind::get_keys(KeyCmd::CloseFile);
                                return EvtActType::Next;
                            } else {
                                if term.tabs.len() == 1 {
                                    term.curt().editor.keys = Keybind::get_keys(KeyCmd::CloseFile);
                                    return EvtActType::Next;
                                } else {
                                    term.idx = if idx == term.hbar.file_vec.len() - 1 { idx - 1 } else { idx };
                                    term.del_tab(idx);
                                    return EvtActType::DrawOnly;
                                }
                            }
                        }
                        if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                            term.idx = idx;
                            term.curt().editor.keys = Keys::Null;
                            return EvtActType::DrawOnly;
                        }
                    }
                    if term.hbar.all_filenm_rest_area.0 <= x && x <= term.hbar.all_filenm_rest_area.1 {
                        let keycmd = &term.curt().editor.keycmd.clone();
                        if term.hbar.history.count_multi_click(keycmd) == 2 {
                            term.new_tab();
                            return EvtActType::DrawOnly;
                        }
                    }
                }
                if term.hbar.is_left_arrow_disp {
                    if term.hbar.left_arrow_area.0 <= x && x <= term.hbar.left_arrow_area.1 {
                        if term.hbar.disp_base_idx > 0 {
                            term.hbar.disp_base_idx -= 1;
                            return EvtActType::DrawOnly;
                        }
                    }
                }
                if term.hbar.is_right_arrow_disp {
                    if term.hbar.right_arrow_area.0 <= x && x <= term.hbar.right_arrow_area.1 {
                        term.hbar.disp_base_idx += 1;
                        return EvtActType::DrawOnly;
                    }
                }
                if term.hbar.plus_btn_area.0 <= x && x <= term.hbar.plus_btn_area.1 {
                    if term.curt().state.is_open_file {
                        term.clear_curt_tab();
                    } else {
                        Prompt::open_file(term);
                    }
                    return EvtActType::DrawOnly;
                } else if term.hbar.menu_btn_area.0 <= x && x <= term.hbar.menu_btn_area.1 {
                    if term.curt().state.is_menu {
                        term.clear_curt_tab();
                    } else {
                        Prompt::menu(term);
                    }
                    return EvtActType::DrawOnly;
                } else if term.hbar.close_btn_area.0 <= x && x <= term.hbar.close_btn_area.1 {
                    if term.close_all_tab() {
                        return EvtActType::Exit;
                    }
                    return EvtActType::DrawOnly;
                }
                return EvtActType::Hold;
            }
            _ => return EvtActType::Hold,
        }
    }
}
