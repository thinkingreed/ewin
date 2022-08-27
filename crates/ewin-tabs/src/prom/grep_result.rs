use crate::{tab::*, tabs::*};
use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, evt::*, file::*, key::*},
};

use ewin_job::job::*;
use ewin_key::{global::*, key::cmd::*, model::*};
use ewin_prom::each::grep::*;
use ewin_state::term::*;
use ewin_utils::files::file::*;

use std::{ffi::OsStr, io::Write, path::*};

impl Tabs {
    pub fn draw_grep_result<T: Write>(&mut self, out: &mut T, job_grep: JobGrep) {
        Log::debug_key("draw_grep_result");

        if Grep::is_canceling() {
            Log::debug_s("EvtAct::is_grep_Canceling()");
            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceled).unwrap();
            self.exit_grep_result(out, true);
        } else if Grep::is_canceled() {
            Log::debug_s("EvtAct::is_grep_canceled()");
            return;
        } else if job_grep.is_end {
            Log::debug_s("grep is end");
            self.exit_grep_result(out, false);
        } else {
            if job_grep.grep_str.trim().is_empty() {
                return;
            }
            let path = PathBuf::from(&self.curt().editor.search.fullpath);
            let filenm = path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string();
            let replace_folder = self.curt().editor.search.fullpath.replace(&filenm, "");
            let mut row_str = job_grep.grep_str.replace(&replace_folder, "");
            Log::debug("line_str", &row_str);

            del_nl(&mut row_str);
            row_str.push(NEW_LINE_LF);

            self.curt().editor.set_grep_result(row_str);

            if self.curt().editor.buf.len_rows() > self.curt().editor.get_curt_row_len() && self.curt().editor.rnw_org == self.curt().editor.rnw {
                let y = self.curt().editor.win_mgr.curt().offset.y + self.curt().editor.get_curt_row_len() - 2;
                self.curt().editor.draw_range = E_DrawRange::ScrollDown(y - 2, y);
                self.draw(out, if cfg!(target_os = "windows") { &DParts::All } else { &DParts::ScrollUpDown(ScrollUpDownType::Grep) });
            } else {
                self.draw(out, &DParts::All);
            }
        }
    }

    pub fn exit_grep_result<T: Write>(&mut self, out: &mut T, is_cancel: bool) {
        Log::debug_key("exit_grep_result");

        self.curt().prom.clear();

        let is_empty = self.curt().editor.grep_result_vec.is_empty();
        State::get().curt_mut_state().grep.is_cancel = is_cancel;
        State::get().curt_mut_state().grep.is_empty = is_empty;
        self.curt().prom_show_com(&CmdType::GrepResultProm);
        self.curt().editor.set_cur_default();
        self.curt().editor.scroll();
        State::get().curt_mut_state().editor.is_read_only = true;
        self.curt().editor.init_editor_scrlbar_h();
        self.draw_all(out, DParts::All);
    }

    pub fn grep_result(&mut self) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match self.curt().prom.cmd.cmd_type {
            CmdType::Confirm => {
                let y = self.curt().editor.win_mgr.curt().cur.y;
                let grep_result = self.curt().editor.grep_result_vec[y].clone();
                Log::debug("State::get().curt_mut_state().grep", &State::get().curt_state().grep);

                if grep_result.row_num != USIZE_UNDEFINED {
                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = State::get().curt_state().grep.search_str.clone();
                    tab_grep.editor.search.row_num = grep_result.row_num - 1;
                    tab_grep.editor.set_cmd(Cmd::to_cmd(CmdType::FindNext));
                    Log::debug("tab.editor.search", &tab_grep.editor.search);

                    let folder = if self.curt().editor.search.dir.is_empty() { "".to_string() } else { format!("{}{}", &self.curt().editor.search.dir, MAIN_SEPARATOR) };
                    let act_type = self.open_file(&format!("{}{}", &folder, &grep_result.filenm), FileOpenType::Nomal, Some(&mut tab_grep), None);

                    if let ActType::Draw(DParts::MsgBar(_)) = act_type {
                        return act_type;
                    };
                    Log::debug("tabs.curt().editor.search", &self.curt().editor.search);

                    self.curt().editor.search_str(true, false);
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
