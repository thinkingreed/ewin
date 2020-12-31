extern crate ropey;
use ropey::{iter::Chars, Rope, RopeSlice};

pub fn is_line_end(c: char) -> bool {
    ['\u{000a}', '\u{000b}', '\u{000c}', '\u{000d}', '\u{0085}', '\u{2028}', '\u{2029}'].contains(&c)
}
pub fn search_and_replace(rope: &mut Rope, search_pattern: &str, replacement_text: &str) {
    const BATCH_SIZE: usize = 256;
    let replacement_text_len = replacement_text.chars().count();

    let mut head = 0; // Keep track of where we are between searches
    let mut matches = Vec::with_capacity(BATCH_SIZE);
    loop {
        // Collect the next batch of matches.  Note that we don't use
        // `Iterator::collect()` to collect the batch because we want to
        // re-use the same Vec to avoid unnecessary allocations.
        matches.clear();
        for m in SearchIter::from_rope_slice(&rope.slice(head..), &search_pattern).take(BATCH_SIZE) {
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
            rope.remove(start_d..end_d);
            rope.insert(start_d, &replacement_text);

            // Update the index offset.
            let match_len = (end - start) as isize;
            index_diff = index_diff - match_len + replacement_text_len as isize;
        }

        // Update head for next iteration.
        head = (head as isize + index_diff + matches.last().unwrap().1 as isize) as usize;
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
