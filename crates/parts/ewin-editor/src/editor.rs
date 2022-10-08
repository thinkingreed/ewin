use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::{
    def::*,
    models::{env::*, event::*},
    term::*,
};

use ewin_key::{cur::*, key::keys::*, sel_range::*};
use ewin_key::{key::cmd::*, model::*};
use ewin_state::{tabs::editor::*, term::*};
use ewin_utils::{char_edit::*, files::nl::*, global::*, util::*};
use std::{cmp::min, collections::BTreeSet, io::stdout, ops::Range};

use crate::{model::*, window::window::*};

impl Editor {
    pub const RNW_MARGIN: usize = 1;

    pub fn set_cur_default(&mut self) {
        // self.rnw = self.get_rnw();
        self.set_rnw();
        self.win_mgr.curt_mut().cur = Cur { y: 0, x: 0, disp_x: 0 };
    }

    pub fn set_cur_target_by_x(&mut self, y: usize, x: usize, is_ctrlchar_incl: bool) {
        self.win_mgr.curt_mut().cur.y = y;

        let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_range(y, ..x), 0, is_ctrlchar_incl);
        // self.rnw = self.get_rnw();
        self.set_rnw();
        self.win_mgr.curt_mut().cur.disp_x = width;
        self.win_mgr.curt_mut().cur.x = cur_x;
    }
    pub fn set_cur_target_by_disp_x(&mut self, y: usize, x: usize) {
        self.win_mgr.curt_mut().cur.y = y;

        let (cur_x, width) = get_until_disp_x(&self.buf.char_vec_row(y), x + self.win_mgr.curt_mut().offset.disp_x, false);
        // self.rnw = self.get_rnw();
        self.set_rnw();
        self.win_mgr.curt_mut().cur.x = cur_x;
        self.win_mgr.curt_mut().cur.disp_x = width;
    }

    pub fn get_rnw(&self) -> usize {
        return if State::get().curt_ref_state().editor.row_no.is_enable { self.buf.len_rows().to_string().len() } else { 0 };
    }
    pub fn set_rnw(&mut self) {
        self.rnw = if State::get().curt_ref_state().editor.row_no.is_enable { self.buf.len_rows().to_string().len() } else { 0 };
    }

    pub fn get_rnw_and_margin(&self) -> usize {
        if State::get().curt_ref_state().editor.row_no.is_enable {
            return self.get_rnw() + Editor::RNW_MARGIN;
        } else {
            return 0;
        };
    }

    pub fn set_org_state(&mut self) {
        Log::debug_key("set_org_state");
        // let tab = term.tabs.get_mut(term.idx).unwrap();

        self.win_mgr.curt_mut().row_len_org = self.get_curt_row_len();
        self.win_mgr.curt_mut().cur_org = self.win_mgr.curt_mut().cur;
        self.win_mgr.curt_mut().offset.y_org = self.win_mgr.curt_mut().offset.y;
        self.win_mgr.curt_mut().offset.x_org = self.win_mgr.curt_mut().offset.x;
        self.rnw_org = self.get_rnw();
        self.win_mgr.curt_mut().sel_org = self.win_mgr.curt_mut().sel;

        self.search_org = self.search.clone();

        self.buf_len_rows_org = self.buf.len_rows();
        self.win_mgr.curt_mut().scrl_v.view.y_org = self.win_mgr.curt_mut().scrl_v.view.y;
        self.win_mgr.scrl_h_info.row_max_width_org = self.win_mgr.scrl_h_info.row_max_width;
        self.win_mgr.curt_mut().scrl_h.view_org = self.win_mgr.curt_mut().scrl_h.view.clone();
        self.win_mgr.curt_mut().scrl_h.is_show_org = self.win_mgr.curt_mut().scrl_h.is_show;
        self.win_mgr.split_type_org = self.win_mgr.split_type;

        self.input_comple.mode_org = self.input_comple.mode;
    }

    pub fn set_cmd_keys(&mut self, cmd: Cmd, keys: Keys) {
        self.cmd = cmd;
        self.keys = keys;
    }

    pub fn record_key(&mut self) {
        if State::get().curt_ref_state().editor.key_macro.is_record && self.cmd.config.is_record {
            match &self.cmd.cmd_type {
                CmdType::FindNext | CmdType::FindBack => self.key_vec.push(KeyMacro { cmd_type: self.cmd.cmd_type.clone(), search: Search { str: self.search.str.clone(), ..Search::default() } }),
                _ => self.key_vec.push(KeyMacro { cmd_type: self.cmd.cmd_type.clone(), ..KeyMacro::default() }),
            };
        }
    }
    pub fn box_select_mode(&mut self) {
        Log::debug_key("box_select_mode");
        self.win_mgr.curt_mut().sel.clear();
        self.win_mgr.curt_mut().sel.mode = match self.win_mgr.curt_mut().sel.mode {
            SelMode::Normal => SelMode::BoxSelect,
            SelMode::BoxSelect => SelMode::Normal,
        };
        if self.win_mgr.curt_mut().sel.mode == SelMode::BoxSelect {
            // Initial processing for Box Insert without moving the cursor
            let cur = self.win_mgr.curt_mut().cur;
            self.win_mgr.curt_mut().sel.set_sel_posi(true, cur);
            self.win_mgr.curt_mut().sel.set_sel_posi(false, cur);
        }
    }
    pub fn init(&mut self) {
        Log::debug_key("Editor.init");
        match self.cmd.cmd_type {
            // Up, Down
            CmdType::CursorUp | CmdType::CursorDown | CmdType::CursorUpSelect | CmdType::CursorDownSelect | CmdType::MouseScrollUp | CmdType::MouseScrollDown | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) => {}
            _ => self.win_mgr.curt_mut().updown_x = 0,
        }

        // Box Mode
        match self.cmd.cmd_type {
            CmdType::InsertStr(_) => {
                if self.win_mgr.curt_mut().sel.mode == SelMode::BoxSelect {
                    self.box_insert.mode = BoxInsertMode::Insert;
                }
            }
            CmdType::Undo | CmdType::Redo | CmdType::DelNextChar | CmdType::DelPrevChar => {}
            _ => self.box_insert.mode = BoxInsertMode::Normal,
        }
    }

    pub fn research(&mut self) {
        // Re-search when searching

        if self.cmd.config.is_edit && !self.search.ranges.is_empty() {
            //  if Editor::is_edit(&self.e_cmd, true) {
            let len_chars = self.buf.len_chars();
            let search_str = &self.search.str.clone();

            self.search.ranges = self.get_search_ranges(search_str, 0, len_chars, 0);
        }
    }

    pub fn check_read_only(&mut self) -> ActType {
        Log::debug_key("check_read_only");
        // read_only
        //  if self.state.is_read_only && !self.cmd.config.is_record {
        if State::get().curt_ref_state().editor.is_read_only && !self.cmd.config.is_record {
            return ActType::Cancel;
        }
        return ActType::Next;
    }

    pub fn ctrl_mouse_capture(&mut self) {
        Log::debug_key("ctrl_mouse_capture");

        if let Some(mut tab_state) = State::get_result() {
            match tab_state.curt_ref_state().editor.mouse {
                Mouse::Enable => {
                    tab_state.curt_mut_state().editor.mouse = Mouse::Disable;
                    execute!(stdout(), DisableMouseCapture).unwrap();
                }
                Mouse::Disable => {
                    tab_state.curt_mut_state().editor.mouse = Mouse::Enable;
                    execute!(stdout(), EnableMouseCapture).unwrap();
                }
            };
            tab_state.curt_mut_state().editor.toggle_state(TabsEditerStateType::RowNo);
        }
    }

    pub fn adjust_cur_posi(&mut self) {
        Log::debug_key("Editor.adjust_cur_posi");

        if self.win_mgr.curt_mut().cur.y > self.buf.len_rows() - 1 {
            self.set_cur_target_by_x(self.buf.len_rows() - 1, 0, false);
            self.scroll();
        } else if self.win_mgr.curt_mut().cur.x > self.buf.char_vec_row(self.win_mgr.curt_mut().cur.y).len() {
            self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.buf.char_vec_row(self.win_mgr.curt_ref().cur.y).len(), false);
            self.scroll_horizontal();
        } else {
            self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.win_mgr.curt_ref().cur.x, false);
        };
    }
    pub fn is_move_position_by_scrolling_enable_and_cmd(&self) -> bool {
        return !self.is_move_cur_posi_scrolling_enable() && ((matches!(self.cmd.cmd_type, CmdType::MouseScrollDown) || matches!(self.cmd.cmd_type, CmdType::MouseScrollUp)) || (self.win_mgr.curt_ref().scrl_v.is_enable && (matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)))));
    }

    pub fn is_move_cur_posi_scrolling_enable(&self) -> bool {
        return Cfg::get().general.editor.cursor.move_position_by_scrolling_enable;
    }
    pub fn is_cur_y_in_screen(&self) -> bool {
        return self.is_y_in_screen(self.win_mgr.curt_ref().cur.y);
    }
    pub fn is_y_in_screen(&self, y: usize) -> bool {
        return self.win_mgr.curt_ref().offset.y <= y && y < self.win_mgr.curt_ref().offset.y + self.get_curt_row_len();
    }
    pub fn is_cur_disp_x_in_screen(&self) -> bool {
        return self.win_mgr.curt_ref().offset.disp_x <= self.win_mgr.curt_ref().cur.disp_x && self.win_mgr.curt_ref().cur.disp_x < self.win_mgr.curt_ref().offset.disp_x + self.get_curt_col_len();
    }

    pub fn get_row_in_screen(&self) -> BTreeSet<usize> {
        return (self.win_mgr.curt_ref().offset.y..min(self.win_mgr.curt_ref().offset.y + self.get_curt_row_len(), self.buf.len_rows())).collect::<BTreeSet<usize>>();
    }
    pub fn get_candidate_new_filenm(&mut self) -> String {
        Log::debug_key("get_candidate_new_filenm");

        let mut new_filenm = String::new();

        if Cfg::get().general.editor.save.use_string_first_line_for_file_name_of_new_file {
            let len = self.buf.char_vec_row(0).len();
            new_filenm = if len > 20 { self.buf.char_vec_row(0)[..=20].iter().collect() } else { self.buf.char_vec_row(0).iter().collect() };

            Log::debug("candidate_new_filenm", &new_filenm);

            if new_filenm == "." || new_filenm == ".." {
                new_filenm = "".to_string()
            } else if cfg!(linux) || cfg!(macos) || *ENV == Env::WSL {
                new_filenm = new_filenm.trim().replace(&['/', '\u{0000}'], "");
            } else if cfg!(windows) {
                new_filenm = new_filenm.trim().replace(&['¥', '/', ':', '*', '?', '"', '<', '>', '|'], "");
            };
            new_filenm = new_filenm.to_string();
        }

        return new_filenm;
    }

    pub fn change_nl(&mut self, from_nl_str: &str, to_nl_str: &str) {
        let from_nl = &NL::get_nl(from_nl_str);
        let to_nl = &NL::get_nl(to_nl_str);

        let cfg_search = CfgEdit::get_search();
        let search_set = self.buf.search(from_nl, 0, self.buf.text.len_chars(), &cfg_search);
        self.edit_proc_cmd_type(CmdType::ReplaceExec(from_nl.clone(), to_nl.clone(), search_set));
    }

    pub fn get_disp_row_num() -> usize {
        return get_term_size().1 - MENUBAR_HEIGHT - FILEBAR_HEIGHT - STATUSBAR_HEIGHT;
    }

    pub fn is_disp_range_absolute(&self, range: &Range<usize>) -> bool {
        let sy = if State::get().curt_ref_state().editor.scale.is_enable { self.get_curt_row_posi() - 1 } else { self.get_curt_row_posi() };
        let ey = self.get_curt_row_posi() + self.get_curt_row_len();
        return range.end > sy && (sy <= range.start || range.end - 1 <= ey);
    }

    pub fn set_tgt_window(&mut self, keys: Keys) {
        if let Keys::MouseMove(y, x) = keys {
            let (y, x) = (y as usize, x as usize);
            let (mut set_v_idx, mut set_h_idx) = (0, 0);
            for (v_idx, vec_v) in self.win_mgr.win_list.iter().enumerate() {
                for (h_idx, win) in vec_v.iter().enumerate() {
                    if win.view_all.y <= y && y <= win.view_all.y_height() && win.view_all.x <= x && x <= win.view_all.x_width() {
                        set_v_idx = v_idx;
                        set_h_idx = h_idx;
                    }
                }
            }
            self.win_mgr.win_v_idx = set_v_idx;
            self.win_mgr.win_h_idx = set_h_idx;
        }
    }

    pub fn draw_cur(&self, str_vec: &mut Vec<String>) {
        let win = self.win_mgr.curt_ref();
        if win.offset.disp_x <= win.cur.disp_x && win.cur.disp_x <= win.offset.disp_x + self.get_curt_col_len() && win.offset.y <= win.cur.y && win.cur.y <= win.offset.y + self.get_curt_row_len() {
            str_vec.push(MoveTo((win.view.x + win.cur.disp_x - win.offset.disp_x) as u16, (win.cur.y - win.offset.y + self.get_curt_row_posi()) as u16).to_string());
        }
    }

    pub fn get_editor_row_posi(&self) -> usize {
        return self.view.y;
    }

    pub fn get_curt_ref_win(&self) -> &Window {
        return self.win_mgr.curt_ref();
    }

    pub fn get_curt_col_posi(&self) -> usize {
        return self.win_mgr.curt_ref().view.x;
    }
    pub fn get_curt_col_len(&self) -> usize {
        return self.win_mgr.curt_ref().width();
    }

    pub fn get_curt_row_posi(&self) -> usize {
        return self.win_mgr.curt_ref().view.y;
    }

    pub fn get_curt_row_len(&self) -> usize {
        return self.win_mgr.curt_ref().height();
    }

    pub fn enable_syntax_highlight(&mut self) {
        let file = State::get().curt_ref_state().file.clone();

        if CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&file.ext).cloned().is_some() && file.len < Cfg::get().colors.theme.disable_syntax_highlight_file_size as u64 * 1024000.0 as u64 && is_enable_syntax_highlight(&file.ext) {
            self.is_enable_syntax_highlight = true;
        }
    }
}
