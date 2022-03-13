use crate::model::*;
use ewin_com::{_cfg::key::keycmd::*, def::DELIM_STR, log::Log, util::*};
use std::collections::BTreeSet;

impl InputComple {
    pub fn init(&mut self) {
        Log::debug_key("InputComple.init");

        //     self.set_disp_name();
    }

    pub fn get_candidates_count(&mut self, search_str: &str) -> usize {
        return self.search(search_str).len();
    }

    pub fn set_disp_name(&mut self, search_str: &str) {
        Log::debug_key("set_disp_name");
        let (cols, _) = get_term_size();
        let menunm_max_len = cols as usize / 2 - 8;

        let menu_set = self.search(search_str);

        let mut cont = WindowCont { ..WindowCont::default() };
        for menu_str in menu_set {
            cont.menu_vec.push((WindowMenu { name: cut_str(menu_str, menunm_max_len, false, true), ..WindowMenu::default() }, None))
        }
        self.window.curt_cont = cont;

        let mut parent_max_len = 0;
        for (parent_menu, _) in self.window.curt_cont.menu_vec.iter() {
            let parent_name_len = get_str_width(get_cfg_lang_name(&parent_menu.name));
            parent_max_len = if parent_name_len > parent_max_len { parent_name_len } else { parent_max_len };
        }

        // set name_disp
        for (parent_menu, _) in self.window.curt_cont.menu_vec.iter_mut() {
            let perent_str = get_cfg_lang_name(&parent_menu.name);
            let space = parent_max_len - get_str_width(perent_str);
            parent_menu.name_disp = format!(" {}{} ", perent_str, " ".repeat(space),);
        }

        self.window.curt_cont.height = self.window.curt_cont.menu_vec.len();
        // +1 is Extra
        self.window.curt_cont.width = parent_max_len + 1;
    }

    pub fn analysis_new(&mut self, idx: usize, row_char: &[char]) {
        let s = String::from_iter(row_char);
        let str_vec = split_chars(&s, false, true, &DELIM_STR.chars().collect::<Vec<char>>());
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

        let str_vec = split_chars(&s, false, true, &DELIM_STR.chars().collect::<Vec<char>>());
        let words_new_set = str_vec.iter().cloned().collect::<BTreeSet<String>>();

        Log::debug_key("1111111111111111111111111111");
        Log::debug(" self.row_words_vec", &self.row_words_vec);

        if let Some(row_words_old) = self.row_words_vec.get_mut(idx) {
            Log::debug_key("2222222222222222222222222222");
            let diff_new_set = words_new_set.difference(&row_words_old.words);
            Log::debug_key("3333333333333333333333333");

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
            Log::debug_key("4444444444444444444444444");
            self.analysis_new(idx, row_char);
        }
    }

    pub fn search(&self, search_str: &str) -> BTreeSet<String> {
        let mut result_set = BTreeSet::new();
        let mut is_find = false;
        for (word, _) in self.all_words_map.iter() {
            if let Some(idx) = word.find(search_str) {
                if idx == 0 {
                    is_find = true;
                    result_set.insert(word.clone());
                }
            } else if is_find {
                break;
            }
        }
        result_set.remove(search_str);
        return result_set;
    }
}
