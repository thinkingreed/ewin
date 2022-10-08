use crate::{btn_grourp::*, dialog::*, global::*};
use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
    term::*,
};
use ewin_key::key::cmd::*;

impl Dialog {
    pub fn ctrl_dialog(cmd_type: &CmdType) -> ActType {
        Log::debug_key("ctrl_dialog");

        match cmd_type {
            CmdType::DialogShow(cont_type) => Dialog::init(*cont_type),
            CmdType::MouseDownLeft(y, x) => return Dialog::set_mouse_down_left(*y, *x),
            CmdType::MouseDragLeftLeft(y, x) | CmdType::MouseDragLeftRight(y, x) | CmdType::MouseDragLeftUp(y, x) | CmdType::MouseDragLeftDown(y, x) => {
                return Dialog::set_drag_posi(*y, *x);
            }
            CmdType::MouseMove(_, _) => return ActType::Draw(DrawParts::Dialog),

            _ => return ActType::Next,
        }
    }

    pub fn set_mouse_down_left(y: usize, x: usize) -> ActType {
        if let Some(mut dialog) = DIALOG.get().unwrap().try_lock() {
            if let Some(btn) = dialog.is_btn_grourp_range(y, x) {
                Log::debug("btn", &btn);
                if btn.cfg.is_close {
                    dialog.clear();
                    return ActType::Draw(DrawParts::Absolute(dialog.get_draw_range_y()));
                }
            }
            if dialog.is_close_btn_range(y, x) {
                dialog.clear();
                return ActType::Draw(DrawParts::Absolute(dialog.get_draw_range_y()));
            }

            if dialog.is_header_range(y, x) {
                dialog.base_y = y;
                dialog.base_x = x;
            } else {
                dialog.base_y = USIZE_UNDEFINED;
                dialog.base_x = USIZE_UNDEFINED;
            }
        };
        return ActType::Cancel;
    }

    pub fn set_drag_posi(y: usize, x: usize) -> ActType {
        if let Some(mut dialog) = DIALOG.get().unwrap().try_lock() {
            if dialog.base_y == USIZE_UNDEFINED || dialog.base_x == USIZE_UNDEFINED {
                return ActType::Cancel;
            }
            let (cols, rows) = get_term_size();
            // y
            dialog.view.y_org = dialog.view.y;
            let diff = y as isize - dialog.base_y as isize;
            Log::debug("diff y", &diff);
            if diff < 0 {
                if dialog.view.y != 0 {
                    dialog.view.y -= diff.unsigned_abs();
                }
            } else {
                let diff = diff as usize;
                if dialog.view.y + dialog.view.height + diff <= rows {
                    dialog.view.y += diff;
                }
            }
            // x
            let diff = x as isize - dialog.base_x as isize;
            if diff < 0 {
                let diff = diff.unsigned_abs();
                if dialog.view.x >= diff {
                    dialog.view.x -= diff;
                }
            } else {
                let diff = diff.unsigned_abs();
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
            // close_btn
            dialog.close_btn.x = dialog.view.x + dialog.view.width - Dialog::CLOSE_BTN_WIDTH;
            dialog.close_btn.y = dialog.view.y;

            dialog.base_y = y;
            dialog.base_x = x;

            return ActType::Draw(DrawParts::Absolute(dialog.get_draw_range_y()));
        }
        return ActType::None;
    }
}
