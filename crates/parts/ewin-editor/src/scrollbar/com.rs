use crate::model::*;
use ewin_view::{scrollbar::scrl_h_trait::*, traits::view::*};

impl Editor {
    pub fn set_init_scrlbar(&mut self) {
        self.init_scrlbar_h();
        self.set_size();
        self.calc_scrlbar();
    }

    pub fn calc_scrlbar(&mut self) {
        self.calc_scrlbar_h();
        self.calc_scrlbar_v();
    }
}
