extern crate ropey;
use crate::{def::*, model::*, util::*};
use std::cmp::min;
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};

impl Editor {
    pub fn draw_cache(&mut self) {
        Log::ep_s("　　　　　　　draw_cache");

        let theme = self.syntax.theme.clone();
        let highlighter = Highlighter::new(&theme);

        // char_vec initialize
        let diff: isize = self.buf.len_lines() as isize - self.draw.char_vec.len() as isize;
        if diff > 0 {
            self.draw.char_vec.resize_with(self.buf.len_lines() as usize, || vec![]);
            self.draw.regions.resize_with(self.buf.len_lines() as usize, || vec![]);
        }

        self.draw.sy = self.offset_y;
        self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num - 1);

        let d_range = self.d_range.get_range();

        match d_range.d_type {
            DrawType::Target => {
                self.draw.sy = d_range.sy;
                self.draw.ey = d_range.ey;
            },
            DrawType::After => {
                self.draw.sy = d_range.sy;
            },
            DrawType::ScrollDown | DrawType::ScrollUp =>{
                self.draw.sy = d_range.sy;
                self.draw.ey = d_range.sy;
            },
            _=>{}
        }


        for y in self.draw.sy..=self.draw.ey {
            if self.draw.char_vec[y].len() == 0 {
                self.set_regions(y, &highlighter);
            }
        }

        Log::ep("self.draw.sy", self.draw.sy);
        Log::ep("self.draw.ey", self.draw.ey);

        match self.d_range.d_type {
            DrawType::Target | DrawType::After | DrawType::All | DrawType::ScrollDown => {
                for y in self.draw.sy..=self.draw.ey {
                    self.set_regions(y, &highlighter);
                }
            }
            DrawType::Not | DrawType::None | DrawType::ScrollUp => {}
        }
    }

    pub fn set_regions(&mut self, y: usize, highlighter: &Highlighter) {
        Log::ep_s("set_regions");

        let is_enable_syntax = is_enable_syntax(&self.extension);

        let sel_ranges = self.sel.get_range();
        let search_ranges = self.search.ranges.clone();

        Log::ep("sel_ranges", sel_ranges);

        let row_vec = self.buf.char_vec_line(y);
        let row = row_vec.iter().collect::<String>();

        let mut scope;
        let mut parse;

        if self.draw.syntax_state_vec.len() == 0 || y == 0 {
            scope = ScopeStack::new();
            parse = ParseState::new(&self.syntax.syntax);
        } else {
            let syntax_state = self.draw.syntax_state_vec[y - 1].clone();
            scope = syntax_state.highlight_state.path.clone();
            parse = syntax_state.parse_state.clone();
        }
        let mut highlight_state = HighlightState::new(&highlighter, scope);
        let ops = parse.parse_line(&row, &self.syntax.syntax_set);
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, &highlighter);
        let style_vec: Vec<(Style, &str)> = iter.collect();

        let mut i = 0;
        let mut regions: Vec<Region> = vec![];

        let mut char_style_org = CharStyle::NONE;
        let mut style_type_org = CharStyleType::Nomal;
        for (style, string) in style_vec {
            //eprintln!("style {:?}", style);
            //eprintln!("string {:?}", string);

            let char_style = CharStyle::from(style);

            for c in string.chars() {
                let from_style = self.draw.get_from_style(i, char_style, char_style_org, style_type_org);
                let style_type = self.draw.ctrl_style_type(&row_vec, &sel_ranges, &search_ranges, self.rnw, y, i);

                let to_style = match style_type {
                    CharStyleType::Select => CharStyle::SELECTED,
                    CharStyleType::Nomal => {
                        if is_enable_syntax {
                            char_style
                        } else {
                            CharStyle::DEFAULT
                        }
                    }
                    CharStyleType::CtrlChar => CharStyle::CTRL_CHAR,
                };
                regions.push(Region { c, to: to_style, from: from_style });
                i += 1;
                char_style_org = char_style;
                style_type_org = style_type;
            }
        }
        // Log::ep("regions", regions.clone());

        self.draw.char_vec[y] = row_vec;
        self.draw.regions[y] = regions;
        self.draw.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state: parse, ops });
    }
}

impl Draw {
      pub fn ctrl_style_type(&self, row_char: &Vec<char>, sel_ranges: &SelRange, search_ranges: &Vec<SearchRange>, rnw: usize, y: usize, x: usize) -> CharStyleType {
        let c = row_char[x];

        if  sel_ranges.is_selected() && sel_ranges.sy <= y && y <= sel_ranges.ey {   
            let (_, width) = get_row_width(&row_char[..x], true);
            let disp_x = width + rnw + 1;
            // Lines with the same start and end
            if (sel_ranges.sy == sel_ranges.ey && sel_ranges.s_disp_x <= disp_x && disp_x < sel_ranges.e_disp_x) 
            // Start line
            || (sel_ranges.sy == y && sel_ranges.ey != y && sel_ranges.s_disp_x <= disp_x )
            // End line
            || (sel_ranges.ey == y && sel_ranges.sy != y && disp_x < sel_ranges.e_disp_x )
            // Intermediate line
            || (sel_ranges.sy < y && y < sel_ranges.ey) {
                return CharStyleType::Select;
            } 
        }

        for range in search_ranges {
            if range.y != y {
                continue;
            } else if range.sx <= x && x < range.ex {
                return CharStyleType::Select;
            }
        }
        return if c == NEW_LINE { CharStyleType::CtrlChar } else { CharStyleType::Nomal };
    }

    pub fn get_from_style(&mut self, i: usize, char_style: CharStyle, char_style_org: CharStyle, style_type_org: CharStyleType) -> CharStyle {
        let mut from_style = char_style;
        if i == 0 || char_style.fg != char_style_org.fg {
            from_style = CharStyle::NONE;
        } else if char_style.fg == char_style_org.fg && char_style.bg == char_style_org.bg {
            if style_type_org == CharStyleType::Select {
                from_style = CharStyle::SELECTED;
            } else {
                from_style = char_style;
            }
        }
        return from_style;
    }
}
