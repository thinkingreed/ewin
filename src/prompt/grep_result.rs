use crate::{colors::*, def::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::{io::Write, path::Path, process};
use tokio::process::{Child, Command};
use tokio_util::codec::LinesCodecError;

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar, std_event: Option<Result<String, LinesCodecError>>, is_stdout: bool, child: &mut Child) {
        Log::ep_s("　　　　　　　draw_grep_result");

        if (prom.is_grep_stdout || prom.is_grep_stderr) && !prom.is_grep_result_cancel {
            let mut line_str = String::new();
            match std_event {
                Some(Ok(result_str)) => line_str = result_str,
                Some(Err(e)) => println!("err {:?}", e),
                None => {
                    //
                    if is_stdout {
                        Log::ep_s("prom.is_grep_stdout    false");
                        EvtAct::exit_grep_result(out, editor, mbar, prom, help, sbar, child);
                    } else {
                        Log::ep_s("prom.is_grep_stderr    false");
                        prom.is_grep_stderr = false;
                    }
                    return;
                }
            }
            let line_str = line_str.replace(&editor.search.folder, "");
            editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE));
            editor.set_grep_result();
            if editor.buf.len_lines() > editor.disp_row_num {
                let y = editor.offset_y + editor.disp_row_num - 2;
                editor.d_range = DRange::new(y, y, DrawType::ScrollDown);
            } else {
                editor.d_range.draw_type = DrawType::All;
            }
            Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
        } else {
            Log::ep_s("grep is canceled");
            EvtAct::exit_grep_result(out, editor, mbar, prom, help, sbar, child);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar, child: &mut Child) {
        child.kill();
        prom.clear();
        mbar.msg = Msg::default();
        mbar.set_readonly(&LANG.unable_to_edit);

        if editor.grep_result_vec.is_empty() {
            prom.set_grep_result_after_no_result(editor, mbar, help, sbar);
        } else {
            prom.grep_result_after(editor, mbar, help, sbar);
        }

        editor.buf.insert_end(&EOF_MARK.to_string());
        editor.set_cur_default();
        editor.scroll();
        editor.scroll_horizontal();
        editor.d_range.draw_type = DrawType::All;
        Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
    }

    pub fn exec_grep(editor: &Editor) -> Child {
        Log::ep_s("　　　　　　　　exec_cmd");
        // -r:Subfolder search, -H:File name display, -n:Line number display,
        // -I:Binary file not applicable, -i:Ignore-case
        let mut cmd_option = "-rHnI".to_string();
        if !CFG.get().unwrap().try_lock().unwrap().general.editor.search.case_sens {
            cmd_option.push('i');
        };
        if !CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex {
            cmd_option.push('F');
        };
        return Command::new("grep")
            .arg(cmd_option)
            .arg(editor.search.str.clone())
            .arg(format!("--include={}", editor.search.filenm))
            // folder
            .arg(editor.search.folder.clone())
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
    pub fn grep_result(&mut self, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.disp_row_num = 2;
        Terminal::set_disp_size(editor, mbar, self, help, sbar);
        let mut cont = PromptCont::new_not_edit(self.disp_row_posi as u16);
        cont.set_grep_result();
        self.cont_1 = cont;
    }

    pub fn grep_result_after(&mut self, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.disp_row_num = 2;
        Terminal::set_disp_size(editor, mbar, self, help, sbar);
        let mut cont = PromptCont::new_not_edit(self.disp_row_posi as u16);
        cont.set_grep_result_after();
        self.cont_1 = cont;
    }
    pub fn set_grep_result_after_no_result(&mut self, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.disp_row_num = 2;
        Terminal::set_disp_size(editor, mbar, self, help, sbar);
        let mut cont = PromptCont::new_not_edit(self.disp_row_posi as u16);
        cont.set_grep_result_after_no_result();
        self.cont_1 = cont;
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
