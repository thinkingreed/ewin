use crate::{core::WindowTrait, model::*};
use ewin_com::{_cfg::model::default::*, colors::*, log::Log, util::*};
use std::collections::BTreeSet;

impl InputComple {
    pub const MAX_HEIGHT: usize = 10;

    pub fn set_disp_name(&mut self, search_str: &str) {
        Log::debug_key("set_disp_name");
        let (cols, _) = get_term_size();
        let menunm_max_len = cols as usize / 2 - 8;

        let menu_set = self.search(search_str);
        self.window.scrl_v.is_show = menu_set.len() > InputComple::MAX_HEIGHT;

        let mut cont = WindowCont { ..WindowCont::default() };
        for menu_str in menu_set {
            cont.menu_vec.push((WindowMenu { name: menu_str.clone(), name_disp: cut_str(&menu_str, menunm_max_len, false, true) }, None));
        }
        self.window.curt_cont = cont;

        let mut parent_max_len = 0;
        for (parent_menu, _) in self.window.curt_cont.menu_vec.iter() {
            let parent_name_len = get_str_width(&parent_menu.name_disp);
            parent_max_len = if parent_name_len > parent_max_len { parent_name_len } else { parent_max_len };
        }

        // set name_disp
        for (parent_menu, _) in self.window.curt_cont.menu_vec.iter_mut() {
            let perent_str = get_cfg_lang_name(&parent_menu.name_disp);
            let space = parent_max_len - get_str_width(perent_str);
            parent_menu.name_disp = format!(" {}{} ", perent_str, " ".repeat(space),);
        }

        // +1 is Extra
        self.window.curt_cont.width = parent_max_len + 1;
    }

    pub fn analysis_new(&mut self, idx: usize, row_char: &[char]) {
        let s = String::from_iter(row_char);
        let str_vec = split_chars(&s, false, true, &Cfg::get().general.editor.input_comple.word_delimiter.chars().collect::<Vec<char>>());
        let words_new_set = str_vec.iter().cloned().collect::<BTreeSet<String>>();

        self.row_words_vec.insert(idx, RowWords { words: words_new_set.clone() });
        for word in words_new_set {
            self.all_words_map.entry(word).or_insert_with(|| BTreeSet::from([idx])).insert(idx);
        }
    }

    pub fn analysis_del(&mut self, del_vec: &BTreeSet<usize>) {
        let mut del_words = vec![];
        for (word, set) in self.all_words_map.iter_mut() {
            for del_idx in del_vec {
                if set.contains(del_idx) {
                    if set.len() == 1 {
                        del_words.push(word.clone());
                    } else {
                        set.remove(del_idx);
                    }
                }
            }
        }
        for word in del_words {
            self.all_words_map.remove(&word);
        }
        for (i, del_i) in (0..).zip(del_vec) {
            self.row_words_vec.remove(*del_i - i);
        }
    }

    pub fn analysis_mod(&mut self, idx: usize, row_char: &[char]) {
        Log::debug_key("analysis_mod");
        let s = String::from_iter(row_char);

        let str_vec = split_chars(&s, false, true, &Cfg::get().general.editor.input_comple.word_delimiter.chars().collect::<Vec<char>>());
        let words_new_set = str_vec.iter().cloned().collect::<BTreeSet<String>>();

        if let Some(row_words_old) = self.row_words_vec.get_mut(idx) {
            let diff_new_set = words_new_set.difference(&row_words_old.words);

            for word in diff_new_set {
                self.all_words_map.entry(word.clone()).or_insert_with(|| BTreeSet::from([idx])).insert(idx);
            }

            let diff_old_set = row_words_old.words.difference(&words_new_set);
            for word in diff_old_set {
                if let Some(set) = self.all_words_map.get_mut(word) {
                    if set.len() == 1 {
                        self.all_words_map.remove(word);
                    } else {
                        set.remove(&idx);
                    }
                }
            }
            row_words_old.words = words_new_set;
        } else {
            self.analysis_new(idx, row_char);
        }
    }

    pub fn search(&self, search_str: &str) -> BTreeSet<String> {
        Log::debug_key("InputComple.search");

        let mut result_set = BTreeSet::new();

        let search_str_tmp = if Cfg::get().general.editor.input_comple.case_sensitive { search_str.to_string() } else { search_str.to_lowercase() };

        for (word, _) in self.all_words_map.iter() {
            let word_tmp = if Cfg::get().general.editor.input_comple.case_sensitive { word.to_string() } else { word.to_lowercase() };
            if let Some(idx) = word_tmp.find(&search_str_tmp) {
                if idx == 0 && word_tmp != search_str_tmp {
                    result_set.insert(word.clone());
                }
            }
        }
        result_set.remove(search_str);
        return result_set;
    }
}

impl WindowTrait for InputComple {
    fn clear(&mut self) {
        self.window.clear();
    }

    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("InputComple.draw");
        self.window.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}
