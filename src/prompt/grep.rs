use crate::{colors::*, def::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*, terminal::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::{env, fs, path::Path};

impl EvtAct {
    pub fn grep(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt) -> EvtActType {
        Log::ep_s("Process.grep");

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

                        let cfg = CFG.get().unwrap().try_lock().unwrap();
                        let args = format!(
                            "search_str={} search_file={} search_case_sens={} search_regex={}",
                            search_str,
                            path.to_string_lossy().to_string(),
                            cfg.general.editor.search.case_sens.to_string(),
                            cfg.general.editor.search.regex.to_string()
                        );
                        Log::ep_s(&args);

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
    pub fn grep(&mut self, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.is_grep = true;
        self.disp_row_num = 9;
        Terminal::set_disp_size(editor, mbar, self, help, sbar);
        let mut cont_1 = PromptCont::new_edit(self.disp_row_posi as u16, PromptContPosi::First);
        let mut cont_2 = PromptCont::new_edit(self.disp_row_posi as u16, PromptContPosi::Second);
        let mut cont_3 = PromptCont::new_edit(self.disp_row_posi as u16, PromptContPosi::Third);
        cont_1.set_grep(self);
        cont_2.set_grep(self);
        cont_3.set_grep(self);
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
        self.cont_3 = cont_3;
    }
    pub fn tab(&mut self, is_asc: bool) {
        Log::ep_s("tab");
        Log::ep("is_asc ", &is_asc);

        if self.is_replace {
            match self.buf_posi {
                PromptContPosi::First => self.cursor_down(),
                PromptContPosi::Second => self.cursor_up(),
                _ => {}
            }
        } else if self.is_grep {
            match self.buf_posi {
                PromptContPosi::First => {
                    if is_asc {
                        self.cursor_down();
                    } else {
                        self.buf_posi = PromptContPosi::Third;
                        Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                    }
                }
                PromptContPosi::Second => {
                    if is_asc {
                        self.cursor_down();
                    } else {
                        self.cursor_up();
                    }
                }
                PromptContPosi::Third => {
                    self.cont_3.buf = self.get_tab_candidate(is_asc).chars().collect();
                    let (cur_x, width) = get_row_width(&self.cont_3.buf[..], false);
                    self.cont_3.cur.x = cur_x;
                    self.cont_3.cur.disp_x = width + 1;
                }
            }
        }
        self.clear_sels()
    }

    fn get_tab_candidate(&mut self, is_asc: bool) -> String {
        Log::ep_s("set_path");
        let mut target_path = self.cont_3.buf.iter().collect::<String>();

        // Search target dir
        let mut base_dir = ".".to_string();
        // Character string target up to cur.x
        let _ = target_path.split_off(self.cont_3.cur.x);
        let vec: Vec<(usize, &str)> = target_path.match_indices("/").collect();
        // "/" exist
        if vec.len() > 0 {
            let (base, _) = target_path.split_at(vec[vec.len() - 1].0 + 1);
            base_dir = base.to_string();
        }

        if self.tab_comp.dirs.len() == 0 {
            if let Ok(mut read_dir) = fs::read_dir(&base_dir) {
                while let Some(Ok(path)) = read_dir.next() {
                    if path.path().is_dir() {
                        let mut dir_str = path.path().display().to_string();
                        let v: Vec<(usize, &str)> = dir_str.match_indices(target_path.as_str()).collect();
                        if v.len() > 0 {
                            // Replace "./" for display
                            if &base_dir == "." {
                                dir_str = dir_str.replace("./", "");
                            }
                            self.tab_comp.dirs.push(dir_str);
                        }
                    }
                }
            }
            self.tab_comp.dirs.sort();
        }

        Log::ep("read_dir", &self.tab_comp.dirs.clone().join(" "));

        let mut cont_3_str: String = self.cont_3.buf.iter().collect::<String>();
        for candidate in &self.tab_comp.dirs {
            // One candidate
            if self.tab_comp.dirs.len() == 1 {
                Log::ep_s("　　One candidate");
                cont_3_str = format!("{}{}", candidate.to_string(), "/");
                self.clear_tab_comp();
                break;

            // Multiple candidates
            } else if self.tab_comp.dirs.len() > 1 {
                Log::ep_s("  Multi candidates");
                Log::ep("self.tab_comp.index", &self.tab_comp.index);
                if is_asc && self.tab_comp.index >= self.tab_comp.dirs.len() - 1 || self.tab_comp.index == USIZE_UNDEFINED {
                    self.tab_comp.index = 0;
                } else if !is_asc && self.tab_comp.index == 0 {
                    self.tab_comp.index = self.tab_comp.dirs.len() - 1;
                } else {
                    self.tab_comp.index = if is_asc { self.tab_comp.index + 1 } else { self.tab_comp.index - 1 };
                }
                cont_3_str = self.tab_comp.dirs[self.tab_comp.index].clone();
                break;
            }
        }

        return cont_3_str;
    }

    pub fn clear_tab_comp(&mut self) {
        Log::ep_s("                  clear_tab_comp ");
        self.tab_comp.index = USIZE_UNDEFINED;
        self.tab_comp.dirs.clear();
    }
}

impl PromptCont {
    pub fn set_grep(&mut self, prom: &Prompt) {
        let base_posi = self.disp_row_posi - 1;

        if self.prompt_cont_posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_grep);
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Esc  {}{}:{}Tab {}({})",
                Colors::get_default_fg(),
                &LANG.search,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.move_input_field,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.complement,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.search_folder,
            );
            self.set_opt_case_sens();
            self.set_opt_regex();

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_str, Colors::get_default_fg());

            self.guide_row_posi = base_posi;
            self.key_desc_row_posi = base_posi + 1;
            self.opt_row_posi = base_posi + 2;
            self.buf_desc_row_posi = base_posi + 3;
            self.buf_row_posi = base_posi + 4;
        } else if self.prompt_cont_posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_file, Colors::get_default_fg());

            if prom.cache_search_filenm.len() > 0 {
                self.buf = prom.cache_search_filenm.chars().collect();
            } else {
                self.buf = "*.*".chars().collect();
            }
            self.buf_desc_row_posi = base_posi + 5;
            self.buf_row_posi = base_posi + 6;
        } else {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_folder, Colors::get_default_fg());
            if prom.cache_search_folder.len() > 0 {
                self.buf = prom.cache_search_folder.chars().collect();
            } else {
                if let Ok(path) = env::current_dir() {
                    self.buf = path.to_string_lossy().to_string().chars().collect();
                }
            };
            self.buf_desc_row_posi = base_posi + 7;
            self.buf_row_posi = base_posi + 8;
        }
    }
}
