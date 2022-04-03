use crate::{
    ewin_com::{
        _cfg::{key::keycmd::*, lang::lang_cfg::*},
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
use ewin_com::{_cfg::model::default::Cfg, file::File, global::ENV};
use ropey::RopeBuilder;
use std::{
    cmp::min,
    collections::BTreeSet,
    io::{self, stdout},
};

impl Editor {
    pub const MOVE_ROW_EXTRA_NUM: usize = 3;
    pub const RNW_MARGIN: usize = 1;

    pub fn set_cur_default(&mut self) {
        self.rnw = self.get_rnw();
        self.cur = Cur { y: 0, x: 0, disp_x: 0 };
    }

    pub fn set_cur_target_by_x(&mut self, y: usize, x: usize, is_ctrlchar_incl: bool) {
        self.cur.y = y;

        let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_range(y, ..x), 0, is_ctrlchar_incl);
        self.rnw = self.get_rnw();
        self.cur.disp_x = width;
        self.cur.x = cur_x;
    }
    pub fn set_cur_target_by_disp_x(&mut self, y: usize, x: usize) {
        self.cur.y = y;

        let (cur_x, width) = get_until_disp_x(&self.buf.char_vec_row(y), x + self.offset_disp_x, false);
        self.rnw = self.get_rnw();
        self.cur.x = cur_x;
        self.cur.disp_x = width;
    }

    pub fn get_rnw(&self) -> usize {
        return if self.state.mouse_mode == MouseMode::Normal { self.buf.len_rows().to_string().len() } else { 0 };
    }

    pub fn get_rnw_and_margin(&self) -> usize {
        self.get_rnw() + Editor::RNW_MARGIN
    }

    pub fn set_org_state(&mut self) {
        Log::debug_key("set_org_state");
        // let tab = term.tabs.get_mut(term.idx).unwrap();

        self.row_disp_len_org = self.row_disp_len;
        self.col_len_org = self.col_len;
        self.cur_org = self.cur;
        self.offset_y_org = self.offset_y;
        self.offset_x_org = self.offset_x;
        self.offset_disp_x_org = self.offset_disp_x;
        self.rnw_org = self.get_rnw();
        self.sel_org = self.sel;

        self.search_org = self.search.clone();

        self.state.is_changed_org = self.state.is_changed;
        self.len_rows_org = self.buf.len_rows();
        self.scrl_v.row_posi_org = self.scrl_v.row_posi;
        self.scrl_h.row_max_width_org = self.scrl_h.row_max_width;
        self.scrl_h.clm_posi_org = self.scrl_h.clm_posi;
        self.scrl_h.is_show_org = self.scrl_h.is_show;

        self.state.input_comple_mode_org = self.state.input_comple_mode;
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

        self.change_info = ChangeInfo::default();
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
            E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseUpLeft(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) 
           // | E_Cmd::MouseDownRight(_, _) | E_Cmd::MouseDragRight(_, _)
           |E_Cmd::MouseScrollDown   |E_Cmd::MouseScrollUp      |E_Cmd::MouseMove(_, _) | E_Cmd::MouseDownLeftBox(_, _) | E_Cmd::MouseDragLeftBox(_, _) |
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
            let cfg_search = Cfg::get_edit_search();

            // self.search.set_range(self.get_search_ranges(&cfg_search, search_str, 0, len_chars, 0));

            self.search.ranges = self.get_search_ranges(&cfg_search, search_str, 0, len_chars, 0);
        }
    }

    pub fn editor_overall_check_err(&mut self) -> ActType {
        Log::debug_key("editor_check_err");
        // read_only
        if self.state.is_read_only && Editor::is_edit(&self.e_cmd, true) {
            return ActType::Cancel;
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
            self.set_cur_target_by_x(self.buf.len_rows() - 1, 0, false);
            self.scroll();
        } else if self.cur.x > self.buf.char_vec_row(self.cur.y).len() {
            self.set_cur_target_by_x(self.cur.y, self.buf.char_vec_row(self.cur.y).len(), false);
            self.scroll_horizontal();
        } else {
            self.set_cur_target_by_x(self.cur.y, self.cur.x, false);
        };
    }
    pub fn is_move_position_by_scrolling_enable_and_e_cmd(&self) -> bool {
        return !self.is_move_cur_posi_scrolling_enable() && ((matches!(self.e_cmd, E_Cmd::MouseScrollDown) || matches!(self.e_cmd, E_Cmd::MouseScrollUp)) || (self.scrl_v.is_enable && (matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _)))));
    }

    pub fn is_move_cur_posi_scrolling_enable(&self) -> bool {
        return Cfg::get().general.editor.cursor.move_position_by_scrolling_enable;
    }
    pub fn is_cur_y_in_screen(&self) -> bool {
        return self.is_y_in_screen(self.cur.y);
    }
    pub fn is_y_in_screen(&self, y: usize) -> bool {
        return self.offset_y <= y && y < self.offset_y + self.row_disp_len;
    }
    pub fn is_cur_disp_x_in_screen(&self) -> bool {
        return self.offset_disp_x <= self.cur.disp_x && self.cur.disp_x < self.offset_disp_x + self.col_len;
    }

    pub fn get_row_in_screen(&self) -> BTreeSet<usize> {
        return (self.offset_y..min(self.offset_y + self.row_disp_len, self.buf.len_rows())).collect::<BTreeSet<usize>>();
    }
    pub fn get_candidate_new_filenm(&mut self) -> String {
        Log::debug_key("get_candidate_new_filenm");

        let mut candidate_new_filenm = String::new();

        if Cfg::get().general.editor.save.use_string_first_line_for_file_name_of_new_file {
            let len = self.buf.char_vec_row(0).len();
            candidate_new_filenm = if len > 20 { self.buf.char_vec_row(0)[..=20].iter().collect() } else { self.buf.char_vec_row(0).iter().collect() };

            Log::debug("candidate_new_filenm", &candidate_new_filenm);

            if candidate_new_filenm == "." || candidate_new_filenm == ".." {
                candidate_new_filenm = "".to_string()
            } else if cfg!(linux) || cfg!(macos) || *ENV == Env::WSL {
                candidate_new_filenm = candidate_new_filenm.trim().replace(&['/', '\u{0000}'], "");
            } else if cfg!(windows) {
                candidate_new_filenm = candidate_new_filenm.trim().replace(&['¥', '/', ':', '*', '?', '"', '<', '>', '|'], "");
            };
            candidate_new_filenm = candidate_new_filenm.to_string();
        }

        return candidate_new_filenm;
    }
    pub fn set_encoding(&mut self, h_file: &mut HeaderFile, to_encode: Encode, nl_item_name: &str, apply_item_name: &str, bom_item_name: &str) -> io::Result<()> {
        if apply_item_name == Lang::get().file_reload {
            let (vec, bom, modified_time) = File::read_file(&h_file.filenm)?;
            h_file.bom = bom;

            let (mut decode_str, enc) = File::read_bytes(&vec, to_encode);
            if decode_str.is_empty() {
                decode_str = (*String::from_utf8_lossy(&vec)).to_string();
            }

            h_file.enc = enc;
            h_file.nl = self.buf.check_nl();
            h_file.modified_time = modified_time;

            let mut b = RopeBuilder::new();
            b.append(&decode_str);
            self.buf.text = b.finish();
        } else {
            h_file.enc = to_encode;
            h_file.bom = TextBuffer::get_select_item_bom(&to_encode, bom_item_name);
            h_file.nl_org = h_file.nl.clone();
            h_file.nl = nl_item_name.to_string();

            if h_file.nl != h_file.nl_org {
                self.change_nl(&h_file.nl_org, &h_file.nl);
            }
        }
        Log::info("File info", &h_file);

        Ok(())
    }

    pub fn change_nl(&mut self, from_nl_str: &str, to_nl_str: &str) {
        let from_nl = &NL::get_nl(from_nl_str);
        let to_nl = &NL::get_nl(to_nl_str);

        let cfg_search = Cfg::get_edit_search();
        let search_set = self.buf.search(from_nl, 0, self.buf.text.len_chars(), &cfg_search);

        self.edit_proc(E_Cmd::ReplaceExec(from_nl.clone(), to_nl.clone(), search_set));
    }
}
