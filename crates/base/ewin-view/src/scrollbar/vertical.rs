use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::Colors, log::*};
use ewin_key::key::cmd::*;

use std::cmp::{max, min};

use crate::{model::*, view::*};

impl ScrollbarV {
    pub fn ctrl_scrollbar_v(&mut self, cmd_type: &CmdType, view_y: usize, view_height: usize) {
        Log::debug_key("ScrollbarV.ctrl_scrollbar_v");

        if self.is_show {
            match cmd_type {
                CmdType::MouseDownLeft(y, x) if self.view.x <= *x => {
                    self.is_enable = true;

                    // Except on scrl_v
                    if !(view_y + self.view.y <= *y && *y < view_y + self.view.y_height()) {
                        self.view.y = if y + self.view.height > view_y + view_height - 1 { view_y + view_height - 1 - self.view.height } else { y - view_y };
                    }
                }
                CmdType::MouseDragLeftDown(y, _) | CmdType::MouseDragLeftUp(y, _) | CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) if self.is_enable => {
                    if matches!(cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                        if matches!(cmd_type, CmdType::MouseDragLeftDown(_, _)) && view_height >= self.view.y + self.view.height {
                            self.view.y = if self.view.y + self.view.height >= view_height { self.view.y } else { self.view.y + 1 };
                        } else if (matches!(cmd_type, CmdType::MouseDragLeftUp(_, _))) && view_y <= *y && *y < view_y + view_height {
                            self.view.y = if self.view.y == 0 { self.view.y } else { self.view.y - 1 };
                        }
                    }
                }
                _ => self.is_enable = false,
            };
        }
    }

    pub fn calc_scrlbar_v(&mut self, cmd_type: &CmdType, offset: Offset, view_height: usize, cont_vec_len: usize) {
        Log::debug_key("ScrollbarV.calc_scrlbar_v");

        if !self.is_show {
            return;
        }
        self.calc_com_scrlbar_v(view_height, cont_vec_len);

        self.view.y = match cmd_type {
            CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) if self.is_enable => self.view.y,
            // TODO
            // Revalidation
            _ => (offset.y as f64 / self.move_len as f64).ceil() as usize,
        };
    }

    pub fn calc_com_scrlbar_v(&mut self, view_hieght: usize, cont_vec_len: usize) {
        Log::debug_key("calc_com_scrlbar_v");

        let bar_len = max(1, min((view_hieght as f64 / cont_vec_len as f64 * view_hieght as f64).floor() as usize, view_hieght - 1));
        let scrl_range = view_hieght - bar_len;
        let move_len = ((cont_vec_len - view_hieght) as f64 / scrl_range as f64).ceil() as usize;
        self.view.height = bar_len;
        self.move_len = move_len;
    }

    pub fn draw(&self, str_vec: &mut Vec<String>, view: &View, not_tgt_color_str: String) {
        Log::debug_key("ScrollbarV.draw");
        if self.is_show {
            for i in view.y..view.y_height() {
                str_vec.push(MoveTo(self.view.x as u16, i as u16).to_string());
                str_vec.push(if view.y + self.view.y <= i && i < view.y + self.view.y + self.view.height { Colors::get_scrollbar_v_bg() } else { not_tgt_color_str.clone() });
                str_vec.push(" ".to_string().repeat(self.view.width));
            }
        }
        str_vec.push(Colors::get_default_bg());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarV {
    pub is_show: bool,
    pub is_enable: bool,
    pub view: View,
    pub move_len: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for ScrollbarV {
    fn default() -> Self {
        ScrollbarV { is_show: false, is_enable: false, view: View::default(), move_len: 0 }
    }
}

impl ScrollbarV {
    pub fn clear(&mut self) {
        self.is_show = false;
        self.view.width = 0;
    }
}
