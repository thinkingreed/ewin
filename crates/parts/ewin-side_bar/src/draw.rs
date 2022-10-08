use crate::sidebar::*;
use ewin_cfg::{colors::*, log::*};
use ewin_state::term::*;
use ewin_view::traits::view::*;

impl SideBar {
    pub fn draw(&self, str_vec: &mut Vec<String>) {
        if State::get().sidebar.is_show {
            Log::debug_key("SideBar.draw");

            self.clear_all(str_vec);
            str_vec.push(Colors::get_sidebar_fg_bg());
            self.cont.draw(str_vec);

            Log::debug("self.cont.get_ref_cont_view()", &self.cont.get_ref_cont_view());

            self.cont.draw_scrlbar(str_vec);
        }
    }

    pub fn draw_only<T: std::io::Write>(&self, out: &mut T) {
        Log::debug_key("MsgBar.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
