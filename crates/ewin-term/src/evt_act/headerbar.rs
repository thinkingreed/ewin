use std::io::stdout;

use crate::{
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*},
        def::*,
        log::*,
        model::*,
    },
    model::*,
};

impl EvtAct {
    pub fn ctrl_headerbar(term: &mut Terminal) -> ActType {
        Log::debug_key("ctrl_headerbar");

        match &term.keycmd {
            KeyCmd::HeaderBar(h_cmd) => match &h_cmd {
                H_Cmd::MouseDragLeftUp(y, x) => {
                    EvtAct::match_event(Keys::MouseDragLeft((*y + HEADERBAR_ROW_NUM) as u16, *x as u16), &mut stdout(), term);
                    return ActType::Cancel;
                }
                H_Cmd::MouseDownLeft(y, x) => {
                    let (x, _) = (*x as usize, *y as usize);
                    if term.hbar.all_filenm_space_w >= x {
                        for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                            if !h_file.is_disp {
                                continue;
                            }
                            Log::debug("h_file.close_area", &h_file.close_area);

                            if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                                if term.curt().editor.state.is_changed {
                                    term.idx = idx;
                                    term.set_keys(&Keybind::keycmd_to_keys(&KeyCmd::CloseFile));

                                    return ActType::Next;
                                } else if term.tabs.len() == 1 {
                                    return ActType::Exit;
                                } else {
                                    term.idx = if idx == term.hbar.file_vec.len() - 1 { idx - 1 } else { idx };
                                    term.del_tab(idx);
                                    return ActType::Draw(DParts::All);
                                }
                            }
                            if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                                term.idx = idx;
                                Terminal::set_title(&h_file.fullpath);
                                term.curt().editor.set_cmd(KeyCmd::Null);
                                return ActType::Draw(DParts::All);
                            }
                        }
                        if term.hbar.all_filenm_rest_area.0 <= x && x <= term.hbar.all_filenm_rest_area.1 {
                            let keys = &term.keys.clone();
                            if term.hbar.history.count_multi_click(keys) == 2 {
                                term.new_tab();
                                return ActType::Draw(DParts::All);
                            }
                        }
                    }
                    if term.hbar.is_left_arrow_disp && term.hbar.left_arrow_area.0 <= x && x <= term.hbar.left_arrow_area.1 && term.hbar.disp_base_idx > 0 {
                        term.hbar.disp_base_idx -= 1;
                        return ActType::Draw(DParts::All);
                    }
                    if term.hbar.is_right_arrow_disp && term.hbar.right_arrow_area.0 <= x && x <= term.hbar.right_arrow_area.1 {
                        term.hbar.disp_base_idx += 1;
                        return ActType::Draw(DParts::All);
                    }

                    if term.hbar.plus_btn_area.0 <= x && x <= term.hbar.plus_btn_area.1 {
                        if term.curt().state.is_open_file {
                            term.clear_curt_tab(true);
                        } else {
                            term.curt().prom_open_file(OpenFileType::Normal);
                        }
                        return ActType::Draw(DParts::All);
                    } else if term.hbar.menu_btn_area.0 <= x && x <= term.hbar.menu_btn_area.1 {
                        if term.curt().state.is_menu {
                            term.clear_curt_tab(true);
                        } else {
                            term.curt().prom_menu();
                        }
                        return ActType::Draw(DParts::All);
                    } else if term.hbar.close_btn_area.0 <= x && x <= term.hbar.close_btn_area.1 {
                        return if term.close_tabs(USIZE_UNDEFINED) { ActType::Exit } else { ActType::Draw(DParts::All) };
                    }
                    return ActType::Cancel;
                }
            },
            _ => return ActType::Next,
        }
    }
}
