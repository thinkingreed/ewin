use crate::{
    ewin_com::{_cfg::key::cmd::*, files::file::*, model::*},
    ewin_editor::model::*,
    global_term::*,
    model::*,
    tab::Tab,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use std::{io::ErrorKind, path::Path};

use super::term::*;

impl Terminal {
    pub fn open_file(&mut self, filenm: &str, file_open_type: FileOpenType, tab_opt: Option<&mut Tab>, h_file_opt: Option<&HeaderFile>) -> ActType {
        Log::info("File open start", &filenm);

        let path = Path::new(&filenm);
        let (is_readable, is_writable) = File::is_readable_writable(filenm);

        Log::info("path", &path);

        if !filenm.is_empty() && !path.exists() {
            if file_open_type == FileOpenType::First {
                Terminal::exit_show_msg(&Lang::get().file_not_found);
                return ActType::Exit;
            } else {
                return ActType::Draw(DParts::MsgBar(Lang::get().file_not_found.to_string()));
            };
        } else {
            let mut h_file = HeaderFile::new(filenm);
            if let Some(h_file_org) = h_file_opt {
                h_file.watch_mode = h_file_org.watch_mode;
            }
            let mut tab = if let Some(tab) = tab_opt { tab.clone() } else { self.curt().clone() };

            if !filenm.is_empty() {
                // read
                let result = TextBuffer::read_file(filenm);
                match result {
                    Ok((text_buf, _enc, _new_line, _bom_exsist, _modified_time)) => {
                        h_file.enc = _enc;
                        h_file.nl = _new_line;
                        h_file.bom = _bom_exsist;
                        tab.editor.buf = text_buf;
                        h_file.mod_time = _modified_time;

                        if !is_writable {
                            tab.editor.state.is_read_only = true;
                        }
                    }
                    Err(err) => {
                        let err_str = if err.kind() == ErrorKind::PermissionDenied && !is_readable { Lang::get().no_read_permission.clone() } else { format!("{} {:?}", &Lang::get().file_opening_problem, err) };
                        if self.tabs.is_empty() {
                            Terminal::exit_show_msg(&err_str);
                        } else {
                            return ActType::Draw(DParts::MsgBar(err_str));
                        }
                    }
                }
            }
            Log::info("File info", &h_file);

            match file_open_type {
                FileOpenType::First | FileOpenType::Nomal => {
                    self.add_tab(&mut tab.clone(), h_file, file_open_type);
                    self.curt().editor.set_cur_default();
                }
                FileOpenType::Reopen => {
                    self.reopen_tab(tab.clone(), h_file, file_open_type);
                    self.curt().editor.cmd = Cmd::to_cmd(CmdType::ReOpenFile);
                    self.curt().editor.state.is_changed = false;
                    self.curt().editor.adjust_cur_posi();
                }
            };

            if !filenm.is_empty() {
                self.curt().enable_syntax_highlight(path);
            }

            // for input complement
            for i in 0..tab.editor.buf.len_rows() {
                self.curt().editor.input_comple.analysis_new(i, &tab.editor.buf.char_vec_row(i));
            }

            Log::info_s("File open end");
            return ActType::Next;
        }
    }

    pub fn reopen_curt_file(&mut self) {
        self.curt().clear_curt_tab(true);
        self.set_size();
        let h_file = H_FILE_VEC.get().unwrap().try_lock().unwrap().get_mut(self.tab_idx).unwrap().clone();
        self.open_file(&h_file.fullpath, FileOpenType::Reopen, None, Some(&h_file));
    }

    pub fn close_file(&mut self) -> ActType {
        let act_type = self.curt().prom_show_com(&CmdType::CloseFile);

        if act_type != ActType::Next {
            return act_type;
        }
        if self.tabs.len() == 1 {
            return ActType::Exit;
        } else {
            self.del_tab(self.tab_idx);
            // Redraw the previous tab
            return ActType::Draw(DParts::All);
        }
    }
}
