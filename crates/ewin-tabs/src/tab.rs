use crate::{ewin_editor::model::*, ewin_key::model::*, msgbar::*, statusbar::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*, model::default::*};

use ewin_const::{
    def::*,
    models::{draw::*, evt::*, file::*, model::*},
    term::*,
};
use ewin_file_bar::filebar::*;
use ewin_help::global::*;
use ewin_job::job::Job;
use ewin_key::key::cmd::*;
use ewin_menulist::parts::pulldown::*;
use ewin_prom::{
    cont::parts::pulldown::*,
    each::{enc_nl::*, grep::*, grep_result::*, greping::*, move_row::*, open_file::*, replace::*, save_confirm::*, save_forced::*, save_new_file::*, search::*, watch_file::*},
    model::*,
};
use ewin_state::term::*;
use ewin_utils::{files::file::*, util::*};
use ewin_view::view::*;
use std::{cmp::min, io::Write, path::Path};

impl Tab {
    pub fn save(&mut self, save_type: &SaveFileType) -> ActType {
        Log::debug_key("save");
        Log::debug("save_type", &save_type);

        let mut file_bar = FileBar::get().clone();
        let fullpath = State::get().curt_state().file.fullpath.clone();
        let path = Path::new(&fullpath);

        if save_type == &SaveFileType::NewFile || !path.exists() {
            return self.prom_show_com(&CmdType::SaveFile(SaveFileType::NewFile));
        } else {
            match save_type {
                SaveFileType::Normal | SaveFileType::Confirm => {
                    // Check if the file has been updated after opening
                    let fullpath = State::get().curt_state().file.fullpath.clone();
                    if let Some(latest_modified_time) = File::get_modified_time(&fullpath) {
                        if latest_modified_time > State::get().curt_state().file.mod_time {
                            Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > State::get().tabs.vec.get_mut(self.idx).unwrap().file.mod_time));
                            return self.prom_show_com(&CmdType::SaveFile(SaveFileType::Forced));
                        }
                    }
                }
                SaveFileType::NewFile | SaveFileType::Forced => {}
            }

            let file = State::get().tabs.vec.get_mut(self.idx).unwrap().file.clone();
            Log::info_s(&format!("Save {}, file info {:?}", &file.name, &file));
            let result = self.editor.buf.write_to(&mut State::get().tabs.vec.get_mut(self.idx).unwrap().file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Draw(DParts::AllMsgBar(Lang::get().writing_cannot_convert_encoding.to_string()));
                    } else {
                        if save_type == &SaveFileType::NewFile {
                            View::set_title(&State::get().tabs.vec.get_mut(self.idx).unwrap().file.name);
                        }
                        let mod_time = File::get_modified_time(&State::get().tabs.vec.get_mut(self.idx).unwrap().file.fullpath).unwrap();
                        State::get().tabs.vec.get_mut(self.idx).unwrap().file.mod_time = mod_time;

                        // TODO
                        // TODO
                        // TODO
                        // TODO
                        // prom state => State
                        self.prom.clear();
                        //  self.state.clear();
                        State::get().curt_mut_state().clear();

                        Log::info("Saved file", &file_bar.file_vec.get_mut(self.idx).unwrap());
                        //if self.editor.state.is_changed || save_type == SaveType::NewName || save_type == SaveType::Forced {
                        if State::get().curt_state().editor.is_changed {
                            State::get().curt_mut_state().editor.is_changed = false;
                            if save_type == &SaveFileType::Confirm {
                                Job::send_cmd(CmdType::CloseFileCurt(CloseFileType::Forced));
                                return ActType::None;
                            } else {
                                return ActType::Draw(DParts::All);
                            }
                        } else {
                            return ActType::None;
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
        self.prom.row_bottom_posi = get_term_size().1 - STATUSBAR_ROW_NUM - if HELP.get().unwrap().try_lock().unwrap().is_show { HELP.get().unwrap().try_lock().unwrap().row_num } else { 0 };
        Log::debug("self.prom.row_bottom_posi 222", &self.prom.row_bottom_posi);

        match cmd_type {
            CmdType::FindProm => return self.prom_search(),
            CmdType::ReplaceProm => return self.prom_replace(),
            CmdType::GrepProm => return self.prom_grep(),
            CmdType::GrepingProm(_) => return self.prom_greping(),
            CmdType::GrepResultProm => return self.prom_grep_result(),
            CmdType::MoveRowProm => return self.prom_move_row(),
            CmdType::EncodingProm => return self.prom_enc_nl(),
            CmdType::openFileProm(open_file_type) => return self.prom_open_file(open_file_type),
            CmdType::CloseFileCurt(_) => return self.prom_save_confirm(),
            CmdType::SaveFile(save_type) if &SaveFileType::NewFile == save_type => return self.prom_save_new_file(),
            CmdType::SaveFile(save_type) if &SaveFileType::Forced == save_type => return self.prom_save_forced(),
            CmdType::WatchFileResult => return self.prom_watch_result(),
            _ => ActType::Cancel,
        };

        return ActType::Cancel;
    }
    fn prom_search(&mut self) -> ActType {
        State::get().curt_mut_state().prom = PromState::Search;
        // self.state.prom = PromState::Search;
        self.prom.init(Box::new(PromSearch::new()));
        return ActType::Draw(DParts::All);
    }

    fn prom_save_new_file(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_new_file");
        State::get().curt_mut_state().prom = PromState::SaveNewFile;
        self.prom.init(Box::new(PromSaveNewFile::new(self.editor.get_candidate_new_filenm())));
        if let Ok(pulldown_cont) = self.prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
            let height = min(pulldown_cont.pulldown.menulist.cont.cont_vec.len(), Editor::get_disp_row_num());
            pulldown_cont.pulldown.menulist.init_menu(pulldown_cont.base.row_posi_range.end, Pulldown::MARGIN, height);
        }
        return ActType::Draw(DParts::All);
    }

    fn prom_save_forced(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_forced");
        let last_modified_time = File::get_modified_time(&State::get().curt_state().file.fullpath).unwrap();
        State::get().curt_mut_state().prom = PromState::SaveForced;
        self.prom.init(Box::new(PromSaveForced::new(&State::get().curt_state().file.mod_time, last_modified_time)));
        return ActType::Draw(DParts::All);
    }

    fn prom_save_confirm(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_confirm");
        if State::get().curt_state().editor.is_changed {
            if !State::get().curt_state().is_nomal() {
                self.clear_curt_tab(true);
            }
            self.prom.init(Box::new(PromSaveConfirm::new()));
            // tabs.curt().state.is_save_confirm = true;
            State::get().curt_mut_state().prom = PromState::SaveConfirm;
            return ActType::Draw(DParts::All);
        };
        return ActType::Next;
    }

    fn prom_replace(&mut self) -> ActType {
        State::get().curt_mut_state().prom = PromState::Replase;
        self.prom.init(Box::new(PromReplace::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_open_file(&mut self, open_file_type: &OpenFileType) -> ActType {
        State::get().curt_mut_state().prom = PromState::OpenFile;
        self.prom.init(Box::new(PromOpenFile::new(open_file_type)));
        return ActType::Draw(DParts::All);
    }
    fn prom_move_row(&mut self) -> ActType {
        State::get().curt_mut_state().prom = PromState::MoveRow;
        self.prom.init(Box::new(PromMoveRow::new()));
        return ActType::Draw(DParts::All);
    }

    fn prom_grep(&mut self) -> ActType {
        State::get().curt_mut_state().prom = PromState::Grep;
        self.prom.init(Box::new(PromGrep::new()));
        return ActType::Draw(DParts::All);
    }
    fn prom_greping(&mut self) -> ActType {
        State::get().curt_mut_state().prom = PromState::Greping;
        self.prom.init(Box::new(PromGreping::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_grep_result(&mut self) -> ActType {
        State::get().curt_mut_state().prom = PromState::GrepResult;
        let grep = State::get().curt_state().grep.clone();
        self.prom.init(Box::new(PromGrepResult::new(grep.is_empty, grep.is_cancel)));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_enc_nl(&mut self) -> ActType {
        Log::debug_key("Tab::prom_enc_nl");
        State::get().curt_mut_state().prom = PromState::EncNl;
        self.prom.init(Box::new(PromEncNl::new()));
        self.prom.curt.downcast_mut::<PromEncNl>().unwrap().set_default_choice_enc_nl(&State::get().curt_state().file);
        return ActType::Draw(DParts::All);
    }
    fn prom_watch_result(&mut self) -> ActType {
        Log::debug_key("Tab::prom_watch_result");
        State::get().curt_mut_state().prom = PromState::WatchFile;
        self.prom.init(Box::new(PromWatchFile::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn clear_curt_tab(&mut self, is_clear_editor_state: bool) -> ActType {
        Log::debug_key("clear_curt_tab");
        self.prom.clear();
        // self.state.clear();
        State::get().curt_mut_state().clear();

        self.msgbar.clear();
        if is_clear_editor_state {
            self.editor.cancel_state();
        }
        if !State::get().curt_state().grep.search_str.is_empty() {
            let _ = self.prom_show_com(&CmdType::GrepResultProm);
        };

        return ActType::Draw(DParts::All);
    }

    pub fn is_prom_pulldown(&self) -> bool {
        if !State::get().curt_state().is_nomal() {
            for cont in self.prom.curt.as_base().cont_vec.iter() {
                if let Ok(pulldown_cont) = cont.downcast_ref::<PromContPulldown>() {
                    if pulldown_cont.pulldown.is_show {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn enable_syntax_highlight(&mut self) {
        let file = State::get().curt_state().file.clone();
        if CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&file.ext).cloned().is_some() && file.len < Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 10240000.0 as u64 && is_enable_syntax_highlight(&file.ext) {
            self.editor.is_enable_syntax_highlight = true;
        }
    }

    pub fn draw_editor_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("MsgBar.draw_only");

        let mut v: Vec<String> = vec![];
        self.draw_editor(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_editor(&mut self, str_vec: &mut Vec<String>) {
        // Editor
        match self.editor.draw_range {
            E_DrawRange::Not => {}
            _ => {
                if self.editor.get_curt_row_len() > 0 {
                    let curt_win = self.editor.get_curt_ref_win().clone();
                    if self.editor.draw_range == E_DrawRange::MoveCur {
                        self.editor.draw_move_cur(str_vec, &curt_win);
                        return;
                    }

                    let vec = self.editor.win_mgr.win_list.clone();
                    for (v_idx, vec_v) in vec.iter().enumerate() {
                        for (h_idx, win) in vec_v.iter().enumerate() {
                            if self.editor.draw_range == E_DrawRange::WinOnlyAll && &curt_win != win {
                                continue;
                            }
                            self.editor.draw_cache(win);
                            self.editor.draw_main(str_vec, &self.editor.draw_cache[v_idx][h_idx], win);
                            self.editor.draw_cache[v_idx][h_idx].cells_from = std::mem::take(&mut self.editor.draw_cache[v_idx][h_idx].cells_to);
                            self.editor.draw_scale(str_vec, win);
                            self.editor.draw_scrlbar_v(str_vec, win);
                            self.editor.draw_scrlbar_h(str_vec, win);
                        }
                    }
                }
                self.editor.draw_window_split_line(str_vec);
                str_vec.push(Colors::get_default_fg_bg());
            }
        };
    }

    pub fn new() -> Self {
        Tab { idx: 0, editor: Editor::new(), msgbar: MsgBar::new(), prom: Prom::default(), sbar: StatusBar::new() }
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub idx: usize,
    pub editor: Editor,
    pub msgbar: MsgBar,
    pub prom: Prom,
    pub sbar: StatusBar,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
