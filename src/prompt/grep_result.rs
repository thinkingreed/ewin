use crate::{bar::headerbar::*, bar::msgbar::*, colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::{cell::RefCell, ffi::OsStr, io::Write, path::PathBuf, process, rc::Rc};
use tokio::process::{Child, Command};

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::ep_s("　　　　　　　draw_grep_result");

        let rc = Rc::clone(&term.tabs[term.tab_idx]);
        let mut tab = rc.borrow_mut();

        if !job_grep.is_cancel {
            if !(job_grep.is_stdout_end && job_grep.is_stderr_end) {
                let path = PathBuf::from(&tab.editor.search.filenm);
                let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                let replace_folder = tab.editor.search.filenm.replace(&filenm, "");
                let line_str = job_grep.grep_str.replace(&replace_folder, "");

                tab.editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE));

                let rnw_org = tab.editor.rnw;
                tab.editor.set_grep_result();
                if tab.editor.buf.len_lines() > tab.editor.disp_row_num && rnw_org == tab.editor.rnw {
                    let y = tab.editor.offset_y + tab.editor.disp_row_num - 2;
                    tab.editor.d_range = DRange::new(y, y, DrawType::ScrollDown);
                } else {
                    tab.editor.d_range.draw_type = DrawType::All;
                }
                term.draw(out, &mut tab);
            } else {
                Log::ep_s("grep is end");
                EvtAct::exit_grep_result(out, term, &mut tab);
            }
        } else {
            Log::ep_s("grep is canceled");
            EvtAct::exit_grep_result(out, term, &mut tab);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, tab: &mut Tab) {
        tab.prom.clear();
        // tab.state.clear();
        tab.mbar.msg = Msg::default();
        tab.mbar.set_readonly(&LANG.unable_to_edit);

        if tab.editor.grep_result_vec.is_empty() {
            Prompt::set_grep_result_after_no_result(term, tab);
        } else {
            Prompt::set_grep_result_after(term, tab);
        }

        tab.editor.buf.insert_end(&EOF_MARK.to_string());
        tab.editor.set_cur_default();
        tab.editor.scroll();
        tab.editor.scroll_horizontal();
        tab.editor.d_range.draw_type = DrawType::All;
        term.draw(out, tab);
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

        Log::ep("cmd", &cmd);

        return cmd;
    }

    pub fn grep_result(term: &mut Terminal, editor: &mut Editor) -> EvtActType {
        Log::ep_s("　　　　　　　　grep_result");

        match editor.evt {
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
                    let grep_result = &editor.grep_result_vec[editor.cur.y];
                    if grep_result.row_num != USIZE_UNDEFINED {
                        let mut tab_grep = Tab::new();
                        tab_grep.editor.search.str = editor.search.str.clone();
                        tab_grep.editor.search.row_num = grep_result.row_num - 1;
                        term.tab_idx = term.tabs.len();

                        let mut h_file = HeaderFile::default();
                        h_file.filenm = grep_result.filenm.clone();
                        term.hbar.file_vec.push(h_file);

                        tab_grep.open(&term, &grep_result.filenm);
                        tab_grep.editor.search_str(true, false);
                        term.tabs.push(Rc::new(RefCell::new(tab_grep)));
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
    pub fn set_grep_result(term: &mut Terminal, tab: &mut Tab) {
        tab.prom.disp_row_num = 2;
        term.set_disp_size(tab);
        let mut cont = PromptCont::new_not_edit(tab.prom.disp_row_posi as u16);
        cont.set_grep_result();
        tab.prom.cont_1 = cont;
    }

    pub fn set_grep_result_after(term: &mut Terminal, tab: &mut Tab) {
        tab.prom.disp_row_num = 2;
        term.set_disp_size(tab);
        let mut cont = PromptCont::new_not_edit(tab.prom.disp_row_posi as u16);
        cont.set_grep_result_after();
        tab.prom.cont_1 = cont;
    }
    pub fn set_grep_result_after_no_result(term: &mut Terminal, tab: &mut Tab) {
        tab.prom.disp_row_num = 2;
        term.set_disp_size(tab);
        let mut cont = PromptCont::new_not_edit(tab.prom.disp_row_posi as u16);
        cont.set_grep_result_after_no_result();
        tab.prom.cont_1 = cont;
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
