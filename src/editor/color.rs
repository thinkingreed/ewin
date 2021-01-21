use crate::{def::*, model::*};
use std::collections::BTreeMap;
use termion::color::*;

impl Editor {
    pub fn ctl_new_line_mark_color(&mut self, str_vec: &mut Vec<String>, c: char) {
        if c == NEW_LINE {
            Colors::set_new_line_color(str_vec);
            self.is_default_color = false;
        } else {
            if !self.is_default_color {
                // Log::ep_s("textarea_color textarea_color textarea_color");
                Colors::set_textarea_color(str_vec);
                self.is_default_color = true;
            }
        }
    }
    pub fn is_ctrl_char(&mut self, str_vec: &mut Vec<String>, c: char) -> CharStyleType {
        if c == NEW_LINE {
            CharStyleType::CtrlChar
        } else {
            CharStyleType::Nomal
        }
    }

    pub fn set_eof(&mut self, str_vec: &mut Vec<String>) {
        Colors::set_new_line_color(str_vec);
        str_vec.push(EOF_STR.to_string());
        Colors::set_textarea_color(str_vec);
    }
    /*

        pub fn set_eof(&mut self, i: usize) {
            self.draw_cache.entry(i).and_modify(|s| s.push_str(&Fg(Rgb(110, 110, 110)).to_string()));
            self.draw_cache.entry(i).and_modify(|s| s.push_str(&Bg(Rgb(0, 0, 0)).to_string()));
            self.draw_cache.entry(i).and_modify(|s| s.push_str(&EOF_STR.to_string()));
            self.draw_cache.entry(i).and_modify(|s| s.push_str(&Fg(Rgb(110, 110, 110)).to_string()));
            self.draw_cache.entry(i).or_insert("".to_string()).push_str(&Bg(Rgb(255, 255, 255)).to_string());
        }
    */
}
