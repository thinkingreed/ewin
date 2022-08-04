use crate::{core::*, menulist::*};
use ewin_cfg::{colors::Colors, log::*, model::default::*};
use ewin_key::util::*;
use std::collections::{BTreeMap, BTreeSet};

impl InputComple {
    pub const MAX_HEIGHT: usize = 10;

    pub fn set_disp_name(&mut self, search_str: &str) {
        Log::debug_key("set_disp_name");

        let menu_set = self.search(search_str);
        self.menulist.scrl_v.is_show = menu_set.len() > InputComple::MAX_HEIGHT;
        self.menulist.set_disp_name_single_widget(menu_set, None);
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

    pub fn clear(&mut self) {
        self.menulist.clear();
        self.search_set = BTreeSet::default();
    }
}

impl MenuListTrait for InputComple {
    fn clear(&mut self) {
        self.menulist.clear();
    }

    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("InputComple.draw");
        // calc offset
        self.menulist.calc_scrlbar_v();
        self.menulist.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputComple {
    pub menulist: MenuList,
    pub all_words_map: BTreeMap<String, BTreeSet<usize>>,
    pub row_words_vec: Vec<RowWords>,
    pub search_set: BTreeSet<String>,
}

impl Default for InputComple {
    fn default() -> Self {
        // row_words_vec: vec![RowWords::default()] is Correspondence of initial state
        InputComple { menulist: MenuList::new(MenuListConfig { menulist_type: MenuListType::MenuList, disp_type: MenuListDispType::Dynamic }), all_words_map: BTreeMap::default(), row_words_vec: vec![RowWords::default()], search_set: BTreeSet::default() }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RowWords {
    pub words: BTreeSet<String>,
}
