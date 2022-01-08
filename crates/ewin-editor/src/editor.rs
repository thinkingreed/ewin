use crate::{
    ewin_com::{
        _cfg::{key::keycmd::*, lang::lang_cfg::*},
        global::*,
        log::*,
        model::*,
        util::*,
    },
    model::*,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::{io::stdout, usize};

impl Editor {
    pub const MOVE_ROW_EXTRA_NUM: usize = 3;
    pub const RNW_MARGIN: usize = 1;

    pub fn set_cur_default(&mut self) {
        self.rnw = self.get_rnw();
        self.cur = Cur { y: 0, x: 0, disp_x: 0 };
    }

    pub fn set_cur_target(&mut self, y: usize, x: usize, is_ctrlchar_incl: bool) {
        self.cur.y = y;

        let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_range(y, ..x), 0, is_ctrlchar_incl);
        self.rnw = self.get_rnw();
        self.cur.disp_x = width;
        self.cur.x = cur_x;
    }

    pub fn get_rnw(&self) -> usize {
        return if self.state.mouse_mode == MouseMode::Normal { self.buf.len_rows().to_string().len() } else { 0 };
    }

    pub fn get_rnw_and_margin(&self) -> usize {
        self.get_rnw() + Editor::RNW_MARGIN
    }

    pub fn set_org_state(&mut self) {
        // let tab = term.tabs.get_mut(term.idx).unwrap();

        self.col_len_org = self.col_len;
        self.cur_y_org = self.cur.y;
        self.disp_y_org = self.disp_y;
        self.offset_y_org = self.offset_y;
        self.offset_x_org = self.offset_x;
        self.offset_disp_x_org = self.offset_disp_x;
        self.rnw_org = self.get_rnw();
        self.sel_org = self.sel;
        self.state.is_changed_org = self.state.is_changed;
        self.row_len_org = self.buf.len_rows();
        self.scrl_v.row_posi_org = self.scrl_v.row_posi;
        self.scrl_h.row_max_width_org = self.scrl_h.row_max_width;
        self.scrl_h.clm_posi_org = self.scrl_h.clm_posi;
        self.scrl_h.is_show_org = self.scrl_h.is_show;
    }

    pub fn set_cmd(&mut self, keycmd: KeyCmd) {
        self.e_cmd = match keycmd {
            KeyCmd::Edit(e_cmd) => e_cmd,
            _ => E_Cmd::Null,
        };
    }

    pub fn record_key(&mut self) {
        match &self.e_cmd {
            // Ctrl
            E_Cmd::Copy | E_Cmd::Cut | E_Cmd::AllSelect | E_Cmd::InsertStr(_) | E_Cmd::CursorFileHome | E_Cmd::CursorFileEnd|
            // Shift
            E_Cmd::CursorUpSelect | E_Cmd::CursorDownSelect | E_Cmd::CursorLeftSelect | E_Cmd::CursorRightSelect | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect |
            // Raw
            E_Cmd::InsertRow | E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::CursorPageUp | E_Cmd::CursorPageDown | E_Cmd::CursorUp | E_Cmd::CursorDown | E_Cmd::CursorLeft | E_Cmd::CursorRight | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd => self.key_vec.push(KeyMacro { e_cmd:self.e_cmd.clone(), ..KeyMacro::default() }),
            E_Cmd::FindNext | E_Cmd::FindBack => self.key_vec.push(KeyMacro { e_cmd: self.e_cmd.clone(), search: Search { str: self.search.str.clone(), ..Search::default() } }),
            _ => {}
        }
    }
    pub fn box_select_mode(&mut self) {
        Log::debug_key("box_select_mode");
        self.sel.clear();
        self.sel.mode = match self.sel.mode {
            SelMode::Normal => SelMode::BoxSelect,
            SelMode::BoxSelect => SelMode::Normal,
        };
        if self.sel.mode == SelMode::BoxSelect {
            // Initial processing for Box Insert without moving the cursor
            self.sel.set_sel_posi(true, self.cur);
            self.sel.set_sel_posi(false, self.cur);
        }
    }
    pub fn init(&mut self) {
        Log::debug_key("EvtAct.init");
        match self.e_cmd {
            // Up, Down
            E_Cmd::CursorUp | E_Cmd::CursorDown | E_Cmd::CursorUpSelect | E_Cmd::CursorDownSelect | E_Cmd::MouseScrollUp | E_Cmd::MouseScrollDown | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftUp(_, _) => {}
            _ => self.updown_x = 0,
        }

        // Edit is_change=true, Clear redo_vec,
        if Editor::is_edit(&self.e_cmd, false) {
            self.state.is_changed = true;
            self.history.clear_redo_vec();
        }
        // Box Mode
        match self.e_cmd {
            E_Cmd::InsertStr(_) => {
                if self.sel.mode == SelMode::BoxSelect {
                    self.box_insert.mode = BoxInsertMode::Insert;
                }
            }
            E_Cmd::Undo | E_Cmd::Redo | E_Cmd::DelNextChar | E_Cmd::DelPrevChar => {}
            _ => self.box_insert.mode = BoxInsertMode::Normal,
        }
    }

    pub fn set_vertical_val(&mut self, vertical_val: usize) {
        if self.is_disp_y_enable() {
            self.disp_y = vertical_val;
        } else {
            self.cur.y = vertical_val;
        }
    }
    pub fn get_vertical_val(&self) -> usize {
        if self.is_disp_y_enable() {
            return self.disp_y;
        } else {
            return self.cur.y;
        }
    }
    pub fn get_vertical_org_val(&self) -> usize {
        if self.is_disp_y_enable() {
            return self.disp_y_org;
        } else {
            return self.cur_y_org;
        }
    }
    pub fn increment_vertical_val(&mut self) {
        if self.is_disp_y_enable() {
            self.disp_y += 1;
        } else {
            self.cur.y += 1;
        }
    }

    pub fn decrement_vertical_val(&mut self) {
        Log::debug_s("decrement_vertical_val 111111111111111111");

        if self.is_disp_y_enable() {
            Log::debug_s("222222222222222222222222");
            self.disp_y -= 1;
        } else {
            Log::debug_s("3333333333333333333");
            self.cur.y -= 1;
        }
    }

    pub fn is_disp_y_enable(&self) -> bool {
        return !CFG.get().unwrap().try_lock().unwrap().general.editor.cursor.move_position_by_scrolling_enable && ((matches!(self.e_cmd, E_Cmd::MouseScrollDown) || matches!(self.e_cmd, E_Cmd::MouseScrollUp)) || (self.scrl_v.is_enable && (matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _)))));
    }

    pub fn finalize(&mut self) {
        Log::debug_key("EvtAct.finalize");

        Log::debug("self.sel 111", &self.sel);

        // set sel draw range, Clear sel range
        match self.e_cmd {
            // Select
            E_Cmd::CursorUpSelect | E_Cmd::CursorDownSelect | E_Cmd::CursorRightSelect | E_Cmd::CursorLeftSelect | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::AllSelect |
            // OpenFile, Menu
            E_Cmd::OpenFile(_) | E_Cmd::OpenMenu | E_Cmd::OpenMenuFile | E_Cmd::OpenMenuConvert | E_Cmd::OpenMenuEdit | E_Cmd::OpenMenuSearch |
            // Search
            E_Cmd::FindNext | E_Cmd::FindBack |
            // mouse
            E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) 
           // | E_Cmd::MouseDownRight(_, _) | E_Cmd::MouseDragRight(_, _)
              |E_Cmd::MouseMove(_, _) | E_Cmd::MouseDownLeftBox(_, _) | E_Cmd::MouseDragLeftBox(_, _) |
            // other
            E_Cmd::CtxtMenu(_,_) | E_Cmd::BoxSelectMode => {}
            _ => {
                if self.sel.mode == SelMode::BoxSelect {
                    match self.e_cmd {
                         E_Cmd::CursorUp | E_Cmd::CursorDown | E_Cmd::CursorLeft | E_Cmd::CursorRight => {}
                        _ => {
                            self.sel.clear();
                            self.sel.mode = SelMode::Normal;
                        }
                    }
                } else {
                    self.sel.clear();
                    self.sel.mode = SelMode::Normal;
                }
            }
        }
        Log::debug("self.sel 222", &self.sel);

        // Re-search when searching
        if Editor::is_edit(&self.e_cmd, true) && !self.search.ranges.is_empty() {
            let len_chars = self.buf.len_chars();
            let search_str = &self.search.str.clone();
            let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
            self.search.ranges = self.get_search_ranges(search_str, 0, len_chars, 0, cfg_search);
        }
    }
    pub fn editor_check_err(&mut self) -> ActType {
        // read_only
        if self.state.is_read_only && Editor::is_edit(&self.e_cmd, true) {
            return ActType::Cancel;
        }
        match self.e_cmd {
            E_Cmd::DelNextChar if self.sel.mode != SelMode::BoxSelect && !self.sel.is_selected() && self.cur.y == self.buf.len_rows() - 1 && self.cur.x == self.buf.len_row_chars(self.cur.y) - 1 => return ActType::Cancel,
            E_Cmd::DelNextChar if !self.sel.is_selected() && self.cur.y == 0 && self.cur.x == 0 => return ActType::Cancel,
            E_Cmd::Cut | E_Cmd::Copy if !self.sel.is_selected() => return ActType::Draw(DParts::MsgBar(Lang::get().no_sel_range.to_string())),
            E_Cmd::Undo if self.history.len_undo() == 0 => return ActType::Draw(DParts::MsgBar(Lang::get().no_undo_operation.to_string())),
            E_Cmd::Redo if self.history.len_redo() == 0 => return ActType::Draw(DParts::MsgBar(Lang::get().no_redo_operation.to_string())),
            E_Cmd::ExecRecordKey if self.key_vec.is_empty() => return ActType::Draw(DParts::MsgBar(Lang::get().no_key_record_exec.to_string())),
            _ => {}
        }
        return ActType::Next;
    }

    pub fn ctrl_mouse_capture(&mut self) {
        match self.state.mouse_mode {
            MouseMode::Normal => {
                self.rnw = 0;
                self.state.mouse_mode = MouseMode::Mouse;
                execute!(stdout(), DisableMouseCapture).unwrap();
            }
            MouseMode::Mouse => {
                self.rnw = self.buf.len_rows().to_string().len();
                self.state.mouse_mode = MouseMode::Normal;
                execute!(stdout(), EnableMouseCapture).unwrap();
            }
        };
    }

    pub fn adjust_cur_posi(&mut self) {
        Log::debug_key("Editor.adjust_cur_posi");

        if self.cur.y > self.buf.len_rows() - 1 {
            self.set_cur_target(self.buf.len_rows() - 1, 0, false);
            self.scroll();
        } else if self.cur.x > self.buf.char_vec_row(self.cur.y).len() {
            self.set_cur_target(self.cur.y, self.buf.char_vec_row(self.cur.y).len(), false);
            self.scroll_horizontal();
        } else {
            self.set_cur_target(self.cur.y, self.cur.x, false);
        };
    }
}
