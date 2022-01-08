use crate::{
    ewin_com::{
        _cfg::{key::keycmd::*, lang::lang_cfg::*},
        log::*,
        model::*,
    },
    model::*,
};
use std::cmp::{max, min};

impl Editor {
    pub fn set_draw_range(&mut self) {
        Log::debug_key("editor.set_draw_range");

        // judgment redraw
        Log::debug("self.draw_range Before setting", &self.draw_range);
        Log::debug("self.offset_y_org", &self.offset_y_org);
        Log::debug("self.offset_y", &self.offset_y);

        self.draw_range = if (self.sel!=self.sel_org) 
        // enable_syntax_highlight edit
        ||  (Editor::is_edit(&self.e_cmd, true) && self.is_enable_syntax_highlight)
        || self.rnw_org != self.get_rnw() ||  self.offset_x_org != self.offset_x 
             // All draw at the end of key record
             || self.state.key_macro.is_exec_end
             || self.scrl_h.is_show_org != self.scrl_h.is_show
        {
            E_DrawRange::All
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
                        let y = self.offset_y + self.row_disp_len - 1;
                        E_DrawRange::ScrollDown(y - Editor::SCROLL_UP_DOWN_MARGIN - 1, y)
                    }
                    _ => E_DrawRange::All,
                }
            }
        } else {
            match &self.e_cmd {
                E_Cmd::CursorLeft | E_Cmd::CursorRight | E_Cmd::CursorLeftSelect | E_Cmd::CursorRightSelect | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::MouseDragLeftBox(_, _) => {
                    if self.sel.mode == SelMode::BoxSelect {
                        let sel = self.sel.get_range();
                        E_DrawRange::Target(sel.sy, sel.ey)
                    } else if self.cur.y == self.cur_y_org {
                        if matches!(self.e_cmd, E_Cmd::CursorRightSelect) || matches!(self.e_cmd, E_Cmd::CursorLeftSelect) || matches!(self.e_cmd, E_Cmd::CursorRowHomeSelect) || matches!(self.e_cmd, E_Cmd::CursorRowEndSelect) {
                            E_DrawRange::Target(self.cur.y, self.cur.y)
                        } else {
                            E_DrawRange::MoveCur
                        }
                    } else {
                        E_DrawRange::Target(min(self.cur.y, self.cur_y_org), max(self.cur.y, self.cur_y_org))
                    }
                }
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Cut => {
                    if self.buf.len_rows() != self.row_len_org {
                        E_DrawRange::All
                    } else if self.e_cmd == E_Cmd::DelPrevChar && self.cur.y != self.cur_y_org || self.e_cmd == E_Cmd::DelNextChar && self.buf.len_rows() != self.row_len_org {
                        E_DrawRange::After(self.cur.y)
                    } else {
                        E_DrawRange::Target(min(self.cur.y, self.cur_y_org), max(self.cur.y, self.cur_y_org))
                    }
                }
                E_Cmd::InsertRow => E_DrawRange::After(self.cur.y - 1),
                E_Cmd::InsertStr(str) => {
                    if self.is_enable_syntax_highlight || self.box_insert.mode == BoxInsertMode::Insert {
                        E_DrawRange::All
                    } else if str.is_empty() {
                        E_DrawRange::After(self.cur_y_org)
                    } else {
                        E_DrawRange::Target(self.cur.y, self.cur.y)
                    }
                }
                E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftUp(_, _) if self.scrl_h.is_enable => {
                    if matches!(self.e_cmd, E_Cmd::MouseDragLeftLeft(_, _)) && self.scrl_h.clm_posi_org == 0 || matches!(self.e_cmd, E_Cmd::MouseDragLeftRight(_, _)) && self.scrl_h.clm_posi_org + self.scrl_h.bar_len == self.col_len {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::All
                    }
                }
                E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) => E_DrawRange::Target(self.cur.y, self.cur.y),
                E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftDown(_, _) if self.scrl_v.is_enable => E_DrawRange::All,
                E_Cmd::CursorDown | E_Cmd::CursorDownSelect | E_Cmd::MouseDragLeftDown(_, _) => {
                    if self.cur_y_org == self.buf.len_rows() - 1 {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::Target(self.cur.y - 1, self.cur.y)
                    }
                }
                E_Cmd::CursorUp | E_Cmd::CursorUpSelect | E_Cmd::MouseDragLeftUp(_, _) => {
                    if self.cur_y_org == 0 {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::Target(self.cur.y, if self.cur.y == 0 { 1 } else { self.cur.y + 1 })
                    }
                }
                E_Cmd::MouseScrollDown | E_Cmd::MouseScrollUp => {
                    if self.get_vertical_org_val() == 0 || self.get_vertical_org_val() == self.buf.len_rows() - 1 {
                        E_DrawRange::Not
                    } else {
                        E_DrawRange::All
                    }
                }
                E_Cmd::AllSelect | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::CursorFileHome | E_Cmd::CursorFileEnd | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::CancelModeAndSearchResult | E_Cmd::ReplaceExec(_, _, _, _) | E_Cmd::BoxSelectMode => E_DrawRange::All,
                _ => E_DrawRange::Not,
            }
        };
        Log::debug("self.draw_range After setting", &self.draw_range);
    }

    pub fn set_draw_parts(&mut self, keycmd: &KeyCmd) -> DParts {
        Log::debug_s("editor.set_draw_parts");
        return match keycmd {
            KeyCmd::Unsupported => DParts::MsgBar(Lang::get().unsupported_operation.to_string()),
            KeyCmd::CloseFile => DParts::All,
            KeyCmd::Edit(e_keycmd) => match e_keycmd {
                E_Cmd::ReplacePrompt | E_Cmd::Encoding | E_Cmd::OpenFile(_) | E_Cmd::Find | E_Cmd::MoveRow | E_Cmd::Grep | E_Cmd::OpenMenu | E_Cmd::OpenMenuFile | E_Cmd::OpenMenuConvert | E_Cmd::OpenMenuEdit | E_Cmd::OpenMenuSearch | E_Cmd::OpenMenuMacro => DParts::Prompt,
                E_Cmd::NewTab 
                // | E_Cmd::SaveFile 
                | E_Cmd::MouseModeSwitch | E_Cmd::Help | E_Cmd::Null => DParts::All,
                 _ => {
                    if self.state.is_change_changed() {
                        Log::debug("self.state.is_change_changed()",&self.state.is_change_changed());
                        DParts::All
                    } else {
                        match self.draw_range {
                            E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) => DParts::ScrollUpDown(ScrollUpDownType::Normal),
                            _ => DParts::Editor,
                        }
                    }
                }
            },
            _ => DParts::Editor,
        };
    }
}
