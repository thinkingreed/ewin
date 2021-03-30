use crate::{bar::msgbar::*, colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::{
    ffi::OsStr,
    io::Write,
    path::{Path, PathBuf},
    process,
    sync::Arc,
};
use tokio::process::{Child, Command};

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, job_grep: JobGrep) {
        Log::ep_s("　　　　　　　draw_grep_result");
        Log::ep("job_grep", &job_grep.clone());

        let arc = Arc::clone(&term.tabs.tab_vec[term.tabs_idx]);
        let mut tab = arc.try_lock().unwrap();

        if !job_grep.is_cancel {
            if !(job_grep.is_stdout_end) {
                /*
                if line_str.is_empty() {
                    return;
                }
                 */
                // Log::ep("line_str", &line_str);

                let path = PathBuf::from(&tab.editor.search.file);
                let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                let replace_folder = tab.editor.search.file.replace(&filenm, "");
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
        tab.state.clear();
        tab.mbar.msg = Msg::default();
        tab.mbar.set_readonly(&LANG.unable_to_edit);

        if tab.editor.grep_result_vec.is_empty() {
            Prompt::set_grep_result_after_no_result(term, tab);
        } else {
            Prompt::grep_result_after(term, tab);
        }

        tab.editor.buf.insert_end(&EOF_MARK.to_string());
        tab.editor.set_cur_default();
        tab.editor.scroll();
        tab.editor.scroll_horizontal();
        tab.editor.d_range.draw_type = DrawType::All;
        term.draw(out, tab);
    }

    pub fn exec_grep(search_str: &String, search_folder: &String, search_filenm: &String) -> Child {
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
        return Command::new("grep")
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
    }

    pub fn grep_result(editor: &mut Editor) -> EvtActType {
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) | Right | Left | Down | Up | Home | End => {
                    return EvtActType::Next;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('w') | Char('s') | Char('c') | Char('a') | Char('f') | Home | End => {
                    return EvtActType::Next;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                PageDown | PageUp | Home | End | F(3) | Down | Up | Left | Right => {
                    return EvtActType::Next;
                }

                Enter => {
                    let grep_result = &editor.grep_result_vec[editor.cur.y];
                    if grep_result.row_num != USIZE_UNDEFINED {
                        let search_str = &editor.search.str;
                        let path = Path::new(&editor.search.folder).join(&grep_result.filenm);

                        let cfg = CFG.get().unwrap().try_lock().unwrap();
                        let args = format!(
                            "search_str={} search_file={} search_case_sens={} search_regex={} search_row_num={}",
                            search_str,
                            path.to_string_lossy().to_string(),
                            cfg.general.editor.search.case_sens.to_string(),
                            cfg.general.editor.search.regex.to_string(),
                            grep_result.row_num.to_string()
                        );
                        Log::ep_s("startup_terminal");
                        Log::ep_s("args");
                        Terminal::startup_terminal(args);
                    }
                    editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn grep_result(term: &mut Terminal, tab: &mut Tab) {
        tab.prom.disp_row_num = 2;
        term.set_disp_size(tab);
        let mut cont = PromptCont::new_not_edit(tab.prom.disp_row_posi as u16);
        cont.set_grep_result();
        tab.prom.cont_1 = cont;
    }

    pub fn grep_result_after(term: &mut Terminal, tab: &mut Tab) {
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
        self.key_desc = format!("{}{}:{}Enter", Colors::get_default_fg(), &LANG.open_target_file_in_another_terminal, Colors::get_msg_highlight_fg(),);

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
