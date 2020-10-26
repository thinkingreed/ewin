use crate::model::*;
use crate::util::*;
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn save_sel_del_evtproc(&mut self) {
        let sel = self.sel.get_range();

        let ep = EvtProc {
            e_type: EvtType::Del,
            sy: sel.sy,
            ey: sel.ey,
            x: sel.sx,
            disp_x: get_row_width(&self.buf[self.cur.y], 0, sel.sx).1 + self.lnw + 1,
            str_vec: self.get_sel_range_str(),
        };
        self.undo_vec.push(ep);
    }

    pub fn save_del_char_evtproc(&mut self, is_front: bool) {
        let ep;

        if let Some(c) = self.buf[self.cur.y].get(self.cur.x - self.lnw) {
            // let c = self.buf[self.cur.y].get(self.cur.x - self.lnw).unwrap();
            let v = vec![c.to_string()];
            let mut disp_x = self.cur.disp_x - c.width().unwrap_or(0);

            if !is_front {
                disp_x = self.cur.disp_x + c.width().unwrap_or(0);
            }
            ep = EvtProc {
                e_type: EvtType::Del,
                sy: self.cur.y,
                ey: self.cur.y,
                x: self.cur.x - self.lnw,
                disp_x: disp_x,
                str_vec: v,
            };
        } else {
            ep = EvtProc {
                e_type: EvtType::Del,
                sy: self.cur.y,
                ey: self.cur.y,
                x: self.cur.x - self.lnw,
                disp_x: self.cur.disp_x,
                str_vec: vec![],
            };
        }
        self.undo_vec.push(ep);
    }
}
