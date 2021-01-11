use crate::model::*;

impl Editor {
    pub fn exec_edit_proc(&mut self, evt: EvtType, str: &str) {
        if self.check_evtproc_return(evt) {
            return;
        }
        let sel = self.sel.get_range();
        let is_selected_org = self.sel.is_selected();
        // selected range delete
        if is_selected_org {
            let mut ep = EvtProc { evt_type: EvtType::Del, ..EvtProc::default() };
            self.d_range.d_type = DrawType::All;
            ep.cur_s = Cur { y: sel.sy, x: sel.sx + self.rnw, disp_x: sel.s_disp_x };
            ep.cur_e = self.cur;
            ep.str = self.buf.slice(self.sel.get_range());
            ep.sel = self.sel;
            self.del_sel_range();
            self.sel.clear();
            ep.d_range = self.d_range;
            self.history.regist(self.evt, ep);
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        if (evt == EvtType::InsertChar || evt == EvtType::Paste || evt == EvtType::Enter) || (!is_selected_org && (evt == EvtType::Del || evt == EvtType::BS || evt == EvtType::Cut)) {
            let mut ep = EvtProc { evt_type: evt, ..EvtProc::default() };
            self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::Target);
            ep.cur_s = self.cur;
            ep.str = str.to_string();

            match evt {
                EvtType::Del => self.delete(&mut ep),
                EvtType::BS => self.back_space(&mut ep),
                EvtType::Enter => self.enter(),
                EvtType::Cut => self.cut(),
                EvtType::InsertChar => self.insert_char(str.chars().nth(0).unwrap_or(' ')),
                EvtType::Paste => self.paste(&mut ep),
                _ => {}
            }
            ep.cur_e = self.cur;
            ep.d_range = self.d_range;
            self.history.regist(self.evt, ep);
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_evtproc(&mut self, ep: &EvtProc, cur: &Cur) {
        self.cur.y = cur.y;
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
        self.d_range = ep.d_range;
    }

    pub fn check_evtproc_return(&mut self, evt_type: EvtType) -> bool {
        if evt_type == EvtType::Del {
            // End of last line
            if !self.sel.is_selected() {
                if self.cur.y == self.buf.len_lines() - 1 && self.cur.x == self.buf.len_line_chars(self.cur.y) + self.rnw - 1 {
                    self.d_range.d_type = DrawType::Not;
                    return true;
                }
            }
        } else if evt_type == EvtType::BS {
            // For the starting point
            if !self.sel.is_selected() {
                if self.cur.y == 0 && self.cur.x == self.rnw {
                    self.d_range.d_type = DrawType::Not;
                    return true;
                }
            }
        }
        return false;
    }
}
