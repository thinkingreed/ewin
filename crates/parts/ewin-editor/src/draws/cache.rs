use std::cmp::{max, min};

use crate::{model::*, window::window::*};
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
    pub fn draw_cache(draw_cache: &mut EditorDrawCache, editor: &Editor, win: &Window, draw_range: E_DrawRange) {
        Log::debug_key("EditorDraw.draw_cache");
        Log::debug("win.offset", &win.offset);

        // char_vec initialize
        let diff: isize = editor.buf.len_rows() as isize - draw_cache.cells_all.len() as isize;
        if diff > 0 {
            let len_rows = editor.buf.len_rows();

            draw_cache.cells_all.resize(len_rows, Vec::new());
            draw_cache.style_vecs.resize(len_rows, Vec::new());
        }
        Log::debug("d_range", &draw_range);

        match draw_range {
            E_DrawRange::After(sy) => {
                draw_cache.sy = sy;
                draw_cache.ey = min(win.offset.y + win.height() - 1, editor.buf.len_rows() - 1);
            }
            E_DrawRange::TargetRange(sy, ey) | E_DrawRange::ScrollDown(sy, ey) | E_DrawRange::ScrollUp(sy, ey) => {
                draw_cache.sy = max(sy, win.offset.y);
                draw_cache.ey = min(ey, min(win.offset.y + win.view.y_height() - 1, editor.buf.len_rows() - 1));
            }
            E_DrawRange::All | E_DrawRange::WinOnlyAll | E_DrawRange::Targetpoint => {
                draw_cache.sy = win.offset.y;
                draw_cache.ey = if win.view.y_height() == 0 { 0 } else { min(win.offset.y + win.height() - 1, editor.buf.len_rows() - 1) };
            }
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        }
        Log::debug("draw.sy", &draw_cache.sy);
        Log::debug("draw.ey", &draw_cache.ey);
        match draw_range {
            E_DrawRange::TargetRange(_, _) | E_DrawRange::After(_) | E_DrawRange::All | E_DrawRange::WinOnlyAll | E_DrawRange::Targetpoint | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => Editor::set_draw_regions(draw_cache, editor, win, draw_range),
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
        }
    }

    fn set_draw_regions(draw_cache: &mut EditorDrawCache, editor: &Editor, win: &Window, draw_range: E_DrawRange) {
        Log::debug("self.editor.cells_to_all.len() before", &draw_cache.cells_all.len());
        Log::debug("editor.change_info.del_row", &editor.change_info.del_row_set);
        Log::debug("editor.change_info.new_row", &editor.change_info.new_row);

        if !draw_cache.cells_all.is_empty() {
            for (i, del_i) in editor.change_info.del_row_set.iter().enumerate() {
                draw_cache.cells_all.remove(*del_i - i);

                if editor.is_enable_syntax_highlight {
                    draw_cache.style_vecs.remove(*del_i - i);
                }
            }
            for i in &editor.change_info.new_row {
                draw_cache.cells_all.insert(*i, Vec::new());
                if editor.is_enable_syntax_highlight {
                    draw_cache.style_vecs.insert(*i, Vec::new());
                }
            }
            for i in editor.change_info.restayle_row_set.iter() {
                if draw_cache.cells_all.get(*i).is_some() {
                    draw_cache.cells_all[*i] = Vec::new();
                } else {
                    draw_cache.cells_all.insert(*i, Vec::new());
                }
                if editor.is_enable_syntax_highlight && editor.change_info.change_type == EditerChangeType::Edit {
                    draw_cache.style_vecs[*i] = Vec::new();
                }
            }
        }
        Log::debug("self.editor.cells_to_all.len() after", &draw_cache.cells_all.len());
        let highlighter = Highlighter::new(&CfgSyntax::get().syntax.theme);
        if editor.is_enable_syntax_highlight {
            for y in 0..draw_cache.sy {
                if draw_cache.cells_all[y].is_empty() && draw_cache.style_vecs[y].is_empty() {
                    Log::debug("highlight Preprocessing", &y);

                    let row_vec = editor.buf.char_vec_row(y);
                    let sx_ex_range_opt = Editor::get_draw_x_range(&row_vec, win.offset.disp_x, win.height());
                    Editor::set_regions_highlight(draw_cache, win, y, row_vec, sx_ex_range_opt, &highlighter, editor);
                }
            }
        }

        for y in draw_cache.sy..=draw_cache.ey {
            let mut row_vec = editor.buf.char_vec_row(y);

            if y == editor.buf.len_rows() - 1 && State::get().curt_ref_state().editor.mouse == Mouse::Enable {
                row_vec.append(&mut EOF_STR.chars().collect::<Vec<char>>());
            }
            let sx_ex_range_opt = Editor::get_draw_x_range(&row_vec, win.offset.disp_x, editor.get_curt_col_len());
            if editor.is_enable_syntax_highlight {
                Editor::set_regions_highlight(draw_cache, win, y, row_vec, sx_ex_range_opt, &highlighter, editor);
            } else {
                Editor::set_regions(draw_cache, win, y, &mut row_vec, sx_ex_range_opt, editor);
            }
        }
        let mut change_row_vec: Vec<usize> = vec![];
        match &draw_range {
            E_DrawRange::All | E_DrawRange::WinOnlyAll | E_DrawRange::TargetRange(_, _) | E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => change_row_vec = (draw_cache.sy..=draw_cache.ey).collect::<Vec<usize>>(),
            E_DrawRange::After(_) | E_DrawRange::Targetpoint => {
                for i in draw_cache.sy..=draw_cache.ey {
                    let vec_to = draw_cache.cells_to.get(&i).unwrap();
                    if let Some(vec_from) = draw_cache.cells_from.get(&i) {
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

        draw_cache.change_row_vec = change_row_vec;
        Log::debug("self.editor.change_row_vec", &draw_cache.change_row_vec);
    }

    fn set_regions(draw_cache: &mut EditorDrawCache, win: &Window, y: usize, row_vec: &mut [char], sx_ex_range_opt: Option<(DrawRangX, DrawRangX)>, editor: &Editor) {
        let mut cells: Vec<Cell> = vec![];
        let (mut x, mut width) = (0, 0);
        let mut style_org = CharStyle::none();

        if let Some((sx_range, ex_range)) = sx_ex_range_opt {
            let column_alignment_space_char = Cfg::get().general.editor.column_char_width_gap_space.character;
            let (sx, ex) = (sx_range.get_x(), ex_range.get_x());

            if !draw_cache.cells_all[y].is_empty() && draw_cache.cells_all[y].len() >= ex {
                let vec = draw_cache.cells_all[y][sx..ex].to_vec();
                draw_cache.cells_to.insert(y, vec);
            } else {
                for c in row_vec[sx..ex].iter() {
                    width += match *c {
                        NEW_LINE_LF | NEW_LINE_CR => 1,
                        TAB_CHAR => get_tab_width(width, Cfg::get().general.editor.tab.size),
                        _ => get_c_width(c),
                    };

                    let char_style_type = Editor::is_select_or_search_style_type(win, *c, win.offset.disp_x + width, y, sx + x, editor);

                    Editor::set_style(Editor::get_to_style(Cfg::get(), editor.is_enable_syntax_highlight, char_style_type, &CharStyle::normal(Cfg::get())), *c, &mut style_org, &mut cells);
                    x += 1;
                }
                draw_cache.cells_to.insert(y, cells.to_vec());
                draw_cache.cells_all[y] = cells;
            }
            Editor::set_column_char_width_gap_cell(draw_cache, y, sx_range, ex_range, column_alignment_space_char);
        } else {
            draw_cache.cells_to.insert(y, cells);
        }
    }

    fn set_regions_highlight(draw_cache: &mut EditorDrawCache, win: &Window, y: usize, row_vec: Vec<char>, sx_ex_range_opt: Option<(DrawRangX, DrawRangX)>, highlighter: &Highlighter, editor: &Editor) {
        let mut cells: Vec<Cell> = vec![];
        if let Some((sx_range, ex_range)) = sx_ex_range_opt {
            let (sx, ex) = (sx_range.get_x(), ex_range.get_x());
            let column_alignment_space_char = Cfg::get().general.editor.column_char_width_gap_space.character;
            let row = row_vec.iter().collect::<String>();
            let scope;
            let mut parse;

            if !draw_cache.cells_all[y].is_empty() && draw_cache.cells_all[y].len() >= ex {
                let vec = draw_cache.cells_all[y][sx..ex].to_vec();
                draw_cache.cells_to.insert(y, vec);
            } else {
                if draw_cache.style_vecs[y].is_empty() {
                    if draw_cache.syntax_state_vec.is_empty() {
                        scope = ScopeStack::new();
                        // let h_file =  FileBar::get().curt_h_file().clone();
                        parse = ParseState::new(&CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&State::get().curt_ref_state().file.ext).cloned().unwrap());
                    } else {
                        // Process from the previous row
                        let syntax_state = draw_cache.syntax_state_vec[if y == 0 { 0 } else { y - 1 }].clone();
                        scope = syntax_state.highlight_state.path.clone();
                        parse = syntax_state.parse_state;
                    }
                    let mut highlight_state = HighlightState::new(highlighter, scope);
                    let ops = parse.parse_line(&row, &CfgSyntax::get().syntax.syntax_set).unwrap();
                    let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &row, highlighter);
                    draw_cache.style_vecs[y] = iter.map(|(style, str)| (style, str.to_string())).collect::<Vec<(Style, String)>>();

                    let syntax_state = SyntaxState { highlight_state, parse_state: parse, ops };
                    if draw_cache.syntax_state_vec.get(y).is_none() {
                        draw_cache.syntax_state_vec.insert(y, syntax_state);
                    } else {
                        draw_cache.syntax_state_vec[y] = syntax_state;
                    }
                }
                // }
                let mut style_org = CharStyle::none();
                let (mut x, mut width) = (0, 0);

                // If the target is highlight at the first display, all lines are read for highlight_state, but Style is only the display line.
                for (style, string) in &draw_cache.style_vecs[y] {
                    let style = CharStyle::from_syntect_style(Cfg::get(), style);

                    for c in string.chars() {
                        width += match c {
                            NEW_LINE_LF | NEW_LINE_CR => 1,
                            TAB_CHAR => get_tab_width(width, Cfg::get().general.editor.tab.size),
                            _ => get_c_width(&c),
                        };
                        let style_type = Editor::is_select_or_search_style_type(win, c, width, y, x, editor);
                        Editor::set_style(Editor::get_to_style(Cfg::get(), editor.is_enable_syntax_highlight, style_type, &style), c, &mut style_org, &mut cells);
                        x += 1;
                    }
                }

                draw_cache.cells_to.insert(y, cells[sx_range.get_x()..ex_range.get_x()].to_vec());
                draw_cache.cells_all[y] = cells;
            }
            Editor::set_column_char_width_gap_cell(draw_cache, y, sx_range, ex_range, column_alignment_space_char);
        } else {
            draw_cache.cells_to.insert(y, cells);
        }
    }

    fn get_to_style(cfg: &Cfg, is_enable_syntax_highlight: bool, style_type: CharStyleType, style: &CharStyle) -> CharStyle {
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

    fn set_style(to_style: CharStyle, c: char, style_org: &mut CharStyle, cells: &mut Vec<Cell>) {
        cells.push(Cell { c, to: to_style, from: *style_org });
        *style_org = to_style;
    }

    pub fn is_select_or_search_style_type(window: &Window, c: char, width: usize, y: usize, x: usize, editor: &Editor) -> CharStyleType {
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

    fn set_column_char_width_gap_cell(draw_cache: &mut EditorDrawCache, y: usize, sx_range: DrawRangX, ex_range: DrawRangX, column_alignment_space_char: char) {
        if sx_range.is_margin() || ex_range.is_margin() {
            let column_char_width_gap_cell = Cell { c: column_alignment_space_char, to: CharStyle::column_char_width_gap_space(Cfg::get()), from: CharStyle::default() };
            if sx_range.is_margin() {
                draw_cache.cells_to.get_mut(&y).unwrap()[0].from = CharStyle::none();
                draw_cache.cells_to.get_mut(&y).unwrap().insert(0, column_char_width_gap_cell);
            }
            if ex_range.is_margin() && Cfg::get().general.editor.column_char_width_gap_space.end_of_line_enable {
                draw_cache.cells_to.get_mut(&y).unwrap().push(column_char_width_gap_cell);
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
