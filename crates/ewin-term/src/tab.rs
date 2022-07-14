use crate::{
    bar::{msgbar::*, statusbar::*},
    ewin_com::{files::file::*, model::*},
    ewin_editor::model::*,
    global_term::*,
    model::*,
    terms::term::*,
};
use ewin_cfg::{
    lang::lang_cfg::*,
    log::*,
    model::default::{Cfg, CfgSyntax},
};
use ewin_com::{_cfg::key::cmd::*, util::*};
use ewin_const::def::*;
use ewin_editor::window::*;
use ewin_prom::{
    cont::parts::pulldown::*,
    each::{enc_nl::*, grep::*, grep_result::*, greping::*, move_row::*, open_file::*, replace::*, save_confirm::*, save_forced::*, save_new_file::*, search::*, watch_file::*},
    model::*,
};
use ewin_widget::widget::pulldown::*;
use std::{cmp::min, ffi::OsStr, fs::metadata, path::Path};

impl Tab {
    pub fn save(&mut self, save_type: SaveType) -> ActType {
        Log::debug_key("save");
        Log::debug("save_type", &save_type);

        let mut vec = H_FILE_VEC.get().unwrap().try_lock().unwrap();
        let path = Path::new(&vec.get(self.idx).unwrap().fullpath);
        if save_type != SaveType::NewName && (!path.is_file() || !path.exists()) {
            return self.prom_show_com(&CmdType::SaveNewFile);
        } else {
            match save_type {
                SaveType::Normal => {
                    // Check if the file has been updated after opening
                    if let Some(latest_modified_time) = File::get_modified_time(&vec.get_mut(self.idx).unwrap().fullpath) {
                        if latest_modified_time > vec.get_mut(self.idx).unwrap().mod_time {
                            Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > vec.get_mut(self.idx).unwrap().mod_time));
                            return self.prom_show_com(&CmdType::Saveforced);
                        }
                    }
                }
                SaveType::NewName | SaveType::Forced => {}
            }

            let h_file = vec.get_mut(self.idx).unwrap().clone();
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));
            let result = self.editor.buf.write_to(&h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Draw(DParts::AllMsgBar(Lang::get().writing_cannot_convert_encoding.to_string()));
                    } else {
                        if save_type == SaveType::NewName {
                            Terminal::set_title(&vec.get_mut(self.idx).unwrap().filenm);
                        }
                        vec.get_mut(self.idx).unwrap().mod_time = File::get_modified_time(&vec.get_mut(self.idx).unwrap().fullpath).unwrap();

                        self.prom.clear();
                        self.state.clear();

                        Log::info("Saved file", &vec.get_mut(self.idx).unwrap());
                        if self.editor.state.is_changed || save_type == SaveType::NewName || save_type == SaveType::Forced {
                            self.editor.state.is_changed = false;
                            return ActType::Draw(DParts::All);
                        } else {
                            return ActType::Draw(DParts::None);
                        };
                    }
                }
                Err(err) => {
                    Log::error("err", &err.to_string());
                    return ActType::Draw(DParts::AllMsgBar(format!("{} {:?}", Lang::get().file_saving_problem, err.kind())));
                }
            }
        }
    }

    pub fn prom_show_com(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("Tab::prom_show_com");
        Log::debug("cmd_type", &cmd_type);
        Log::debug("self.prom.row_bottom_posi 111", &self.prom.row_bottom_posi);
        self.prom.row_bottom_posi = get_term_size().1 - STATUSBAR_ROW_NUM - if HELP_DISP.get().unwrap().try_lock().unwrap().is_disp { HELP_DISP.get().unwrap().try_lock().unwrap().row_num } else { 0 };
        Log::debug("self.prom.row_bottom_posi 222", &self.prom.row_bottom_posi);

        match cmd_type {
            CmdType::FindProm => return self.prom_search(),
            CmdType::ReplaceProm => return self.prom_replace(),
            CmdType::GrepProm => return self.prom_grep(),
            CmdType::GrepingProm => return self.prom_greping(),
            CmdType::GrepResultProm => return self.prom_grep_result(),
            CmdType::MoveRowProm => return self.prom_move_row(),
            CmdType::EncodingProm => return self.prom_enc_nl(),
            CmdType::openFileProm(open_file_type) => return self.prom_open_file(open_file_type),
            CmdType::CloseFile => return self.prom_save_confirm(),
            CmdType::SaveNewFile => return self.prom_save_new_file(),
            CmdType::Saveforced => return self.prom_save_forced(),
            CmdType::WatchFileResult => return self.prom_watch_result(),
            _ => ActType::Cancel,
        };

        return ActType::Cancel;
    }
    fn prom_search(&mut self) -> ActType {
        self.state.prom = PromState::Search;
        self.prom.init(Box::new(PromSearch::new()));
        return ActType::Draw(DParts::All);
    }

    fn prom_save_new_file(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_new_file");
        self.state.prom = PromState::SaveNewFile;
        self.prom.init(Box::new(PromSaveNewFile::new(self.editor.get_candidate_new_filenm())));
        if let Ok(pulldown_cont) = self.prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
            let height = min(pulldown_cont.pulldown.widget.cont.cont_vec.len(), Editor::get_disp_row_num());
            pulldown_cont.pulldown.widget.init_menu(pulldown_cont.base.row_posi_range.end, Pulldown::MARGIN, height);
        }
        return ActType::Draw(DParts::All);
    }

    fn prom_save_forced(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_forced");
        let vec = H_FILE_VEC.get().unwrap().try_lock().unwrap();
        let h_file = vec.get(self.idx).unwrap();
        let last_modified_time = File::get_modified_time(&h_file.fullpath).unwrap();
        self.state.prom = PromState::SaveForced;
        self.prom.init(Box::new(PromSaveForced::new(&h_file.mod_time, last_modified_time)));
        return ActType::Draw(DParts::All);
    }

    fn prom_save_confirm(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_confirm");
        if self.editor.state.is_changed {
            if !self.state.is_nomal() {
                self.clear_curt_tab(true);
            }
            self.prom.init(Box::new(PromSaveConfirm::new()));
            // term.curt().state.is_save_confirm = true;
            self.state.prom = PromState::SaveConfirm;
            return ActType::Draw(DParts::All);
        };
        return ActType::Next;
    }

    fn prom_replace(&mut self) -> ActType {
        self.state.prom = PromState::Replase;
        self.prom.init(Box::new(PromReplace::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_open_file(&mut self, open_file_type: &OpenFileType) -> ActType {
        self.state.prom = PromState::OpenFile;
        self.prom.init(Box::new(PromOpenFile::new(open_file_type)));
        return ActType::Draw(DParts::All);
    }
    fn prom_move_row(&mut self) -> ActType {
        self.state.prom = PromState::MoveRow;
        self.prom.init(Box::new(PromMoveRow::new()));
        return ActType::Draw(DParts::All);
    }

    fn prom_grep(&mut self) -> ActType {
        self.state.prom = PromState::Grep;
        self.prom.init(Box::new(PromGrep::new()));
        return ActType::Draw(DParts::All);
    }
    fn prom_greping(&mut self) -> ActType {
        self.state.prom = PromState::Greping;
        self.prom.init(Box::new(PromGreping::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_grep_result(&mut self) -> ActType {
        self.state.prom = PromState::GrepResult;
        self.prom.init(Box::new(PromGrepResult::new(self.state.grep.is_empty, self.state.grep.is_cancel)));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_enc_nl(&mut self) -> ActType {
        Log::debug_key("Tab::prom_enc_nl");
        self.state.prom = PromState::EncNl;
        self.prom.init(Box::new(PromEncNl::new()));
        self.prom.curt.downcast_mut::<PromEncNl>().unwrap().set_default_choice_enc_nl(H_FILE_VEC.get().unwrap().try_lock().unwrap().get(self.idx).unwrap());
        return ActType::Draw(DParts::All);
    }
    fn prom_watch_result(&mut self) -> ActType {
        Log::debug_key("Tab::prom_watch_result");
        self.state.prom = PromState::WatchFile;
        self.prom.init(Box::new(PromWatchFile::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn clear_curt_tab(&mut self, is_clear_editor_state: bool) {
        Log::debug_key("clear_curt_tab");
        self.prom.clear();
        self.state.clear();
        self.msgbar.clear();
        if is_clear_editor_state {
            self.editor.cancel_state();
        }
        if self.is_grep_result_state() {
            let _ = self.prom_show_com(&CmdType::GrepResultProm);
        };
    }

    pub fn is_grep_result_state(&mut self) -> bool {
        return !self.state.grep.search_str.is_empty();
    }

    pub fn is_prom_pulldown(&self) -> bool {
        if self.state.prom != PromState::None {
            for cont in self.prom.curt.as_base().cont_vec.iter() {
                if let Ok(pulldown_cont) = cont.downcast_ref::<PromContPulldown>() {
                    if pulldown_cont.pulldown.is_disp {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    pub fn draw_cache(&mut self, win: &Window) {
        self.editor_draw_vec[win.v_idx][win.h_idx].draw_cache(&self.editor, win);
    }

    pub fn enable_syntax_highlight(&mut self, path: &Path) {
        let file_meta = metadata(path).unwrap();
        let ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
        //  self.editor_draw_vec[self.idx].syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
        let win_v_idx = self.editor.win_mgr.win_v_idx;
        let win_h_idx = self.editor.win_mgr.win_h_idx;
        self.editor_draw_vec[win_v_idx][win_h_idx].syntax_reference = CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&ext).cloned();

        Log::debug("file_meta.len()", &file_meta.len());
        Log::debug("Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 1024.0 as u64", &(Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 1024.0 as u64));

        if self.editor_draw_vec[win_v_idx][win_h_idx].syntax_reference.is_some() && file_meta.len() < Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 10240000.0 as u64 && is_enable_syntax_highlight(&ext) {
            self.editor.is_enable_syntax_highlight = true;
        }
    }

    pub fn resize_editor_draw_vec(&mut self) {
        let vec = self.editor.win_mgr.win_list.clone();
        self.editor_draw_vec.resize_with(vec.len(), Vec::new);
        let editor_draw = self.editor_draw_vec[0].get_mut(0).unwrap().clone();
        for (i, v) in vec.iter().enumerate() {
            self.editor_draw_vec[i].resize(v.len(), editor_draw.clone());
        }
    }

    pub fn set_syntax_highlight(&mut self) {
        for (v_idx, vec_v) in self.editor.win_mgr.win_list.iter().enumerate() {
            for (h_idx, _) in vec_v.iter().enumerate() {
                self.editor_draw_vec[v_idx][h_idx].syntax_reference = CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&H_FILE_VEC.get().unwrap().try_lock().unwrap().get(self.idx).unwrap().ext).cloned();
            }
        }
    }

    pub fn new() -> Self {
        Tab { idx: 0, editor: Editor::new(), editor_draw_vec: vec![], msgbar: MsgBar::new(), prom: Prom::default(), sbar: StatusBar::new(), state: TabState::default() }
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub idx: usize,
    pub editor: Editor,
    pub editor_draw_vec: Vec<Vec<EditorDraw>>,
    pub msgbar: MsgBar,
    pub prom: Prom,
    pub sbar: StatusBar,
    pub state: TabState,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
