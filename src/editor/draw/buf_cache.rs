extern crate ropey;
use crate::{editor::draw::char_style::*, model::*};
use rayon::prelude::*;
use std::cmp::min;
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};

impl Editor {
    pub fn draw_cache(&mut self) {
        Log::ep_s("　　　　　　　draw_cache");

        // char_vec initialize
        let diff: isize = self.buf.len_lines() as isize - self.draw.char_vec.len() as isize;
        if diff > 0 {
            self.draw.char_vec.resize_with(self.buf.len_lines() as usize, || vec![]);
            self.draw.regions.resize_with(self.buf.len_lines() as usize, || vec![]);
        }

        self.draw.sy = self.offset_y;
        self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num);

        let d_range = self.d_range.get_range();
        if d_range.d_type == DrawType::Target {
            self.draw.sy = d_range.sy;
            self.draw.ey = d_range.ey;
        } else if d_range.d_type == DrawType::After {
            self.draw.sy = d_range.sy;
        }

        Log::ep("self.draw.sy", self.draw.sy);
        Log::ep("self.draw.ey", self.draw.ey);

        for y in self.draw.sy..=self.draw.ey {
            if self.draw.char_vec[y].len() == 0 {
                self.set_regions(y);
            }
        }

        if self.history.len_history() > 0 {
            let hist: &HistoryInfo = self.history.get_history_last();
            let ep = hist.evt_proc.clone();
            match ep.d_range.d_type {
                DrawType::Target | DrawType::After | DrawType::All | DrawType::None => {
                    if self.is_edit_evt(true) {
                        for y in self.draw.sy..=self.draw.ey {
                            self.set_regions(y);
                        }
                    }
                }
                DrawType::Not => {}
            }
        }
    }

    pub fn set_regions(&mut self, y: usize) {
        Log::ep_s("set_regions");

        let sel_ranges = self.sel.get_range();
        let search_ranges = self.search.ranges.clone();

        let row_vec = self.buf.char_vec_line(y);
        let row = row_vec.iter().collect::<String>();

        let highlighter = &Highlighter::new(&self.syntax.theme_set.themes["base16-ocean.dark"]);

        let mut scope_stack;
        let mut parse_state;

        if self.draw.syntax_state_vec.len() == 0 || y == 0 {
            scope_stack = ScopeStack::new();
            parse_state = ParseState::new(&self.syntax.syntax);
        } else {
            let syntax_state = self.draw.syntax_state_vec[y - 1].clone();
            scope_stack = syntax_state.highlight_state.path.clone();
            parse_state = syntax_state.parse_state.clone();
        }
        let mut highlight_state = HighlightState::new(highlighter, scope_stack);
        let ops = parse_state.parse_line(&row, &self.syntax.syntax_set);
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, &highlighter);
        let style_vec: Vec<(Style, &str)> = iter.collect();

        let mut i = 0;
        let mut regions: Vec<Region> = vec![];

        for (style, string) in style_vec {
            let mut style_type_org = CharStyleType::None;
            for (xx, c) in string.chars().enumerate() {
                let style_type = self.ctrl_charstyletype(&row_vec, &sel_ranges, &search_ranges, y, i);
                let mut char_style = None;
                match style_type {
                    CharStyleType::Select => {
                        if style_type == CharStyleType::Select && style_type != style_type_org {
                            char_style = Some(styles::SELECTED);
                        }
                        regions.push(Region { c, to: char_style, from: styles::DEFAULT });
                    }
                    CharStyleType::None => {
                        if xx == 0 {
                            char_style = Some(CharStyle::from(style));
                        }
                        regions.push(Region { c, to: char_style, from: styles::DEFAULT });
                    }
                    CharStyleType::CtrlChar => regions.push(Region { c, to: Some(styles::CTRL_CHAR), from: styles::DEFAULT }),
                }
                style_type_org = style_type;
                i += 1;
            }
        }
        self.draw.char_vec[y] = row_vec;
        self.draw.regions[y] = regions;
        self.draw.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state, ops });
    }
}
