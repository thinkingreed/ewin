extern crate ropey;
use crate::{_cfg::cfg::Cfg, def::*, editor::view::char_style::*, global::*, log::*, model::*, util::*};
use std::cmp::min;
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn draw_cache(&mut self) {
        // char_vec initialize
        let diff: isize = self.buf.len_lines() as isize - self.draw.cells.len() as isize;
        if diff > 0 {
            self.draw.cells.resize_with(self.buf.len_lines() as usize, || vec![]);
        }

        let d_range = self.d_range.get_range();
        match d_range.draw_type {
            DrawType::Target | DrawType::After | DrawType::ScrollDown | DrawType::ScrollUp => {
                self.draw.sy = d_range.sy;
                self.draw.ey = if d_range.draw_type == DrawType::After { min(self.offset_y + self.disp_row_num - 1, self.buf.len_lines() - 1) } else { d_range.ey };
            }
            DrawType::All | DrawType::None => {
                self.draw.sy = self.offset_y;
                self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num - 1);
            }
            _ => {}
        }

        // If highlight is enabled, read the full text first
        if self.is_enable_syntax_highlight && self.draw.syntax_state_vec.len() == 0 {
            self.draw.sy = 0;
            self.draw.ey = self.buf.len_lines() - 1;
        }
        Log::debug("self.draw.sy", &self.draw.sy);
        Log::debug("self.draw.ey", &self.draw.ey);

        match self.d_range.draw_type {
            DrawType::None | DrawType::Target | DrawType::After | DrawType::All | DrawType::ScrollDown | DrawType::ScrollUp => self.set_draw_regions(),
            DrawType::Not | DrawType::MoveCur => {}
        }
    }
    fn set_draw_regions(&mut self) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        let sel_ranges = self.sel.get_range();

        for y in self.draw.sy..=self.draw.ey {
            let row_vec = self.buf.char_vec_line(y);
            let sx = if y == self.cur.y { self.offset_x } else { 0 };
            let ex = min(sx + self.disp_col_num - self.get_rnw() - Editor::RNW_MARGIN, self.buf.len_line_chars(y));

            if self.is_enable_syntax_highlight {
                self.set_regions_highlight(&cfg, y, row_vec, sel_ranges, sx, ex);
            } else {
                self.set_regions(&cfg, y, row_vec, sel_ranges, sx, ex);
            }
        }
    }

    fn set_regions_highlight(&mut self, cfg: &Cfg, y: usize, row_vec: Vec<char>, sel_ranges: SelRange, sx: usize, ex: usize) {
        // Log::ep_s("                  set_regions_highlight");

        let highlighter = Highlighter::new(&cfg.syntax.theme);
        let mut cells: Vec<Cell> = vec![];
        let row = row_vec.iter().collect::<String>();

        let scope;
        let mut parse;

        if self.draw.syntax_state_vec.len() == 0 {
            scope = ScopeStack::new();
            parse = ParseState::new(&self.draw.syntax_reference.clone().unwrap());
        } else {
            let y = if y == 0 { 1 } else { y };
            let syntax_state = self.draw.syntax_state_vec[y - 1].clone();
            scope = syntax_state.highlight_state.path.clone();
            parse = syntax_state.parse_state.clone();
        }

        let mut highlight_state = HighlightState::new(&highlighter, scope);
        let ops = parse.parse_line(&row, &cfg.syntax.syntax_set);
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, &highlighter);
        let style_vec: Vec<(Style, &str)> = iter.collect();

        let (mut style_org, mut style_type_org) = (CharStyle::none(), CharStyleType::Nomal);
        let (mut x, mut width) = (0, 0);

        // If the target is highlight at the first display, all lines are read for highlight_state, but Style is only the display line.
        for (style, string) in style_vec {
            // Log::ep("style ", &style);
            // Log::ep("string ", &string);

            let mut style = CharStyle::from_syntect_style(cfg, style);

            for c in string.chars() {
                width += match c {
                    NEW_LINE_LF | NEW_LINE_CR => 1,
                    TAB => get_char_width_exec(&c, width, cfg.general.editor.tab.width),
                    _ => c.width().unwrap_or(0),
                };
                self.set_style(cfg, c, width, y, x, &mut style, &mut style_org, &mut style_type_org, sel_ranges, &mut cells);
                x += 1;
            }
        }
        self.draw.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state: parse, ops });
        self.draw.cells[y] = cells.drain(sx..ex).collect();
    }

    fn set_regions(&mut self, cfg: &Cfg, y: usize, row_vec: Vec<char>, sel_ranges: SelRange, sx: usize, ex: usize) {
        // Log::ep_s("                  set_regions");

        let mut cells: Vec<Cell> = vec![];
        let (mut x, mut width) = (0, 0);
        let (mut style_org, mut style_type_org) = (CharStyle::none(), CharStyleType::Nomal);

        let mut row: Vec<char> = vec![];
        row.resize(ex - sx, ' ');
        row.copy_from_slice(&row_vec[sx..ex]);

        for c in row {
            width += match c {
                NEW_LINE_LF | NEW_LINE_CR => 1,
                TAB => get_char_width_exec(&c, width, cfg.general.editor.tab.width),
                _ => c.width().unwrap_or(0),
            };
            let offset_x = if y == self.cur.y { self.offset_x } else { 0 };
            self.set_style(cfg, c, width, y, offset_x + x, &CharStyle::normal(cfg), &mut style_org, &mut style_type_org, sel_ranges, &mut cells);
            x += 1;
        }
        self.draw.cells[y] = cells;
    }

    fn set_style(&mut self, cfg: &Cfg, c: char, width: usize, y: usize, x: usize, style: &CharStyle, style_org: &mut CharStyle, style_type_org: &mut CharStyleType, sel_ranges: SelRange, regions: &mut Vec<Cell>) {
        let from_style = self.draw.get_from_style(cfg, x, &style, &style_org, style_type_org);
        let style_type = self.draw.ctrl_style_type(c, width, &sel_ranges, &self.search.ranges, y, x);

        let to_style = match style_type {
            CharStyleType::Select => CharStyle::selected(&cfg),
            CharStyleType::Nomal => {
                if self.is_enable_syntax_highlight {
                    *style
                } else {
                    CharStyle::normal(cfg)
                }
            }
            CharStyleType::CtrlChar => CharStyle::control_char(&cfg),
        };
        regions.push(Cell { c, to: to_style, from: from_style });
        *style_org = to_style;
        *style_type_org = style_type;
    }
}

impl Draw {
    pub fn ctrl_style_type(&self, c: char, width: usize, sel_range: &SelRange, search_ranges: &Vec<SearchRange>, y: usize, x: usize) -> CharStyleType {
        if sel_range.is_selected() && sel_range.sy <= y && y <= sel_range.ey {
            // Lines with the same start and end
            // Start line
            // End line
            // Intermediate line
            if (sel_range.sy == sel_range.ey && sel_range.disp_x_s < width && width <= sel_range.disp_x_e) || (sel_range.sy == y && sel_range.ey != y && sel_range.disp_x_s <= width) || (sel_range.ey == y && sel_range.sy != y && width < sel_range.disp_x_e) || (sel_range.sy < y && y < sel_range.ey) {
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
        match c {
            NEW_LINE_LF | TAB => return CharStyleType::CtrlChar,
            _ => return CharStyleType::Nomal,
        }
    }

    pub fn get_from_style(&mut self, cfg: &Cfg, i: usize, style: &CharStyle, style_org: &CharStyle, style_type_org: &CharStyleType) -> CharStyle {
        let mut from_style = style;
        let selected = &CharStyle::selected(&cfg);

        if i == 0 || style.fg != style_org.fg {
            return CharStyle::none();
        } else if style.fg == style_org.fg && style.bg == style_org.bg {
            from_style = if style_type_org == &CharStyleType::Select { selected } else { style }
        }
        return *from_style;
    }
}
