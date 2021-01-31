use crate::{global::*, model::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::env;
use std::io::Write;
use std::path::Path;

impl EvtAct {
    pub fn grep<T: Write>(out: &mut T, editor: &mut Core, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let search_str = prom.cont_1.buf.iter().collect::<String>();
                    let search_filenm = prom.cont_2.buf.iter().collect::<String>();
                    let mut search_folder = prom.cont_3.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        mbar.set_err(&LANG.not_entered_search_str);
                    } else if search_filenm.len() == 0 {
                        mbar.set_err(&LANG.not_entered_search_file);
                    } else if search_folder.len() == 0 {
                        mbar.set_err(&LANG.not_entered_search_folder);
                    } else {
                        mbar.clear();
                        prom.clear();

                        let current_dir = env::current_dir().unwrap().display().to_string();
                        Log::ep_s(&current_dir);
                        Log::ep_s(&search_folder);
                        if search_folder.chars().nth(0).unwrap() != '/' {
                            search_folder = format!("{}/{}", current_dir, search_folder);
                        }
                        Log::ep_s(&search_folder);
                        let path = Path::new(&search_folder).join(&search_filenm);

                        prom.cache_search_filenm = search_filenm.clone();
                        prom.cache_search_folder = search_folder.clone();

                        Terminal::startup_terminal(format!(r#"search_str={} search_file={}"#, search_str, path.to_string_lossy().to_string()));
                    }
                    editor.d_range.draw_type = DrawType::All;
                    Terminal::draw(out, editor, mbar, prom, sbar).unwrap();
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
        let mut cont_1 = PromptCont::new();
        let mut cont_2 = PromptCont::new();
        let mut cont_3 = PromptCont::new();
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
            self.guide = format!("{}{}", Colors::get_msg_fg(), &LANG.set_grep);
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c  {}{}:{}Tab {}({})",
                Colors::get_default_fg(),
                &LANG.search,
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                &LANG.move_input_field,
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                &LANG.complement,
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                &LANG.search_folder,
            );
            self.buf_desc = format!("{}{}{}", Colors::get_msg_fg(), &LANG.search_str, Colors::get_default_fg());
        } else if cont_type == PromptBufPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_fg(), &LANG.search_file, Colors::get_default_fg());

            if prom.cache_search_filenm.len() > 0 {
                self.buf = prom.cache_search_filenm.chars().collect();
            } else {
                self.buf = "*.*".chars().collect();
            }
        } else {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_fg(), &LANG.search_folder, Colors::get_default_fg());
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
