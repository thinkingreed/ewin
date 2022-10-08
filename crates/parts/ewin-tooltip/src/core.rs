use crate::{global::*, tooltip::*};
use parking_lot::MutexGuard;

impl ToolTip {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, ToolTip> {
        return TOOLTIP.get().unwrap().try_lock().unwrap();
    }

    #[track_caller]
    pub fn get_result() -> Option<MutexGuard<'static, ToolTip>> {
        return TOOLTIP.get().unwrap().try_lock();
    }
}
