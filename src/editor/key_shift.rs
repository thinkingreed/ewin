use crate::{_cfg::keys::KeyCmd, global::*, log::*, model::*, tab::Tab, terminal::*};
use std::io::Write;

impl Editor {
    fn shift_move_com(&mut self, do_type: EvtType) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match do_type {
            EvtType::ShiftRight => self.cur_right(),
            EvtType::ShiftLeft => self.cur_left(),
            EvtType::ShiftUp => self.cur_up(),
            EvtType::ShiftDown => self.cur_down(),
            EvtType::ShiftHome => {
                self.cur.x = 0;
                self.cur.disp_x = 0;
                self.scroll_horizontal();
            }
            EvtType::ShiftEnd => {
                self.set_cur_target(self.cur.y, self.buf.char_vec_line(self.cur.y).len(), false);

                self.scroll();
                self.scroll_horizontal();
            }
            _ => {}
        }
        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.d_range.set_target(self.cur_y_org, self.cur.y);
        self.sel.check_overlap();
    }
    pub fn shift_right(&mut self) {
        Log::debug_key("shift_right");
        self.shift_move_com(EvtType::ShiftRight);
    }

    pub fn shift_left(&mut self) {
        Log::debug_key("shift_left");
        self.shift_move_com(EvtType::ShiftLeft);
    }

    pub fn shift_down(&mut self) {
        Log::debug_key("shift_down");
        if self.cur.y == self.buf.len_lines() - 1 {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        self.shift_move_com(EvtType::ShiftDown);
    }

    pub fn shift_up(&mut self) {
        Log::debug_key("shift_up");
        if self.cur.y == 0 {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        self.shift_move_com(EvtType::ShiftUp);
    }

    pub fn shift_home(&mut self) {
        Log::debug_key("s   hift_home");
        self.shift_move_com(EvtType::ShiftHome);
    }

    pub fn shift_end(&mut self) {
        Log::debug_key("shift_end");
        self.shift_move_com(EvtType::ShiftEnd);
    }

    pub fn record_key(&mut self) {
        match self.keycmd {
            // Ctrl
            KeyCmd::Copy | KeyCmd::CutSelect | KeyCmd::AllSelect | KeyCmd::Paste | KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd => self.key_record_vec.push(KeyRecord { keys: self.keys, ..KeyRecord::default() }),
            // Shift
            KeyCmd::InsertChar(_) | KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => self.key_record_vec.push(KeyRecord { keys: self.keys, ..KeyRecord::default() }),
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
