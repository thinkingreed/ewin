use crate::{
    ewin_core::{_cfg::keys::*, def::*, global::*, log::Log, model::*},
    model::*,
    tab::Tab,
    terminal::*,
};
use std::{ffi::OsStr, io::Write, path::*, process};
use tokio::process::{Child, Command};

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::debug_key("draw_grep_result");

        if job_grep.is_cancel {
            Log::debug_s("grep is canceled");
            EvtAct::exit_grep_result(out, term, job_grep, true);
        } else if !(job_grep.is_stdout_end && job_grep.is_stderr_end) {
            if job_grep.grep_str.trim().len() == 0 {
                return;
            }
            let path = PathBuf::from(&term.curt().editor.search.filenm);
            let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
            let replace_folder = term.curt().editor.search.filenm.replace(&filenm, "");
            let line_str = job_grep.grep_str.replace(&replace_folder, "");

            // New line code is fixed to LF because it is a non-editable file
            term.curt().editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE_LF));

            let rnw_org = term.curt().editor.rnw;
            term.curt().editor.set_grep_result(line_str);
            if term.curt().editor.buf.len_lines() > term.curt().editor.disp_row_num && rnw_org == term.curt().editor.rnw {
                let y = term.curt().editor.offset_y + term.curt().editor.disp_row_num - 2;
                term.curt().editor.draw_type = DrawType::ScrollDown(y - 2, y);
            } else {
                term.curt().editor.draw_type = DrawType::All;
            }
            term.draw(out);
        } else {
            Log::debug_s("grep is end");
            EvtAct::exit_grep_result(out, term, job_grep, false);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep, is_cancel: bool) {
        term.curt().prom.clear();
        term.curt().state.grep_state.is_stdout_end = job_grep.is_stdout_end;
        term.curt().state.grep_state.is_stderr_end = job_grep.is_stderr_end;
        term.curt().state.grep_state.is_cancel = job_grep.is_cancel;

        term.curt().mbar.msg = Msg::default();
        term.curt().mbar.set_readonly(&LANG.unable_to_edit);
        term.curt().state.is_read_only = true;

        let is_grep_result_vec_empty = term.curt().editor.grep_result_vec.is_empty();
        term.curt().prom.set_grep_result(is_grep_result_vec_empty, is_cancel);
        term.curt().editor.buf.insert_end(&EOF_MARK.to_string());
        term.curt().editor.set_cur_default();
        term.curt().editor.scroll();
        term.curt().editor.scroll_horizontal();
        term.curt().editor.draw_type = DrawType::All;
        term.draw(out);
        Terminal::draw_cur(out, term);
    }

    #[cfg(target_os = "linux")]
    pub fn get_grep_child(search_str: &String, search_folder: &String, search_filenm: &String) -> Child {
        Log::debug_key("get_grep_child linux");
        // -r:Subfolder search, -H:File name display, -n:Line number display,
        // -I:Binary file not applicable, -i:Ignore-case
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
        let cmd = Command::new("grep")
            .arg(cmd_option)
            .arg(search_str)
            .arg(format!("--include={}", search_filenm))
            // folder
            .arg(search_folder)
            .arg("--text")
            .kill_on_drop(true)
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .unwrap();

        return cmd;
    }
    #[cfg(target_os = "windows")]
    pub fn get_grep_child(search_str: &String, search_folder: &String, search_filenm: &String) -> Child {
        Log::info_s("              get_grep_child windows");
        let mut cmd_option = "".to_string();
        {
            if CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens {
                cmd_option.push_str(&" -CaseSensitive");
            };
        }
        {
            if !CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex {
                cmd_option.push_str(&" -SimpleMatch");
            };
        }
        let path = Path::new(&search_folder).join(&search_filenm);
        Log::debug("search_str", &search_str);

        // The character code to be searched is UTF8 and UTF16 with the default specifications of Select-String.
        // Do it only with "Select-String" without using dir, but adopt the dir method because an error occurs
        let cmd = Command::new("powershell.exe")
            .args(&["dir", "-recurse", &format!(r#""{}""#, path.display()), "-Exclude"])
            .arg("*.exe,*.zip,*.lzh,*.gz,*.Z,*.bz2,*.gif,*.jpg,*.png,*.bmp,*.tif,*.xls,*.doc,*.ppt,*.dvi,*.pdf,*.o,*.a,*.lib,*.rlib,*.rmeta,*.bin,*.pdb,*.dll,*.exp")
            .arg("|")
            .arg("Select-String")
            .arg(format!(r#""{}""#, search_str))
            .arg(cmd_option)
            // Do not adjust the width of the output
            .args(&["|", "Out-String", "-Width", "409600"])
            .kill_on_drop(true)
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .unwrap();

        return cmd;
    }

    pub fn grep_result(term: &mut Terminal) -> EvtActType {
        Log::debug_key("EvtAct::grep_result");

        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.set_disp_size();
                let is_grep_result_vec_empty = term.curt().editor.grep_result_vec.is_empty();
                let is_cancel = term.curt().state.grep_state.is_cancel;
                term.curt().prom.set_grep_result(is_grep_result_vec_empty, is_cancel);
                return EvtActType::Next;
            }
            // Shift
            KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorUpSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect | KeyCmd::FindBack => return EvtActType::Next,
            // Ctrl
            KeyCmd::CloseFile | KeyCmd::SaveFile | KeyCmd::Copy | KeyCmd::AllSelect | KeyCmd::Find | KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd => return EvtActType::Next,
            // Raw
            KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorDown | KeyCmd::CursorUp | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::FindNext | KeyCmd::CursorPageUp | KeyCmd::CursorPageDown => return EvtActType::Next,
            KeyCmd::ConfirmPrompt => {
                let y = term.tabs[term.idx].editor.cur.y;
                let grep_result = term.tabs[term.idx].editor.grep_result_vec[y].clone();

                if grep_result.row_num != USIZE_UNDEFINED {
                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = term.tabs[term.idx].state.grep_state.search_str.clone();
                    tab_grep.editor.search.row_num = grep_result.row_num - 1;
                    tab_grep.editor.set_keys(&Keybind::keycmd_to_keys(&KeyCmd::Find));

                    tab_grep.editor.mouse_mode = term.mouse_mode;

                    let folder = if term.curt().editor.search.folder.is_empty() { "".to_string() } else { format!("{}{}", &term.curt().editor.search.folder, MAIN_SEPARATOR) };
                    term.open(&format!("{}{}", &folder, &grep_result.filenm), &mut tab_grep);
                    term.curt().editor.search_str(true, false);
                }
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}
