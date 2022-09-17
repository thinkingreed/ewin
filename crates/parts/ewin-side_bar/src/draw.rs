use crate::sidebar::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_state::term::*;
use ewin_view::view_traits::view_trait::*;

impl SideBar {
    pub fn draw(&self, str_vec: &mut Vec<String>) {
        if State::get().sidebar.is_show {
            Log::debug_key("SideBar.draw");

            str_vec.push(Colors::get_sidebar_fg_bg());
            self.clear_all(str_vec);
            self.cont.draw(str_vec);
            self.draw_scrlbar_v(str_vec);
        }
    }

    pub fn draw_only<T: std::io::Write>(&self, out: &mut T) {
        Log::debug_key("MsgBar.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    fn draw_scrlbar_v(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("TreeFileView.draw_scrlbar_v");
        if self.scrl_v.is_show {
            for i in self.cont.get_cont_view().y..self.cont.get_cont_view().y + self.cont.get_cont_view().height {
                str_vec.push(MoveTo(self.scrl_v.view.x as u16, i as u16).to_string());
                str_vec.push(if self.cont.get_cont_view().y + self.scrl_v.view.y <= i && i < self.cont.get_cont_view().y + self.scrl_v.view.y + self.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                str_vec.push(" ".to_string().repeat(self.scrl_v.bar_width));
            }
        }
    }
}
