use crate::{global::*, term::*};
use parking_lot::MutexGuard;

impl State {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, State> {
        return TABS.get().unwrap().lock();
    }

    #[track_caller]
    pub fn get_result() -> Option<MutexGuard<'static, State>> {
        return TABS.get().unwrap().try_lock();
    }
}
