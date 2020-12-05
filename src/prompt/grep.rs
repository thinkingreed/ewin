use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::env;
use std::io::Write;
use std::path::Path;

impl EvtAct {
    pub fn grep<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.evt {
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
                    let search_filenm = prom.cont_2.buf.iter().collect::<String>();
                    let search_folder = prom.cont_3.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_str.clone());
                    } else if search_filenm.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_file.clone());
                    } else if search_folder.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_folder.clone());
                    } else {
                        mbar.clear();
                        prom.clear();
                        let path = Path::new(&search_folder).join(&search_filenm);

                        prom.cache_search_filenm = search_filenm.clone();
                        prom.cache_search_folder = search_folder.clone();

                        term.startup_terminal(format!(r#"search_str={} search_file={}"#, search_str, path.to_string_lossy().to_string()));
                    }
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn grep(&mut self) {
        self.disp_row_num = 8;
        let mut cont_1 = PromptCont::new(self.lang.clone());
        let mut cont_2 = PromptCont::new(self.lang.clone());
        let mut cont_3 = PromptCont::new(self.lang.clone());
        cont_1.set_grep(self, PromptBufPosi::First);
        cont_2.set_grep(self, PromptBufPosi::Second);
        cont_3.set_grep(self, PromptBufPosi::Third);
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
        self.cont_3 = cont_3;
    }
}

impl PromptCont {
    pub fn set_grep(&mut self, prom: &Prompt, cont_type: PromptBufPosi) {
        if cont_type == PromptBufPosi::First {
            self.guide = format!("{}{}{}", Colors::get_msg_fg(), self.lang.set_grep.clone(), "\n");
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c",
                Colors::get_default_fg(),
                self.lang.search.clone(),
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                self.lang.move_input_field.clone(),
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                self.lang.close.clone(),
                Colors::get_msg_fg()
            );
            self.buf_desc = format!("{}{}{}", Colors::get_msg_fg(), self.lang.search_str.clone(), Colors::get_default_fg());
        } else if cont_type == PromptBufPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_fg(), self.lang.search_file.clone(), Colors::get_default_fg());

            if prom.cache_search_filenm.len() > 0 {
                self.buf = prom.cache_search_filenm.chars().collect();
            }

            self.buf = "*.*".chars().collect();
        } else {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_fg(), self.lang.search_folder.clone(), Colors::get_default_fg());
            if prom.cache_search_folder.len() > 0 {
                self.buf = prom.cache_search_folder.chars().collect();
            } else {
                if let Ok(path) = env::current_dir() {
                    self.buf = path.to_string_lossy().to_string().chars().collect();
                }
            };
        }
    }
}
