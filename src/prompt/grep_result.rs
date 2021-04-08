use crate::{bar::headerbar::*, bar::msgbar::*, colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::{ffi::OsStr, io::Write, path::PathBuf, process};
use tokio::process::{Child, Command};

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::ep_s("　　　　　　　draw_grep_result");

        // let rc = Rc::clone(&term.tabs[term.tab_idx]);
        // let mut tab = term.tabs[term.tab_idx];

        if !job_grep.is_cancel {
            if !(job_grep.is_stdout_end && job_grep.is_stderr_end) {
                let path = PathBuf::from(&term.tabs[term.idx].editor.search.filenm);
                let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                let replace_folder = term.tabs[term.idx].editor.search.filenm.replace(&filenm, "");
                let line_str = job_grep.grep_str.replace(&replace_folder, "");

                term.tabs[term.idx].editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE));

                let rnw_org = term.tabs[term.idx].editor.rnw;
                term.tabs[term.idx].editor.set_grep_result();
                if term.tabs[term.idx].editor.buf.len_lines() > term.tabs[term.idx].editor.disp_row_num && rnw_org == term.tabs[term.idx].editor.rnw {
                    let y = term.tabs[term.idx].editor.offset_y + term.tabs[term.idx].editor.disp_row_num - 2;
                    term.tabs[term.idx].editor.d_range = DRange::new(y - 2, y, DrawType::ScrollDown);
                } else {
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                }
                term.draw(out);
            } else {
                Log::ep_s("grep is end");
                EvtAct::exit_grep_result(out, term);
            }
        } else {
            Log::ep_s("grep is canceled");
            EvtAct::exit_grep_result(out, term);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal) {
        term.tabs[term.idx].prom.clear();
        // tab.state.clear();
        term.tabs[term.idx].mbar.msg = Msg::default();
        term.tabs[term.idx].mbar.set_readonly(&LANG.unable_to_edit);

        if term.tabs[term.idx].editor.grep_result_vec.is_empty() {
            Prompt::set_grep_result_after_no_result(term);
        } else {
            Prompt::set_grep_result_after(term);
        }

        term.tabs[term.idx].editor.buf.insert_end(&EOF_MARK.to_string());
        term.tabs[term.idx].editor.set_cur_default();
        term.tabs[term.idx].editor.scroll();
        term.tabs[term.idx].editor.scroll_horizontal();
        term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
        term.draw(out);
    }

    pub fn get_grep_child(search_str: &String, search_folder: &String, search_filenm: &String) -> Child {
        Log::ep_s("　　　　　　　　exec_cmd");
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

    pub fn grep_result(term: &mut Terminal) -> EvtActType {
        Log::ep_s("　　　　　　　　grep_result");

        match term.tabs[term.idx].editor.evt {
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
                    let grep_result = &term.tabs[term.idx].editor.grep_result_vec[term.tabs[term.idx].editor.cur.y];
                    if grep_result.row_num != USIZE_UNDEFINED {
                        let mut tab_grep = Tab::new();
                        tab_grep.editor.search.str = term.tabs[term.idx].editor.search.str.clone();
                        tab_grep.editor.search.row_num = grep_result.row_num - 1;
                        term.idx = term.tabs.len();

                        let mut h_file = HeaderFile::default();
                        h_file.filenm = grep_result.filenm.clone();
                        term.hbar.file_vec.push(h_file);

                        tab_grep.open(&term.hbar.file_vec[term.idx], &grep_result.filenm);
                        tab_grep.editor.search_str(true, false);
                        term.tabs.push(tab_grep);
                    }
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn set_grep_result(term: &mut Terminal) {
        term.tabs[term.idx].prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit(term.tabs[term.idx].prom.disp_row_posi as u16);
        cont.set_grep_result();
        term.tabs[term.idx].prom.cont_1 = cont;
    }

    pub fn set_grep_result_after(term: &mut Terminal) {
        term.tabs[term.idx].prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit(term.tabs[term.idx].prom.disp_row_posi as u16);
        cont.set_grep_result_after();
        term.tabs[term.idx].prom.cont_1 = cont;
    }
    pub fn set_grep_result_after_no_result(term: &mut Terminal) {
        term.tabs[term.idx].prom.disp_row_num = 2;
        term.set_disp_size();
        let mut cont = PromptCont::new_not_edit(term.tabs[term.idx].prom.disp_row_posi as u16);
        cont.set_grep_result_after_no_result();
        term.tabs[term.idx].prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_grep_result(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.long_time_to_search);
        self.key_desc = format!("{}{}:{}Esc", Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
    pub fn set_grep_result_after(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.show_search_result);
        self.key_desc = format!("{}{}:{}Enter", Colors::get_default_fg(), &LANG.open_target_file, Colors::get_msg_highlight_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
    pub fn set_grep_result_after_no_result(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.show_search_no_result);
        self.key_desc = format!("{}{}:{}Ctrl + w", Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
}
