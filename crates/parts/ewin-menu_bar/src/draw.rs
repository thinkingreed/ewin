use crate::menubar::*;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use ewin_cfg::colors::*;
use ewin_cfg::log::*;
use ewin_const::models::view::*;
use std::io::Write;

impl MenuBar {
    pub fn draw(&self, str_vec: &mut Vec<String>) {
        Log::info_key("MenuBar.draw");
        Log::debug(" self.sel_idx ", &self.sel_idx);

        str_vec.push(format!("{}{}", MoveTo(0, self.view.y as u16), Clear(ClearType::CurrentLine)));

        if self.view.height > 0 {
            let close_btn = format!(" {} ", 'x');
            let left_arrow_btn = "< ".to_string();
            let right_arrow_btn = " >".to_string();

            if self.is_left_arrow_disp {
                str_vec.push(format!("{}{}", &Colors::get_menubar_active_fg_bg(), left_arrow_btn));
            }
            str_vec.push(Colors::get_menubar_default_bg());

            for (i, menu_cont) in self.menu_vec.iter().enumerate() {
                if !menu_cont.is_disp {
                    continue;
                }
                Log::debug("self.on_mouse_idx", &self.on_mouse_idx);
                let state_color = if i == self.sel_idx || i == self.on_mouse_idx { Colors::get_menubar_active_fg_bg() } else { Colors::get_menubar_passive_fg_bg() };
                str_vec.push(format!("{}{}{}", &state_color, &menu_cont.dispnm, Colors::get_menubar_default_bg()));
            }

            Log::debug("self.menu_rest", &self.menu_rest);

            str_vec.push(format!("{}{}", &Colors::get_menubar_default_bg(), &get_space(self.menu_rest)));

            if self.is_right_arrow_disp {
                str_vec.push(format!("{}{}", Colors::get_menubar_active_fg_bg(), right_arrow_btn));
            }

            str_vec.push(format!("{}{}{}", Colors::get_menubar_passive_fg_bg(), close_btn, Colors::get_default_bg()));
        }
    }

    pub fn draw_only<T: Write>(&self, out: &mut T) {
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
