use crate::{sidebar::*, traits::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*};
use ewin_key::key::cmd::*;

impl SideBar {
    pub fn set_size_scrlbar_h(&mut self) {
        Log::debug_key("SideBar.set_size_scrlbar_h");
        self.scrl_h.is_show = self.scrl_h_info.row_max_width > self.cont.as_base().view.width;

        if self.scrl_h.is_show {
            self.scrl_h.view.y = self.cont.as_base().view.y_height() - 1;
            self.scrl_h.view.x = self.cont.as_base().view.x;
            self.scrl_h.view.width = self.cont.as_base().view.width;
            self.scrl_h.view.height = Cfg::get().general.sidebar.scrollbar.horizontal.height;

            self.cont.get_mut_cont_view().height -= self.scrl_h.view.height;
        }
    }
}
