use crate::{
    _cfg::keys::{KeyCmd, Keybind},
    bar::headerbar::HeaderFile,
    colors::*,
    global::*,
    log::*,
    model::*,
    prompt::cont::promptcont::*,
    prompt::prompt::prompt::*,
    tab::Tab,
    terminal::*,
};
use std::{env, path::Path};

impl EvtAct {
    pub fn grep(term: &mut Terminal) -> EvtActType {
        Log::debug_s("Process.grep");
        match term.curt().editor.keycmd {
            KeyCmd::InsertLine => {
                let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let search_filenm = term.curt().prom.cont_2.buf.iter().collect::<String>();
                let mut search_folder = term.curt().prom.cont_3.buf.iter().collect::<String>();

                Log::debug("search_str", &search_str);
                Log::debug("search_filenm", &search_filenm);
                Log::debug("search_folder", &search_folder);

                if search_str.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_search_str);
                } else if search_filenm.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_search_file);
                } else if search_folder.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_search_folder);
                } else {
                    term.clear_curt_tab();
                    term.curt().state.clear_grep_info();

                    if search_folder.chars().nth(0).unwrap() != '/' && search_folder.chars().nth(0).unwrap() != 'C' {
                        let current_dir = env::current_dir().unwrap().display().to_string();
                        search_folder = format!("{}/{}", current_dir, search_folder);
                    }
                    Log::debug_s(&search_folder);
                    let path = Path::new(&search_folder).join(&search_filenm);

                    term.curt().prom.prom_grep.cache_search_filenm = search_filenm.clone();
                    term.curt().prom.prom_grep.cache_search_folder = search_folder.clone();

                    let mut tab_grep = Tab::new();
                    tab_grep.editor.search.str = search_str.clone();
                    tab_grep.editor.search.filenm = path.to_string_lossy().to_string();
                    tab_grep.editor.search.folder = search_folder.clone();
                    tab_grep.editor.mouse_mode = term.mode;

                    tab_grep.mbar.set_info(&LANG.searching);

                    tab_grep.state.grep_state.is_result = true;
                    tab_grep.state.grep_state.is_stdout_end = false;
                    tab_grep.state.grep_state.is_stderr_end = false;
                    tab_grep.state.grep_state.search_str = search_str.clone();
                    tab_grep.state.grep_state.search_filenm = search_filenm.clone();
                    tab_grep.state.grep_state.search_folder = search_folder.clone();
                    term.idx = term.tabs.len();
                    {
                        GREP_INFO_VEC.get().unwrap().try_lock().unwrap().push(tab_grep.state.grep_state.clone());
                    }
                    GREP_CANCEL_VEC.get().unwrap().try_lock().unwrap().resize_with(GREP_INFO_VEC.get().unwrap().try_lock().unwrap().len(), || false);

                    term.add_tab(tab_grep, HeaderFile::new(&format!(r#"{} "{}""#, &LANG.grep, &search_str)));
                    Prompt::set_grep_working(term);
                    term.curt().editor.d_range.draw_type = DrawType::All;

                    return EvtActType::DrawOnly;
                }
                term.curt().editor.d_range.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn grep(term: &mut Terminal) {
        term.curt().state.grep_state.is_grep = true;
        term.curt().prom.disp_row_num = 9;
        term.set_disp_size();
        term.curt().prom.cont_1 = PromptCont::new_edit_type(term.curt(), PromptContPosi::First).get_grep(&term.curt().prom);
        term.curt().prom.cont_2 = PromptCont::new_edit_type(term.curt(), PromptContPosi::Second).get_grep(&term.curt().prom);
        term.curt().prom.cont_3 = PromptCont::new_edit_type(term.curt(), PromptContPosi::Third).get_grep(&term.curt().prom);
    }

    pub fn draw_grep(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &self.cont_3.buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_row_posi, &self.cont_3.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn get_grep(&mut self, prom: &Prompt) -> PromptCont {
        let base_posi = self.disp_row_posi;

        if self.posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_grep);
            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}↓↑  {}{}:{}{}  {}{}:{}Tab {}({})",
                Colors::get_default_fg(),
                &LANG.search,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::ConfirmPrompt),
                Colors::get_default_fg(),
                &LANG.move_setting_location,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::EscPrompt),
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
        } else if self.posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_file, Colors::get_default_fg());

            if prom.prom_grep.cache_search_filenm.len() > 0 {
                self.buf = prom.prom_grep.cache_search_filenm.chars().collect();
            } else {
                self.buf = "*.*".chars().collect();
            }
            self.buf_desc_row_posi = base_posi + 5;
            self.buf_row_posi = base_posi + 6;
        } else {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.search_folder, Colors::get_default_fg());
            if prom.prom_grep.cache_search_folder.len() > 0 {
                self.buf = prom.prom_grep.cache_search_folder.chars().collect();
            } else {
                if let Ok(path) = env::current_dir() {
                    self.buf = path.to_string_lossy().to_string().chars().collect();
                }
            };
            self.buf_desc_row_posi = base_posi + 7;
            self.buf_row_posi = base_posi + 8;
        }
        return self.clone();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromGrep {
    pub cache_search_filenm: String,
    pub cache_search_folder: String,
    pub tab_comp: TabComp,
}

impl Default for PromGrep {
    fn default() -> Self {
        PromGrep { cache_search_filenm: String::new(), cache_search_folder: String::new(), tab_comp: TabComp::default() }
    }
}
