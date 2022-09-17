use crate::view::*;
use ewin_cfg::{log::*, model::general::default::*};
use ewin_key::key::cmd::*;
use std::cmp::{max, min};

impl ScrollbarH {
    pub fn calc_scrlbar_h(&mut self, view_width: usize, row_max_width: usize, row_max_chars: usize, offset_disp_x: usize) {
        Log::debug_s("calc_editor_scrlbar_h reset");

        self.bar_len = max(2, min(view_width - 1, (view_width as f64 / row_max_width as f64 * view_width as f64).floor() as usize));

        if !self.is_enable {
            self.view.x = min(self.scrl_range, (self.scrl_range as f64 * offset_disp_x as f64 / (row_max_width - view_width) as f64).ceil() as usize);
        }

        if row_max_width > view_width {
            self.scrl_range = view_width - self.bar_len;

            let rate = row_max_width as f64 / row_max_chars as f64;

            //  let move_cur_x = ((row_max_width - view_width) as f64 / self.scrl_range as f64 / rate).ceil() as usize;
            //  let move_cur_x = (row_max_chars as f64 / (self.scrl_range - self.view.x) as f64).ceil() as usize;
            //  let move_cur_x = (row_max_chars as f64 / (self.scrl_range) as f64).round() as usize;
            let move_cur_x = ((row_max_width - view_width) as f64 / self.scrl_range as f64 / rate).ceil() as usize;

            self.move_char_x = if move_cur_x == 0 { 1 } else { move_cur_x };
        }
    }

    pub fn ctrl_scrollbar_h(&mut self, y: usize, cmd_type: &CmdType, view_x: usize, view_width: usize) {
        if !self.is_show {
            return;
        }
        // scrlbar_h
        let height = Cfg::get().general.editor.scrollbar.horizontal.height;
        match cmd_type {
            CmdType::MouseDownLeft(_, x) if self.view.y <= y && y < self.view.y + height => {
                // self.set_scrlbar_h_posi(x);
                self.is_enable = true;
                // Except on scrl_h
                if view_x <= *x && *x < view_x + view_width {
                    // Excluded if within bar range
                    if !(view_x + self.view.x <= *x && *x < view_x + self.view.x + self.bar_len) {
                        self.view.x = if x + self.bar_len < view_x + view_width {
                            if *x >= view_x {
                                *x - view_x
                            } else {
                                0
                            }
                        } else {
                            self.scrl_range
                        };
                    } else {
                        return;
                    }
                } else {
                    return;
                }
                return;
            }
            CmdType::MouseDragLeftLeft(_, x) if self.is_enable => {
                if 0 < self.view.x {
                    self.view.x -= 1;
                };
            }

            CmdType::MouseDragLeftRight(_, x) if self.is_enable => {
                if self.view.x < self.scrl_range {
                    self.view.x += 1;
                }
            }
            CmdType::MouseDragLeftDown(_, x) | CmdType::MouseDragLeftUp(_, x) if self.is_enable => {
                return;
            }
            _ => self.is_enable = false,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarH {
    pub is_show: bool,
    pub is_show_org: bool,
    pub is_enable: bool,
    pub view: View,
    pub view_org: View,
    pub bar_len: usize,
    pub bar_height: usize,
    pub move_char_x: usize,
    pub scrl_range: usize,
}

impl Default for ScrollbarH {
    fn default() -> Self {
        ScrollbarH { is_show: false, is_show_org: false, is_enable: false, view: View::default(), view_org: View::default(), bar_len: 0, bar_height: 0, move_char_x: 0, scrl_range: 0 }
    }
}
