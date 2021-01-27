use crate::{def::*, global::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use std::path::Path;
use std::process;
use tokio::process::{Child, Command};
use tokio_util::codec::LinesCodecError;

impl EvtAct {
    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, std_event: Option<Result<String, LinesCodecError>>, is_stdout: bool, child: &mut Child) {
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
                        EvtAct::exit_grep_result(out, term, editor, mbar, prom, sbar, child);
                    } else {
                        Log::ep_s("prom.is_grep_stderr    false");
                        prom.is_grep_stderr = false;
                    }
                    return;
                }
            }

            let line_str = line_str.replace(&editor.search.folder, "");
            editor.buf.insert_end(&format!("{}{}", line_str, NEW_LINE));
        } else {
            Log::ep_s("grep is canceled");
            EvtAct::exit_grep_result(out, term, editor, mbar, prom, sbar, child);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, child: &mut Child) {
        child.kill();
        prom.clear();
        mbar.msg = String::new();
        mbar.set_readonly(&LANG.unable_to_edit);

        editor.set_grep_result();

        if editor.grep_result_vec.len() > 0 {
            prom.grep_result_after();
        } else {
            prom.set_grep_result_after_no_result();
        }
        editor.buf.insert_end(&EOF_MARK.to_string());
        editor.set_cur_default();
        editor.d_range.draw_type = DrawType::All;
        term.draw(out, editor, mbar, prom, sbar).unwrap();
    }

    pub fn exec_grep(editor: &Editor) -> Child {
        Log::ep_s("　　　　　　　　exec_cmd");

        return Command::new("grep")
            // -r:サブフォルダ検索、-H:ファイル名表示、-n:行番号表示、-I:バイナリファイル対象外
            .arg("-rHnI")
            .arg(editor.search.str.clone())
            .arg(format!("--include={}", editor.search.filenm))
            // folder
            .arg(editor.search.folder.clone())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .unwrap();
    }

    pub fn grep_result(term: &mut Terminal, editor: &mut Editor) -> EvtActType {
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

                        Log::ep_s("startup_terminal");
                        term.startup_terminal(format!("search_str={} search_file={} search_row_num={}", search_str, path.to_string_lossy().to_string(), grep_result.row_num.to_string()));
                    }
                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn grep_result(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_grep_result();
        self.cont_1 = cont;
    }

    pub fn grep_result_after(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_grep_result_after();
        self.cont_1 = cont;
    }
    pub fn set_grep_result_after_no_result(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_grep_result_after_no_result();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_grep_result(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_fg(), self.lang.long_time_to_search.clone());
        self.key_desc = format!("{}{}:{}Ctrl + c", Colors::get_default_fg(), self.lang.cancel.clone(), Colors::get_msg_fg(),);
    }
    pub fn set_grep_result_after(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_fg(), self.lang.show_search_result.clone());
        self.key_desc = format!("{}{}:{}Enter", Colors::get_default_fg(), self.lang.open_target_file_in_another_terminal.clone(), Colors::get_msg_fg(),);
    }
    pub fn set_grep_result_after_no_result(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_fg(), self.lang.show_search_no_result.clone());
        self.key_desc = format!("{}{}:{}Ctrl + w", Colors::get_default_fg(), self.lang.close.clone(), Colors::get_msg_fg(),);
    }
}
