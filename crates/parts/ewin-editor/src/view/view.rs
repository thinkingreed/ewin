use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::model::*;
use ewin_view::view_trait::view_evt_trait::*;

impl ViewEvtTrait for Editor {
    fn resize(&mut self) -> ActType {
        Log::debug_key("Editor.resize");
        self.input_comple.clear();
        return ActType::Draw(DParts::All);
    }
}
