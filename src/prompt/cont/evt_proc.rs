use crate::{model::*, prompt::cont::promptcont::PromptCont};

impl PromptCont {
    pub fn exec_edit_proc(&mut self, evt_type: EvtType, str: &str) {
        if evt_type == EvtType::Del {
            if !self.sel.is_selected() && self.cur.x == self.buf.len() {
                return;
            }
        } else if evt_type == EvtType::BS {
            if !self.sel.is_selected() && self.cur.x == 0 {
                return;
            }
        }
        let is_selected_org = self.sel.is_selected();
        let mut ep_org = EvtProc::default();
        // selected range delete
        if self.sel.is_selected() {
            ep_org = EvtProc { evt_type: if evt_type == EvtType::Del { EvtType::Del } else { EvtType::BS }, ..EvtProc::default() };
            ep_org.cur_s = Cur { y: self.sel.sy, x: self.sel.sx, disp_x: self.sel.disp_x_s };
            ep_org.cur_e = self.cur;
            let sel = self.sel.get_range();
            ep_org.str = self.buf[sel.sx..sel.ex].iter().collect::<String>();
            ep_org.sel = self.sel;
            self.del_sel_range();
            self.sel.clear();
            self.history.regist_edit(self.keycmd, &ep_org);
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        if (evt_type == EvtType::InsertChar || evt_type == EvtType::Paste || evt_type == EvtType::Enter || evt_type == EvtType::Cut) || (!is_selected_org && (evt_type == EvtType::Del || evt_type == EvtType::BS)) {
            let mut ep = EvtProc { evt_type: evt_type, ..EvtProc::default() };

            ep.cur_s = self.cur;
            if evt_type == EvtType::InsertChar || evt_type == EvtType::Paste {
                ep.str = str.to_string();
            }

            match evt_type {
                EvtType::Del => self.delete(&mut ep),
                EvtType::BS => self.backspace(&mut ep),
                EvtType::Cut => self.cut(ep_org.str),
                EvtType::InsertChar => self.insert_char(str.chars().nth(0).unwrap_or(' ')),
                EvtType::Paste => self.paste(&mut ep),
                _ => {}
            }
            if evt_type != EvtType::Cut {
                ep.cur_e = self.cur;
                self.history.regist_edit(self.keycmd, &ep);
            }
        }
    }
}
