use crate::{global::*, model::*};
use ewin_state::term::*;
use parking_lot::MutexGuard;

impl EditorGr {
    #[track_caller]
    pub fn get() -> MutexGuard<'static, EditorGr> {
        return EDITOR_GR.get().unwrap().lock();
    }

    #[track_caller]
    pub fn get_result() -> Option<MutexGuard<'static, EditorGr>> {
        return EDITOR_GR.get().unwrap().try_lock();
    }

    #[track_caller]
    pub fn curt_mut(&mut self) -> &mut Editor {
        return self.vec.get_mut(State::get().tabs.idx).unwrap();
    }

    pub fn curt_ref(&self) -> &Editor {
        return self.vec.get(State::get().tabs.idx).unwrap();
    }

    pub fn add_tab(&mut self, editor: Editor) {
        self.vec.push(editor);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EditorGr {
    pub vec: Vec<Editor>,
}
