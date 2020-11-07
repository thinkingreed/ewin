use crate::model::*;
use crate::util::*;
use crossterm::event::{Event, EventStream};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::SystemTime;
use termion::color;
use tokio::process::{Child, Command};
use tokio_util::codec::{FramedRead, LinesCodecError};

impl EvtAct {
    pub fn grep<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::SHIFT }) => match code {
                Char(c) => {
                    let c_up = c.to_ascii_uppercase();
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.insert_char(c_up),
                        PromptBufPosi::Second => prom.cont_2.insert_char(c_up),
                        PromptBufPosi::Third => prom.cont_3.insert_char(c_up),
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(c) => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.insert_char(c),
                        PromptBufPosi::Second => prom.cont_2.insert_char(c),
                        PromptBufPosi::Third => prom.cont_3.insert_char(c),
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Down => {
                    prom.cursor_down();
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Up => {
                    prom.cursor_up();
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Left => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.cursor_left(),
                        PromptBufPosi::Second => prom.cont_2.cursor_left(),
                        PromptBufPosi::Third => prom.cont_3.cursor_left(),
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Right => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.cursor_right(),
                        PromptBufPosi::Second => prom.cont_2.cursor_right(),
                        PromptBufPosi::Third => prom.cont_3.cursor_right(),
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Delete => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.delete(),
                        PromptBufPosi::Second => prom.cont_2.delete(),
                        PromptBufPosi::Third => prom.cont_3.delete(),
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Backspace => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.backspace(),
                        PromptBufPosi::Second => prom.cont_2.backspace(),
                        PromptBufPosi::Third => prom.cont_3.backspace(),
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Tab => {
                    prom.tab(true);
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }

                Enter => {
                    let search_str = prom.cont_1.buf.iter().collect::<String>();
                    let search_file = prom.cont_2.buf.iter().collect::<String>();
                    let search_folder = prom.cont_3.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_str.clone());
                    } else if search_file.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_file.clone());
                    } else if search_folder.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_folder.clone());
                    } else {
                        mbar.clear();
                        prom.clear();
                        let path = Path::new(&search_folder).join(&search_file);
                        Log::ep("path", path.to_string_lossy().to_string());

                        EvtAct::startup_terminal(&term, search_str, path.to_string_lossy().to_string());
                    }
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
    fn startup_terminal(term: &Terminal, search_str: String, search_file: String) {
        if term.env == Env::WSL {
            Log::ep_s("startup_terminal ");

            if let Err(err) = Command::new("/mnt/c/windows/system32/cmd.exe").arg("/c").arg("start").arg("wsl").arg("--").arg(PKG_NAME).stdin(process::Stdio::piped()).stdout(process::Stdio::null()).spawn() {
                Log::ep("exec_grep err", err.to_string());
            }
        } else {
        };
    }

    pub fn draw_grep_result<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, std_event: Option<Result<String, LinesCodecError>>, is_stdout: bool, child: &mut Child) {
        if prom.is_grep_stdout || prom.is_grep_stderr {
            let mut line_str = String::new();
            match std_event {
                Some(Ok(result_str)) => line_str = result_str,
                Some(Err(e)) => println!("err {:?}", e),
                None => {
                    //
                    //
                    // 検索行数が合っているかの確認
                    //
                    //
                    //   child.kill();
                    if is_stdout {
                        Log::ep_s("prom.is_grep_stdout    false");
                        prom.is_grep_stdout = false;
                    } else {
                        Log::ep_s("prom.is_grep_stderr    false");
                        prom.is_grep_stderr = false;
                    }
                    // eprintln!("stdout end {:?}", SystemTime::now());
                    return;
                }
            }

            let line_str = line_str.replace(&editor.search.folder, "");

            let rnw_org = editor.rnw;
            let v = line_str.trim_end().chars().collect();
            if editor.buf[0].len() == 0 {
                editor.buf[0] = v
            } else {
                editor.buf.push(v);
            }
            editor.rnw = editor.buf.len().to_string().len();
            editor.cur = Cur { y: editor.buf.len() - 1, x: editor.rnw, disp_x: 0 };
            editor.cur.disp_x = editor.rnw + get_cur_x_width(&editor.buf[editor.cur.y], editor.cur.x - editor.rnw);
            editor.scroll();

            let y = editor.buf.len() - 1;

            if rnw_org == editor.rnw {
                editor.d_range = DRnage::new(y, y, DType::Target);
            } else {
                editor.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
            }

            let vec: Vec<&str> = line_str.split(":").collect();

            if vec.len() == 3 && vec[0] != "grep" {
                let pre_str = format!("{}:{}:", vec[0], vec[1]);
                let pre_str_vec = pre_str.chars().collect();
                let (pre_str_x, _) = get_row_width(&pre_str_vec, 0, pre_str_vec.len());

                Log::ep("pre_str", pre_str);
                Log::ep("pre_str_x", pre_str_x);

                let v: Vec<(usize, &str)> = vec[2].match_indices(&editor.search.str).collect();
                for (index, _) in v {
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
                if let Ok(row_num) = result {
                    let grep_result = GrepResult::new(vec[0].to_string(), row_num);
                    editor.grep_result_vec.push(grep_result);
                }
            }
        }
    }
    pub fn exec_cmd(editor: &Editor) -> Child {
        Log::ep_s("★　exec_cmd");
        if editor.search.file.len() > 0 {
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
        } else {
            // 無害なコマンド実行
            // TODO 別の方法検討
            return Command::new("echo").arg(" ").stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn().unwrap();
        }
    }
}

impl Prompt {
    pub fn grep(&mut self) {
        self.disp_row_num = 8;
        let mut cont_1 = PromptCont::new(self.lang.clone());
        let mut cont_2 = PromptCont::new(self.lang.clone());
        let mut cont_3 = PromptCont::new(self.lang.clone());
        cont_1.set_grep(PromptBufPosi::First);
        cont_2.set_grep(PromptBufPosi::Second);
        cont_3.set_grep(PromptBufPosi::Third);
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
        self.cont_3 = cont_3;
    }
}

impl PromptCont {
    pub fn set_grep(&mut self, cont_type: PromptBufPosi) {
        if cont_type == PromptBufPosi::First {
            self.guide = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_grep.clone(), "\n");
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c",
                &color::Fg(color::White).to_string(),
                self.lang.search.clone(),
                &color::Fg(color::LightGreen).to_string(),
                &color::Fg(color::White).to_string(),
                self.lang.move_input_field.clone(),
                &color::Fg(color::LightGreen).to_string(),
                &color::Fg(color::White).to_string(),
                self.lang.close.clone(),
                &color::Fg(color::LightGreen).to_string(),
            );
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.search_str.clone(),);
        } else if cont_type == PromptBufPosi::Second {
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.search_file.clone(),);
        } else {
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.search_folder.clone(),);
        }
    }
}
