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

            let mut line_str = line_str.replace(&editor.search.folder, "");
            line_str.push_str(&NEW_LINE.to_string());

            let rnw_org = editor.rnw;
            let cur_y_org = editor.cur.y;
            // let v = line_str.trim_end().chars().collect();

            editor.t_buf.insert_end(&line_str);
            /*  if editor.t_buf.len() == 0 {
                editor.t_buf.insert(0, 0, &line_str);
            } else {
                editor.t_buf.insert_end(&line_str);
            }*/
            editor.rnw = editor.t_buf.len().to_string().len();
            editor.cur = Cur { y: editor.t_buf.len() - 1, x: editor.rnw, disp_x: 0 };
            editor.cur.disp_x = editor.rnw + get_cur_x_width(&editor.t_buf.char_vec(editor.cur.y), editor.cur.x - editor.rnw);
            editor.scroll();

            let y = editor.t_buf.len() - 1;

            if rnw_org == editor.rnw && cur_y_org == editor.cur.y {
                editor.d_range = DRnage::new(y, y, DType::Target);
            } else {
                editor.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
            }

            let vec: Vec<&str> = line_str.split(":").collect();

            if vec.len() > 2 && vec[0] != "grep" {
                let pre_str = format!("{}:{}:", vec[0], vec[1]);
                let pre_str_vec: Vec<char> = pre_str.chars().collect();
                let (pre_str_x, _) = get_row_width(&pre_str_vec[..], false);

                let search_target_str = &line_str.replace(&pre_str, "");

                let match_vec: Vec<(usize, &str)> = search_target_str.match_indices(&editor.search.str).collect();
                for (index, _) in match_vec {
                    let x = get_char_count(&line_str.chars().collect(), pre_str_x + index);
                    editor.search.search_ranges.push(SearchRange {
                        y: y,
                        sx: x,
                        ex: x + &editor.search.str.chars().count() - 1,
                    });
                }
            }
            term.draw(out, editor, mbar, prom, sbar).unwrap();

            if vec.len() > 1 {
                let result: Result<usize, _> = vec[1].parse();

                let grep_result = match result {
                    // text.txt:100:検索文字列 等
                    Ok(row_num) => GrepResult::new(vec[0].to_string(), row_num),
                    // grep: text.txt: 許可がありません
                    Err(_) => GrepResult::new(vec[1].to_string().as_str().trim().to_string(), USIZE_UNDEFINED),
                };
                editor.grep_result_vec.push(grep_result);
            }
        } else {
            Log::ep_s("grep is canceled");
            EvtAct::exit_grep_result(out, term, editor, mbar, prom, sbar, child);
        }
    }

    pub fn exit_grep_result<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, child: &mut Child) {
        child.kill();
        prom.clear();
        mbar.msg = String::new();
        mbar.set_readonly(&LANG.lock().unwrap().unable_to_edit);
        if editor.grep_result_vec.len() > 0 {
            prom.grep_result_after();
        } else {
            prom.set_grep_result_after_no_result();
        }
        editor.t_buf.insert_end(&EOF_MARK.to_string());
        editor.set_cur_default();
        term.draw(out, editor, mbar, prom, sbar).unwrap();
    }

    pub fn exec_cmd(editor: &Editor) -> Child {
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
