use crate::{
    ewin_com::{_cfg::key::cmd::*, global::*, model::*, util::*},
    model::*,
};
use ewin_cfg::log::*;
use ewin_const::def::*;
use std::{ffi::OsStr, io::Write, path::*};

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::debug_key("draw_grep_result");

        if EvtAct::is_grep_canceling() {
            Log::debug_s("EvtAct::is_grep_Canceling()");
            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceled).unwrap();
            EvtAct::exit_grep_result(out, term, true);
        } else if EvtAct::is_grep_canceled() {
            Log::debug_s("EvtAct::is_grep_canceled()");
            return;
        } else if job_grep.is_end {
            Log::debug_s("grep is end");
            EvtAct::exit_grep_result(out, term, false);
        } else {
            if job_grep.grep_str.trim().is_empty() {
                return;
            }
            let path = PathBuf::from(&term.curt().editor.search.fullpath);
            let filenm = path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string();
            let replace_folder = term.curt().editor.search.fullpath.replace(&filenm, "");
            let mut row_str = job_grep.grep_str.replace(&replace_folder, "");
            Log::debug("line_str", &row_str);

            del_nl(&mut row_str);
            row_str.push(NEW_LINE_LF);

            // New line code is fixed to LF because it is a non-editable file
            term.curt().editor.buf.insert_end(&row_str);

            term.curt().editor.set_grep_result(row_str);

            // let rnw_org = term.curt().editor.rnw_org;
            if term.curt().editor.buf.len_rows() > term.curt().editor.get_curt_row_len() && term.curt().editor.rnw_org == term.curt().editor.rnw {
                let y = term.curt().editor.win_mgr.curt().offset.y + term.curt().editor.get_curt_row_len() - 2;
                term.curt().editor.win_mgr.curt().draw_range = E_DrawRange::ScrollDown(y - 2, y);
                term.draw(out, if cfg!(target_os = "windows") { &DParts::All } else { &DParts::ScrollUpDown(ScrollUpDownType::Grep) });
            } else {
                term.draw(out, &DParts::All);
            }
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, is_cancel: bool) {
        Log::debug_key("exit_grep_result");

        term.curt().prom.clear();

        let is_empty = term.curt().editor.grep_result_vec.is_empty();
        term.curt().state.grep.is_cancel = is_cancel;
        term.curt().state.grep.is_empty = is_empty;
        term.curt().prom_show_com(&CmdType::GrepResultProm);
        term.curt().editor.set_cur_default();
        term.curt().editor.scroll();
        term.curt().editor.state.is_read_only = true;
        term.curt().editor.init_editor_scrlbar_h();
        term.draw_all(out, DParts::All);
    }

    pub fn grep_result(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match term.curt().prom.cmd.cmd_type {
            CmdType::Confirm => {
                let y = term.curt().editor.win_mgr.curt().cur.y;
                let grep_result = term.curt().editor.grep_result_vec[y].clone();
                Log::debug("term.tabs[term.idx].state.grep", &term.curt().state.grep);

                if grep_result.row_num != USIZE_UNDEFINED {
                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = term.curt().state.grep.search_str.clone();
                    tab_grep.editor.search.row_num = grep_result.row_num - 1;
                    tab_grep.editor.set_cmd(Cmd::to_cmd(CmdType::FindNext));
                    Log::debug("tab.editor.search", &tab_grep.editor.search);

                    let folder = if term.curt().editor.search.dir.is_empty() { "".to_string() } else { format!("{}{}", &term.curt().editor.search.dir, MAIN_SEPARATOR) };
                    let act_type = term.open_file(&format!("{}{}", &folder, &grep_result.filenm), FileOpenType::Nomal, Some(&mut tab_grep), None);

                    if let ActType::Draw(DParts::MsgBar(_)) = act_type {
                        return act_type;
                    };
                    Log::debug("term.curt().editor.search", &term.curt().editor.search);

                    term.curt().editor.search_str(true, false);
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
