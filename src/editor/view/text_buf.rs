extern crate ropey;
use crate::{def::*, log::*, model::*, prompt::prompt::*};
use anyhow::Result;
use ropey::iter::Chars;
use ropey::{Rope, RopeSlice};
use std::fs::File;
use std::io;
use std::io::BufWriter;

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer { text: Rope::default() }
    }
}

impl TextBuffer {
    pub fn from_path(path: &str) -> io::Result<TextBuffer> {
        let text = Rope::from_reader(&mut io::BufReader::new(File::open(&path)?))?;
        Ok(TextBuffer { text: text })
    }

    pub fn line<'a>(&'a self, i: usize) -> RopeSlice<'a> {
        self.text.line(i)
    }

    pub fn len_line_chars<'a>(&'a self, i: usize) -> usize {
        self.text.line(i).len_chars()
    }

    pub fn len_chars<'a>(&'a self) -> usize {
        self.text.len_chars()
    }

    pub fn char_vec_line<'a>(&'a self, i: usize) -> Vec<char> {
        self.line(i).chars().collect()
    }

    pub fn char_vec_range<'a>(&'a self, y: usize, x: usize) -> Vec<char> {
        self.line(y).slice(..x).chars().collect()
    }

    pub fn insert_char(&mut self, y: usize, x: usize, c: char) {
        let i = self.text.line_to_char(y) + x;
        self.text.insert_char(i, c);
    }

    pub fn insert(&mut self, i: usize, s: &str) {
        self.text.insert(i, s);
    }

    pub fn insert_end(&mut self, s: &str) {
        self.text.insert(self.text.len_chars(), s);
    }
    pub fn remove(&mut self, s_idx: usize, e_idx: usize) {
        self.text.remove(s_idx..e_idx);
    }
    pub fn remove_type(&mut self, do_type: EvtType, y: usize, x: usize) {
        let mut i = self.text.line_to_char(y) + x;

        let mut del_num = 1;
        // not select del
        if do_type == EvtType::Del {
            if NEW_LINE_CR == self.char(y, x) && NEW_LINE == self.char(y, x + 1) {
                del_num = 2;
            }
        } else if do_type == EvtType::BS {
            if x > 0 {
                if NEW_LINE == self.char(y, x) && NEW_LINE_CR == self.char(y, x - 1) {
                    i -= 1;
                    del_num = 2;
                }
            }
        }
        self.text.remove(i..i + del_num);
    }

    pub fn remove_range(&mut self, sel: SelRange) {
        Log::ep("remove_range sel", &sel);

        let i_s = self.text.line_to_char(sel.sy) + sel.sx;
        let i_e = self.text.line_to_char(sel.ey) + sel.ex;

        self.text.remove(i_s..i_e);
    }

    pub fn write_to(&mut self, path: &str) -> Result<()> {
        self.text.remove(self.text.len_chars() - 1..self.text.len_chars());
        self.text.write_to(BufWriter::new(File::create(path)?))?;
        self.insert_end(EOF_MARK.to_string().as_str());
        Ok(())
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
        let s = self.text.line_to_char(sel.sy) + sel.sx;
        let e = self.text.line_to_char(sel.ey) + sel.ex;

        self.text.slice(s..e).to_string()
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

    pub fn char_to_line<'a>(&'a self, i: usize) -> usize {
        self.text.char_to_line(i)
    }

    pub fn char_to_line_idx<'a>(&'a self, i: usize) -> usize {
        i - self.text.line_to_char(self.text.char_to_line(i))
    }

    pub fn search(&self, search_str: &str, start_idx: usize, end_idx: usize) -> Vec<(usize, usize)> {
        const BATCH_SIZE: usize = 256;

        let mut head = start_idx; // Keep track of where we are between searches
        let mut matches = Vec::with_capacity(BATCH_SIZE);
        let mut tmp_vec: Vec<Vec<(usize, usize)>> = vec![];
        let mut rtn_vec: Vec<(usize, usize)> = vec![];

        loop {
            // Collect the next batch of matches.  Note that we don't use
            // `Iterator::collect()` to collect the batch because we want to
            // re-use the same Vec to avoid unnecessary allocations.
            matches.clear();
            for m in SearchIter::from_rope_slice(&self.text.slice(head..end_idx), &search_str).take(BATCH_SIZE) {
                matches.push(m);
            }
            if matches.is_empty() {
                break;
            }
            tmp_vec.push(matches.clone());

            // Update head for next iteration.
            head = (head as isize + matches.last().unwrap().1 as isize) as usize;
        }
        for vec in tmp_vec {
            for t in vec {
                rtn_vec.push(t);
            }
        }
        rtn_vec
    }

    pub fn search_and_replace(&mut self, search_pattern: &str, replacement_text: &str) {
        const BATCH_SIZE: usize = 256;
        let replacement_text_len = replacement_text.chars().count();

        let mut head = 0; // Keep track of where we are between searches
        let mut matches = Vec::with_capacity(BATCH_SIZE);

        loop {
            // Collect the next batch of matches.  Note that we don't use
            // `Iterator::collect()` to collect the batch because we want to
            // re-use the same Vec to avoid unnecessary allocations.
            matches.clear();
            for m in SearchIter::from_rope_slice(&self.text.slice(head..), &search_pattern).take(BATCH_SIZE) {
                matches.push(m);
            }

            // If there are no matches, we're done!
            if matches.is_empty() {
                break;
            }

            // Replace the collected matches.
            let mut index_diff: isize = 0;
            for &(start, end) in matches.iter() {
                // Get the properly offset indices.
                let start_d = (head as isize + start as isize + index_diff) as usize;
                let end_d = (head as isize + end as isize + index_diff) as usize;

                // Do the replacement.
                self.text.remove(start_d..end_d);
                self.text.insert(start_d, &replacement_text);

                // Update the index offset.
                let match_len = (end - start) as isize;
                index_diff = index_diff - match_len + replacement_text_len as isize;
            }

            // Update head for next iteration.
            head = (head as isize + index_diff + matches.last().unwrap().1 as isize) as usize;
        }
    }
}

impl<'a> SearchIter<'a> {
    fn from_rope_slice<'b>(slice: &'b RopeSlice, search_pattern: &'b str) -> SearchIter<'b> {
        assert!(!search_pattern.is_empty(), "Can't search using an empty search pattern.");
        SearchIter {
            char_iter: slice.chars(),
            search_pattern: search_pattern,
            search_pattern_char_len: search_pattern.chars().count(),
            cur_index: 0,
            possible_matches: Vec::new(),
        }
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

            // Check the rope's char against the next character in each of
            // the potential matches, removing the potential matches that
            // don't match.  We're using indexing instead of iteration here
            // so that we can remove the possible matches as we go.
            let mut i = 0;
            while i < self.possible_matches.len() {
                let pattern_char = self.possible_matches[i].next().unwrap();
                if next_char == pattern_char {
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
}
