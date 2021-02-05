extern crate ropey;
use crate::{def::*, editor::draw::char_style::*, global::*, model::*, util::*};
use std::cmp::min;
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn draw_cache(&mut self) {
        Log::ep_s("　　　　　　　draw_cache");

        let highlighter = Highlighter::new(&CFG.get().unwrap().syntax.theme);

        // char_vec initialize
        let diff: isize = self.buf.len_lines() as isize - self.draw.char_vec.len() as isize;
        if diff > 0 {
            self.draw.char_vec.resize_with(self.buf.len_lines() as usize, || vec![]);
            self.draw.regions.resize_with(self.buf.len_lines() as usize, || vec![]);
        }

        let d_range = self.d_range.get_range();
        match d_range.draw_type {
            DrawType::Target | DrawType::After | DrawType::ScrollDown | DrawType::ScrollUp => {
                self.draw.sy = d_range.sy;
                self.draw.ey = d_range.ey;
            }
            DrawType::All | DrawType::None => {
                self.draw.sy = self.offset_y;
                self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num - 1);
            }
            _ => {}
        }

        Log::ep("self.draw.sy", &self.draw.sy);
        Log::ep("self.draw.ey", &self.draw.ey);

        match self.d_range.draw_type {
            DrawType::None => {
                // If highlight is enabled, read the full text first
                if is_enable_syntax_highlight(&self.ext) && self.draw.syntax_state_vec.len() == 0 {
                    self.draw.sy = 0;
                    self.draw.ey = self.buf.len_lines() - 1;
                    self.set_draw_regions(&highlighter);
                } else {
                    self.set_draw_regions(&highlighter);
                }
            }
            DrawType::Target | DrawType::After | DrawType::All | DrawType::ScrollDown => self.set_draw_regions(&highlighter),
            DrawType::Not | DrawType::ScrollUp => {}
        }
    }
    fn set_draw_regions(&mut self, highlighter: &Highlighter) {
        let sel_ranges = self.sel.get_range();

        for y in self.draw.sy..=self.draw.ey {
            let row_vec = self.buf.char_vec_line(y);
            if is_enable_syntax_highlight(&self.ext) {
                self.set_regions_highlight(y, row_vec, sel_ranges, &highlighter);
            } else {
                self.set_regions(y, row_vec, sel_ranges);
            }
        }
    }

    fn set_regions_highlight(&mut self, y: usize, row_vec: Vec<char>, sel_ranges: SelRange, highlighter: &Highlighter) {
        // Log::ep_s("                  set_regions");
        let mut regions: Vec<Region> = vec![];
        let row = row_vec.iter().collect::<String>();

        let scope;
        let mut parse;

        if self.draw.syntax_state_vec.len() == 0 {
            scope = ScopeStack::new();
            parse = ParseState::new(&CFG.get().unwrap().syntax.syntax_reference);
        } else {
            let y = if y == 0 { 1 } else { y };
            let syntax_state = self.draw.syntax_state_vec[y - 1].clone();
            scope = syntax_state.highlight_state.path.clone();
            parse = syntax_state.parse_state.clone();
        }
        let mut highlight_state = HighlightState::new(&highlighter, scope);
        let ops = parse.parse_line(&row, &CFG.get().unwrap().syntax.syntax_set);
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, &highlighter);
        let style_vec: Vec<(Style, &str)> = iter.collect();

        let (mut style_org, mut style_type_org) = (CharStyle::none(), CharStyleType::Nomal);
        let (mut x, mut width) = (0, 0);

        // If the target is highlight at the first display, all lines are read for highlight_state, but Style is only the display line.
        for (style, string) in style_vec {
            //Log::ep("style ", &style);
            //Log::ep("string ", &string);

            let mut style = CharStyle::from(style);
            for c in string.chars() {
                width += if c == NEW_LINE || c == NEW_LINE_CR { 1 } else { c.width().unwrap_or(0) };
                self.set_style(c, width, y, x, &mut style, &mut style_org, &mut style_type_org, sel_ranges, &mut regions);
                x += 1;
            }
        }
        self.draw.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state: parse, ops });
        self.draw.char_vec[y] = row_vec;
        self.draw.regions[y] = regions;
        // Log::ep("regions", regions.clone());
    }

    fn set_regions(&mut self, y: usize, row_vec: Vec<char>, sel_ranges: SelRange) {
        // Log::ep_s("                  set_regions");

        let mut regions: Vec<Region> = vec![];
        let (mut x, mut width) = (0, 0);
        let (mut style_org, mut style_type_org) = (CharStyle::none(), CharStyleType::Nomal);

        let sx = if y == self.cur.y { self.offset_x } else { 0 };
        let ex = min(sx + self.disp_col_num, self.buf.len_line_chars(y));

        let mut row: Vec<char> = vec![];
        row.resize(ex - sx, ' ');
        row.copy_from_slice(&row_vec[sx..ex]);

        for c in row {
            width += if c == NEW_LINE || c == NEW_LINE_CR { 1 } else { c.width().unwrap_or(0) };
            self.set_style(c, width, y, x, &CharStyle::normal(), &mut style_org, &mut style_type_org, sel_ranges, &mut regions);
            x += 1;
        }
        self.draw.char_vec[y] = row_vec;
        self.draw.regions[y] = regions;
    }

    fn set_style(&mut self, c: char, width: usize, y: usize, x: usize, style: &CharStyle, style_org: &mut CharStyle, style_type_org: &mut CharStyleType, sel_ranges: SelRange, regions: &mut Vec<Region>) {
        let from_style = self.draw.get_from_style(x, &style, &style_org, style_type_org);
        let style_type = self.draw.ctrl_style_type(c, width, &sel_ranges, &self.search.ranges, self.rnw, y, x);

        ダブルクリックの不具合から

        let to_style = match style_type {
            CharStyleType::Select => CharStyle::selected(),
            CharStyleType::Nomal => {
                if is_enable_syntax_highlight(&self.ext) {
                    *style
                } else {
                    CharStyle::normal()
                }
            }
            CharStyleType::CtrlChar => CharStyle::control_char(),
        };
        regions.push(Region { c, to: to_style, from: from_style });
        *style_org = to_style;
        *style_type_org = style_type;
    }
}

impl Draw {
    pub fn ctrl_style_type(&self, c: char, width: usize, sel_range: &SelRange, search_ranges: &Vec<SearchRange>, rnw: usize, y: usize, x: usize) -> CharStyleType {
        if sel_range.is_selected() && sel_range.sy <= y && y <= sel_range.ey {
            Log::ep("ccc", &c);
            Log::ep("xxx", &x);
            Log::ep("width", &width);

            let disp_x = width + rnw;
            Log::ep("disp_x", &disp_x);

            // Lines with the same start and end
            // Start line
            // End line
            // Intermediate line
            if (sel_range.sy == sel_range.ey && sel_range.s_disp_x <= disp_x && disp_x < sel_range.e_disp_x)
                || (sel_range.sy == y && sel_range.ey != y && sel_range.s_disp_x <= disp_x)
                || (sel_range.ey == y && sel_range.sy != y && disp_x < sel_range.e_disp_x)
                || (sel_range.sy < y && y < sel_range.ey)
            {
                Log::ep_s("Select Select Select Select Select Select Select");
                return CharStyleType::Select;
            }
        }
        for range in search_ranges {
            if range.y == y && range.sx <= x && x < range.ex {
                return CharStyleType::Select;
            } else if range.y > y {
                break;
            }
        }
        return if c == NEW_LINE { CharStyleType::CtrlChar } else { CharStyleType::Nomal };
    }

    pub fn get_from_style(&mut self, i: usize, style: &CharStyle, style_org: &CharStyle, style_type_org: &CharStyleType) -> CharStyle {
        let mut from_style = style;
        let selected = &CharStyle::selected();

        if i == 0 || style.fg != style_org.fg {
            return CharStyle::none();
        } else if style.fg == style_org.fg && style.bg == style_org.bg {
            from_style = if style_type_org == &CharStyleType::Select { selected } else { style }
        }
        return *from_style;
    }
}
