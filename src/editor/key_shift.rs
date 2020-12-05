use crate::model::*;
use crate::util::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;

impl Editor {
    pub fn shift_right(&mut self) {
        Log::ep_s("　　　　　　　  shift_right");

        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_right();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;
        // shift_leftからのshift_right
        if e_disp_x_org == disp_x_org {
            self.sel.ey = self.cur.y;
            self.sel.ex = self.cur.x - self.rnw;
            self.sel.e_disp_x = self.cur.disp_x;
        }

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.scroll();
        self.scroll_horizontal();

        self.d_range = DRnage { sy: self.cur.y, ey: self.cur.y, d_type: DType::Target };
    }

    pub fn shift_left(&mut self) {
        Log::ep_s("　　　　　　　  shift_left");

        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_left();

        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;

        // shift_rightからのshift_left
        if e_disp_x_org != 0 && e_disp_x_org < disp_x_org {
            self.sel.e_disp_x -= 1;
        }
        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
        self.scroll();
        self.scroll_horizontal();

        self.d_range = DRnage { sy: self.cur.y, ey: self.cur.y, d_type: DType::Target };
    }

    pub fn shift_down(&mut self) {
        Log::ep_s("　　　　　　　　shift_down");

        if self.cur.y == self.buf.len() - 1 {
            self.d_range = DRnage { d_type: DType::Not, ..DRnage::default() };
            return;
        }
        let y_offset_org: usize = self.y_offset;

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_down();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;

        self.d_range = DRnage {
            // ShiftUp,Down繰り返す場合の対応でcur.y - 1,
            sy: self.cur.y - 1,
            ey: self.cur.y,
            d_type: DType::Target,
        };

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.scroll();
        self.scroll_horizontal();

        if y_offset_org != self.y_offset {
            self.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
        }
    }

    pub fn shift_up(&mut self) {
        Log::ep_s("　　　　　　　　shift_up");

        if self.cur.y == 0 {
            self.d_range = DRnage { d_type: DType::Not, ..DRnage::default() };
            return;
        }
        let y_offset_org: usize = self.y_offset;

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
            // 行頭の場合に先頭文字を含めない
            if self.cur.x == self.rnw {
                self.sel.s_disp_x = self.cur.disp_x - 1;
            }
        }
        self.cursor_up();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;

        self.d_range = DRnage {
            // ShiftUp,Down繰り返す場合の対応でcur.y + 1,
            sy: self.cur.y + 1,
            ey: self.cur.y,
            d_type: DType::Target,
        };

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.scroll();
        self.scroll_horizontal();

        if y_offset_org != self.y_offset {
            self.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
        }
    }
    pub fn shift_home(&mut self) {
        Log::ep_s("　　　　　　　　shift_home");
        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cur.x = self.rnw;
        self.cur.disp_x = self.rnw + 1;

        self.sel.ey = self.cur.y;
        self.sel.ex = 0;
        self.sel.e_disp_x = self.rnw + 1;

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.d_range = DRnage { sy: self.cur.y, ey: self.cur.y, d_type: DType::Target };
    }
    pub fn shift_end(&mut self) {
        Log::ep_s("　　　　　　　  shift_end");

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        let (_, width) = get_row_width(&self.buf[self.cur.y], self.cur.x - self.rnw, self.buf[self.cur.y].len());
        self.cur.disp_x = self.cur.disp_x + width;
        self.cur.x = self.buf[self.cur.y].len() + self.rnw;

        self.sel.ey = self.cur.y;
        self.sel.ex = self.buf[self.cur.y].len();
        self.sel.e_disp_x = self.cur.disp_x;

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.d_range = DRnage { sy: self.cur.y, ey: self.cur.y, d_type: DType::Target };
    }

    pub fn record_key_start(&mut self, term: &mut Terminal, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        Log::ep_s("　　　　　　　　macro_record_start");
        if prom.is_key_record {
            prom.is_key_record = false;
            mbar.clear_macro();
            {
                // disp_row_num変更の可能性がある為にoffset_y再計算
                term.set_disp_size(self, mbar, prom, sbar);
                self.scroll();
            }
            self.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
        } else {
            prom.is_key_record = true;
            mbar.set_keyrecord(mbar.lang.key_recording.clone());
            self.key_record_vec = vec![];
        }
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
                    evt: Key(KeyEvent {
                        code: Char(c.to_ascii_uppercase()),
                        modifiers: KeyModifiers::SHIFT,
                    }),
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

    pub fn exec_record_key<T: Write>(&mut self, out: &mut T, term: &mut Terminal, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) {
        if self.key_record_vec.len() > 0 {
            prom.is_key_record_exec = true;
            let macro_vec = self.key_record_vec.clone();
            for (i, mac) in macro_vec.iter().enumerate() {
                self.evt = mac.evt;
                if i == macro_vec.len() - 1 {
                    prom.is_key_record_exec_draw = true;
                }
                EvtAct::match_event(out, term, self, mbar, prom, sbar);
            }
            prom.is_key_record_exec = false;
            prom.is_key_record_exec_draw = false;
        }
    }
}
