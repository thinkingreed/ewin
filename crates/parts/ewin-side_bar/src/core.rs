use crate::{explorer::explorer::*, global::*, sidebar::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::def::*;
use ewin_key::key::cmd::CmdType;
use ewin_state::term::*;
use ewin_view::{scrollbar::scrl_h_trait::*, traits::view::*};
use parking_lot::MutexGuard;

impl SideBar {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, SideBar> {
        return SIDE_BAR.get().unwrap().lock();
    }

    #[track_caller]
    pub fn get_result() -> Option<MutexGuard<'static, SideBar>> {
        return SIDE_BAR.get().unwrap().try_lock();
    }

    pub fn get_width_all(&mut self) -> usize {
        Log::debug_key("SideBar.get_width_all");

        Log::debug("State::get().sidebar.is_show", &State::get().sidebar.is_show);

        return if State::get().sidebar.is_show { CfgEdit::get().general.sidebar.width + SIDEBAR_SPLIT_LINE_WIDTH } else { 0 };
    }

    pub fn init(&mut self, tgt_file: &str, is_force_show: bool) {
        Log::debug_key("SideBar.init");

        if CfgEdit::get().general.sidebar.width > 0 || is_force_show {
            State::get().sidebar.is_show = true;

            self.set_init_width();
            self.cont = Explorer::create_cont(tgt_file);
            self.set_size();
            self.cont.downcast_mut::<Explorer>().unwrap().open_file(tgt_file);

            Log::debug("self.cont.as_mut_base().view", &self.cont.as_mut_base().view);

            self.init_scrlbar_h();
            self.calc_scrlbar();
        } else {
            State::get().sidebar.is_show = false;
        }
    }

    pub fn calc_scrlbar(&mut self) {
        if self.cont.as_base().scrl_h.is_show {
            self.cont.calc_scrlbar_h();
        }
        if self.cont.as_base().scrl_v.is_show {
            self.cont.calc_scrlbar_v();
        }
    }
}
