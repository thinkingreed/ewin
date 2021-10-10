use ewin_core::global::LANG;

use crate::{
    ewin_core::{_cfg::key::keycmd::*, model::*},
    model::*,
};

use std::{
    cmp::{max, min},
    usize,
};
// The Draw range setting is basically done in the initial processing and the final processing,
// but the detailed case is done in each Event processing.

impl Editor {
    pub fn set_draw_range_init(&mut self) {
        // judgment redraw
        self.draw_range = EditorDrawRange::Not;

        match self.e_cmd {
            E_Cmd::CursorUp | E_Cmd::CursorDown | E_Cmd::CursorLeft | E_Cmd::CursorRight | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd => {
                if self.sel.mode == SelMode::BoxSelect {
                    self.draw_range = EditorDrawRange::Target(min(self.sel.sy, self.sel.ey), max(self.sel.sy, self.sel.ey));

                    // When moving after overlap
                } else if self.sel.is_selected() {
                    self.draw_range = EditorDrawRange::All;
                } else if self.e_cmd == E_Cmd::CursorUp || self.e_cmd == E_Cmd::CursorDown {
                    let (y, y_after) = self.get_up_down_draw_range();
                    self.draw_range = EditorDrawRange::Target(min(y, y_after), max(y, y_after));
                } else {
                    self.draw_range = EditorDrawRange::MoveCur;
                }
            }
            E_Cmd::MouseDragLeft(y, _) => {
                if self.sel.is_selected() {
                    self.draw_range = EditorDrawRange::Target(y, y);
                }
            }
            E_Cmd::MouseDragBoxLeft(_, _) => {
                if self.sel.is_selected() {
                    self.draw_range = EditorDrawRange::All;
                }
            }
            E_Cmd::MouseScrollUp | E_Cmd::MouseScrollDown => {
                if self.sel.is_selected() {
                    let sel = self.sel.get_range();
                    self.draw_range = EditorDrawRange::Target(max(sel.sy, self.offset_y), sel.ey);
                } else {
                    let (y, y_after) = self.get_up_down_draw_range();
                    self.draw_range = EditorDrawRange::Target(min(y, y_after), max(y, y_after));
                }
            }
            _ => self.draw_range = EditorDrawRange::All,
        };
    }

    pub fn set_draw_range_finalize(&mut self, is_key_macro_exec_end: bool) {
        if self.draw_range != EditorDrawRange::All {
            if self.rnw_org != self.get_rnw() {
                self.draw_range = EditorDrawRange::All;
            } else if (self.offset_x > 0 && self.cur_y_org != self.cur.y) || self.offset_x_org != self.offset_x {
                self.draw_range = EditorDrawRange::All;
            } else if self.offset_y_org != self.offset_y {
                match self.e_cmd {
                    E_Cmd::CursorUp | E_Cmd::MouseScrollUp => self.draw_range = EditorDrawRange::ScrollUp(self.offset_y, self.offset_y + Editor::UP_DOWN_EXTRA + 1),
                    E_Cmd::CursorDown | E_Cmd::MouseScrollDown => {
                        let y = self.offset_y + self.row_num - 1;
                        self.draw_range = EditorDrawRange::ScrollDown(y - Editor::UP_DOWN_EXTRA - 1, y);
                    }
                    _ => self.draw_range = EditorDrawRange::All,
                }

                // All draw at the end of key record
            } else if is_key_macro_exec_end {
                self.draw_range = EditorDrawRange::All;
            }
        }
    }

    pub fn get_up_down_draw_range(&mut self) -> (usize, usize) {
        let y = self.cur.y;

        let y_after = match self.e_cmd {
            E_Cmd::CursorDown | E_Cmd::MouseScrollDown => min(y + 1, self.buf.len_lines() - 1),
            // UP・ScrollUp
            _ => {
                if y == 0 {
                    0
                } else {
                    y - 1
                }
            }
        };

        return (y, y_after);
    }

    pub fn set_draw_range_each_process(&mut self, draw_type: EditorDrawRange) {
        if self.is_enable_syntax_highlight {
            self.draw_range = EditorDrawRange::All;
        } else if self.sel.is_selected() {
            let sel = self.sel.get_range();
            self.draw_range = EditorDrawRange::After(sel.sy);
        } else {
            self.draw_range = draw_type;
        }
    }

    pub fn set_draw_parts(&mut self, keycmd: &KeyCmd) -> DParts {
        return match keycmd {
            KeyCmd::Unsupported => DParts::MsgBar(LANG.unsupported_operation.to_string()),
            KeyCmd::CloseFile => DParts::All,
            KeyCmd::Edit(e_keycmd) => match e_keycmd {
                E_Cmd::ReplacePrompt | E_Cmd::Encoding | E_Cmd::OpenFile(_) | E_Cmd::Find | E_Cmd::MoveRow | E_Cmd::Grep | E_Cmd::OpenMenu | E_Cmd::OpenMenuFile | E_Cmd::OpenMenuConvert | E_Cmd::OpenMenuEdit | E_Cmd::OpenMenuSearch | E_Cmd::OpenMenuMacro => DParts::Prompt,
                E_Cmd::NewTab | E_Cmd::SaveFile | E_Cmd::MouseModeSwitch | E_Cmd::Help | E_Cmd::Null => DParts::All,
                _ => {
                    if self.state.is_change_changed() {
                        DParts::All
                    } else {
                        match self.draw_range {
                            EditorDrawRange::ScrollDown(_, _) | EditorDrawRange::ScrollUp(_, _) => DParts::ScrollUpDown(ScrollUpDownType::Normal),
                            _ => DParts::Editor,
                        }
                    }
                }
            },
            _ => DParts::Editor,
        };
    }
}
