use std::cmp::{max, min};

use crate::{model::*, window::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::{def::*, models::draw::*};
use ewin_key::{model::*, sel_range::*};
use ewin_state::term::*;
use ewin_utils::char_edit::*;
use ewin_view::{char_style::*, model::*};
use syntect::{
    highlighting::{HighlightIterator, HighlightState, Highlighter, Style},
    parsing::{ParseState, ScopeStack},
};

impl Editor {
    pub fn tgt_editor_draw(&mut self, win: &Window) -> &mut EditorDraw {
        return &mut self.draw_cache[win.v_idx][win.h_idx];
    }

    pub fn resize_draw_vec(&mut self) {
        let vec = self.win_mgr.win_list.clone();
        self.draw_cache.resize_with(vec.len(), Vec::new);
        let editor_draw = self.draw_cache[0].get_mut(0).unwrap().clone();
        for (i, v) in vec.iter().enumerate() {
            self.draw_cache[i].resize(v.len(), editor_draw.clone());
        }
    }

    pub fn draw_cache(&mut self, win: &Window) {
        Log::debug_key("EditorDraw.draw_cache");
        Log::debug("win", &win);

        // char_vec initialize
        let diff: isize = self.buf.len_rows() as isize - self.tgt_editor_draw(win).cells_all.len() as isize;
        if diff > 0 {
            //  self.cells_to.resize(editor.buf.len_rows() as usize, Vec::new());
            let len_rows = self.buf.len_rows();
            self.tgt_editor_draw(win).cells_all.resize(len_rows, Vec::new());
            self.tgt_editor_draw(win).style_vecs.resize(len_rows, Vec::new());
        }
        Log::debug("editor.d_range", &self.draw_range);

        match self.draw_range {
            E_DrawRange::After(sy) => {
                self.tgt_editor_draw(win).sy = sy;
                self.tgt_editor_draw(win).ey = min(win.offset.y + win.height() - 1, self.buf.len_rows() - 1);
            }
            E_DrawRange::TargetRange(sy, ey) | E_DrawRange::ScrollDown(sy, ey) | E_DrawRange::ScrollUp(sy, ey) => {
                self.tgt_editor_draw(win).sy = max(sy, win.offset.y);
                self.tgt_editor_draw(win).ey = min(ey, min(win.offset.y + win.area_v.1 - 1, self.buf.len_rows() - 1));
            }
            E_DrawRange::All | E_DrawRange::WinOnlyAll | E_DrawRange::Targetpoint => {
                self.tgt_editor_draw(win).sy = win.offset.y;
                self.tgt_editor_draw(win).ey = if win.area_v.1 == 0 { 0 } else { min(win.offset.y + win.height() - 1, self.buf.len_rows() - 1) };
            }
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        }
        Log::debug("draw.sy", &self.tgt_editor_draw(win).sy);
        Log::debug("draw.ey", &self.tgt_editor_draw(win).ey);
        Log::debug("editor.offset_y", &self.win_mgr.curt_ref().offset.y);
        Log::debug("draw_cache editor.offset_x", &self.win_mgr.curt_ref().offset.x);
        Log::debug("draw_cache editor.offset_disp_x", &self.win_mgr.curt_ref().offset.disp_x);
        match self.draw_range {
            E_DrawRange::TargetRange(_, _) | E_DrawRange::After(_) | E_DrawRange::All | E_DrawRange::WinOnlyAll | E_DrawRange::Targetpoint | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => self.set_draw_regions(win),
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        }
    }

    fn set_draw_regions(&mut self, win: &Window) {
        let (sy, ey) = if self.is_enable_syntax_highlight && self.tgt_editor_draw(win).syntax_state_vec.is_empty() { (0, self.buf.len_rows() - 1) } else { (self.tgt_editor_draw(win).sy, self.tgt_editor_draw(win).ey) };

        Log::debug("self.cells_to_all.len() before", &self.tgt_editor_draw(win).cells_all.len());
        Log::debug("editor.change_info.del_row", &self.change_info.del_row_set);
        Log::debug("editor.change_info.new_row", &self.change_info.new_row);
        Log::debug("editor.change_info.restayle_row_set", &self.change_info.restayle_row_set);

        if !self.draw_cache[win.v_idx][win.h_idx].cells_all.is_empty() {
            for (i, del_i) in self.change_info.del_row_set.iter().enumerate() {
                self.draw_cache[win.v_idx][win.h_idx].cells_all.remove(*del_i - i);

                if self.is_enable_syntax_highlight {
                    self.draw_cache[win.v_idx][win.h_idx].style_vecs.remove(*del_i - i);
                }
            }
            for i in &self.change_info.new_row {
                self.draw_cache[win.v_idx][win.h_idx].cells_all.insert(*i, Vec::new());
                if self.is_enable_syntax_highlight {
                    self.draw_cache[win.v_idx][win.h_idx].style_vecs.insert(*i, Vec::new());
                }
            }
            for i in self.change_info.restayle_row_set.iter() {
                if self.draw_cache[win.v_idx][win.h_idx].cells_all.get(*i).is_some() {
                    self.draw_cache[win.v_idx][win.h_idx].cells_all[*i] = Vec::new();
                } else {
                    self.draw_cache[win.v_idx][win.h_idx].cells_all.insert(*i, Vec::new());
                }
                if self.is_enable_syntax_highlight && self.change_info.change_type == EditerChangeType::Edit {
                    self.draw_cache[win.v_idx][win.h_idx].style_vecs[*i] = Vec::new();
                }
            }
        }
        Log::debug("self.cells_to_all.len() after", &self.tgt_editor_draw(win).cells_all.len());
        let highlighter = Highlighter::new(&CfgSyntax::get().syntax.theme);
        if self.is_enable_syntax_highlight {
            for y in 0..sy {
                if self.tgt_editor_draw(win).cells_all[y].is_empty() && self.tgt_editor_draw(win).style_vecs[y].is_empty() {
                    Log::debug("highlight Preprocessing", &y);

                    let row_vec = self.buf.char_vec_row(y);
                    let sx_ex_range_opt = Editor::get_draw_x_range(&row_vec, win.offset.disp_x, win.height());
                    self.set_regions_highlight(win, y, row_vec, sx_ex_range_opt, &highlighter);
                }
            }
        }

        for y in sy..=ey {
            let mut row_vec = self.buf.char_vec_row(y);

            if y == self.buf.len_rows() - 1 && State::get().curt_state().editor.mouse == Mouse::Enable {
                row_vec.append(&mut EOF_STR.chars().collect::<Vec<char>>());
            }
            let sx_ex_range_opt = Editor::get_draw_x_range(&row_vec, win.offset.disp_x, self.get_curt_col_len());
            if self.is_enable_syntax_highlight {
                self.set_regions_highlight(win, y, row_vec, sx_ex_range_opt, &highlighter);
            } else {
                self.set_regions(win, y, &mut row_vec, sx_ex_range_opt);
            }
        }
        let mut change_row_vec: Vec<usize> = vec![];
        match &self.draw_range {
            E_DrawRange::All | E_DrawRange::WinOnlyAll | E_DrawRange::TargetRange(_, _) | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => change_row_vec = (self.tgt_editor_draw(win).sy..=self.tgt_editor_draw(win).ey).collect::<Vec<usize>>(),
            E_DrawRange::After(_) | E_DrawRange::Targetpoint => {
                for i in sy..=ey {
                    let vec_to = self.draw_cache[win.v_idx][win.h_idx].cells_to.get(&i).unwrap();
                    if let Some(vec_from) = self.draw_cache[win.v_idx][win.h_idx].cells_from.get(&i) {
                        if vec_to != vec_from {
                            change_row_vec.push(i);
                        }
                    } else {
                        change_row_vec.push(i);
                    }
                }
            }
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        };

        self.tgt_editor_draw(win).change_row_vec = change_row_vec;
        Log::debug("self.change_row_vec", &self.tgt_editor_draw(win).change_row_vec);
    }

    fn set_regions(&mut self, win: &Window, y: usize, row_vec: &mut [char], sx_ex_range_opt: Option<(DrawRangX, DrawRangX)>) {
        let mut cells: Vec<Cell> = vec![];
        let (mut x, mut width) = (0, 0);
        let mut style_org = CharStyle::none();

        if let Some((sx_range, ex_range)) = sx_ex_range_opt {
            let column_alignment_space_char = Cfg::get().general.editor.column_char_width_gap_space.character;
            let (sx, ex) = (sx_range.get_x(), ex_range.get_x());

            if !self.draw_cache[win.v_idx][win.h_idx].cells_all[y].is_empty() && self.draw_cache[win.v_idx][win.h_idx].cells_all[y].len() >= ex {
                let vec = self.draw_cache[win.v_idx][win.h_idx].cells_all[y][sx..ex].to_vec();
                self.draw_cache[win.v_idx][win.h_idx].cells_to.insert(y, vec);
            } else {
                for c in row_vec[sx..ex].iter() {
                    width += match *c {
                        NEW_LINE_LF | NEW_LINE_CR => 1,
                        TAB_CHAR => get_tab_width(width, Cfg::get().general.editor.tab.size),
                        _ => get_c_width(c),
                    };

                    let char_style_type = self.is_select_or_search_style_type(win, *c, win.offset.disp_x + width, y, sx + x);

                    self.set_style(self.get_to_style(Cfg::get(), self.is_enable_syntax_highlight, char_style_type, &CharStyle::normal(Cfg::get())), *c, &mut style_org, &mut cells);
                    x += 1;
                }
                self.tgt_editor_draw(win).cells_to.insert(y, cells.to_vec());
                self.tgt_editor_draw(win).cells_all[y] = cells;
            }
            self.set_column_char_width_gap_cell(y, sx_range, ex_range, column_alignment_space_char, win);
        } else {
            self.tgt_editor_draw(win).cells_to.insert(y, cells);
        }
    }

    fn set_regions_highlight(&mut self, win: &Window, y: usize, row_vec: Vec<char>, sx_ex_range_opt: Option<(DrawRangX, DrawRangX)>, highlighter: &Highlighter) {
        let mut cells: Vec<Cell> = vec![];
        if let Some((sx_range, ex_range)) = sx_ex_range_opt {
            let (sx, ex) = (sx_range.get_x(), ex_range.get_x());
            let column_alignment_space_char = Cfg::get().general.editor.column_char_width_gap_space.character;
            let row = row_vec.iter().collect::<String>();
            let scope;
            let mut parse;

            if !self.tgt_editor_draw(win).cells_all[y].is_empty() && self.tgt_editor_draw(win).cells_all[y].len() >= ex {
                let vec = self.tgt_editor_draw(win).cells_all[y][sx..ex].to_vec();
                self.tgt_editor_draw(win).cells_to.insert(y, vec);
            } else {
                if self.tgt_editor_draw(win).style_vecs[y].is_empty() {
                    if self.tgt_editor_draw(win).syntax_state_vec.is_empty() {
                        scope = ScopeStack::new();
                        // let h_file =  FileBar::get().curt_h_file().clone();
                        parse = ParseState::new(&CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&State::get().curt_state().file.ext).cloned().unwrap());
                    } else {
                        // Process from the previous row
                        let syntax_state = self.tgt_editor_draw(win).syntax_state_vec[if y == 0 { 0 } else { y - 1 }].clone();
                        scope = syntax_state.highlight_state.path.clone();
                        parse = syntax_state.parse_state;
                    }
                    let mut highlight_state = HighlightState::new(highlighter, scope);
                    let ops = parse.parse_line(&row, &CfgSyntax::get().syntax.syntax_set).unwrap();
                    let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, highlighter);
                    self.tgt_editor_draw(win).style_vecs[y] = iter.map(|(style, str)| (style, str.to_string())).collect::<Vec<(Style, String)>>();

                    let syntax_state = SyntaxState { highlight_state, parse_state: parse, ops };
                    if self.tgt_editor_draw(win).syntax_state_vec.get(y).is_none() {
                        self.tgt_editor_draw(win).syntax_state_vec.insert(y, syntax_state);
                    } else {
                        self.tgt_editor_draw(win).syntax_state_vec[y] = syntax_state;
                    }
                }
                // }
                let mut style_org = CharStyle::none();
                let (mut x, mut width) = (0, 0);

                // If the target is highlight at the first display, all lines are read for highlight_state, but Style is only the display line.
                for (style, string) in &self.draw_cache[win.v_idx][win.h_idx].style_vecs[y] {
                    let style = CharStyle::from_syntect_style(Cfg::get(), style);

                    for c in string.chars() {
                        width += match c {
                            NEW_LINE_LF | NEW_LINE_CR => 1,
                            TAB_CHAR => get_tab_width(width, Cfg::get().general.editor.tab.size),
                            _ => get_c_width(&c),
                        };
                        let style_type = self.is_select_or_search_style_type(win, c, width, y, x);
                        self.set_style(self.get_to_style(Cfg::get(), self.is_enable_syntax_highlight, style_type, &style), c, &mut style_org, &mut cells);
                        x += 1;
                    }
                }

                self.tgt_editor_draw(win).cells_to.insert(y, cells[sx_range.get_x()..ex_range.get_x()].to_vec());
                self.tgt_editor_draw(win).cells_all[y] = cells;
            }
            self.set_column_char_width_gap_cell(y, sx_range, ex_range, column_alignment_space_char, win);
        } else {
            self.tgt_editor_draw(win).cells_to.insert(y, cells);
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

    pub fn is_select_or_search_style_type(&self, window: &Window, c: char, width: usize, y: usize, x: usize) -> CharStyleType {
        let sel_range = &window.sel.get_range();

        if sel_range.is_selected() && sel_range.sy <= y && y <= sel_range.ey {
            if self.win_mgr.curt_ref().sel.mode == SelMode::Normal && self.box_insert.mode == BoxInsertMode::Normal {
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
        for range in &self.search.ranges {
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
                if y == self.buf.len_rows() - 1 {
                    let last_row_chars = self.buf.line(self.buf.len_rows() - 1).len_chars();
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

    fn set_column_char_width_gap_cell(&mut self, y: usize, sx_range: DrawRangX, ex_range: DrawRangX, column_alignment_space_char: char, win: &Window) {
        if sx_range.is_margin() || ex_range.is_margin() {
            let column_char_width_gap_cell = Cell { c: column_alignment_space_char, to: CharStyle::column_char_width_gap_space(Cfg::get()), from: CharStyle::default() };
            if sx_range.is_margin() {
                self.tgt_editor_draw(win).cells_to.get_mut(&y).unwrap()[0].from = CharStyle::none();
                self.tgt_editor_draw(win).cells_to.get_mut(&y).unwrap().insert(0, column_char_width_gap_cell);
            }
            if ex_range.is_margin() && Cfg::get().general.editor.column_char_width_gap_space.end_of_line_enable {
                self.tgt_editor_draw(win).cells_to.get_mut(&y).unwrap().push(column_char_width_gap_cell);
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

impl EditorDraw {
    pub fn clear(&mut self) {
        self.cells_all.clear();
        self.style_vecs.clear();
    }
}
