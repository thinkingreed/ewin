use ewin_com::def::HEADERBAR_ROW_NUM;

use crate::{
    ewin_com::{
        _cfg::{key::keycmd::*, lang::lang_cfg::*},
        log::*,
        model::*,
    },
    model::*,
};
use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

impl Editor {
    pub fn set_draw_range(&mut self) {
        Log::debug_key("editor.set_draw_range");

        // judgment redraw
        Log::debug("self.draw_range Before setting", &self.draw_range);
        Log::debug("self.e_cmd", &self.e_cmd);
        Log::debug("self.offset_y_org", &self.offset_y_org);
        Log::debug("self.offset_y", &self.offset_y);
        Log::debug("self.sel", &self.sel);
        Log::debug("self.sel_org", &self.sel_org);
        Log::debug("self.search.ranges", &self.search);
        Log::debug("self.search.ranges_org", &self.search_org);
        Log::debug("self.buf.len_rows()", &self.buf.len_rows());
        Log::debug("self.row_len_org", &self.len_rows_org);

        Log::debug("self.change_info.restayle_row before", &self.change_info.restayle_row_set);

        self.draw_range = if matches!(self.e_cmd, E_Cmd::Resize(_, _))
        // enable_syntax_highlight edit
      ||  (Editor::is_edit(&self.e_cmd, true) && self.is_enable_syntax_highlight)
        || self.rnw_org != self.get_rnw() ||  self.offset_x_org != self.offset_x 
             // All draw at the end of key record
             || self.state.key_macro.is_exec_end
             || self.scrl_h.is_show_org != self.scrl_h.is_show
        {
            E_DrawRange::All
        } else if !Editor::is_edit(&self.e_cmd, true) && !self.sel.is_selected() && self.sel_org.is_selected() {
            let sel_org = self.sel_org.get_range();
            E_DrawRange::TargetRange(sel_org.sy, sel_org.ey)
        } else if (matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftLeft(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftRight(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _))) && self.scrl_v.is_enable {
            if self.offset_y_org == self.offset_y && self.scrl_v.row_posi_org == self.scrl_v.row_posi {
                E_DrawRange::Not
            } else {
                E_DrawRange::All
            }
        } else if self.offset_y_org != self.offset_y {
            if (self.offset_y_org as isize - self.offset_y as isize).abs() as usize > self.row_disp_len {
                E_DrawRange::All
            } else {
                match self.e_cmd {
                    E_Cmd::CursorUp | E_Cmd::MouseScrollUp | E_Cmd::MouseDragLeftUp(_, _) => E_DrawRange::ScrollUp(self.offset_y, self.offset_y + Editor::SCROLL_UP_DOWN_MARGIN + 1),
                    E_Cmd::CursorDown | E_Cmd::MouseScrollDown | E_Cmd::MouseDragLeftDown(_, _) => {
                        let y = min(self.buf.len_rows() - 1, self.offset_y + self.row_disp_len - 1);
                        E_DrawRange::ScrollDown(y - Editor::SCROLL_UP_DOWN_MARGIN - 1, y)
                    }
                    _ => E_DrawRange::All,
                }
            }
        } else {
            match &self.e_cmd {
                E_Cmd::InsertRow | E_Cmd::CursorDown | E_Cmd::CursorUp | E_Cmd::CursorRight | E_Cmd::CursorLeft if self.is_input_imple_mode(true) => self.get_input_comple_draw_range_y(),
                E_Cmd::CursorLeft | E_Cmd::CursorRight | E_Cmd::CursorLeftSelect | E_Cmd::CursorRightSelect | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::MouseDragLeftBox(_, _) => {
                    if self.sel.mode == SelMode::BoxSelect {
                        let sel = self.sel.get_range();
                        E_DrawRange::TargetRange(sel.sy, sel.ey)
                    } else if self.cur.y == self.cur_org.y {
                        if matches!(self.e_cmd, E_Cmd::CursorRightSelect) || matches!(self.e_cmd, E_Cmd::CursorLeftSelect) || matches!(self.e_cmd, E_Cmd::CursorRowHomeSelect) || matches!(self.e_cmd, E_Cmd::CursorRowEndSelect) {
                            E_DrawRange::TargetRange(self.cur.y, self.cur.y)
                        } else {
                            E_DrawRange::MoveCur
                        }
                    } else {
                        E_DrawRange::TargetRange(min(self.cur.y, self.cur_org.y), max(self.cur.y, self.cur_org.y))
                    }
                }
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Cut => {
                    if self.buf.len_rows() != self.len_rows_org {
                        E_DrawRange::After(min(self.cur.y, self.cur_org.y))
                    } else {
                        E_DrawRange::TargetRange(min(self.cur.y, self.cur_org.y), max(self.cur.y, self.cur_org.y))
                    }
                }
                E_Cmd::InsertRow => E_DrawRange::After(self.cur.y - 1),
                E_Cmd::InsertStr(_) if !self.is_input_imple_mode(true) && self.is_input_imple_mode(false) =>  E_DrawRange::All,
                E_Cmd::InsertStr(str) => {
                    if self.is_enable_syntax_highlight || self.box_insert.mode == BoxInsertMode::Insert {
                        E_DrawRange::All
                    } else if str.is_empty() {
                        E_DrawRange::After(min(self.cur_org.y, self.cur.y))
                    } else {
                        E_DrawRange::TargetRange(self.cur.y, self.cur.y)
                    }
                }
                E_Cmd::MouseDownLeft(y, x) if self.is_input_imple_mode(true) => {
                    if !self.input_comple.window.is_mouse_within_range(*y, *x, false) {
                        E_DrawRange::All
                    } else {
                        self.clear_input_comple();
                        E_DrawRange::All
                    }
                }
                E_Cmd::MouseMove(y, x) if self.is_input_imple_mode(true) => {
                    if self.input_comple.window.is_mouse_within_range(*y, *x, false) {
                        self.get_input_comple_draw_range_y()
                    } else {
                        E_DrawRange::Not
                    }
                }
                E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftUp(_, _) if self.scrl_h.is_enable => {
                    if matches!(self.e_cmd, E_Cmd::MouseDragLeftLeft(_, _)) && self.scrl_h.clm_posi_org == 0 || matches!(self.e_cmd, E_Cmd::MouseDragLeftRight(_, _)) && self.scrl_h.clm_posi_org + self.scrl_h.bar_len == self.col_len {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::All
                    }
                }

                E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) => E_DrawRange::TargetRange(self.cur.y, self.cur.y),
                E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftDown(_, _) if self.scrl_v.is_enable => E_DrawRange::All,
                E_Cmd::CursorDown | E_Cmd::CursorDownSelect | E_Cmd::MouseDragLeftDown(_, _) => {
                    if self.cur_org.y == self.buf.len_rows() - 1 {
                        E_DrawRange::Not
                    } else if matches!(self.e_cmd, E_Cmd::CursorDown) && self.sel.mode == SelMode::Normal {
                        E_DrawRange::MoveCur
                    } else {
                        E_DrawRange::TargetRange(self.cur.y - 1, self.cur.y)
                    }
                }
                E_Cmd::CursorUp | E_Cmd::CursorUpSelect | E_Cmd::MouseDragLeftUp(_, _) => {
                    if self.cur_org.y == 0 {
                        E_DrawRange::Not
                    } else if matches!(self.e_cmd, E_Cmd::CursorUp) && self.sel.mode == SelMode::Normal {
                        E_DrawRange::MoveCur
                    } else {
                        E_DrawRange::TargetRange(self.cur.y, if self.cur.y == 0 { 1 } else { self.cur.y + 1 })
                    }
                }
                E_Cmd::MouseScrollDown | E_Cmd::MouseScrollUp => {
                    if self.offset_y == 0 || self.offset_y + self.row_disp_len >= self.buf.len_rows() - 1 {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::All
                    }
                }
                E_Cmd::AllSelect => {
                    self.change_info.restayle_row_set = self.get_row_in_screen();
                    E_DrawRange::All
                }
                E_Cmd::Undo | E_Cmd::Redo | E_Cmd::CursorFileHome | E_Cmd::CursorFileEnd | E_Cmd::ReplaceExec(_, _, _) | E_Cmd::BoxSelectMode => E_DrawRange::All,
                E_Cmd::FindNext | E_Cmd::FindBack => E_DrawRange::MoveCur,
                E_Cmd::CancelState => {
                    if self.sel_org.is_selected() {
                        self.change_info.restayle_row_set = (self.sel_org.sy..=self.sel_org.ey).collect::<BTreeSet<usize>>();
                    }
                    for s in &self.search_org.ranges {
                        if self.is_y_in_screen(s.y) {
                            self.change_info.restayle_row_set.insert(s.y);
                        }
                    }
                    E_DrawRange::Targetpoint
                }

                E_Cmd::InputComple => E_DrawRange::All,
                _ => E_DrawRange::Not,
            }
        };
        if !Editor::is_edit(&self.e_cmd, true) {
            if let E_DrawRange::TargetRange(sy, ey) = self.draw_range {
                self.change_info.restayle_row_set.append(&mut (sy..=ey).collect::<BTreeSet<usize>>());
            }
            if let E_DrawRange::All = self.draw_range {
                self.change_info.restayle_row_set.append(&mut (0..self.buf.len_rows()).collect::<BTreeSet<usize>>());
            }
        }

        Log::debug("self.change_info.restayle_row after", &self.change_info.restayle_row_set);

        Log::debug("self.draw_range After setting", &self.draw_range);
    }

    pub fn get_input_comple_draw_range_y(&mut self) -> E_DrawRange {
        let (offset_y, editor_row_len) = (self.offset_y, self.row_disp_len);
        let draw_range_y_opt = self.input_comple.window.get_draw_range_y(offset_y, HEADERBAR_ROW_NUM, editor_row_len);
        if let Some((sy, ey)) = draw_range_y_opt {
            E_DrawRange::TargetRange(min(sy, self.cur.y), max(ey, self.cur.y))
        } else {
            E_DrawRange::Not
        }
    }

    pub fn set_draw_parts(&mut self, keycmd: &KeyCmd) -> RParts {
        Log::debug_s("editor.set_draw_parts");

        let parts = match keycmd {
            KeyCmd::Unsupported => RParts::MsgBar(Lang::get().unsupported_operation.to_string()),
            KeyCmd::Edit(e_keycmd) => match e_keycmd {
                E_Cmd::ReplacePrompt | E_Cmd::Encoding | E_Cmd::OpenFile(_) | E_Cmd::Find | E_Cmd::MoveRow | E_Cmd::Grep | E_Cmd::OpenMenu | E_Cmd::OpenMenuFile | E_Cmd::OpenMenuConvert | E_Cmd::OpenMenuEdit | E_Cmd::OpenMenuSearch | E_Cmd::OpenMenuMacro => RParts::Prompt,
                E_Cmd::CloseFile |   E_Cmd::NewTab 
                // | E_Cmd::SaveFile 
                | E_Cmd::Resize(_,_) | E_Cmd::MouseModeSwitch | E_Cmd::Help | E_Cmd::Null => RParts::All,
                 _ => {
                    if self.state.is_change_changed() {
                        Log::debug("self.state.is_change_changed()",&self.state.is_change_changed());
                        RParts::All
                    } else {
                        match self.draw_range {
                            E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => RParts::ScrollUpDown(ScrollUpDownType::Normal),
                            _ => RParts::Editor,
                        }
                    }
                }
            },
            _ => RParts::Editor,
        };
        Log::debug("editor.set_draw_parts after", &parts);
        return parts;
    }
}
