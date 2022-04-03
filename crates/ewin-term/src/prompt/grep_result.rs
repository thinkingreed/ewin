use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, _cfg::model::default::*, def::*, global::*, log::Log, model::*},
    model::*,
    tab::*,
};
use std::{ffi::OsStr, io::Write, path::*, process};
use tokio::process::*;

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::debug_key("draw_grep_result");
        // Log::debug("job_grep", &job_grep);

        if EvtAct::is_grep_canceling() {
            Log::debug_s("EvtAct::is_grep_Canceling()");
            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceled).unwrap();
            EvtAct::exit_grep_result(out, term, job_grep, true);
        } else if EvtAct::is_grep_canceled() {
            Log::debug_s("EvtAct::is_grep_canceled()");
            return;
        } else if job_grep.is_end {
            Log::debug_s("grep is end");
            EvtAct::exit_grep_result(out, term, job_grep, false);
        } else {
            if job_grep.grep_str.trim().is_empty() {
                return;
            }
            let path = PathBuf::from(&term.curt().editor.search.fullpath);
            let filenm = path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string();
            let replace_folder = term.curt().editor.search.fullpath.replace(&filenm, "");
            let mut line_str = job_grep.grep_str.replace(&replace_folder, "");
            if !line_str.contains(NEW_LINE_LF) {
                line_str.push(NEW_LINE_LF);
            }
            Log::debug("line_str", &line_str);

            // New line code is fixed to LF because it is a non-editable file
            term.curt().editor.buf.insert_end(&line_str);

            let rnw_org = term.curt().editor.rnw;
            term.curt().editor.set_grep_result(line_str);

            if term.curt().editor.buf.len_rows() > term.curt().editor.row_disp_len && rnw_org == term.curt().editor.rnw {
                let y = term.curt().editor.offset_y + term.curt().editor.row_disp_len - 2;
                term.curt().editor.draw_range = E_DrawRange::ScrollDown(y - 2, y);

                if cfg!(target_os = "windows") {
                    term.render(out, &RParts::All);
                } else {
                    term.render(out, &RParts::ScrollUpDown(ScrollUpDownType::Grep));
                }
            } else {
                term.render(out, &RParts::All);
            }
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep, is_cancel: bool) {
        term.curt().prom.clear();

        term.curt().state.grep.is_end = job_grep.is_end;
        term.curt().state.grep.is_cancel = job_grep.is_cancel;

        term.curt().mbar.clear();
        term.curt().mbar.set_readonly(&Lang::get().unable_to_edit);
        term.curt().editor.state.is_read_only = true;

        let is_grep_result_vec_empty = term.curt().editor.grep_result_vec.is_empty();
        term.curt().prom.set_grep_result(is_grep_result_vec_empty, is_cancel);
        term.curt().editor.set_cur_default();
        term.curt().editor.scroll();
        term.curt().editor.scroll_horizontal();
        term.render_all(out, RParts::All);
    }

    pub fn grep_result(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match term.curt().prom.keycmd {
            KeyCmd::Prom(P_Cmd::Resize(_, _)) => {
                term.set_disp_size();
                let is_grep_result_vec_empty = term.curt().editor.grep_result_vec.is_empty();
                let is_cancel = term.curt().state.grep.is_cancel;
                term.curt().prom.set_grep_result(is_grep_result_vec_empty, is_cancel);
                return ActType::Render(RParts::All);
            }
            KeyCmd::Prom(P_Cmd::ConfirmPrompt) => {
                let y = term.curt().editor.cur.y;
                let grep_result = term.curt().editor.grep_result_vec[y].clone();

                Log::debug("term.tabs[term.idx].state.grep", &term.curt().state.grep);

                if grep_result.row_num != USIZE_UNDEFINED {
                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = term.curt().state.grep.search_str.clone();
                    tab_grep.editor.search.row_num = grep_result.row_num - 1;
                    tab_grep.editor.set_cmd(KeyCmd::Edit(E_Cmd::FindNext));
                    Log::debug("tab.editor.search", &tab_grep.editor.search);

                    let folder = if term.curt().editor.search.folder.is_empty() { "".to_string() } else { format!("{}{}", &term.curt().editor.search.folder, MAIN_SEPARATOR) };
                    let act_type = term.open_file(&format!("{}{}", &folder, &grep_result.filenm), FileOpenType::Nomal, Some(&mut tab_grep), None);

                    if let ActType::Render(RParts::MsgBar(_)) = act_type {
                        return act_type;
                    };
                    Log::debug("term.curt().editor.search", &term.curt().editor.search);

                    term.curt().editor.search_str(true, false);
                }
                return ActType::Render(RParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
