use parking_lot::MutexGuard;

use crate::{global::*, menubar::*};

impl MenuBar {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, MenuBar> {
        return MENU_BAR.get().unwrap().try_lock().unwrap();
    }

    #[track_caller]
    pub fn get_result() -> Option<MutexGuard<'static, MenuBar>> {
        return MENU_BAR.get().unwrap().try_lock();
    }

    pub fn new() -> Self {
        MenuBar { ..MenuBar::default() }
    }
}
