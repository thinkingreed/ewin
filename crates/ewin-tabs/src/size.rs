use crate::tabs::*;
use ewin_cfg::log::*;
use ewin_editor::editor_gr::*;
use ewin_file_bar::filebar::*;
use ewin_key::model::*;
use ewin_prom::model::*;
use ewin_state::term::*;
use ewin_view::traits::view::*;

impl Tabs {
    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");

        FileBar::get().set_size();
        Prom::get().set_size();

        if State::get().curt_ref_state().prom == PromState::OpenFile {
            EditorGr::get().curt_mut().view.height = 0;
        } else {
            EditorGr::get().curt_mut().set_size();
        }

        return true;
    }
}
