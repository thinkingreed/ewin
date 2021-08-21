extern crate ropey;
use crate::sel_range::SelMode;
use crate::{_cfg::cfg::Cfg, def::*, editor::view::char_style::*, global::*, log::*, model::*, util::*};
use std::cmp::min;
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};
use unicode_width::UnicodeWidthChar;

impl EditorDraw {
    pub fn draw_cache(&mut self, editor: &mut Editor) {
        // char_vec initialize
        let diff: isize = editor.buf.len_lines() as isize - self.cells.len() as isize;
        if diff > 0 {
            self.cells.resize_with(editor.buf.len_lines() as usize, || vec![]);
        }

        let d_range = editor.draw_type.clone();

        Log::debug("draw_cache.d_range", &d_range);
        match d_range {
           DrawType::After(sy) => {
            self.sy =sy;
            self.ey =  min(editor.offset_y + editor.disp_row_num - 1, editor.buf.len_lines() - 1) ;
            }
            DrawType::Target(sy,ey)  | DrawType::ScrollDown(sy,ey) | DrawType::ScrollUp(sy,ey) => {
                self.sy =sy;
                self.ey =  min(ey, editor.buf.len_lines() - 1) ;
            }
            DrawType::All | DrawType::None => {
                self.sy = editor.offset_y;
                self.ey = if editor.disp_row_num == 0 { 0 } else { min(editor.offset_y + editor.disp_row_num - 1, editor.buf.len_lines() - 1) };
            }
            _ => {}
        }

        let mut is_syntax_highlight_first_draw = false;
        // If highlight is enabled, read the full text first
        if editor.is_enable_syntax_highlight && self.syntax_state_vec.len() == 0 {
            is_syntax_highlight_first_draw = true;
            self.sy = 0;
            self.ey = editor.buf.len_lines() - 1;
        }
        Log::debug("draw.sy", &self.sy);
        Log::debug("draw.ey", &self.ey);

        match editor.draw_type {
            DrawType::None | DrawType::Target(_,_) | DrawType::After(_) | DrawType::All | DrawType::ScrollDown(_,_) | DrawType::ScrollUp(_,_) => self.set_draw_regions(&editor),
            DrawType::Not | DrawType::MoveCur => {}
        }

        // Correspondence of offset_y > 0 or more in file selection with "grep result"
        if is_syntax_highlight_first_draw && editor.offset_y > 0 {
            self.sy = editor.offset_y;
        }
    }
    fn set_draw_regions(&mut self,editor:&  Editor) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();

        for y in self.sy..=self.ey {
            let row_vec = editor.buf.char_vec_line(y);
            let sx = if y == editor.cur.y { editor.offset_x } else { 0 };
            let ex = min(sx + editor.disp_col_num, editor.buf.len_line_chars(y));

            if editor.is_enable_syntax_highlight {
                self.set_regions_highlight(&cfg, editor,y, row_vec, sx, ex);
            } else {
                self.set_regions(&cfg,editor, y, row_vec, sx, ex);
            }
        }
    }

    fn set_regions_highlight(&mut self, cfg: &Cfg,editor:& Editor, y: usize, row_vec: Vec<char>, sx: usize, ex: usize) {
        // Log::ep_s("                  set_regions_highlight");

        let highlighter = Highlighter::new(&cfg.syntax.theme);
        let mut cells: Vec<Cell> = vec![];
        let row = row_vec.iter().collect::<String>();

        let scope;
        let mut parse;

        if self.syntax_state_vec.len() == 0 {
            scope = ScopeStack::new();
            parse = ParseState::new(&self.syntax_reference.clone().unwrap());
        } else {
            let y = if y == 0 { 1 } else { y };
            let syntax_state = self.syntax_state_vec[y - 1].clone();
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
            let mut style = CharStyle::from_syntect_style(cfg, style);

            for c in string.chars() {
                width += match c {
                    NEW_LINE_LF | NEW_LINE_CR => 1,
                    TAB_CHAR => get_char_width_tab(&c, width, cfg.general.editor.tab.size),
                    _ => c.width().unwrap_or(0),
                };
                self.set_style(cfg, editor, c, width, y, x, &mut style, &mut style_org, &mut style_type_org, &mut cells);
                x += 1;
            }
        }
        self.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state: parse, ops });
        self.cells[y] = cells.drain(sx..ex).collect();
    }

    fn set_regions(&mut self, cfg: &Cfg,editor:& Editor, y: usize, row_vec: Vec<char>, sx: usize, ex: usize) {
        // Log::debug_key("set_regions");

        let mut cells: Vec<Cell> = vec![];
        let (mut x, mut width) = (0, 0);
        let (mut style_org, mut style_type_org) = (CharStyle::none(), CharStyleType::Nomal);

        let mut row: Vec<char> = vec![];
        row.resize(ex - sx, ' ');
        row.copy_from_slice(&row_vec[sx..ex]);

        for c in row {
            width += match c {
                NEW_LINE_LF | NEW_LINE_CR => 1,
                TAB_CHAR => get_char_width_tab(&c, width, cfg.general.editor.tab.size),
                _ => c.width().unwrap_or(0),
            };
            let offset_x = if y == editor.cur.y { editor.offset_x } else { 0 };
            let offset_disp_x = if y == editor.cur.y { editor.offset_disp_x } else { 0 };
            self.set_style(cfg, editor, c, offset_disp_x + width, y, offset_x + x, &CharStyle::normal(cfg), &mut style_org, &mut style_type_org, &mut cells);
            x += 1;
        }
        self.cells[y] = cells;
    }

    fn set_style(& self, cfg: &Cfg,editor:&Editor, c: char, width: usize, y: usize, x: usize, style: &CharStyle, style_org: &mut CharStyle, style_type_org: &mut CharStyleType, regions: &mut Vec<Cell>) {
        let style_type = self.ctrl_style_type(editor, c, width,  y, x);

        let to_style = match style_type {
            CharStyleType::Select => CharStyle::selected(&cfg),
            CharStyleType::Nomal => {
                if editor.is_enable_syntax_highlight {
                    *style
                } else {
                    CharStyle::normal(cfg)
                }
            }
            CharStyleType::CtrlChar => CharStyle::control_char(&cfg),
        };
        regions.push(Cell { c, to: to_style, from: *style_org });
        *style_org = to_style;
        *style_type_org = style_type;
    }
    pub fn ctrl_style_type(&self, editor:&Editor, c: char, width: usize, y: usize, x: usize) -> CharStyleType {
        let sel_range =  &editor.sel.get_range();
    
        if sel_range.is_selected() && sel_range.sy <= y && y <= sel_range.ey {
           
            if editor.sel.mode == SelMode::Normal {
                // Lines with the same start and end
                if (sel_range.sy == sel_range.ey &&  sel_range.s_disp_x < width && width <= sel_range.e_disp_x && c != EOF_MARK) 
                // Start line
                || (sel_range.sy == y && sel_range.ey != y && sel_range.s_disp_x < width) 
                // End line
                || (sel_range.ey == y && sel_range.sy != y && width <= sel_range.e_disp_x) 
                // Intermediate line
                || (sel_range.sy < y && y < sel_range.ey) {
                    return CharStyleType::Select;
            }
            // SelectMode::BoxSelect
        } else {

            let width_org = width- get_char_width(&c, width);
            if sel_range.s_disp_x < width && width_org < sel_range.e_disp_x &&  c != EOF_MARK && c != NEW_LINE_LF && c != NEW_LINE_CR  {
                return CharStyleType::Select;
            }
        }
    }
    for range in &editor.search.ranges {
        if range.y == y && range.sx <= x && x < range.ex {
            return CharStyleType::Select;
        } else if range.y > y {
            break;
        }
    }
    match c {
        // Ignore NEW_LINE_CR
        NEW_LINE_LF | TAB_CHAR => return CharStyleType::CtrlChar,
        _ => return CharStyleType::Nomal,
    }
}
}

