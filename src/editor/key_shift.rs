use crate::{_cfg::keys::KeyCmd, global::*, log::*, model::*, sel_range::SelMode, tab::Tab, terminal::*};
use std::io::Write;

impl Editor {
    pub fn shift_move_com(&mut self) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match self.keycmd {
            KeyCmd::CursorUpSelect => self.cur_up(),
            KeyCmd::CursorDownSelect => self.cur_down(),
            KeyCmd::CursorLeftSelect => self.cur_left(),
            KeyCmd::CursorRightSelect => self.cur_right(),
            KeyCmd::CursorRowHomeSelect => self.cur_home(),
            KeyCmd::CursorRowEndSelect => self.cur_end(),
            _ => {}
        }

        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.d_range.set_target(self.sel.mode, self.cur_y_org, self.cur.y);
        self.sel.check_overlap();
    }

    pub fn box_select_mode(&mut self) {
        self.sel.clear();
        self.sel.mode = match self.sel.mode {
            SelMode::Normal => SelMode::BoxSelect,
            SelMode::BoxSelect => SelMode::Normal,
        };
        // self.sel_range.sel_mode =
    }

    pub fn record_key(&mut self) {
        match self.keycmd {
            // Ctrl
            KeyCmd::Copy | KeyCmd::CutSelect | KeyCmd::AllSelect | KeyCmd::InsertStr(_) | KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd => self.key_record_vec.push(KeyRecord { keys: self.keys, ..KeyRecord::default() }),
            // Shift
            KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => self.key_record_vec.push(KeyRecord { keys: self.keys, ..KeyRecord::default() }),
            KeyCmd::FindBack => self.key_record_vec.push(KeyRecord { keys: self.keys, search: Search { str: self.search.str.clone(), ..Search::default() } }),
            // Raw
            KeyCmd::InsertLine | KeyCmd::DeletePrevChar | KeyCmd::DeleteNextChar | KeyCmd::CursorPageUp | KeyCmd::CursorPageDown | KeyCmd::Tab | KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => self.key_record_vec.push(KeyRecord { keys: self.keys, ..KeyRecord::default() }),
            KeyCmd::FindNext => self.key_record_vec.push(KeyRecord { keys: self.keys, search: Search { str: self.search.str.clone(), ..Search::default() } }),
            _ => {}
        }
    }
}

impl Tab {
    pub fn record_key_start(&mut self) {
        Log::debug_key("macro_record_start");
        if self.state.key_record_state.is_record {
            self.state.key_record_state.is_record = false;
            self.mbar.clear_keyrecord();
            self.editor.d_range.draw_type = DrawType::All;
        } else {
            self.state.key_record_state.is_record = true;
            self.mbar.set_keyrecord(&LANG.key_recording);
            self.editor.key_record_vec = vec![];
        }
    }
    pub fn exec_record_key<T: Write>(out: &mut T, term: &mut Terminal) {
        Log::debug("key_record_vec", &term.curt().editor.key_record_vec);

        if term.curt().editor.key_record_vec.len() > 0 {
            term.curt().state.key_record_state.is_exec = true;

            let macro_vec = term.curt().editor.key_record_vec.clone();
            for (i, mac) in macro_vec.iter().enumerate() {
                term.curt().editor.keys = mac.keys;
                if i == macro_vec.len() - 1 {
                    term.curt().state.key_record_state.is_exec_end = true;
                }
                EvtAct::match_event(term.curt().editor.keys, out, term);
            }
            term.curt().state.key_record_state.is_exec = false;
            term.curt().state.key_record_state.is_exec_end = false;
        } else {
            term.curt().mbar.set_err(&LANG.no_key_record_exec.to_string());
        }
    }
}
