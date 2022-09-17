use crate::{global::*, sidebar::*, tree_file_view::tree::*};
use ewin_cfg::{log::Log, model::general::default::*};
use ewin_const::def::*;
use ewin_key::key::cmd::*;
use ewin_state::term::State;
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
        if CfgEdit::get().general.sidebar.width > 0 || is_force_show {
            self.set_init_width();
            State::get().sidebar.is_show = true;
            self.cont = TreeFileView::create_cont(tgt_file);

            self.judge_show_scrollbar();
            self.scrl_v.calc_scrlbar_v(&CmdType::Null, self.cont.as_base().offset, self.cont.get_cont_view().height, self.cont.get_cont_vec_len(), true)
        } else {
            State::get().sidebar.is_show = false;
        }
    }

    pub fn judge_show_scrollbar(&mut self) {
        if self.cont.get_cont_vec_len() > self.cont.get_cont_view().height {
            self.scrl_v.is_show = true;
            self.scrl_v.bar_width = Cfg::get().general.sidebar.scrollbar.vertical.width;
            self.cont.get_cont_view().width -= self.scrl_v.bar_width;
            self.scrl_v.view.x = self.cont.get_cont_view().width - Cfg::get().general.sidebar.scrollbar.vertical.width;
            self.scrl_v.calc_scrlbar_v(&CmdType::Null, self.cont.as_base().offset, self.cont.get_cont_view().height, self.cont.get_cont_vec_len(), true)
        }
    }
}
