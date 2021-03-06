use crate::{
    _cfg::{cfg::CfgSearch, keys::KeyCmd},
    def::*,
    global::*,
    log::Log,
    model::*,
    sel_range::{SelMode, SelRange},
};
use regex::RegexBuilder;
use ropey::{iter::Chars, Rope, RopeSlice};
use std::collections::BTreeSet;

impl TextBuffer {
    pub fn line<'a>(&'a self, i: usize) -> RopeSlice<'a> {
        self.text.line(i)
    }

    pub fn len_line_chars<'a>(&'a self, i: usize) -> usize {
        self.text.line(i).len_chars()
    }

    pub fn len_chars<'a>(&'a self) -> usize {
        self.text.len_chars()
    }

    pub fn len_bytes<'a>(&'a self) -> usize {
        self.text.len_bytes()
    }
    pub fn char_vec_line<'a>(&'a self, i: usize) -> Vec<char> {
        self.line(i).chars().collect()
    }

    pub fn char_vec_range<'a>(&'a self, y: usize, x: usize) -> Vec<char> {
        self.line(y).slice(..x).chars().collect()
    }

    /*
    pub fn insert_char(&mut self, y: usize, x: usize, c: char) {
        let i = self.text.line_to_char(y) + x;
        self.text.insert_char(i, c);
    } */
    pub fn insert(&mut self, y: usize, x: usize, s: &str) {
        let i = self.text.line_to_char(y) + x;
        self.text.insert(i, s);
    }

    pub fn insert_end(&mut self, s: &str) {
        self.text.insert(self.text.len_chars(), s);
    }
    pub fn remove(&mut self, s_idx: usize, e_idx: usize) {
        self.text.remove(s_idx..e_idx);
    }
    pub fn remove_del_bs(&mut self, keycmd: KeyCmd, y: usize, x: usize) {
        let mut i = self.text.line_to_char(y) + x;

        let mut del_num = 1;
        let c = self.char(y, x);
        // not select del
        if keycmd == KeyCmd::DeleteNextChar {
            if NEW_LINE_CR == c && NEW_LINE_LF == self.char(y, x + 1) {
                del_num = 2;
            }
        } else if keycmd == KeyCmd::DeletePrevChar {
            if x > 0 {
                if NEW_LINE_LF == c && NEW_LINE_CR == self.char(y, x - 1) {
                    i -= 1;
                    del_num = 2;
                }
            }
        }
        self.text.remove(i..i + del_num);
    }

    pub fn remove_range(&mut self, sel: SelRange, ep: &EvtProc) {
        match sel.mode {
            SelMode::Normal => {
                let i_s = self.text.line_to_char(sel.sy) + sel.sx;
                let i_e = self.text.line_to_char(sel.ey) + sel.ex;
                self.text.remove(i_s..i_e);
            }
            SelMode::BoxSelect => {
                for (sel, _) in &ep.box_sel_vec {
                    if sel.sy > self.text.len_lines() - 1 {
                        break;
                    }
                    let i_s = self.text.line_to_char(sel.sy) + sel.sx;
                    let i_e = self.text.line_to_char(sel.ey) + sel.ex;
                    self.text.remove(i_s..i_e);
                }
            }
        }
    }

    pub fn char<'a>(&'a self, y: usize, x: usize) -> char {
        let mut char = ' ';
        if self.len_line_chars(y) > x {
            char = self.line(y).char(x);
        }
        char
    }
    pub fn char_idx<'a>(&'a self, i: usize) -> char {
        self.text.char(i)
    }

    pub fn slice<'a>(&'a self, sel: SelRange) -> String {
        Log::debug("slice sel", &sel);

        let s = self.text.line_to_char(sel.sy) + sel.sx;
        let e = self.text.line_to_char(sel.ey) + sel.ex;

        return self.text.slice(s..e).to_string();
    }

    pub fn chars<'a>(&'a self) -> Chars<'a> {
        self.text.chars()
    }

    pub fn len_lines<'a>(&'a self) -> usize {
        self.text.len_lines()
    }

    pub fn line_to_char<'a>(&'a self, i: usize) -> usize {
        self.text.line_to_char(i)
    }
    pub fn line_to_byte<'a>(&'a self, i: usize) -> usize {
        self.text.line_to_byte(i)
    }

    pub fn char_to_line<'a>(&'a self, i: usize) -> usize {
        self.text.char_to_line(i)
    }

    pub fn byte_to_line<'a>(&'a self, i: usize) -> usize {
        self.text.byte_to_line(i)
    }

    pub fn char_to_line_char_idx<'a>(&'a self, i: usize) -> usize {
        i - self.text.line_to_char(self.text.char_to_line(i))
    }

    pub fn byte_to_line_char_idx<'a>(&'a self, i: usize) -> usize {
        self.text.byte_to_char(i) - self.text.line_to_char(self.text.byte_to_line(i))
    }

    pub fn append(&mut self, rope: Rope) {
        self.text.append(rope);
    }

    pub fn search(&self, search_pattern: &str, s_idx: usize, e_idx: usize) -> BTreeSet<(usize, usize)> {
        const BATCH_SIZE: usize = 256;

        let mut head = s_idx; // Keep track of where we are between searches
        let mut rtn_set: BTreeSet<(usize, usize)> = BTreeSet::new();

        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;

        if !cfg_search.regex {
            // normal
            let mut next_head = 0;
            loop {
                let mut is_end = true;
                for (sx, ex) in SearchIter::from_rope_slice(&self.text.slice(head..e_idx), &search_pattern, &cfg_search).take(BATCH_SIZE) {
                    rtn_set.insert((sx + head, ex + head));
                    next_head = ex + head;
                    is_end = false;
                }
                if is_end {
                    break;
                }
                head = next_head;
            }
        } else {
            // regex
            let result = RegexBuilder::new(&search_pattern).case_insensitive(!cfg_search.case_sens).build();
            let re = match result {
                Ok(re) => re,
                Err(_) => return rtn_set,
            };
            let s_line_idx = self.text.byte_to_line(s_idx);
            let e_line_idx = self.text.byte_to_line(e_idx);
            let lines = self.text.lines_at(s_line_idx);
            let mut len_chars_sum = s_idx;

            for (i, line) in lines.enumerate() {
                let str = String::from(line);

                for m in re.find_iter(&str) {
                    rtn_set.insert((len_chars_sum + m.start(), len_chars_sum + m.end()));
                }
                len_chars_sum += line.len_bytes();
                if i + s_line_idx == e_line_idx {
                    break;
                }
            }
        }

        rtn_set
    }
    pub fn replace_onece(&mut self, replace_str: &str, sel: &SelRange) -> usize {
        let i_s = self.text.line_to_char(sel.sy) + sel.sx;
        let i_e = self.text.line_to_char(sel.ey) + sel.ex;
        let mut set = BTreeSet::new();
        set.insert((i_s, i_e));
        let end_char_idx = self.replace(replace_str, &set);
        return end_char_idx;
    }

    pub fn replace(&mut self, replace_str: &str, search_set: &BTreeSet<(usize, usize)>) -> usize {
        let replace_str_len = replace_str.chars().count();
        // let search_set = self.search(search_pattern, 0, self.text.len_chars());

        let mut idx_diff: isize = 0;
        let mut end_char_idx: usize = 0;
        for (i, &(start, end)) in search_set.iter().enumerate() {
            let start = (start as isize + idx_diff) as usize;
            let end = (end as isize + idx_diff) as usize;

            self.text.remove(start..end);
            self.text.insert(start, &replace_str);

            // Update the index offset.
            let match_len = (end - start) as isize;
            idx_diff = idx_diff - match_len + replace_str_len as isize;

            if i == search_set.len() - 1 && start + replace_str_len > 0 {
                end_char_idx = start + replace_str_len - 1;
            }
        }
        return end_char_idx;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer { text: Rope::default() }
    }
}
impl<'a> SearchIter<'a> {
    fn from_rope_slice<'b>(slice: &'b RopeSlice, search_pattern: &'b str, cfg_search: &'b CfgSearch) -> SearchIter<'b> {
        assert!(!search_pattern.is_empty(), "Can't search using an empty search pattern.");
        SearchIter { char_iter: slice.chars(), search_pattern: search_pattern, search_pattern_char_len: search_pattern.chars().count(), cur_index: 0, possible_matches: Vec::new(), cfg_search: cfg_search }
    }
}

impl<'a> Iterator for SearchIter<'a> {
    type Item = (usize, usize);

    // Return the start/end char indices of the next match.
    fn next(&mut self) -> Option<(usize, usize)> {
        while let Some(next_char) = self.char_iter.next() {
            self.cur_index += 1;

            // Push new potential match, for a possible match starting at the
            // current char.
            self.possible_matches.push(self.search_pattern.chars());

            let mut i = 0;
            while i < self.possible_matches.len() {
                let pattern_char = self.possible_matches[i].next().unwrap();

                let equal = if self.cfg_search.case_sens { next_char == pattern_char } else { next_char.to_ascii_lowercase() == pattern_char.to_ascii_lowercase() };
                if equal {
                    if self.possible_matches[i].clone().next() == None {
                        // We have a match!  Reset possible matches and
                        // return the successful match's char indices.
                        let char_match_range = (self.cur_index - self.search_pattern_char_len, self.cur_index);
                        self.possible_matches.clear();
                        return Some(char_match_range);
                    } else {
                        // Match isn't complete yet, move on to the next.
                        i += 1;
                    }
                } else {
                    // Doesn't match, remove it.
                    self.possible_matches.swap_remove(i);
                }
            }
        }
        None
    }
}

/// An iterator over simple textual matches in a RopeSlice.
///
/// This implementation is somewhat naive, and could be sped up by using a
/// more sophisticated text searching algorithm such as Boyer-Moore or
/// Knuth-Morris-Pratt.
///
/// The important thing, however, is the interface.  For example, a regex
/// implementation providing an equivalent interface could easily be dropped
/// in, and the search-and-replace function above would work with it quite
/// happily.
struct SearchIter<'a> {
    char_iter: Chars<'a>,
    search_pattern: &'a str,
    search_pattern_char_len: usize,
    cur_index: usize,                           // The current char index of the search head.
    possible_matches: Vec<std::str::Chars<'a>>, // Tracks where we are in the search pattern for the current possible matches.
    cfg_search: &'a CfgSearch,
}
