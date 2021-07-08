use crate::{
    _cfg::keys::{KeyCmd, Keybind, Keys},
    bar::msgbar::*,
    colors::*,
    def::*,
    global::*,
    log::*,
    model::*,
    prompt::cont::promptcont::*,
    prompt::prompt::prompt::*,
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
        } else {
            if !(job_grep.is_stdout_end && job_grep.is_stderr_end) {
                if job_grep.grep_str.trim().len() == 0 {
                    return;
                }
                let path = PathBuf::from(&term.curt().editor.search.filenm);

                let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();

                let replace_folder = term.curt().editor.search.filenm.replace(&filenm, "");

                let line_str = job_grep.grep_str.replace(&replace_folder, "");
                // Log::debug("line_str", &line_str);

                term.curt().editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE_LF));

                let rnw_org = term.curt().editor.rnw;
                term.curt().editor.set_grep_result(line_str);
                if term.curt().editor.buf.len_lines() > term.curt().editor.disp_row_num && rnw_org == term.curt().editor.rnw {
                    let y = term.curt().editor.offset_y + term.curt().editor.disp_row_num - 2;
                    term.curt().editor.d_range = DRange::new(y - 2, y, DrawType::ScrollDown);
                } else {
                    term.curt().editor.d_range.draw_type = DrawType::All;
                }
                term.draw(out);
            } else {
                Log::debug_s("grep is end");
                EvtAct::exit_grep_result(out, term, job_grep, false);
            }
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

        Prompt::set_grep_result(term, is_cancel);
        term.curt().editor.buf.insert_end(&EOF_MARK.to_string());
        term.curt().editor.set_cur_default();
        term.curt().editor.scroll();
        term.curt().editor.scroll_horizontal();
        term.curt().editor.d_range.draw_type = DrawType::All;
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
        Log::debug_key("　grep_result");

        match term.curt().editor.keycmd {
            // Shift
            KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorUpSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect | KeyCmd::FindBack => return EvtActType::Next,
            // Ctrl
            KeyCmd::CloseFile | KeyCmd::SaveFile | KeyCmd::Copy | KeyCmd::AllSelect | KeyCmd::Find | KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd => return EvtActType::Next,
            //
            KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorDown | KeyCmd::CursorUp | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::FindNext | KeyCmd::CursorPageUp | KeyCmd::CursorPageDown => return EvtActType::Next,
            KeyCmd::InsertLine => {
                let y = term.tabs[term.idx].editor.cur.y;
                let grep_result = term.tabs[term.idx].editor.grep_result_vec[y].clone();

                if grep_result.row_num != USIZE_UNDEFINED {
                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = term.tabs[term.idx].state.grep_state.search_str.clone();
                    tab_grep.editor.search.row_num = grep_result.row_num - 1;
                    tab_grep.editor.keys = Keys::Null;
                    tab_grep.editor.mouse_mode = term.mouse_mode;

                    Log::debug("grep_result.filenm", &grep_result.filenm);
                    Log::debug("term.curt().editor.search.folder", &term.curt().editor.search.folder);

                    let folder = if term.curt().editor.search.folder.is_empty() { "".to_string() } else { format!("{}{}", &term.curt().editor.search.folder, MAIN_SEPARATOR) };
                    term.open(&format!("{}{}", &folder, &grep_result.filenm), &mut tab_grep);
                    term.curt().editor.search_str(true, false);
                    return EvtActType::Next;
                } else {
                    return EvtActType::DrawOnly;
                }
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn set_grep_working(term: &mut Terminal) {
        term.curt().prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.curt());
        cont.set_grep_working();
        term.curt().prom.cont_1 = cont;
    }

    pub fn set_grep_result(term: &mut Terminal, is_cancel: bool) {
        term.curt().prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.curt());
        cont.set_grep_result(term.curt().editor.grep_result_vec.is_empty(), is_cancel);

        term.curt().prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_grep_working(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.long_time_to_search);
        self.key_desc = format!("{}{}:{}{}", Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::EscPrompt),);

        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }

    pub fn set_grep_result(&mut self, is_grep_result_empty: bool, is_cancel: bool) {
        let cancel_str = if is_cancel { &LANG.processing_canceled } else { "" };

        if is_grep_result_empty {
            self.guide = format!("{}{} {}", Colors::get_msg_highlight_fg(), cancel_str, &LANG.show_search_no_result);
            self.key_desc = format!("{}{}:{}Ctrl + w", Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(),);
        } else {
            self.guide = format!("{}{} {}", Colors::get_msg_highlight_fg(), cancel_str, &LANG.show_search_result);
            self.key_desc = format!("{}{}:{}Enter  {}{}:{}Ctrl + f", Colors::get_default_fg(), &LANG.open_target_file, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.search, Colors::get_msg_highlight_fg(),);
        }
        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
}

impl Editor {
    pub fn set_grep_result(&mut self, line_str: String) {
        self.rnw = if self.mouse_mode == MouseMode::Normal { self.buf.len_lines().to_string().len() } else { 0 };
        self.cur = Cur { y: self.buf.len_lines() - 1, x: 0, disp_x: 0 };

        self.scroll();

        // Pattern
        //   text.txt:100:string
        //   grep:text.txt:No permission
        let vec: Vec<&str> = line_str.splitn(3, ":").collect();

        if vec.len() > 2 && vec[0] != "grep" {
            let ignore_prefix_str = format!("{}:{}:", vec[0], vec[1]);

            let regex = CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex;
            let row = self.buf.len_lines() - 2;

            let (start_idx, end_idx, ignore_prefix_len) = match regex {
                true => (self.buf.line_to_byte(row), self.buf.len_bytes(), ignore_prefix_str.len()),
                false => (self.buf.line_to_char(row), self.buf.len_chars(), ignore_prefix_str.chars().count()),
            };

            let mut search_vec: Vec<SearchRange> = self.get_search_ranges(&self.search.str, start_idx, end_idx, ignore_prefix_len);
            self.search.ranges.append(&mut search_vec);
        }

        if vec.len() > 1 {
            let result: Result<usize, _> = vec[1].parse();

            let grep_result = match result {
                // text.txt:100:string
                Ok(row_num) => {
                    let filenm;
                    if cfg!(target_os = "linux") {
                        filenm = vec[0].to_string();
                    } else {
                        // For Windows
                        // If the grep search folder contains the current folder,
                        // the relative path is returned in the grep result, otherwise the absolute path is returned.
                        if is_include_path(&*CURT_DIR, &self.search.folder) {
                            let path = PathBuf::from(&*CURT_DIR).join(&vec[0].to_string());
                            filenm = path.to_string_lossy().to_string().replace(&self.search.folder, "");
                        } else {
                            filenm = vec[0].to_string();
                        }
                        Log::debug("setting_filenm", &filenm);
                    }
                    GrepResult::new(filenm, row_num)
                }
                // grep:text.txt:No permission
                Err(_) => GrepResult::new(vec[1].to_string().as_str().trim().to_string(), USIZE_UNDEFINED),
            };
            self.grep_result_vec.push(grep_result);
        }
    }
}
pub fn is_include_path(src: &str, dst: &str) -> bool {
    let sec_vec: Vec<&str> = src.split(MAIN_SEPARATOR).collect();
    let dst_vec: Vec<&str> = dst.split(MAIN_SEPARATOR).collect();

    let mut is_include = false;
    for (i, src) in sec_vec.iter().enumerate() {
        if let Some(dst) = dst_vec.get(i) {
            is_include = if src == dst { true } else { false };
        } else {
            is_include = false;
        }
    }
    return is_include;
}
