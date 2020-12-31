extern crate ropey;
use crate::_cfg::lang::cfg::LangCfg;
use crate::def::*;
use crossterm::event::{Event, Event::Key, KeyCode::End};
use ropey::iter::{Bytes, Chars, Chunks, Lines};
use ropey::{Rope, RopeSlice};
use std::fmt;
use std::fs::File;
use std::io;
use std::path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}

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

    pub fn line_len<'a>(&'a self, i: usize) -> usize {
        self.text.line(i).len_chars()
    }

    pub fn char_vec<'a>(&'a self, i: usize) -> Vec<char> {
        self.line(i).chars().collect()
    }

    pub fn insert_char(&mut self, y: usize, x: usize, c: char) {
        let i = self.text.line_to_char(y) + x;
        self.text.insert_char(i, c);
    }

    pub fn insert(&mut self, y: usize, x: usize, s: &str) {
        let i = self.text.line_to_char(y) + x;
        self.text.insert(i, s);
    }

    pub fn remove(&mut self, do_type: DoType, y: usize, x: usize) {
        // new line CR
        let mut i = self.text.line_to_char(y) + x;

        Log::ep_s("★★★★★★★★★★");
        Log::ep("self.char(y, x)", self.char(y, x));
        Log::ep("NEW_LINE == self.char(y, x)", NEW_LINE == self.char(y, x));
        Log::ep("self.char(y, x - 1)", self.char(y, x - 1));
        Log::ep("NEW_LINE_CR == self.char(y, x - 1)", NEW_LINE_CR == self.char(y, x - 1));

        Log::ep("iiiii 111", i);

        let mut del_num = 1;

        if do_type == DoType::Del {
            if NEW_LINE_CR == self.char(y, x) && NEW_LINE == self.char(y, x + 1) {
                del_num += 1;
            }
        } else if do_type == DoType::BS {
            if NEW_LINE == self.char(y, x) && NEW_LINE_CR == self.char(y, x - 1) {
                i -= 1;
                del_num += 1;
            }
        }

        Log::ep("iiiii 222", i);

        self.text.remove(i..i + del_num);

        Log::ep("self.text", &self.text);
    }

    pub fn bytes<'a>(&'a self) -> Bytes<'a> {
        self.text.bytes()
    }

    pub fn char<'a>(&'a self, y: usize, x: usize) -> char {
        self.line(y).char(x)
    }

    pub fn chars<'a>(&'a self) -> Chars<'a> {
        self.text.chars()
    }

    pub fn len<'a>(&'a self) -> usize {
        self.text.len_lines()
    }

    pub fn lines<'a>(&'a self) -> Lines<'a> {
        self.text.lines()
    }

    pub fn chunks<'a>(&'a self) -> Chunks<'a> {
        self.text.chunks()
    }

    /*
    fn edit(&mut self, start: usize, end: usize, text: &str) {
        if start != end {
            self.text.remove(start..end);
        }
        if !text.is_empty() {
            self.text.insert(start, text);
        }
        self.dirty = true;
    }*/
}
