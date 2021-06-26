use crate::model::*;

impl Editor {
    pub fn exec_edit_proc(&mut self, evt: EvtType, str: &str, str_replace: &str) {
        if self.check_evtproc_return(evt) {
            return;
        }
        let is_selected_org = self.sel.is_selected();
        let mut ep_org = EvtProc::default();
        // selected range delete
        if self.sel.is_selected() {
            ep_org = EvtProc { evt_type: if evt == EvtType::Del { EvtType::Del } else { EvtType::BS }, ..EvtProc::default() };
            ep_org.cur_s = Cur { y: self.sel.sy, x: self.sel.sx, disp_x: self.sel.disp_x_s };
            ep_org.cur_e = self.cur;
            ep_org.str = self.buf.slice(self.sel.get_range());
            ep_org.sel = self.sel;
            self.del_sel_range();
            self.sel.clear();
            ep_org.d_range = self.d_range;
            self.history.regist_edit(self.keycmd, &ep_org);
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        if (evt == EvtType::InsertChar || evt == EvtType::Paste || evt == EvtType::Enter || evt == EvtType::Cut || evt == EvtType::Replace) || (!is_selected_org && (evt == EvtType::Del || evt == EvtType::BS)) {
            let mut ep = EvtProc { evt_type: evt, ..EvtProc::default() };

            ep.cur_s = self.cur;
            if evt == EvtType::InsertChar || evt == EvtType::Paste || evt == EvtType::Replace {
                ep.str = str.to_string();
                if evt == EvtType::Replace {
                    ep.str_replace = str_replace.to_string();
                }
            }

            match evt {
                EvtType::Del => self.delete(&mut ep),
                EvtType::BS => self.backspace(&mut ep),
                EvtType::Enter => self.enter(),
                EvtType::Cut => self.cut(ep_org.str),
                EvtType::InsertChar => self.insert_char(str.chars().nth(0).unwrap_or(' ')),
                EvtType::Paste => self.paste(&mut ep),
                // In case of replace, only registration of Evt process
                EvtType::Replace => self.replace(&mut ep),
                _ => {}
            }
            if evt != EvtType::Cut {
                ep.cur_e = self.cur;
                ep.d_range = self.d_range;
                self.history.regist_edit(self.keycmd, &ep);
            }
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
                if self.cur.y == self.buf.len_lines() - 1 && self.cur.x == self.buf.len_line_chars(self.cur.y) - 1 {
                    self.d_range.draw_type = DrawType::Not;
                    return true;
                }
            }
        } else if evt_type == EvtType::BS {
            // For the starting point
            if !self.sel.is_selected() {
                if self.cur.y == 0 && self.cur.x == 0 {
                    self.d_range.draw_type = DrawType::Not;
                    return true;
                }
            }
        }
        return false;
    }
}
