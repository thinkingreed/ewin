// use std::sync::MutexGuard;

use parking_lot::MutexGuard;

use crate::{global::*, msgbar::*};

impl MsgBar {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, MsgBar> {
        return MSG_BAR.get().unwrap().lock();
    }
}
