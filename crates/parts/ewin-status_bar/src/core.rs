use parking_lot::MutexGuard;

use crate::{global::*, statusbar::*};

impl StatusBar {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, StatusBar> {
        return STATUS_BAR.get().unwrap().lock();
    }
}
