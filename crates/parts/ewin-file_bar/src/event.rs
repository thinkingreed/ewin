use crate::filebar::*;
use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
};
use ewin_ctx_menu::traits::traits::*;
use ewin_job::job::*;
use ewin_key::key::{cmd::*, keys::*};
use ewin_state::term::*;

impl FileBar {
    pub fn ctrl_filebar(cmd_type: &CmdType, keys: Keys) -> ActType {
        Log::debug_key("ctrl_filebar");

        if let Some(mut fbar) = FileBar::get_result() {
            Log::debug("cmd_type", &cmd_type);
            Log::debug("keys", &keys);
            Log::debug("fbarfbarfbarfbarfbarfbar", &fbar);
            Log::debug("fbar.view", &fbar.view);
            match cmd_type {
                CmdType::MouseDownLeft(y, x) => {
                    Log::debug("xxx", &x);

                    let (x, _) = (*x as usize, *y as usize);
                    State::get().filebar.clear();
                    Log::debug("fbar.all_filenm_space_w", &fbar.all_filenm_space_w);

                    if fbar.all_filenm_space_w >= x {
                        for (idx, h_file) in fbar.file_vec.iter().enumerate() {
                            if !h_file.is_disp {
                                continue;
                            }
                            Log::debug("h_file.close_area", &h_file.close_area);

                            if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                                return Job::send_cmd(CmdType::CloseFileTgt(idx));
                            }
                            if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                                return Job::send_cmd(CmdType::ChangeFile(idx));
                            }
                        }
                        // Double-click outside the file range
                        if fbar.all_filenm_rest_area.0 <= x && x <= fbar.all_filenm_rest_area.1 && fbar.history.count_multi_click(&keys) == 2 {
                            return Job::send_cmd(CmdType::OpenNewFile);
                        }
                    }
                    if fbar.is_on_left_arraw(x) {
                        fbar.disp_base_idx -= 1;
                        return ActType::Draw(DrawParts::TabsAll);
                    }
                    if fbar.is_on_right_arraw(x) {
                        fbar.disp_base_idx += 1;
                        return ActType::Draw(DrawParts::TabsAll);
                    }

                    if fbar.menu_btn_area.0 <= x && x <= fbar.menu_btn_area.1 {
                        // if term.tabs.curt().state.is_menu {
                        // TODO
                        // TODO
                        // TODO
                        // TODO
                        // term.tabs.curt().clear_curt_tab(true);
                        // } else {
                        //  term.tabs.curt().prom_menu();
                        // }
                        return ActType::Draw(DrawParts::TabsAll);
                    }
                    return ActType::Cancel;
                }
                CmdType::MouseDragLeftRight(y, x) | CmdType::MouseDragLeftLeft(y, x) => {
                    let (x, _) = (*x as usize, *y as usize);
                    // Log::debug("term.hbar.all_filenm_space_w", &fbar.all_filenm_space_w);
                    if fbar.all_filenm_space_w >= x {
                        State::get().filebar.is_dragging = true;
                        let mut dst_idx = USIZE_UNDEFINED;
                        let file_vec = fbar.file_vec.clone();
                        for (idx, h_file) in file_vec.iter().enumerate() {
                            if !h_file.is_disp {
                                continue;
                            }
                            if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                                let change_range = ((h_file.filenm_area.1 as f64 - h_file.filenm_area.0 as f64) as f64 / 2_f64).floor() as usize;
                                if matches!(cmd_type, CmdType::MouseDragLeftLeft(_, _)) {
                                    if h_file.filenm_area.0 + change_range >= x {
                                        dst_idx = idx;
                                    }
                                } else if matches!(cmd_type, CmdType::MouseDragLeftRight(_, _)) && h_file.filenm_area.0 + change_range <= x {
                                    dst_idx = idx;
                                }
                            }

                            if fbar.is_on_left_arraw(x) || fbar.is_on_right_arraw(x) {
                                fbar.disp_base_idx = if fbar.is_on_left_arraw(x) { fbar.disp_base_idx - 1 } else { fbar.disp_base_idx + 1 };
                                dst_idx = fbar.disp_base_idx;
                            }
                        }
                        if dst_idx != USIZE_UNDEFINED {
                            return Job::send_cmd(CmdType::SwapFile(State::get().tabs.idx, dst_idx));
                        }
                    }
                    return ActType::Cancel;
                }
                CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) => return ActType::Cancel,
                CmdType::CtxMenu(y, x) => return fbar.init_ctx_menu(*y, *x),
                _ => return ActType::Next,
            }
        }
        return ActType::Next;
    }

    pub fn is_on_left_arraw(&self, x: usize) -> bool {
        return self.is_left_arrow_disp && self.left_arrow_area.0 <= x && x <= self.left_arrow_area.1 && self.disp_base_idx > 0;
    }

    pub fn is_on_right_arraw(&self, x: usize) -> bool {
        return self.is_right_arrow_disp && self.right_arrow_area.0 <= x && x <= self.right_arrow_area.1;
    }

    pub fn judge_when_filebar(keys: Keys, editor_is_dragging: bool) -> bool {
        match keys {
            Keys::MouseDownLeft(y, _) | Keys::MouseDownRight(y, _) if y == FileBar::get().view.y as u16 => return true,
            Keys::MouseDragLeft(y, _) if y == FileBar::get().view.y as u16 => return !editor_is_dragging,
            _ => return false,
        }
    }
}
