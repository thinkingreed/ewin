use crate::statusbar::*;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use ewin_cfg::{colors::*, log::*};
use ewin_const::{models::view::*, term::*};
use ewin_state::term::*;
use ewin_utils::str_edit::*;
use std::fmt::Write;

impl StatusBar {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::info_key("StatusBar.draw");

        let (cur_cont, mut opt_vec) = StatusBar::get_editor_conts();

        let cols = get_term_size().0;
        let file = &State::get().curt_ref_state().file.clone();

        let mut normal_vec = vec![];
        let enc_nl = StatusBarCont::new(format!("{}({})", file.enc, file.nl));
        normal_vec.push(enc_nl.clone());
        normal_vec.push(cur_cont.clone());
        normal_vec.reverse();

        let mut normal_str = String::new();
        for normal in &normal_vec {
            normal_str.push_str(&normal.disp_str);
        }
        let mut opt_str = String::new();
        for opt in opt_vec.iter_mut() {
            if !opt.disp_str.is_empty() {
                let _ = write!(opt_str, "[{}]", opt.disp_str);
            }
        }
        let normal_str_w = get_str_width(&normal_str);
        let opt_str_w = get_str_width(&opt_str);
        let other_w = cols - normal_str_w - opt_str_w;
        let normal_w_s = other_w + opt_str_w;

        self.cur_area = (normal_w_s, normal_w_s + get_str_width(&cur_cont.disp_str) - 1);
        self.enc_nl_area = (self.cur_area.1 + 1, self.cur_area.1 + 1 + get_str_width(&enc_nl.disp_str));

        let mut msg_str = format!("{}{}{}{}{}{}", Colors::get_statusbar_fg_bg(), get_space(other_w), Colors::get_statusbar_inversion_fg_bg(), opt_str, Colors::get_statusbar_fg_bg(), normal_str);
        Log::debug("sbar.view.y", &self.view.y);
        msg_str = format!("{}{}{}", MoveTo(0, self.view.y as u16), Clear(ClearType::CurrentLine), msg_str);

        str_vec.push(msg_str);
        str_vec.push(Colors::get_default_fg_bg());
    }

    pub fn draw_only<T: std::io::Write>(&mut self, out: &mut T) {
        Log::debug_key("StatusBar.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
