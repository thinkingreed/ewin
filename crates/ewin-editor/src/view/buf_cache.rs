use crate::{
    ewin_com::{_cfg::cfg::*, _cfg::key::keycmd::*, char_style::*, def::*, global::*, log::*, model::*, util::*},
    model::*,
};
use std::cmp::min;
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};
use unicode_width::UnicodeWidthChar;

impl EditorDraw {
    pub fn draw_cache(&mut self, editor: &mut Editor) {
        // char_vec initialize
        let diff: isize = editor.buf.len_rows() as isize - self.cells.len() as isize;
        if diff > 0 {
            self.cells.resize_with(editor.buf.len_rows() as usize, Vec::new);

            // When there is a change to offset_y in paste for highlight target
            if editor.is_enable_syntax_highlight && editor.offset_y != editor.offset_y_org && editor.e_cmd == E_Cmd::InsertStr("".to_string()) {
                self.syntax_state_vec.clear();
            }
        }

        Log::debug("draw_cache.d_range", &editor.draw_range);
        match editor.draw_range {
            E_DrawRange::After(sy) => {
                self.sy = sy;
                self.ey = min(editor.offset_y + editor.row_disp_len - 1, editor.buf.len_rows() - 1);
            }
            E_DrawRange::Target(sy, ey) | E_DrawRange::ScrollDown(sy, ey) | E_DrawRange::ScrollUp(sy, ey) => {
                self.sy = sy;
                self.ey = min(ey, editor.buf.len_rows() - 1);
            }
            E_DrawRange::All | E_DrawRange::None => {
                self.sy = editor.offset_y;
                self.ey = if editor.row_disp_len == 0 { 0 } else { min(editor.offset_y + editor.row_disp_len - 1, editor.buf.len_rows() - 1) };
            }
            _ => {}
        }

        Log::debug("draw.sy", &self.sy);
        Log::debug("draw.ey", &self.ey);
        Log::debug("editor.offset_y", &editor.offset_y);

        match editor.draw_range {
            E_DrawRange::None | E_DrawRange::Target(_, _) | E_DrawRange::After(_) | E_DrawRange::All | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => self.set_draw_regions(editor),
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        }
    }

    fn set_draw_regions(&mut self, editor: &Editor) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        let (sy, ey) = if editor.is_enable_syntax_highlight && self.syntax_state_vec.is_empty() { (0, editor.buf.len_rows() - 1) } else { (self.sy, self.ey) };

        for y in sy..=ey {
            let row_vec = editor.buf.char_vec_line(y);
            let (sx, ex) = if editor.offset_x > row_vec.len() {
                (0, 0)
            } else {
                let sx = editor.offset_x;
                (sx, min(sx + editor.col_len, row_vec.len()))
            };

            if editor.is_enable_syntax_highlight {
                self.set_regions_highlight(&cfg, editor, y, row_vec, sx, ex);
            } else {
                self.set_regions(&cfg, editor, y, row_vec, sx, ex);
            }
        }
    }

    fn set_regions_highlight(&mut self, cfg: &Cfg, editor: &Editor, y: usize, row_vec: Vec<char>, sx: usize, ex: usize) {
        // Log::ep_s("                  set_regions_highlight");

        let highlighter = Highlighter::new(&cfg.syntax.theme);
        let mut cells: Vec<Cell> = vec![];
        let row = row_vec.iter().collect::<String>();

        let scope;
        let mut parse;

        if self.syntax_state_vec.is_empty() {
            scope = ScopeStack::new();
            parse = ParseState::new(&self.syntax_reference.clone().unwrap());
        } else {
            let y = if y == 0 { 1 } else { y };
            // Process from the previous row
            let syntax_state = self.syntax_state_vec[y - 1].clone();
            scope = syntax_state.highlight_state.path.clone();
            parse = syntax_state.parse_state;
        }

        let mut highlight_state = HighlightState::new(&highlighter, scope);
        let ops = parse.parse_line(&row, &cfg.syntax.syntax_set);
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, &highlighter);
        let style_vec: Vec<(Style, &str)> = iter.collect();

        let mut style_org = CharStyle::none();
        let (mut x, mut width) = (0, 0);

        // If the target is highlight at the first display, all lines are read for highlight_state, but Style is only the display line.
        for (style, string) in style_vec {
            let style = CharStyle::from_syntect_style(cfg, style);

            for c in string.chars() {
                width += match c {
                    NEW_LINE_LF | NEW_LINE_CR => 1,
                    TAB_CHAR => get_tab_width(width, cfg.general.editor.tab.size),
                    _ => c.width().unwrap_or(0),
                };
                let style_type = self.ctrl_style_type(editor, c, width, y, x);
                self.set_style(self.get_to_style(cfg, editor.is_enable_syntax_highlight, style_type, &style), c, &mut style_org, &mut cells);

                x += 1;
            }
        }
        self.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state: parse, ops });
        self.cells[y] = cells.drain(sx..ex).collect();
    }

    fn set_regions(&mut self, cfg: &Cfg, editor: &Editor, y: usize, row_vec: Vec<char>, sx: usize, ex: usize) {
        // Log::debug_key("set_regions");

        let mut cells: Vec<Cell> = vec![];
        let (mut x, mut width) = (0, 0);
        let mut style_org = CharStyle::none();

        let mut row: Vec<char> = vec![];
        row.resize(ex - sx, ' ');
        row.copy_from_slice(&row_vec[sx..ex]);

        for c in row {
            width += match c {
                NEW_LINE_LF | NEW_LINE_CR => 1,
                TAB_CHAR => get_tab_width(width, cfg.general.editor.tab.size),
                _ => c.width().unwrap_or(0),
            };
            let offset_x = if y == editor.cur.y { editor.offset_x } else { 0 };
            let offset_disp_x = if y == editor.cur.y { editor.offset_disp_x } else { 0 };
            let style_type = self.ctrl_style_type(editor, c, offset_disp_x + width, y, offset_x + x);

            self.set_style(self.get_to_style(cfg, editor.is_enable_syntax_highlight, style_type, &CharStyle::normal(cfg)), c, &mut style_org, &mut cells);
            x += 1;
        }
        self.cells[y] = cells;
    }

    fn get_to_style(&self, cfg: &Cfg, is_enable_syntax_highlight: bool, style_type: CharStyleType, style: &CharStyle) -> CharStyle {
        match style_type {
            CharStyleType::Select => CharStyle::selected(cfg),
            CharStyleType::Search => CharStyle::searched(cfg),
            CharStyleType::Nomal => {
                if is_enable_syntax_highlight {
                    *style
                } else {
                    CharStyle::normal(cfg)
                }
            }
            CharStyleType::CtrlChar => CharStyle::control_char(cfg),
        }
    }

    fn set_style(&self, to_style: CharStyle, c: char, style_org: &mut CharStyle, cells: &mut Vec<Cell>) {
        cells.push(Cell { c, to: to_style, from: *style_org });
        *style_org = to_style;
    }

    pub fn ctrl_style_type(&self, editor: &Editor, c: char, width: usize, y: usize, x: usize) -> CharStyleType {
        let sel_range = &editor.sel.get_range();

        if sel_range.is_selected() && sel_range.sy <= y && y <= sel_range.ey {
            if editor.sel.mode == SelMode::Normal {
                // Lines with the same start and end
                if (sel_range.sy == sel_range.ey &&  sel_range.s_disp_x < width && width <= sel_range.e_disp_x && c != EOF_MARK) 
                // Start line
                || (sel_range.sy == y && sel_range.ey != y && sel_range.s_disp_x < width) 
                // End line
                || (sel_range.ey == y && sel_range.sy != y && width <= sel_range.e_disp_x) 
                // Intermediate line
                || (sel_range.sy < y && y < sel_range.ey)
                {
                    return CharStyleType::Select;
                }
                // SelectMode::BoxSelect
            } else {
                let width_org = width - get_char_width(&c, width);
                if sel_range.s_disp_x < width && width_org < sel_range.e_disp_x && c != EOF_MARK && c != NEW_LINE_LF && c != NEW_LINE_CR {
                    return CharStyleType::Select;
                }
            }
        }
        for range in &editor.search.ranges {
            if range.y == y && range.sx <= x && x < range.ex {
                return CharStyleType::Search;
            } else if range.y > y {
                break;
            }
        }
        match c {
            // Ignore NEW_LINE_CR
            NEW_LINE_LF | TAB_CHAR => CharStyleType::CtrlChar,
            _ => CharStyleType::Nomal,
        }
    }
}
