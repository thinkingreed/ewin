use crate::{bar::msgbar::*, colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::{ffi::OsStr, io::Write, path::*, process};
use tokio::process::{Child, Command};

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::debug_s("              draw_grep_result");

        if !job_grep.is_cancel {
            if !(job_grep.is_stdout_end && job_grep.is_stderr_end) {
                if job_grep.grep_str.trim().len() == 0 {
                    return;
                }

                let path = PathBuf::from(&term.curt().editor.search.filenm);
                let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                let replace_folder = term.curt().editor.search.filenm.replace(&filenm, "");
                let line_str = job_grep.grep_str.replace(&replace_folder, "");

                term.curt().editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE_LF));

                let rnw_org = term.curt().editor.rnw;
                term.curt().editor.set_grep_result();
                if term.curt().editor.buf.len_lines() > term.curt().editor.disp_row_num && rnw_org == term.curt().editor.rnw {
                    let y = term.curt().editor.offset_y + term.curt().editor.disp_row_num - 2;
                    term.curt().editor.d_range = DRange::new(y - 2, y, DrawType::ScrollDown);
                } else {
                    term.curt().editor.d_range.draw_type = DrawType::All;
                }
                term.draw(out);
            } else {
                Log::debug_s("grep is end");
                EvtAct::exit_grep_result(out, term, job_grep);
            }
        } else {
            Log::debug_s("grep is canceled");
            EvtAct::exit_grep_result(out, term, job_grep);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        term.curt().prom.clear();
        term.curt().state.grep_info.is_stdout_end = job_grep.is_stdout_end;
        term.curt().state.grep_info.is_stderr_end = job_grep.is_stderr_end;
        term.curt().state.grep_info.is_cancel = job_grep.is_cancel;

        term.curt().mbar.msg = Msg::default();
        term.curt().mbar.set_readonly(&LANG.unable_to_edit);
        term.curt().state.is_read_only = true;

        if term.curt().editor.grep_result_vec.is_empty() {
            Prompt::set_grep_no_result(term);
        } else {
            Prompt::set_grep_result(term);
        }
        term.curt().editor.buf.insert_end(&EOF_MARK.to_string());
        term.curt().editor.set_cur_default();
        term.curt().editor.scroll();
        term.curt().editor.scroll_horizontal();
        term.curt().editor.d_range.draw_type = DrawType::All;
        term.draw(out);
    }

    #[cfg(target_os = "linux")]
    pub fn get_grep_child(search_str: &String, search_folder: &String, search_filenm: &String) -> Child {
        Log::debug_s("              　get_grep_child linux");
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

        let cmd = Command::new("powershell.exe")
            .arg("sls")
            .arg(format!(r#""{}""#, search_str))
            .arg(format!("(dir -recurse {:?})", path))
            .args(&["-exclude", "*.exe,*.zip,*.lzh,*.gz,*.Z,*.bz2,*.gif,*.jpg,*.png,*.bmp,*.tif,*.xls,*.doc,*.ppt,*.dvi,*.pdf,*.o,*.a,*.lib,*.rlib,*.rmeta,*.bin,*.pdb,*.dll"])
            .arg(cmd_option)
            .args(&["|", "Out-String", "-Width", "409600"])
            .kill_on_drop(true)
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .unwrap();

        return cmd;
    }

    pub fn grep_result(term: &mut Terminal) -> EvtActType {
        Log::debug_s("              　grep_result");

        let evt = term.curt().editor.evt.clone();
        match evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) | Right | Left | Down | Up | Home | End => return EvtActType::Next,
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('w') | Char('s') | Char('c') | Char('a') | Char('f') | Home | End => return EvtActType::Next,
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                PageDown | PageUp | Home | End | F(3) | Down | Up | Left | Right => return EvtActType::Next,
                Enter => {
                    let y = term.tabs[term.idx].editor.cur.y;
                    let grep_result = term.tabs[term.idx].editor.grep_result_vec[y].clone();

                    if grep_result.row_num != USIZE_UNDEFINED {
                        let mut tab_grep = Tab::new();
                        tab_grep.editor.search.str = term.tabs[term.idx].state.grep_info.search_str.clone();
                        tab_grep.editor.search.row_num = grep_result.row_num - 1;
                        tab_grep.editor.evt = KEY_NULL;
                        tab_grep.editor.mode = term.mode;

                        term.open(&grep_result.filenm, &mut tab_grep);
                        term.curt().editor.search_str(true, false);
                        return EvtActType::Next;
                    } else {
                        return EvtActType::DrawOnly;
                    }
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn set_grep_working(term: &mut Terminal) {
        term.curt().prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.curt().prom.disp_row_posi as u16);
        cont.set_grep_working();
        term.curt().prom.cont_1 = cont;
    }

    pub fn set_grep_result(term: &mut Terminal) {
        term.curt().prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.curt().prom.disp_row_posi as u16);
        cont.set_grep_result();
        term.curt().prom.cont_1 = cont;
    }
    pub fn set_grep_no_result(term: &mut Terminal) {
        term.curt().prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit_type(term.curt().prom.disp_row_posi as u16);
        cont.set_grep_no_result();
        term.curt().prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_grep_working(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.long_time_to_search);
        self.key_desc = format!("{}{}:{}Esc", Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
    pub fn set_grep_result(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.show_search_result);
        self.key_desc = format!("{}{}:{}Enter  {}{}:{}Ctrl + f", Colors::get_default_fg(), &LANG.open_target_file, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.search, Colors::get_msg_highlight_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
    pub fn set_grep_no_result(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.show_search_no_result);
        self.key_desc = format!("{}{}:{}Ctrl + w", Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
}
