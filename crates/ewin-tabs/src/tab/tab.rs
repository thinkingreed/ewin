use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
    term::*,
};
use ewin_editor::{editor_gr::*, model::*};
use ewin_help::{global::*, help::*};
use ewin_key::key::cmd::*;
use ewin_msg_bar::msgbar::*;
use ewin_prom::{
    each::{enc_nl::*, grep::*, grep_result::*, greping::*, move_row::*, open_file::*, replace::*, save_confirm::*, save_forced::*, save_new_file::*, search::*, watch_file::*},
    model::*,
};
use ewin_state::term::*;

impl Tab {
    pub fn prom_show_com(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("Tab::prom_show_com");
        Log::debug("cmd_type", &cmd_type);
        Log::debug("self.prom.row_bottom_posi 111", &Prom::get().row_bottom_posi);
        Prom::get().row_bottom_posi = get_term_size().1 - STATUSBAR_HEIGHT - if Help::get().is_show { Help::get().view.height } else { 0 } - MSGBAR_HEIGHT;
        Log::debug("self.prom.row_bottom_posi 222", &Prom::get().row_bottom_posi);

        match cmd_type {
            CmdType::FindProm => return PromSearch::init(),
            CmdType::ReplaceProm => return PromReplace::init(),
            CmdType::GrepProm => return PromGrep::init(),
            CmdType::GrepingProm(_) => return PromGreping::init(),
            CmdType::GrepResultProm => return PromGrepResult::init(),
            CmdType::MoveRowProm => {
                let rnw = EditorGr::get().curt_ref().get_rnw();
                let rnwlen_rows = EditorGr::get().curt_ref().buf.len_rows();
                return PromMoveRow::init(rnw, rnwlen_rows);
            }
            CmdType::EncodingProm => return PromEncNl::init(),
            CmdType::openFileProm(open_file_type) => return PromOpenFile::init(open_file_type),
            CmdType::CloseFileCurt(_) => return PromSaveConfirm::init(),
            CmdType::SaveNewFileProm => return PromSaveNewFile::init(EditorGr::get().curt_mut().get_candidate_new_filenm(), Editor::get_disp_row_num()),
            CmdType::SaveForceProm => return PromSaveForced::init(),
            CmdType::WatchFileResultProm => return PromWatchFile::init(),
            _ => ActType::Cancel,
        };

        return ActType::Cancel;
    }

    pub fn clear_curt_tab(&mut self, is_clear_editor_state: bool) -> ActType {
        Log::debug_key("clear_curt_tab");
        Prom::get().clear();
        // self.state.clear();
        State::get().curt_mut_state().clear();

        MsgBar::get().clear();
        if is_clear_editor_state {
            EditorGr::get().curt_mut().cancel_state();
        }
        if !State::get().curt_ref_state().grep.search.str.is_empty() {
            let _ = self.prom_show_com(&CmdType::GrepResultProm);
        };

        return ActType::Draw(DrawParts::TabsAll);
    }

    pub fn resize_draw_vec(&mut self) {
        let vec = EditorGr::get().curt_ref().win_mgr.win_list.clone();
        self.draw_cache_vecs.resize_with(vec.len(), Vec::new);
        let editor_draw = self.draw_cache_vecs[0].get_mut(0).unwrap().clone();
        for (i, v) in vec.iter().enumerate() {
            self.draw_cache_vecs[i].resize(v.len(), editor_draw.clone());
        }
    }

    pub fn new() -> Self {
        Tab { draw_cache_vecs: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub draw_cache_vecs: Vec<Vec<EditorDrawCache>>,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
