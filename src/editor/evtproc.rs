use crate::{def::*, editor::rope::rope_util::*, model::*, util::*};

impl Editor {
    pub fn save_del_sel_evtproc(&mut self, evt_type: EvtType) {
        let sel = self.sel.get_range();

        let ep = EvtProc {
            evt_type,
            str: self.buf.slice(self.sel.get_range()),
            cur_s: Cur {
                y: sel.sy,
                x: sel.sx + self.rnw,
                disp_x: get_row_width(&self.buf.char_vec_range(self.cur.y, sel.sx), false).1 + self.rnw + 1,
            },
            cur_e: Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x },
            sel: self.sel,
            d_range: self.d_range,
            ..EvtProc::default()
        };
        self.history.regist(self.evt, ep);
    }

    pub fn exec_edit_proc(&mut self, evt_type: EvtType) {
        if self.check_evtproc_return(evt_type) {
            return;
        }
        let sel = self.sel.get_range();
        let mut ep = EvtProc { evt_type: evt_type, ..EvtProc::default() };
        if self.sel.is_selected() {
            match evt_type {
                EvtType::Del | EvtType::BS | EvtType::Cut => {
                    self.d_range.d_type = DrawType::All;
                    ep.cur_s = Cur { y: sel.sy, x: sel.sx + self.rnw, disp_x: sel.s_disp_x };
                    ep.cur_e = self.cur;
                    ep.str = self.buf.slice(self.sel.get_range());
                    ep.sel = self.sel;
                    self.del_sel_range();
                    self.sel.clear();
                }
                _ => {}
            }
        } else {
            self.d_range = DRnage::new(self.cur.y, self.cur.y, DrawType::Target);
            ep.cur_s = self.cur;

            match evt_type {
                EvtType::Del => self.delete(&mut ep),
                EvtType::BS => self.back_space(&mut ep),
                EvtType::Enter => self.enter(),
                EvtType::Cut => self.cut(),
                EvtType::InsertChar => {}
                EvtType::Paste => {}
                _ => {}
            }
            ep.cur_e = self.cur;
        }

        ep.d_range = self.d_range;

        self.history.regist(self.evt, ep);
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
            if self.cur.y == self.buf.len_lines() - 1 && self.cur.x == self.buf.len_line(self.cur.y) + self.rnw - 1 {
                self.d_range.d_type = DrawType::Not;
                return true;
            }
        } else if evt_type == EvtType::BS {
            // For the starting point
            if self.cur.y == 0 && self.cur.x == self.rnw {
                self.d_range.d_type = DrawType::Not;
                return true;
            }
        }
        return false;
    }
}
