use parking_lot::MutexGuard;

use crate::{global::*, model::*};

impl Prom {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, Prom> {
        return PROM.get().unwrap().lock();
    }
}
