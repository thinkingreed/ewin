use crate::model::*;
use crate::util::*;

impl Editor {
    pub fn save_sel_del_evtproc(&mut self, do_type: DoType) {
        let sel = self.sel.get_range();

        let ep = EvtProc {
            do_type: do_type,
            str_vec: get_sel_range_str(&mut self.buf, &mut self.sel),
            cur_s: Cur {
                y: sel.sy,
                x: sel.sx + self.rnw,
                disp_x: get_row_width(&self.buf[self.cur.y], 0, sel.sx, false).1 + self.rnw + 1,
            },
            cur_e: Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x },
            sel: self.sel,
            d_range: self.d_range,
            ..EvtProc::default()
        };

        self.undo_vec.push(ep);
    }

    pub fn save_del_char_evtproc(&mut self, do_type: DoType) {
        let mut ep = EvtProc::new(do_type, self);

        if let Some(c) = self.buf[self.cur.y].get(self.cur.x - self.rnw) {
            Log::ep("save_del_char_evtproc", c.to_string());
            ep.str_vec = vec![c.to_string()];
        }
        if !self.is_undo {
            self.undo_vec.push(ep);
        }
    }

    pub fn set_evtproc(&mut self, ep: &EvtProc, cur: Cur) {
        self.cur.y = cur.y;
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
        self.d_range = ep.d_range;
    }
}
