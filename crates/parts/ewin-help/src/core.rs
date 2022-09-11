use crate::{global::*, help::*};
use parking_lot::MutexGuard;

impl Help {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, Help> {
        return HELP.get().unwrap().lock();
    }
}
