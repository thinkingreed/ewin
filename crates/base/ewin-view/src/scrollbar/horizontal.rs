use crate::view::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::Colors, log::*, model::general::default::*};
use ewin_const::models::view::*;
use ewin_key::key::cmd::*;
use std::cmp::{max, min};

impl ScrollbarH {
    // Including new line code
    pub const SCROLL_BAR_H_END_LINE_MARGIN: usize = 4;

    pub fn calc_scrlbar_h(&mut self, view_width: usize, scrl_h_info: &ScrlHInfo, offset_disp_x: usize) {
        Log::debug_s("ScrollbarH.calc_editor_scrlbar_h");

        self.view.width = max(2, min(view_width - 1, (view_width as f64 / scrl_h_info.row_max_width as f64 * view_width as f64).floor() as usize));

        Log::debug("self.view.x 111", &self.view.x);

        if !self.is_enable {
            self.view.x = min(self.scrl_range, (self.scrl_range as f64 * offset_disp_x as f64 / (scrl_h_info.row_max_width - view_width) as f64).ceil() as usize);
        }
        Log::debug("self.view.x 222", &self.view.x);

        if scrl_h_info.row_max_width > view_width {
            Log::debug("self.view.x 33333333333333333333333333333333333", &self.view.x);

            self.scrl_range = view_width - self.view.width;

            let rate = scrl_h_info.row_max_width as f64 / scrl_h_info.row_max_chars as f64;

            //  let move_cur_x = ((row_max_width - view_width) as f64 / self.scrl_range as f64 / rate).ceil() as usize;
            //  let move_cur_x = (row_max_chars as f64 / (self.scrl_range - self.view.x) as f64).ceil() as usize;
            //  let move_cur_x = (row_max_chars as f64 / (self.scrl_range) as f64).round() as usize;
            let move_cur_x = ((scrl_h_info.row_max_width - view_width) as f64 / self.scrl_range as f64 / rate).ceil() as usize;

            self.move_char_x = if move_cur_x == 0 { 1 } else { move_cur_x };
        }
    }

    pub fn ctrl_scrollbar_h(&mut self, cmd_type: &CmdType, view_x: usize, view_width: usize) {
        Log::debug_s("ScrollbarH.ctrl_scrollbar_h");
        if !self.is_show {
            return;
        }
        // scrlbar_h
        let height = Cfg::get().general.editor.scrollbar.horizontal.height;
        match cmd_type {
            CmdType::MouseDownLeft(y, x) if self.view.y <= *y && *y < self.view.y + height => {
                // self.set_scrlbar_h_posi(x);
                self.is_enable = true;
                // Except on scrl_h
                if view_x <= *x && *x < view_x + view_width {
                    // Excluded if within bar range
                    if !(view_x + self.view.x <= *x && *x < view_x + self.view.x + self.view.width) {
                        self.view.x = if x + self.view.width < view_x + view_width {
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
            CmdType::MouseDragLeftLeft(_, _) if self.is_enable => {
                if 0 < self.view.x {
                    self.view.x -= 1;
                };
            }

            CmdType::MouseDragLeftRight(_, _) if self.is_enable => {
                if self.view.x < self.scrl_range - 1 {
                    self.view.x += 1;
                }
            }
            CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) if self.is_enable => {
                return;
            }
            _ => self.is_enable = false,
        };
    }

    pub fn draw(&self, str_vec: &mut Vec<String>, view: &View) {
        Log::debug_key("ScrollbarH.draw");
        Log::debug("self.view", &self.view);
        Log::debug("self.is_show", &self.is_show);

        if self.is_show {
            for i in self.view.y..self.view.y_height() {
                str_vec.push(format!("{}{}", MoveTo(view.x as u16, self.view.y as u16), get_space(view.width)));
                str_vec.push(Colors::get_default_bg());
                str_vec.push(MoveTo((view.x + self.view.x) as u16, i as u16).to_string());
                str_vec.push(Colors::get_scrollbar_h_fg());
                str_vec.push("▄".to_string().repeat(self.view.width));
                str_vec.push(Colors::get_default_bg());
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScrollbarH {
    pub is_show: bool,
    pub is_show_org: bool,
    pub is_enable: bool,
    pub view: View,
    pub view_org: View,
    pub move_char_x: usize,
    pub scrl_range: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScrlHInfo {
    pub row_max_width_idx: usize,
    pub row_max_width: usize,
    pub row_max_width_org: usize,
    pub row_max_chars: usize,
    pub row_width_chars_vec: Vec<(usize, usize)>,
}
