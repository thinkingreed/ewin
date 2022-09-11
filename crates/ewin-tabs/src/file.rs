use crate::{ewin_editor::model::*, tab::*, tabs::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};

use ewin_const::{
    def::*,
    models::{draw::*, event::*, file::*},
};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_state::term::*;
use ewin_utils::files::file::*;
use std::path::Path;

impl Tabs {
    pub fn open_file(&mut self, filenm: &str, file_open_type: FileOpenType, tab: Tab, file_org_opt: Option<&File>) -> ActType {
        Log::info("File open start", &filenm);
        let mut tab = tab;

        let absolute_path = if filenm.is_empty() { "".to_string() } else { File::get_absolute_path(filenm) };
        let path = Path::new(&absolute_path);
        let (_, is_writable) = File::is_readable_writable(filenm);
        Log::info("path", &path);

        if !filenm.is_empty() && !path.exists() {
            if self.vec.is_empty() {
                return ActType::ExitMsg(Lang::get().file_not_found.clone());
            } else {
                return ActType::Draw(DrawParts::MsgBar(Lang::get().file_not_found.to_string()));
            };
        } else {
            let mut file = File::new(&absolute_path);
            Log::info("file", &file);

            if let Some(h_file_org) = file_org_opt {
                file.watch_mode = h_file_org.watch_mode;
            }
            let mut is_read_only = false;
            if !filenm.is_empty() {
                let specify_encode_opt = match file_open_type {
                    FileOpenType::ReopenEncode(encode) => Some(encode),
                    _ => None,
                };

                let result = TextBuffer::read_file(&mut file, specify_encode_opt);
                match result {
                    Ok((text_buf, _bom_exsist)) => {
                        file.bom = _bom_exsist;
                        tab.editor.buf = text_buf;
                        if !is_writable {
                            is_read_only = true;
                        }
                    }
                    Err(err) => {
                        let err_str = File::get_io_err_str(err);
                        if self.vec.is_empty() {
                            return ActType::ExitMsg(err_str);
                        } else {
                            return ActType::Draw(DrawParts::MsgBar(err_str));
                        }
                    }
                }
            }
            Log::info("File info", &file);

            match file_open_type {
                FileOpenType::Nomal => {
                    self.add_tab(&mut tab, file, file_open_type);
                    self.curt().editor.set_cur_default();
                }
                FileOpenType::Reopen | FileOpenType::ReopenEncode(_) => {
                    self.reopen_tab(tab.clone(), file, file_open_type);
                    self.curt().editor.cmd = Cmd::to_cmd(CmdType::ReOpenFile(file_open_type));
                    State::get().curt_mut_state().editor.is_changed = false;
                    self.curt().editor.adjust_cur_posi();
                }
            };

            if is_read_only {
                State::get().curt_mut_state().editor.is_read_only = true;
            }

            if !filenm.is_empty() {
                self.curt().editor.enable_syntax_highlight();
            }

            // for input complement
            for i in 0..tab.editor.buf.len_rows() {
                self.curt().editor.input_comple.analysis_new(i, &tab.editor.buf.char_vec_row(i));
            }

            Log::info_s("File open end");
            return ActType::Next;
        }
    }

    pub fn reopen_curt_file(&mut self, file_open_type: FileOpenType) -> ActType {
        self.curt().clear_curt_tab(true);
        self.set_size();
        let file = State::get().curt_state().file.clone();

        let tab = self.curt().clone();
        self.open_file(&file.fullpath, file_open_type, tab, Some(&file));

        return ActType::Draw(DrawParts::TabsAll);
    }

    pub fn close_file(&mut self, tab_idx: usize, close_type: CloseFileType) -> ActType {
        Log::debug_key("Tabs::close_file");
        Log::debug("tab_idx", &tab_idx);

        if State::get().tgt_state(tab_idx).editor.is_changed && close_type == CloseFileType::Normal {
            Log::debug_s("is_changed");
            self.idx = tab_idx;
            State::get().tabs.idx = tab_idx;

            return self.curt().prom_show_com(&CmdType::CloseFileCurt(CloseFileType::Normal));
        } else if self.vec.len() == 1 {
            return ActType::Exit;
        } else {
            self.del_tab(tab_idx);

            if State::get().tabs_all().is_all_close_confirm {
                // TODO TEST
                // TODO TEST
                // TODO TEST
                Job::send_cmd(CmdType::CloseOtherThanThisTab(USIZE_UNDEFINED));
                return ActType::None;
            } else if State::get().tabs.all.close_other_than_this_tab_idx != USIZE_UNDEFINED {
                Job::send_cmd(CmdType::CloseOtherThanThisTab(State::get().tabs.all.close_other_than_this_tab_idx));
                return ActType::None;
            }
            return ActType::Draw(DrawParts::TabsAll);
        }
    }
}
