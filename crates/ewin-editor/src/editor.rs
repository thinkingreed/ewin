use crate::{
    ewin_com::{_cfg::key::keycmd::*, files::file::*, global::*, model::*, util::*},
    model::*,
    proc::proc_config::*,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use ewin_cfg::{log::*, model::default::*};
use ewin_const::{def::*, model::*};
use ropey::RopeBuilder;
use std::{
    cmp::min,
    collections::BTreeSet,
    io::{self, stdout},
    ops::Range,
};

impl Editor {
    pub const RNW_MARGIN: usize = 1;

    pub fn set_cur_default(&mut self) {
        // self.rnw = self.get_rnw();
        self.set_rnw();
        self.cur = Cur { y: 0, x: 0, disp_x: 0 };
    }

    pub fn set_cur_target_by_x(&mut self, y: usize, x: usize, is_ctrlchar_incl: bool) {
        self.cur.y = y;

        let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_range(y, ..x), 0, is_ctrlchar_incl);
        // self.rnw = self.get_rnw();
        self.set_rnw();
        self.cur.disp_x = width;
        self.cur.x = cur_x;
    }
    pub fn set_cur_target_by_disp_x(&mut self, y: usize, x: usize) {
        self.cur.y = y;

        let (cur_x, width) = get_until_disp_x(&self.buf.char_vec_row(y), x + self.offset_disp_x, false);
        // self.rnw = self.get_rnw();
        self.set_rnw();
        self.cur.x = cur_x;
        self.cur.disp_x = width;
    }

    pub fn get_rnw(&self) -> usize {
        return if CfgEdit::get().general.editor.row_no.is_enable { self.buf.len_rows().to_string().len() } else { 0 };
    }
    pub fn set_rnw(&mut self) {
        self.rnw = if CfgEdit::get().general.editor.row_no.is_enable { self.buf.len_rows().to_string().len() } else { 0 };
    }

    pub fn get_rnw_and_margin(&self) -> usize {
        self.get_rnw() + Editor::RNW_MARGIN
    }

    pub fn set_org_state(&mut self) {
        Log::debug_key("set_org_state");
        // let tab = term.tabs.get_mut(term.idx).unwrap();

        self.row_len_org = self.row_len;
        self.col_len_org = self.col_len;
        self.cur_org = self.cur;
        self.offset_y_org = self.offset_y;
        self.offset_x_org = self.offset_x;
        self.offset_disp_x_org = self.offset_disp_x;
        self.rnw_org = self.get_rnw();
        self.sel_org = self.sel;

        self.search_org = self.search.clone();

        self.state.is_changed_org = self.state.is_changed;
        self.buf_rows_org = self.buf.len_rows();
        self.scrl_v.row_posi_org = self.scrl_v.row_posi;
        self.scrl_h.row_max_width_org = self.scrl_h.row_max_width;
        self.scrl_h.clm_posi_org = self.scrl_h.clm_posi;
        self.scrl_h.is_show_org = self.scrl_h.is_show;

        self.state.input_comple_mode_org = self.state.input_comple_mode;
    }

    pub fn set_keycmd(&mut self, keycmd: KeyCmd) {
        self.e_cmd = match keycmd {
            KeyCmd::Edit(e_cmd) => e_cmd,
            _ => E_Cmd::Null,
        };
        self.cmd_config = E_CmdConfig::new(&self.e_cmd)
    }

    pub fn record_key(&mut self) {
        if self.state.key_macro.is_record && self.cmd_config.is_record {
            match &self.e_cmd {
                E_Cmd::FindNext | E_Cmd::FindBack => self.key_vec.push(KeyMacro { e_cmd: self.e_cmd.clone(), search: Search { str: self.search.str.clone(), ..Search::default() } }),
                _ => self.key_vec.push(KeyMacro { e_cmd: self.e_cmd.clone(), ..KeyMacro::default() }),
            };
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

    pub fn research(&mut self) {
        // Re-search when searching

        if self.cmd_config.is_edit && !self.search.ranges.is_empty() {
            //  if Editor::is_edit(&self.e_cmd, true) {
            let len_chars = self.buf.len_chars();
            let search_str = &self.search.str.clone();

            self.search.ranges = self.get_search_ranges(search_str, 0, len_chars, 0);
        }
    }

    pub fn check_read_only(&mut self) -> ActType {
        Log::debug_key("check_read_only");
        // read_only
        if self.state.is_read_only && !self.cmd_config.is_edit {
            return ActType::Cancel;
        }
        return ActType::Next;
    }

    pub fn ctrl_mouse_capture(&mut self) {
        match self.state.mouse {
            Mouse::Enable => {
                self.state.mouse = Mouse::Disable;
                execute!(stdout(), DisableMouseCapture).unwrap();
            }
            Mouse::Disable => {
                self.state.mouse = Mouse::Enable;
                execute!(stdout(), EnableMouseCapture).unwrap();
            }
        };
        CfgEdit::switch_editor_row_no_enable();
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
        return self.offset_y <= y && y < self.offset_y + self.row_len;
    }
    pub fn is_cur_disp_x_in_screen(&self) -> bool {
        return self.offset_disp_x <= self.cur.disp_x && self.cur.disp_x < self.offset_disp_x + self.col_len;
    }

    pub fn get_row_in_screen(&self) -> BTreeSet<usize> {
        return (self.offset_y..min(self.offset_y + self.row_len, self.buf.len_rows())).collect::<BTreeSet<usize>>();
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
    pub fn reload_with_specify_encoding(&mut self, h_file: &mut HeaderFile, enc_name: &str) -> io::Result<bool> {
        let encode = Encode::from_name(&enc_name);

        let (vec, bom, modified_time) = File::read_file(&h_file.filenm)?;
        h_file.bom = bom;
        let (mut decode_str, enc, had_errors) = Encode::read_bytes(&vec, encode);
        if had_errors {
            decode_str = (*String::from_utf8_lossy(&vec)).to_string();
        }

        h_file.enc = enc;
        h_file.nl = self.buf.check_nl();
        h_file.mod_time = modified_time;

        let mut b = RopeBuilder::new();
        b.append(&decode_str);
        self.buf.text = b.finish();

        Log::info("File info", &h_file);

        Ok(had_errors)
    }

    pub fn change_nl(&mut self, from_nl_str: &str, to_nl_str: &str) {
        let from_nl = &NL::get_nl(from_nl_str);
        let to_nl = &NL::get_nl(to_nl_str);

        let cfg_search = CfgEdit::get_search();
        let search_set = self.buf.search(from_nl, 0, self.buf.text.len_chars(), &cfg_search);

        self.edit_proc(E_Cmd::ReplaceExec(from_nl.clone(), to_nl.clone(), search_set));
    }

    pub fn get_disp_row_num() -> usize {
        return get_term_size().1 - MENUBAR_ROW_NUM - FILEBAR_ROW_NUM - STATUSBAR_ROW_NUM;
    }

    pub fn get_disp_range_absolute(&self) -> Range<usize> {
        return Range { start: if Cfg::get().general.editor.scale.is_enable { self.row_posi - 1 } else { self.row_posi }, end: self.row_posi * self.row_len };
    }
}
