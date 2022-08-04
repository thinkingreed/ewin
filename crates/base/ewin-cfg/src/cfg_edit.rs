use crate::{global::*, model::default::*};
use std::sync::MutexGuard;

impl CfgEdit {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, Cfg> {
        return CFG_EDIT.get().unwrap().try_lock().unwrap();
    }

    pub fn get_search() -> CfgSearch {
        let regex = CfgEdit::get().general.editor.search.regex;
        let case_sensitive = CfgEdit::get().general.editor.search.case_sensitive;
        return CfgSearch { regex, case_sensitive };
    }
}
