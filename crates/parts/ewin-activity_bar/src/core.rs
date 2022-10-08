use crate::{activitybar::*, global::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_state::term::*;
use parking_lot::MutexGuard;

impl ActivityBar {
    pub fn get_width(&mut self) -> usize {
        Log::debug_key("ActivityBar.get_width");

        return if State::get().activitybar.is_show { Cfg::get().general.activitybar.width } else { 0 };
    }

    #[track_caller]
    pub fn get() -> MutexGuard<'static, ActivityBar> {
        return ACTIVITY_BAR.get().unwrap().try_lock().unwrap();
    }

    #[track_caller]
    pub fn get_result() -> Option<MutexGuard<'static, ActivityBar>> {
        return ACTIVITY_BAR.get().unwrap().try_lock();
    }
}
