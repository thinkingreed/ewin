use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::model::*;
use ewin_key::{key::cmd::*, model::*};
use ewin_view::sel_range::*;
use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

impl Editor {
    pub fn set_draw_range(&mut self) {
        Log::debug_key("editor.set_draw_range");

        // judgment redraw
        Log::debug("self.draw_range Before setting", &self.draw_range);

        self.draw_range = if matches!(self.cmd.cmd_type, CmdType::Resize(_, _))
        // enable_syntax_highlight edit
      ||  (self.cmd.config.is_edit && self.is_enable_syntax_highlight)
        || self.rnw_org != self.get_rnw() ||  self.win_mgr.curt().offset.x_org != self.win_mgr.curt().offset.x 
             // All draw at the end of key record
             || self.state.key_macro.is_exec_end
             || self.win_mgr.curt().scrl_h.is_show_org != self.win_mgr.curt().scrl_h.is_show
             || !self.cmd.config.is_edit && !self.win_mgr.curt().sel.is_selected() && self.win_mgr.curt().sel_org.is_selected()
             || self.search != self.search_org
        {
            E_DrawRange::All
        } else if (matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftLeft(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftRight(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _))) && self.win_mgr.curt().scrl_v.is_enable {
            if self.win_mgr.curt().offset.y_org == self.win_mgr.curt().offset.y && self.win_mgr.curt().scrl_v.row_posi_org == self.win_mgr.curt().scrl_v.row_posi {
                E_DrawRange::Not
            } else {
                E_DrawRange::WinOnlyAll
            }
        } else if self.win_mgr.curt().offset.y_org != self.win_mgr.curt().offset.y {
            if (self.win_mgr.curt().offset.y_org as isize - self.win_mgr.curt().offset.y as isize).unsigned_abs() > self.get_curt_row_len() {
                E_DrawRange::All
            } else {
                match self.cmd.cmd_type {
                    CmdType::CursorUp | CmdType::MouseScrollUp | CmdType::MouseDragLeftUp(_, _) | CmdType::CursorDown | CmdType::MouseScrollDown | CmdType::MouseDragLeftDown(_, _) if self.win_mgr.split_type != WindowSplitType::None => E_DrawRange::WinOnlyAll,
                    CmdType::CursorUp | CmdType::MouseScrollUp | CmdType::MouseDragLeftUp(_, _) => E_DrawRange::ScrollUp(self.win_mgr.curt().offset.y, self.win_mgr.curt().offset.y + Editor::SCROLL_UP_DOWN_MARGIN + 1),
                    CmdType::CursorDown | CmdType::MouseScrollDown | CmdType::MouseDragLeftDown(_, _) => {
                        let y = min(self.buf.len_rows() - 1, self.win_mgr.curt().offset.y + self.get_curt_row_len() - 1);
                        E_DrawRange::ScrollDown(y - Editor::SCROLL_UP_DOWN_MARGIN - 1, y)
                    }
                    _ => E_DrawRange::All,
                }
            }
        } else {
            match &self.cmd.cmd_type {
                //  E_Cmd::InsertRow | E_Cmd::CursorDown | E_Cmd::CursorUp | E_Cmd::CursorRight | E_Cmd::CursorLeft if self.is_input_imple_mode(true) =>    self.input_comple.window.get_draw_range_y(self.offset_y, HEADERBAR_ROW_NUM, self.row_disp_len),
                CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorLeftSelect | CmdType::CursorRightSelect | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect | CmdType::MouseDragLeftBox(_, _) => {
                    if self.win_mgr.curt().sel.mode == SelMode::BoxSelect {
                        let sel = self.win_mgr.curt().sel.get_range();
                        E_DrawRange::TargetRange(sel.sy, sel.ey)
                    } else if self.win_mgr.curt().cur.y == self.win_mgr.curt().cur_org.y {
                        if matches!(self.cmd.cmd_type, CmdType::CursorRightSelect) || matches!(self.cmd.cmd_type, CmdType::CursorLeftSelect) || matches!(self.cmd.cmd_type, CmdType::CursorRowHomeSelect) || matches!(self.cmd.cmd_type, CmdType::CursorRowEndSelect) {
                            E_DrawRange::TargetRange(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur.y)
                        } else {
                            E_DrawRange::MoveCur
                        }
                    } else {
                        E_DrawRange::TargetRange(min(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur_org.y), max(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur_org.y))
                    }
                }
                CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut => {
                    if self.buf.len_rows() != self.buf_len_rows_org {
                        // E_DrawRange::After(min(self.cur.y, self.cur_org.y))
                        E_DrawRange::All
                    } else {
                        E_DrawRange::TargetRange(min(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur_org.y), max(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur_org.y))
                    }
                }
                CmdType::InsertRow => E_DrawRange::After(self.win_mgr.curt().cur.y - 1),
                CmdType::InsertStr(_) if !self.is_input_imple_mode(true) && self.is_input_imple_mode(false) => E_DrawRange::All,
                CmdType::InsertStr(str) => {
                    if self.is_enable_syntax_highlight || self.box_insert.mode == BoxInsertMode::Insert {
                        E_DrawRange::All
                    } else if str.is_empty() {
                        E_DrawRange::After(min(self.win_mgr.curt().cur_org.y, self.win_mgr.curt().cur.y))
                    } else if self.win_mgr.curt().sel.is_selected() {
                        let sel = self.win_mgr.curt().sel.get_range();
                        E_DrawRange::After(sel.sy)
                    } else {
                        E_DrawRange::TargetRange(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur.y)
                    }
                }
                CmdType::MouseDownLeft(y, x) if self.is_input_imple_mode(true) => {
                    if !self.input_comple.menulist.is_mouse_within_area(*y, *x) {
                        E_DrawRange::All
                    } else {
                        self.clear_input_comple();
                        E_DrawRange::All
                    }
                }

                CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) if self.win_mgr.curt().scrl_h.is_enable => {
                    if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftLeft(_, _)) && self.win_mgr.curt().scrl_h.clm_posi_org == 0 || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftRight(_, _)) && self.win_mgr.curt().scrl_h.clm_posi_org + self.win_mgr.curt().scrl_h.bar_len == self.get_curt_col_len() {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::WinOnlyAll
                    }
                }
                CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) => E_DrawRange::TargetRange(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur.y),
                CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) if self.win_mgr.curt().scrl_v.is_enable => E_DrawRange::All,
                CmdType::CursorDown | CmdType::CursorDownSelect | CmdType::MouseDragLeftDown(_, _) => {
                    if self.win_mgr.curt().cur_org.y == self.buf.len_rows() - 1 {
                        E_DrawRange::Not
                    } else if matches!(self.cmd.cmd_type, CmdType::CursorDown) && self.win_mgr.curt().sel.mode == SelMode::Normal {
                        E_DrawRange::MoveCur
                    } else if self.win_mgr.curt().cur.y > 0 {
                        E_DrawRange::TargetRange(self.win_mgr.curt().cur.y - 1, self.win_mgr.curt().cur.y)
                    } else {
                        // self.cur.y == 0
                        E_DrawRange::TargetRange(0, 0)
                    }
                }
                CmdType::CursorUp | CmdType::CursorUpSelect | CmdType::MouseDragLeftUp(_, _) => {
                    if self.win_mgr.curt().cur_org.y == 0 {
                        E_DrawRange::Not
                    } else if matches!(self.cmd.cmd_type, CmdType::CursorUp) && self.win_mgr.curt().sel.mode == SelMode::Normal {
                        E_DrawRange::MoveCur
                    } else {
                        E_DrawRange::TargetRange(self.win_mgr.curt().cur.y, min(self.win_mgr.curt().cur.y + 1, self.buf.len_rows() - 1))
                    }
                }
                CmdType::MouseScrollDown | CmdType::MouseScrollUp => {
                    if self.win_mgr.curt().offset.y == 0 || self.win_mgr.curt().offset.y + self.get_curt_row_len() >= self.buf.len_rows() - 1 {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::All
                    }
                }
                CmdType::AllSelect | CmdType::Undo | CmdType::Redo | CmdType::CursorFileHome | CmdType::CursorFileEnd | CmdType::ReplaceExec(_, _, _) | CmdType::BoxSelectMode | CmdType::CancelEditorState | CmdType::InputComple | CmdType::WindowSplit(_) => E_DrawRange::All,
                CmdType::FindNext | CmdType::FindBack => {
                    if self.search.str != self.search_org.str {
                        for s in &self.search_org.ranges {
                            if self.is_y_in_screen(s.y) {
                                self.change_info.restayle_row_set.insert(s.y);
                            }
                        }
                        // self.search_org.clear();
                        for s in &self.search.ranges {
                            if self.is_y_in_screen(s.y) {
                                self.change_info.restayle_row_set.insert(s.y);
                            }
                        }
                        E_DrawRange::Targetpoint
                    } else {
                        E_DrawRange::MoveCur
                    }
                }
                _ => E_DrawRange::Not,
            }
        };

        Log::debug("self.draw_range After setting", &self.draw_range);
    }

    pub fn get_draw_parts(&mut self) -> DParts {
        Log::debug_s("editor.set_draw_parts");
        self.set_draw_range();
        // set change_info
        if self.change_info.change_type != EditerChangeType::Edit {
            if let E_DrawRange::TargetRange(sy, ey) = self.draw_range {
                self.change_info.restayle_row_set.append(&mut (sy..=ey).collect::<BTreeSet<usize>>());
            }
            if let E_DrawRange::All = self.draw_range {
                self.change_info.restayle_row_set = (0..self.buf.len_rows()).collect::<BTreeSet<usize>>();
            }
        }

        let parts = match self.cmd.cmd_type {
          //  KeyCmd::Unsupported => DParts::MsgBar(Lang::get().unsupported_operation.to_string()),
          CmdType::OpenMenuFile | CmdType::OpenMenuConvert | CmdType::OpenMenuEdit | CmdType::OpenMenuSearch | CmdType::OpenMenuMacro => DParts::All,
          CmdType::CloseFile |   CmdType::CreateNewFile 
                // | E_Cmd::SaveFile 
                | CmdType::Resize(_,_) | CmdType::MouseModeSwitch | CmdType::Help | CmdType::Null => DParts::All,
                 _ => {
                    if self.state.is_change_changed() {
                        DParts::All
                    } else {
                        match self.draw_range {
                            E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => DParts::ScrollUpDown(ScrollUpDownType::Normal),
                            _ =>
                                DParts::Editor(self.draw_range),
                            
                        }
                    }
                }
            
         };
        Log::debug("editor.set_draw_parts after", &parts);
        return parts;
    }
}
