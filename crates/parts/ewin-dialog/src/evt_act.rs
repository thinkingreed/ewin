use crate::{btn_grourp::*, dialog::*, global::*};
use ewin_cfg::log::*;
use ewin_const::{def::USIZE_UNDEFINED, model::*, term::*};
use ewin_key::key::cmd::*;
use ewin_view::view_trait::view_evt_trait::*;

impl Dialog {
    pub fn ctrl_dialog(cmd_type: &CmdType) -> ActType {
        Log::debug_key("ctrl_dialog");

        match cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                Log::debug("yyy", &y);
                Log::debug("xxx", &x);

                if let Ok(mut dialog) = DIALOG.get().unwrap().try_lock() {
                    if let Some(btn) = dialog.is_btn_grourp_range(*y, *x) {
                        Log::debug("btn", &btn);
                        if btn.cfg.is_close {
                            dialog.clear();
                            return ActType::Draw(DParts::Absolute(dialog.get_draw_range_y()));
                        }
                    }
                    if dialog.is_close_btn_range(*y, *x) {
                        dialog.clear();
                        return ActType::Draw(DParts::Absolute(dialog.get_draw_range_y()));
                    }

                    if dialog.is_header_range(*y, *x) {
                        dialog.base_y = *y;
                        dialog.base_x = *x;
                    } else {
                        dialog.base_y = USIZE_UNDEFINED;
                        dialog.base_x = USIZE_UNDEFINED;
                    }
                };
                return ActType::Cancel;
            }
            CmdType::MouseDragLeftLeft(y, x) | CmdType::MouseDragLeftRight(y, x) | CmdType::MouseDragLeftUp(y, x) | CmdType::MouseDragLeftDown(y, x) => {
                Log::debug_key("MouseDrag");
                let mut act_type = ActType::None;
                if let Ok(mut dialog) = DIALOG.get().unwrap().try_lock() {
                    if dialog.base_y == USIZE_UNDEFINED || dialog.base_x == USIZE_UNDEFINED {
                        return ActType::Cancel;
                    }

                    let (cols, rows) = get_term_size();
                    // y
                    dialog.view.y_org = dialog.view.y;
                    let diff = *y as isize - dialog.base_y as isize;
                    Log::debug("diff y", &diff);
                    if diff < 0 {
                        if dialog.view.y != 0 {
                            dialog.view.y -= diff.abs() as usize;
                        }
                    } else {
                        let diff = diff as usize;
                        if dialog.view.y + dialog.view.height + diff <= rows {
                            dialog.view.y += diff;
                        }
                    }
                    // x
                    let diff = *x as isize - dialog.base_x as isize;
                    if diff < 0 {
                        let diff = diff.abs() as usize;
                        if dialog.view.x >= diff {
                            dialog.view.x -= diff;
                        }
                    } else {
                        let diff = diff.abs() as usize;
                        if dialog.view.x + dialog.view.width + diff <= cols {
                            dialog.view.x += diff as usize;
                        }
                    }
                    match dialog.btn_group.btn_type {
                        DialogBtnGrourpType::Ok => {
                            dialog.btn_group.vec[0].view.x = dialog.view.x + dialog.view.width / 2 - dialog.btn_group.vec[0].name_width / 2;
                            dialog.btn_group.vec[0].view.y = dialog.view.y + dialog.view.height - 1;
                        }
                        DialogBtnGrourpType::OkCancel => {}
                    };

                    dialog.base_y = *y;
                    dialog.base_x = *x;

                    act_type = ActType::Draw(DParts::Absolute(dialog.get_draw_range_y()));
                };
                return act_type;
            }
            CmdType::MouseMove(_, _) => return ActType::Draw(DParts::Dialog),
            CmdType::Resize(_, _) => {
                if let Ok(mut dialog) = DIALOG.get().unwrap().try_lock() {
                    dialog.resize();
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Next,
        }
    }
}
