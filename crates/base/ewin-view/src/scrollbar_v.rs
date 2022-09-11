use ewin_cfg::{log::Log, model::general::default::*};
use ewin_key::key::cmd::CmdType;

use std::cmp::{max, min};

use crate::{model::*, view::*};

impl ScrollbarV {
    pub fn ctrl_scrollbar_v(&mut self, y: usize, cmd_type: &CmdType, scrl_v_bar_x: usize, view_y: usize, view_hight: usize) {
        if self.is_show && view_y <= y && y <= view_y + view_hight {
            match cmd_type {
                CmdType::MouseDownLeft(y, x) if scrl_v_bar_x <= *x => {
                    self.set_scrlbar_v_posi(*y, cmd_type, view_y, view_hight);
                }
                CmdType::MouseDragLeftDown(y, _) | CmdType::MouseDragLeftUp(y, _) | CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) if self.is_enable => {
                    if matches!(cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                        self.set_scrlbar_v_posi(*y, cmd_type, view_y, view_hight);
                    }
                }
                _ => self.is_enable = false,
            };
        }
    }
    pub fn set_scrlbar_v_posi(&mut self, y: usize, cmd_type: &CmdType, view_y: usize, view_height: usize) {
        // MouseDownLeft
        if matches!(cmd_type, CmdType::MouseDownLeft(_, _)) {
            self.is_enable = true;
            // Except on scrl_v
            if !(view_y + self.view.y <= y && y < view_y + self.view.y + self.bar_len) {
                self.view.y = if y + self.bar_len > view_y + view_height - 1 { view_y + view_height - 1 - self.bar_len } else { y - view_y };
            } else {
                return;
            }
            // MouseDragLeftDown・MouseDragLeftUp
        } else if self.is_enable {
            if matches!(cmd_type, CmdType::MouseDragLeftDown(_, _)) && view_height >= self.view.y + self.bar_len {
                self.view.y = if self.view.y + self.bar_len >= view_height { self.view.y } else { self.view.y + 1 };
            } else if (matches!(cmd_type, CmdType::MouseDragLeftUp(_, _))) && view_y <= y && y < view_y + view_height {
                self.view.y = if self.view.y == 0 { self.view.y } else { self.view.y - 1 };
            }
        }
    }

    // TODO
    // is_calc_com
    pub fn calc_scrlbar_v(&mut self, cmd_type: &CmdType, offset: Offset, view_height: usize, cont_height: usize, is_calc_com: bool) {
        if !self.is_show {
            return;
        }
        // if self.bar_len == 0 || is_calc_com {
        self.calc_com_scrlbar_v(true, view_height, cont_height);
        // }

        self.view.y = match cmd_type {
            CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) if self.is_enable => self.view.y,
            // TODO
            // Revalidation
            _ => (offset.y as f64 / self.move_len as f64).ceil() as usize,
        };
    }

    pub fn calc_com_scrlbar_v(&mut self, is_editor_scrlbar_v: bool, view_hight: usize, cont_len: usize) {
        Log::debug_key("calc_com_scrlbar_v");
        Log::debug("row_len - 1", &(view_hight - 1));
        Log::debug("(row_len as f64 / cont_len as f64 * row_len as f64).ceil()", &(view_hight as f64 / cont_len as f64 * view_hight as f64).ceil());

        let bar_len = max(1, min((view_hight as f64 / cont_len as f64 * view_hight as f64).ceil() as usize, view_hight - 1));
        let scrl_range = view_hight - bar_len;
        let move_len = if is_editor_scrlbar_v {
            if Cfg::get().general.editor.cursor.move_position_by_scrolling_enable {
                (cont_len as f64 / scrl_range as f64).ceil() as usize
            } else {
                ((cont_len - view_hight) as f64 / scrl_range as f64).ceil() as usize
            }
            // input comple scrlbar_v ..
        } else {
            Log::debug("cont_len", &cont_len);
            Log::debug("area_len", &view_hight);

            ((cont_len - view_hight) as f64 / scrl_range as f64).ceil() as usize
        };
        self.bar_len = bar_len;
        self.move_len = move_len;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarV {
    pub is_show: bool,
    pub is_enable: bool,
    // Not include　editor.row_posi
    pub view: View,
    pub bar_width: usize,
    pub bar_len: usize,
    pub move_len: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for ScrollbarV {
    fn default() -> Self {
        ScrollbarV { is_show: false, is_enable: false, view: View::default(), bar_len: 0, bar_width: 0, move_len: 0 }
    }
}

impl ScrollbarV {
    pub fn clear(&mut self) {
        self.is_show = false;
        self.bar_width = 0;
    }
}
