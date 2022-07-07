use crate::{
    ewin_com::{char_style::*, model::*, util::*},
    model::*,
};
use ewin_cfg::{log::*, model::default::*};
use ewin_com::_cfg::key::cmd::*;
use ewin_const::def::*;
use std::cmp::{max, min};
use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style};
use syntect::parsing::{ParseState, ScopeStack};

impl EditorDraw {
    pub fn draw_cache(&mut self, editor: &mut Editor, window: &Window) {
        // char_vec initialize
        let diff: isize = editor.buf.len_rows() as isize - self.cells_to_all.len() as isize;
        if diff > 0 {
            //  self.cells_to.resize(editor.buf.len_rows() as usize, Vec::new());
            self.cells_to_all.resize(editor.buf.len_rows() as usize, Vec::new());
            self.style_vecs.resize(editor.buf.len_rows() as usize, Vec::new());

            // When there is a change to offset_y in paste for highlight target
            if editor.is_enable_syntax_highlight && window.offset.y != window.offset.y_org && editor.cmd.cmd_type == CmdType::InsertStr("".to_string()) {
                self.clear();
            }
        }
        Log::debug("draw_cache.d_range", &window.draw_range);

        match window.draw_range {
            E_DrawRange::After(sy) => {
                self.sy = sy;
                self.ey = min(window.offset.y + window.area_v.1 - 1, editor.buf.len_rows() - 1);
            }
            E_DrawRange::TargetRange(sy, ey) | E_DrawRange::ScrollDown(sy, ey) | E_DrawRange::ScrollUp(sy, ey) => {
                self.sy = max(sy, window.offset.y);
                self.ey = min(ey, min(window.offset.y + window.area_v.1 - 1, editor.buf.len_rows() - 1));
            }
            E_DrawRange::Init | E_DrawRange::All | E_DrawRange::Targetpoint => {
                self.sy = window.offset.y;
                self.ey = if window.area_v.1 == 0 { 0 } else { min(window.offset.y + window.area_v.1 - window.area_v.0 - 1, editor.buf.len_rows() - 1) };
            }
            _ => {}
        }
        Log::debug("draw.sy", &self.sy);
        Log::debug("draw.ey", &self.ey);
        Log::debug("editor.offset_y", &editor.win_mgr.curt().offset.y);
        Log::debug("draw_cache editor.offset_x", &editor.win_mgr.curt().offset.x);
        Log::debug("draw_cache editor.offset_disp_x", &editor.win_mgr.curt().offset.disp_x);
        match window.draw_range {
            E_DrawRange::Init | E_DrawRange::TargetRange(_, _) | E_DrawRange::After(_) | E_DrawRange::All | E_DrawRange::Targetpoint | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => self.set_draw_regions(editor, window),
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        }
    }

   
    fn set_draw_regions(&mut self, editor: &Editor, win: &Window) {
        //  let (sy, ey) = if editor.is_enable_syntax_highlight && (self.syntax_state_vec.is_empty() || Editor::is_edit(&editor.e_cmd, true)) { (0, editor.buf.len_rows() - 1) } else { (self.sy, self.ey) };
        let (sy, ey) = if editor.is_enable_syntax_highlight && self.syntax_state_vec.is_empty() { (0, win.area_v.1) } else { (self.sy, self.ey) };

        Log::debug("self.cells_to_all.len() before", &self.cells_to_all.len());
        Log::debug("editor.change_info.del_row", &editor.change_info.del_row_set);
        Log::debug("editor.change_info.new_row", &editor.change_info.new_row);
        Log::debug("editor.change_info.restayle_row_set", &editor.change_info.restayle_row_set);

        if !self.cells_to_all.is_empty() {
            for (i, del_i) in editor.change_info.del_row_set.iter().enumerate() {
                self.cells_to_all.remove(*del_i - i);

                if editor.is_enable_syntax_highlight {
                    self.style_vecs.remove(*del_i - i);
                }
            }
            for i in &editor.change_info.new_row {
                self.cells_to_all.insert(*i, Vec::new());
                if editor.is_enable_syntax_highlight {
                    self.style_vecs.insert(*i, Vec::new());
                }
            }
            for i in &editor.change_info.restayle_row_set {
                self.cells_to_all[*i] = Vec::new();
                if editor.is_enable_syntax_highlight && editor.change_info.change_type == EditerChangeType::Edit {
                    self.style_vecs[*i] = Vec::new();
                }
            }
        }
        Log::debug("self.cells_to_all.len() after", &self.cells_to_all.len());
        let highlighter = Highlighter::new(&CfgSyntax::get().syntax.theme);
        if editor.is_enable_syntax_highlight {
            for y in 0..sy {
                if self.cells_to_all[y].is_empty() && self.style_vecs[y].is_empty() {
                    Log::debug("highlight Preprocessing", &y);

                    let row_vec = editor.buf.char_vec_row(y);
                    let sx_ex_range_opt = EditorDraw::get_draw_x_range(&row_vec, win.offset.disp_x, win.area_v.1-win.area_v.0);
                    self.set_regions_highlight(editor, win, y, row_vec, sx_ex_range_opt, &highlighter);
                }
            }
        }

      
        for y in sy..=ey {
            let mut row_vec = editor.buf.char_vec_row(y);

            if y == editor.buf.len_rows() - 1 && editor.state.mouse == Mouse::Enable {
                row_vec.append(&mut EOF_STR.chars().collect::<Vec<char>>());
            }
            let sx_ex_range_opt = EditorDraw::get_draw_x_range(&row_vec, win.offset.disp_x, editor.get_curt_col_len());
            if editor.is_enable_syntax_highlight {
                self.set_regions_highlight(editor, win, y, row_vec, sx_ex_range_opt, &highlighter);
            } else {
                self.set_regions(editor, win, y, &mut row_vec, sx_ex_range_opt);
            }
        }

        let mut change_style_vec: Vec<usize> = vec![];

        match &win.draw_range {
            E_DrawRange::Init | E_DrawRange::All | E_DrawRange::TargetRange(_, _) | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => change_style_vec = (sy..=ey).collect::<Vec<usize>>(),
            E_DrawRange::After(_) | E_DrawRange::Targetpoint => {
                for i in sy..=ey {
                    let vec_to = self.cells_to.get(&i).unwrap();
                    if let Some(vec_from) = self.cells_from.get(&i) {
                        if vec_to != vec_from {
                            change_style_vec.push(i);
                        }
                    } else {
                        change_style_vec.push(i);
                    }
                }
            }
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        };

        self.change_row_vec = change_style_vec;
        Log::debug("self.change_style_vec", &self.change_row_vec);
    }

   
    fn set_regions(&mut self, editor: &Editor, win: &Window, y: usize, row_vec: &mut [char], sx_ex_range_opt: Option<(DrawRangX, DrawRangX)>) {
        let mut cells: Vec<Cell> = vec![];
        let (mut x, mut width) = (0, 0);
        let mut style_org = CharStyle::none();

        if let Some((sx_range, ex_range)) = sx_ex_range_opt {
            let column_alignment_space_char = Cfg::get().general.editor.column_char_width_gap_space.character;
            let (sx, ex) = (sx_range.get_x(), ex_range.get_x());

            if !self.cells_to_all[y].is_empty() && self.cells_to_all[y].len() >= ex {
                self.cells_to.insert(y, self.cells_to_all[y][sx..ex].to_vec());
            } else {
                for c in row_vec[sx..ex].iter() {
                    width += match *c {
                        NEW_LINE_LF | NEW_LINE_CR => 1,
                        TAB_CHAR => get_tab_width(width, Cfg::get().general.editor.tab.size),
                        _ => get_c_width(c),
                    };

                    let char_style_type = self.is_select_or_search_style_type(editor, win, *c, win.offset.disp_x + width, y, sx + x);

                    self.set_style(self.get_to_style(Cfg::get(), editor.is_enable_syntax_highlight, char_style_type, &CharStyle::normal(Cfg::get())), *c, &mut style_org, &mut cells);
                    x += 1;
                }
                self.cells_to.insert(y, cells.to_vec());
                self.cells_to_all[y] = cells;
            }
            self.set_column_char_width_gap_cell(y, sx_range, ex_range, column_alignment_space_char);
        } else {
            // self.cells_to[y] = cells;
            self.cells_to.insert(y, cells);
        }
    }

    fn set_regions_highlight(&mut self, editor: &Editor, win: &Window, y: usize, row_vec: Vec<char>, sx_ex_range_opt: Option<(DrawRangX, DrawRangX)>, highlighter: &Highlighter) {
        Log::debug_key("                  set_regions_highlight");

        let mut cells: Vec<Cell> = vec![];
        if let Some((sx_range, ex_range)) = sx_ex_range_opt {
            let (sx, ex) = (sx_range.get_x(), ex_range.get_x());
            let column_alignment_space_char = Cfg::get().general.editor.column_char_width_gap_space.character;
            let row = row_vec.iter().collect::<String>();
            let scope;
            let mut parse;

            if !self.cells_to_all[y].is_empty() && self.cells_to_all[y].len() >= ex {
                self.cells_to.insert(y, self.cells_to_all[y][sx..ex].to_vec());
            } else {
                if self.style_vecs[y].is_empty() {
                    if self.syntax_state_vec.is_empty() {
                        scope = ScopeStack::new();
                        parse = ParseState::new(&self.syntax_reference.clone().unwrap());
                    } else {
                        // Process from the previous row
                        let syntax_state = self.syntax_state_vec[if y == 0 { 1 } else { y } - 1].clone();
                        scope = syntax_state.highlight_state.path.clone();
                        parse = syntax_state.parse_state;
                    }
                    let mut highlight_state = HighlightState::new(highlighter, scope);
                    let ops = parse.parse_line(&row, &CfgSyntax::get().syntax.syntax_set).unwrap();
                    let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, highlighter);
                    self.style_vecs[y] = iter.map(|(style, str)| (style, str.to_string())).collect::<Vec<(Style, String)>>();

                    if self.syntax_state_vec.get(y).is_none() {
                        self.syntax_state_vec.insert(y, SyntaxState { highlight_state, parse_state: parse, ops });
                    } else {
                        self.syntax_state_vec[y] = SyntaxState { highlight_state, parse_state: parse, ops };
                    }
                }
                // }
                let mut style_org = CharStyle::none();
                let (mut x, mut width) = (0, 0);

                // If the target is highlight at the first display, all lines are read for highlight_state, but Style is only the display line.
                for (style, string) in &self.style_vecs[y] {
                    let style = CharStyle::from_syntect_style(Cfg::get(), style);

                    for c in string.chars() {
                        width += match c {
                            NEW_LINE_LF | NEW_LINE_CR => 1,
                            TAB_CHAR => get_tab_width(width, Cfg::get().general.editor.tab.size),
                            _ => get_c_width(&c),
                        };
                        let style_type = self.is_select_or_search_style_type(editor, win, c, width, y, x);
                        self.set_style(self.get_to_style(Cfg::get(), editor.is_enable_syntax_highlight, style_type, &style), c, &mut style_org, &mut cells);
                        x += 1;
                    }
                }

                self.cells_to.insert(y, cells[sx_range.get_x()..ex_range.get_x()].to_vec());
                self.cells_to_all[y] = cells;
            }
            self.set_column_char_width_gap_cell(y, sx_range, ex_range, column_alignment_space_char);
        } else {
            //  self.cells_to[y] = cells;
            self.cells_to.insert(y, cells);
        }
    }

    fn get_to_style(&self, cfg: &Cfg, is_enable_syntax_highlight: bool, style_type: CharStyleType, style: &CharStyle) -> CharStyle {
        return match style_type {
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
            CharStyleType::ColumnCharAlignmentSpace => CharStyle::column_char_width_gap_space(cfg),
        };
    }

    fn set_style(&self, to_style: CharStyle, c: char, style_org: &mut CharStyle, cells: &mut Vec<Cell>) {
        cells.push(Cell { c, to: to_style, from: *style_org });
        *style_org = to_style;
    }

    pub fn is_select_or_search_style_type(&self, editor: &Editor, window: &Window, c: char, width: usize, y: usize, x: usize) -> CharStyleType {
        let sel_range = &window.sel.get_range();

        if sel_range.is_selected() && sel_range.sy <= y && y <= sel_range.ey {
            if editor.win_mgr.curt_ref().sel.mode == SelMode::Normal && editor.box_insert.mode == BoxInsertMode::Normal {
                // Lines with the same start and end
                if (sel_range.sy == sel_range.ey &&  sel_range.s_disp_x < width && width <= sel_range.e_disp_x ) 
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

                if sel_range.s_disp_x < width && width_org < sel_range.e_disp_x && !is_nl_char(c) {
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
            NEW_LINE_LF | TAB_CHAR | FULL_SPACE => CharStyleType::CtrlChar,
            _ => {
                if y == editor.buf.len_rows() - 1 {
                    let last_row_chars = editor.buf.line(editor.buf.len_rows() - 1).len_chars();
                    if x >= last_row_chars {
                        CharStyleType::CtrlChar
                    } else {
                        CharStyleType::Nomal
                    }
                } else {
                    CharStyleType::Nomal
                }
            }
        }
    }

    fn set_column_char_width_gap_cell(&mut self, y: usize, sx_range: DrawRangX, ex_range: DrawRangX, column_alignment_space_char: char) {
        if sx_range.is_margin() || ex_range.is_margin() {
            let column_char_width_gap_cell = Cell { c: column_alignment_space_char, to: CharStyle::column_char_width_gap_space(Cfg::get()), from: CharStyle::default() };
            if sx_range.is_margin() {
                self.cells_to.get_mut(&y).unwrap()[0].from = CharStyle::none();
                self.cells_to.get_mut(&y).unwrap().insert(0, column_char_width_gap_cell);
            }
            if ex_range.is_margin() && Cfg::get().general.editor.column_char_width_gap_space.end_of_line_enable {
                self.cells_to.get_mut(&y).unwrap().push(column_char_width_gap_cell);
            }
        }
    }
    fn get_draw_x_range(char_vec: &[char], offset_disp_x: usize, col_len: usize) -> Option<(DrawRangX, DrawRangX)> {
        let (mut cur_x, mut width) = (0, 0);
        let mut s_disp_opt: Option<DrawRangX> = None;
        let mut e_disp_opt: Option<DrawRangX> = None;

        for c in char_vec.iter() {
            let c_len = get_char_width(c, width);
            if s_disp_opt.is_none() {
                if offset_disp_x == 0 {
                    s_disp_opt = Some(DrawRangX::Range(0, false));
                } else if width + c_len == offset_disp_x {
                    s_disp_opt = Some(DrawRangX::Range(cur_x + 1, false));
                } else if width + c_len == offset_disp_x + 1 {
                    s_disp_opt = Some(DrawRangX::Range(cur_x + 1, true));
                }
            }

            if width >= offset_disp_x {
                if width + c_len - offset_disp_x == col_len {
                    e_disp_opt = Some(DrawRangX::Range(cur_x + 1, false));
                    break;
                } else if width + c_len - offset_disp_x == col_len + 1 {
                    e_disp_opt = Some(DrawRangX::Range(cur_x, true));
                    break;
                }
            }

            width += c_len;
            cur_x += 1;
        }

        if s_disp_opt.is_some() && e_disp_opt.is_none() {
            e_disp_opt = Some(DrawRangX::Range(cur_x, false));
        }

        return if let (Some(s_disp), Some(e_disp)) = (s_disp_opt, e_disp_opt) { Some((s_disp, e_disp)) } else { None };
    }
}
