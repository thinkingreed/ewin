use crate::{btn_grourp::*, dialog::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_utils::str_edit::*;
use std::{
    cmp::{max, min},
    io::Write,
    ops::Range,
};

impl Dialog {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("Dialog.draw");
        if self.is_show {
            Log::debug("self.is_show", &self.is_show);
            let ey = self.view.y + self.view.height;
            for i in self.view.y..ey {
                str_vec.push(format!("{}", MoveTo(self.view.x as u16, i as u16)));

                // title
                if i == self.view.y {
                    str_vec.push(Colors::get_dialog_fg_bg_header());
                    let rest = self.view.width - get_str_width(&self.cont.as_base().title) - 1;
                    let (pre_color, post_color) = if self.close_btn.is_on_mouse { (Colors::get_dialog_bg_sel(), Colors::get_dialog_bg_default()) } else { ("".to_string(), "".to_string()) };

                    str_vec.push(format!(" {}{}{}{}{}", self.cont.as_base().title, " ".repeat(rest - Dialog::CLOSE_BTN_WIDTH), pre_color, " x ", post_color));
                    str_vec.push(Colors::get_dialog_fg_bg_default());

                    // dividing line
                } else if i == ey - 2 {
                    let margin = " ";
                    str_vec.push(format!("{}{}{}", margin, "─".repeat(self.view.width - margin.len() * 2), margin));
                    // btn area
                } else if i == ey - 1 {
                    match self.btn_group.btn_type {
                        DialogBtnGrourpType::Ok => {
                            let (pre_color, post_color) = if self.btn_group.vec[0].view.is_on_mouse { (Colors::get_dialog_bg_sel(), Colors::get_dialog_bg_default()) } else { ("".to_string(), "".to_string()) };
                            str_vec.push(format!("{}[{}{}{}]{}", " ".repeat(self.btn_group.vec[0].view.x - 1 - self.view.x), pre_color, self.btn_group.vec[0].name, post_color, " ".repeat(self.view.x + self.view.width - (self.btn_group.vec[0].view.x + self.btn_group.vec[0].view.width + 1))));
                        }
                        DialogBtnGrourpType::OkCancel => {}
                    };
                } else if self.cont.as_base().cont_vec.len() > i - self.view.y - Dialog::HEADER_HEIGHT {
                    str_vec.push(self.cont.as_base().cont_vec[i - self.view.y - Dialog::HEADER_HEIGHT].clone());
                }
            }
            str_vec.push(Colors::get_default_fg_bg());
        }
    }

    pub fn draw_only<T: Write>(out: &mut T) {
        Log::debug_key("Dialog::draw_only");
        let mut v: Vec<String> = vec![];
        Dialog::get().draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn get_draw_range_y(&self) -> Range<usize> {
        return Range { start: min(self.view.y, self.view.y_org), end: max(self.view.y, self.view.y_org) + self.view.height };
    }
}
