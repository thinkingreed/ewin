use crate::{
    bar::headerbar::*,
    colors::*,
    def::*,
    global::*,
    log::*,
    model::*,
    prompt::prompt::*,
    prompt::promptcont::promptcont::*,
    tab::{Tab, TabState},
    terminal::*,
    util::*,
};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::{env, fs, path::Path};

impl EvtAct {
    pub fn grep(term: &mut Terminal) -> EvtActType {
        Log::ep_s("Process.grep");

        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let search_str = term.tabs[term.idx].prom.cont_1.buf.iter().collect::<String>();
                    let search_filenm = term.tabs[term.idx].prom.cont_2.buf.iter().collect::<String>();
                    let mut search_folder = term.tabs[term.idx].prom.cont_3.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_search_str);
                    } else if search_filenm.len() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_search_file);
                    } else if search_folder.len() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_search_folder);
                    } else {
                        term.tabs[term.idx].mbar.clear();
                        term.tabs[term.idx].prom.clear();
                        term.tabs[term.idx].state.clear();

                        let current_dir = env::current_dir().unwrap().display().to_string();
                        Log::ep_s(&current_dir);
                        Log::ep_s(&search_folder);
                        if search_folder.chars().nth(0).unwrap() != '/' {
                            search_folder = format!("{}/{}", current_dir, search_folder);
                        }
                        Log::ep_s(&search_folder);
                        let path = Path::new(&search_folder).join(&search_filenm);

                        term.tabs[term.idx].prom.cache_search_filenm = search_filenm.clone();
                        term.tabs[term.idx].prom.cache_search_folder = search_folder.clone();

                        let mut tab_grep = Tab::new();
                        tab_grep.editor.search.str = search_str.clone();
                        tab_grep.editor.search.filenm = path.to_string_lossy().to_string();
                        tab_grep.editor.search.folder = search_folder.clone();

                        tab_grep.mbar.set_info(&LANG.searching);

                        tab_grep.state.grep_info.is_result = true;
                        tab_grep.state.grep_info.is_stdout_end = false;
                        tab_grep.state.grep_info.is_stderr_end = false;
                        tab_grep.state.grep_info.search_str = search_str.clone();
                        tab_grep.state.grep_info.search_filenm = search_filenm.clone();
                        tab_grep.state.grep_info.search_folder = search_folder.clone();
                        term.idx = term.tabs.len();

                        Log::ep("term.tab_idx", &term.idx);
                        Log::ep("tab_grep.state.grep_info", &tab_grep.state.grep_info);

                        GREP_CANCEL_VEC.get().unwrap().try_lock().unwrap().resize_with(term.idx + 1, || false);
                        GREP_INFO_VEC.get().unwrap().try_lock().unwrap().insert(term.idx, tab_grep.state.grep_info.clone());

                        let mut h_file = HeaderFile::default();
                        h_file.filenm = format!(r#"{} "{}""#, &LANG.grep, &search_str);
                        term.hbar.file_vec.push(h_file);

                        HeaderBar::set_header_filenm(term);

                        // term.set_disp_size();
                        term.tabs.push(tab_grep);
                        Prompt::set_grep_result(term);

                        return EvtActType::Next;
                    }
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn grep(term: &mut Terminal) {
        term.tabs[term.idx].state.grep_info.is_grep = true;
        term.tabs[term.idx].prom.disp_row_num = 9;
        term.set_disp_size();
        let mut cont_1 = PromptCont::new_edit(term.tabs[term.idx].prom.disp_row_posi as u16, PromptContPosi::First);
        let mut cont_2 = PromptCont::new_edit(term.tabs[term.idx].prom.disp_row_posi as u16, PromptContPosi::Second);
        let mut cont_3 = PromptCont::new_edit(term.tabs[term.idx].prom.disp_row_posi as u16, PromptContPosi::Third);
        cont_1.set_grep(&term.tabs[term.idx].prom);
        cont_2.set_grep(&term.tabs[term.idx].prom);
        cont_3.set_grep(&term.tabs[term.idx].prom);
        term.tabs[term.idx].prom.cont_1 = cont_1;
        term.tabs[term.idx].prom.cont_2 = cont_2;
        term.tabs[term.idx].prom.cont_3 = cont_3;
    }
    pub fn tab(&mut self, is_asc: bool, tab_state: &TabState) {
        Log::ep_s("tab");
        Log::ep("is_asc ", &is_asc);

        if tab_state.is_replace {
            match self.buf_posi {
                PromptContPosi::First => self.cursor_down(tab_state),
                PromptContPosi::Second => self.cursor_up(tab_state),
                _ => {}
            }
        } else if tab_state.grep_info.is_grep {
            match self.buf_posi {
                PromptContPosi::First => {
                    if is_asc {
                        self.cursor_down(tab_state);
                    } else {
                        self.buf_posi = PromptContPosi::Third;
                        Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                    }
                }
                PromptContPosi::Second => {
                    if is_asc {
                        self.cursor_down(tab_state);
                    } else {
                        self.cursor_up(tab_state);
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
