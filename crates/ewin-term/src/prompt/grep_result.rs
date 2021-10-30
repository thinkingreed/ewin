use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, def::*, global::*, log::Log, model::*},
    model::*,
    tab::Tab,
    terminal::*,
};
use std::{ffi::OsStr, io::Write, path::*, process};
use tokio::process::*;

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::debug_key("draw_grep_result");

        if job_grep.is_cancel {
            Log::debug_s("grep is canceled");
            EvtAct::exit_grep_result(out, term, job_grep, true);
        } else if !(job_grep.is_stdout_end && job_grep.is_stderr_end) {
            if job_grep.grep_str.trim().is_empty() {
                return;
            }
            let path = PathBuf::from(&term.curt().editor.search.filenm);
            let filenm = path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string();
            let replace_folder = term.curt().editor.search.filenm.replace(&filenm, "");
            let line_str = job_grep.grep_str.replace(&replace_folder, "");

            // New line code is fixed to LF because it is a non-editable file
            term.curt().editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE_LF));

            let rnw_org = term.curt().editor.rnw;
            term.curt().editor.set_grep_result(line_str);

            if term.curt().editor.buf.len_lines() > term.curt().editor.row_num && rnw_org == term.curt().editor.rnw {
                let y = term.curt().editor.offset_y + term.curt().editor.row_num - 2;
                term.curt().editor.draw_range = EditorDrawRange::ScrollDown(y - 2, y);

                if cfg!(target_os = "windows") {
                    term.draw(out, &DParts::All);
                } else {
                    term.draw(out, &DParts::ScrollUpDown(ScrollUpDownType::Grep));
                }
            } else {
                // term.curt().prom.cont_1.guide_row_posi = 0;
                term.draw(out, &DParts::All);
            }
        } else {
            Log::debug_s("grep is end");
            EvtAct::exit_grep_result(out, term, job_grep, false);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep, is_cancel: bool) {
        term.curt().prom.clear();
        term.curt().state.grep.is_stdout_end = job_grep.is_stdout_end;
        term.curt().state.grep.is_stderr_end = job_grep.is_stderr_end;
        term.curt().state.grep.is_cancel = job_grep.is_cancel;

        term.curt().mbar.clear();
        term.curt().mbar.set_readonly(&Lang::get().unable_to_edit);
        term.curt().editor.state.is_read_only = true;

        let is_grep_result_vec_empty = term.curt().editor.grep_result_vec.is_empty();
        term.curt().prom.set_grep_result(is_grep_result_vec_empty, is_cancel);
        term.curt().editor.buf.insert_end(&EOF_MARK.to_string());
        term.curt().editor.set_cur_default();
        term.curt().editor.scroll();
        term.curt().editor.scroll_horizontal();
        term.draw_all(out, DParts::All);
    }

    pub fn grep_result(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.set_disp_size();
                let is_grep_result_vec_empty = term.curt().editor.grep_result_vec.is_empty();
                let is_cancel = term.curt().state.grep.is_cancel;
                term.curt().prom.set_grep_result(is_grep_result_vec_empty, is_cancel);
                return ActType::Draw(DParts::All);
            }
            KeyCmd::Prom(P_Cmd::ConfirmPrompt) => {
                let y = term.curt().editor.cur.y;
                let grep_result = term.curt().editor.grep_result_vec[y].clone();

                Log::debug("term.tabs[term.idx].state.grep", &term.curt().state.grep);

                if grep_result.row_num != USIZE_UNDEFINED {
                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = term.curt().state.grep.search_str.clone();
                    tab_grep.editor.search.row_num = grep_result.row_num - 1;
                    tab_grep.editor.set_keys(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::FindNext)), None);
                    Log::debug("tab.editor.search", &tab_grep.editor.search);

                    let folder = if term.curt().editor.search.folder.is_empty() { "".to_string() } else { format!("{}{}", &term.curt().editor.search.folder, MAIN_SEPARATOR) };
                    let act_type = term.open(&format!("{}{}", &folder, &grep_result.filenm), &mut tab_grep, USIZE_UNDEFINED, FileOpenType::Normal);

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

    #[cfg(target_os = "linux")]
    pub fn get_grep_child(search_str: &str, search_folder: &str, search_filenm: &str) -> Child {
        Log::debug_key("get_grep_child linux");
        // -r:Subfolder search, -H:File name display, -n:Line number display,
        // -I:Binary file not applicable, -i:Ignore-case, -F:Fixed-strings
        let mut cmd_option = "-rHnI".to_string();
        {
            if !CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens {
                cmd_option.push('i');
            };
        }
        {
            if !CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex {
                cmd_option.push('F');
            };
        }

        let mut cmd = Command::new("grep");
        cmd.arg(cmd_option.clone())
            .arg(search_str)
            .arg(format!("--include={}", search_filenm))
            // folder
            .arg(search_folder)
            // Arguments that read everything as text
            // Processing takes time
            // .arg("--text")
            ;

        Log::info("Grep command", &cmd);
        let child = cmd.kill_on_drop(true).stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn().unwrap();

        return child;
    }
    #[cfg(target_os = "windows")]
    pub fn get_grep_child(search_str: &str, search_folder: &str, search_filenm: &str) -> Child {
        Log::info_s("              get_grep_child windows");
        let mut cmd_option = "".to_string();
        {
            if CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens {
                cmd_option.push_str(" -CaseSensitive");
            };
        }
        {
            if !CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex {
                cmd_option.push_str(" -SimpleMatch");
            };
        }
        let path = Path::new(&search_folder).join(&search_filenm);
        Log::debug("search_str", &search_str);

        // The character code to be searched is UTF8 and UTF16 with the default specifications of Select-String.
        // Do it only with "Select-String" without using dir, but adopt the dir method because an error occurs
        let mut cmd = Command::new("powershell.exe");
        cmd.args(&["dir", "-recurse", &format!(r#""{}""#, path.display()), "-Exclude"])
            .arg("*.exe,*.zip,*.lzh,*.gz,*.Z,*.bz2,*.gif,*.jpg,*.png,*.bmp,*.tif,*.xls,*.doc,*.ppt,*.dvi,*.pdf,*.o,*.a,*.lib,*.rlib,*.rmeta,*.bin,*.pdb,*.dll,*.exp")
            .arg("|")
            .arg("Select-String")
            .arg(format!(r#""{}""#, search_str))
            .arg(cmd_option)
            // Do not adjust the width of the output
            .args(&["|", "Out-String", "-Width", "409600"]);

        Log::info("Grep command", &cmd);
        let child = cmd.kill_on_drop(true).stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn().unwrap();

        return child;
    }
}
