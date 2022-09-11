use crate::{global::*, sidebar::*, tree_file_view::tree::*};
use ewin_cfg::{log::Log, model::general::default::*};
use ewin_const::def::*;
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

        return if State::get().term.is_sidebar_show { CfgEdit::get().general.sidebar.width + SIDEBAR_SPLIT_LINE_WIDTH } else { 0 };
    }

    pub fn init(&mut self, tgt_file: &str, is_force_show: bool) {
        if CfgEdit::get().general.sidebar.width > 0 || is_force_show {
            self.set_init_width();
            State::get().term.is_sidebar_show = true;
            self.cont = TreeFileView::create_cont(tgt_file);
        } else {
            State::get().term.is_sidebar_show = false;
        }
    }
}
