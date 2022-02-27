use crate::model::*;
use ewin_com::{_cfg::key::keycmd::*, log::*, model::*, util::*};
use std::{cmp::min, collections::BTreeSet};

impl Editor {
    pub fn set_change_info_edit(&mut self, evt_proc: &EvtProc) {
        Log::debug_key("recalc_scrlbar_h_row");

        self.change_info.change_type = EditerChangeType::Edit;

        Log::debug("self.change_info.restayle_row 111", &self.change_info.restayle_row);

        if let Some(sel_proc) = &evt_proc.sel_proc {
            Log::debug("proc.e_cmd sel_proc.sel", &sel_proc.sel);
            let sel = sel_proc.sel.get_range();
            self.scrl_h.row_width_chars_vec.drain(sel.sy..sel.ey);
            Log::debug("evt_proc self.scrl_h.row_width_vec 111", &self.scrl_h.row_width_chars_vec);
            self.recalc_change_tgt(BTreeSet::from([sel_proc.cur_s.y]));
            Log::debug("evt_proc self.scrl_h.row_width_vec 222", &self.scrl_h.row_width_chars_vec);

            self.change_info.del_row = (sel.sy..sel.ey).collect::<BTreeSet<usize>>();
        };

        Log::debug("self.change_info.restayle_row 222", &self.change_info.restayle_row);

        if let Some(proc) = &evt_proc.proc {
            Log::debug("proc.e_cmd proc", &proc.e_cmd);
            Log::debug("evt_proc self.scrl_h.row_width_vec 111", &self.scrl_h.row_width_chars_vec);
            match &proc.e_cmd {
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar => {
                    if is_row_end_str(&proc.str) {
                        let y = if proc.e_cmd == E_Cmd::DelNextChar { proc.cur_s.y + 1 } else { proc.cur_s.y };
                        self.scrl_h.row_width_chars_vec.remove(y);
                        self.change_info.del_row = BTreeSet::from([y]);
                    }
                    self.recalc_change_tgt(BTreeSet::from([min(proc.cur_s.y, self.buf.len_rows() - 1)]));
                }
                E_Cmd::InsertRow => {
                    self.scrl_h.row_width_chars_vec.insert(proc.cur_e.y, (0, 0));
                    self.change_info.new_row = BTreeSet::from([proc.cur_e.y]);
                    self.recalc_change_tgt(BTreeSet::from([proc.cur_s.y, proc.cur_e.y]));
                }
                // Not Insert box
                E_Cmd::InsertStr(_) if proc.box_sel_vec.is_empty() => {
                    let strings: Vec<&str> = proc.str.split(&NL::get_nl(&self.h_file.nl)).collect();
                    if !strings.is_empty() {
                        for i in 0..strings.len() - 1 {
                            self.scrl_h.row_width_chars_vec.insert(proc.cur_s.y + i, (0, 0));
                            self.change_info.new_row.insert(proc.cur_s.y + i);
                        }
                    }
                    if self.is_enable_syntax_highlight {
                        self.recalc_change_tgt((proc.cur_s.y..=self.buf.len_rows() - 1).collect::<BTreeSet<usize>>());
                    } else {
                        self.recalc_change_tgt((proc.cur_s.y..=proc.cur_s.y + strings.len() - 1).collect::<BTreeSet<usize>>());
                    }
                }
                // Insert box
                E_Cmd::InsertStr(_) | E_Cmd::InsertBox(_) | E_Cmd::DelBox(_) => {
                    /*
                    if self.scrl_h.row_width_chars_vec.len() != self.buf.len_rows() {
                        self.scrl_h.row_width_chars_vec.resize_with(self.buf.len_rows(), Default::default);
                    }
                     */
                    let first_sel = proc.box_sel_vec.first().unwrap().0;
                    let last_sel = proc.box_sel_vec.last().unwrap().0;
                    self.recalc_change_tgt((first_sel.sy..=last_sel.sy).collect::<BTreeSet<usize>>());
                }
                E_Cmd::ReplaceExec(search_str, replace_str, idx_set) => {
                    Log::debug("idx_set 111", &idx_set);

                    let tgt_idx_set = self.get_idx_set(search_str, replace_str, idx_set);

                    Log::debug("tgt_idx_set 222", &tgt_idx_set);
                    let set = tgt_idx_set.iter().map(|x| self.buf.char_to_row(*x)).collect::<BTreeSet<usize>>();
                    Log::debug("set 333", &set);

                    self.recalc_change_tgt(set);
                }
                _ => {}
            }
            Log::debug("evt_proc self.scrl_h.row_width_vec 222", &self.scrl_h.row_width_chars_vec);
        };
        Log::debug("self.change_info.restayle_row 333", &self.change_info.restayle_row);
    }

    pub fn recalc_change_tgt(&mut self, idxs: BTreeSet<usize>) {
        Log::debug("idxs", &idxs);
        self.change_info.restayle_row.extend(&idxs);
        self.recalc_scrlbar_h(idxs);
    }
}
