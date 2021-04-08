use crate::{global::*, log::*, model::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;

impl Editor {
    fn shift_move_com(&mut self, do_type: EvtType) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x - self.get_rnw(), self.cur.disp_x);

        match do_type {
            EvtType::ShiftRight => self.cur_right(),
            EvtType::ShiftLeft => self.cur_left(),
            EvtType::ShiftUp => self.cur_up(),
            EvtType::ShiftDown => self.cur_down(),
            EvtType::ShiftHome => {
                self.cur.x = self.get_rnw();
                self.cur.disp_x = self.get_rnw() + 1;
            }
            EvtType::ShiftEnd => {
                self.set_cur_target(self.cur.y, self.buf.len_line_chars(self.cur.y));
                self.scroll();
                self.scroll_horizontal();
            }
            _ => {}
        }
        self.sel.set_sel_posi(false, self.cur.y, self.cur.x - self.get_rnw(), self.cur.disp_x);
        self.d_range.set_target(self.cur_y_org, self.cur.y);
        self.sel.check_overlap();
    }
    pub fn shift_right(&mut self) {
        Log::ep_s("　　　　　　　  shift_right");
        self.shift_move_com(EvtType::ShiftRight);
    }

    pub fn shift_left(&mut self) {
        Log::ep_s("　　　　　　　  shift_left");
        self.shift_move_com(EvtType::ShiftLeft);
    }

    pub fn shift_down(&mut self) {
        Log::ep_s("　　　　　　　　shift_down");
        if self.cur.y == self.buf.len_lines() - 1 {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        self.shift_move_com(EvtType::ShiftDown);
    }

    pub fn shift_up(&mut self) {
        Log::ep_s("　　　　　　　　shift_up");
        if self.cur.y == 0 {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        self.shift_move_com(EvtType::ShiftUp);
    }

    pub fn shift_home(&mut self) {
        Log::ep_s("　　　　　　　　shift_home");
        self.shift_move_com(EvtType::ShiftHome);
    }

    pub fn shift_end(&mut self) {
        Log::ep_s("　　　　　　　  shift_end");
        self.shift_move_com(EvtType::ShiftEnd);
    }

    pub fn record_key(&mut self) {
        match self.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('c') | Char('x') | Char('a') | Char('v') | Home | End => self.key_record_vec.push(KeyRecord { evt: self.evt.clone(), ..KeyRecord::default() }),
                Char('w') | Char('s') | Char('f') | Char('r') | Char('g') | Char('z') | Char('y') => {}
                _ => {}
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Right | Left | Down | Up | Home | End => self.key_record_vec.push(KeyRecord { evt: self.evt.clone(), ..KeyRecord::default() }),
                Char(c) => self.key_record_vec.push(KeyRecord {
                    evt: Key(KeyEvent { code: Char(c.to_ascii_uppercase()), modifiers: KeyModifiers::SHIFT }),
                    ..KeyRecord::default()
                }),
                F(4) => self.key_record_vec.push(KeyRecord {
                    evt: self.evt.clone(),
                    search: Search { str: self.search.str.clone(), ..Search::default() },
                }),
                F(1) => {}
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Enter | Backspace | Delete | PageDown | PageUp | Home | End | Down | Up | Left | Right => self.key_record_vec.push(KeyRecord { evt: self.evt.clone(), ..KeyRecord::default() }),
                Char(_) => self.key_record_vec.push(KeyRecord { evt: self.evt.clone(), ..KeyRecord::default() }),
                F(3) => self.key_record_vec.push(KeyRecord {
                    evt: self.evt.clone(),
                    search: Search { str: self.search.str.clone(), ..Search::default() },
                }),
                _ => {}
            },
            _ => {}
        }
    }
}

impl Tab {
    pub fn record_key_start(&mut self) {
        Log::ep_s("　　　　　　　　macro_record_start");
        if self.prom.is_key_record {
            self.prom.is_key_record = false;
            self.mbar.clear_keyrecord();
            self.editor.d_range.draw_type = DrawType::All;
        } else {
            self.prom.is_key_record = true;
            self.mbar.set_keyrecord(&LANG.key_recording);
            self.editor.key_record_vec = vec![];
        }
    }
    pub fn exec_record_key<T: Write>(out: &mut T, term: &mut Terminal) {
        // let rc = Rc::clone(&term.tabs[term.tab_idx]);
        // let mut tab = term.tabs[term.tab_idx];

        if term.tabs[term.idx].editor.key_record_vec.len() > 0 {
            term.tabs[term.idx].prom.is_key_record_exec = true;
            let macro_vec = term.tabs[term.idx].editor.key_record_vec.clone();
            for (i, mac) in macro_vec.iter().enumerate() {
                term.tabs[term.idx].editor.evt = mac.evt;
                if i == macro_vec.len() - 1 {
                    term.tabs[term.idx].prom.is_key_record_exec_draw = true;
                }
                EvtAct::match_event(term.tabs[term.idx].editor.evt, out, term);
            }
            term.tabs[term.idx].prom.is_key_record_exec = false;
            term.tabs[term.idx].prom.is_key_record_exec_draw = false;
        } else {
            term.tabs[term.idx].mbar.set_err(&LANG.no_key_record_exec.to_string());
        }
    }
}
