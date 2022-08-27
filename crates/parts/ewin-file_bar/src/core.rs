use crate::{filebar::*, filebar_file::*, global::*};
use ewin_const::def::*;
use ewin_state::term::*;
use tokio::sync::{MutexGuard, TryLockError};

impl FileBar {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, FileBar> {
        return FILE_BAR.get().unwrap().try_lock().unwrap();
    }
    #[track_caller]
    pub fn get_result() -> Result<MutexGuard<'static, FileBar>, TryLockError> {
        return FILE_BAR.get().unwrap().try_lock();
    }

    pub fn add_tab(&mut self) {
        self.disp_base_idx = USIZE_UNDEFINED;
        self.file_vec.push(FilebarFile::new());
    }

    #[track_caller]
    pub fn del_file(&mut self, del_idx: usize) {
        self.file_vec.remove(del_idx);
        self.disp_base_idx = USIZE_UNDEFINED;
    }
}
