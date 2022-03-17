use crate::model::*;
use ewin_com::{_cfg::key::keycmd::*, log::*, model::*, util::*};
use std::{cmp::min, collections::BTreeSet};

impl Editor {
    pub fn set_change_info_edit(&mut self, evt_proc: &EvtProc) {
        Log::debug_key("recalc_scrlbar_h_row");

        if let Some(sel_proc) = &evt_proc.sel_proc {
            Log::debug("proc.e_cmd sel_proc.sel", &sel_proc.sel);
            let sel = sel_proc.sel.get_range();

            self.del_change_tgt((sel.sy..sel.ey).collect::<BTreeSet<usize>>());
            self.mod_change_tgt(BTreeSet::from([sel_proc.cur_s.y]));
        };

        if let Some(proc) = &evt_proc.proc {
            Log::debug("proc.e_cmd proc", &proc.e_cmd);

            match &proc.e_cmd {
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar => {
                    if is_contain_row_end_str(&proc.str) {
                        let y = if proc.e_cmd == E_Cmd::DelNextChar { proc.cur_s.y + 1 } else { proc.cur_s.y };
                        self.del_change_tgt(BTreeSet::from([y]));
                    }
                    self.mod_change_tgt(BTreeSet::from([min(proc.cur_s.y, self.buf.len_rows() - 1)]));
                    Log::debug("self.input_comple.row_words_vec 222", &self.input_comple.row_words_vec);
                }
                E_Cmd::InsertRow => {
                    Log::debug("self.input_comple.row_words_vec 111", &self.input_comple.row_words_vec);

                    self.new_change_tgt(BTreeSet::from([proc.cur_e.y]));
                    self.mod_change_tgt(BTreeSet::from([proc.cur_s.y, proc.cur_e.y]));
                    Log::debug("self.input_comple.row_words_vec 222", &self.input_comple.row_words_vec);
                }
                // Not Insert box
                E_Cmd::InsertStr(_) if proc.box_sel_vec.is_empty() => {
                    let strings: Vec<&str> = proc.str.split(&NL::get_nl(&self.h_file.nl)).collect();
                    if !strings.is_empty() {
                        for i in 0..strings.len() - 1 {
                            self.new_change_tgt(BTreeSet::from([proc.cur_s.y + i]));
                        }
                    }
                    if self.is_enable_syntax_highlight {
                        self.mod_change_tgt((proc.cur_s.y..=self.buf.len_rows() - 1).collect::<BTreeSet<usize>>());
                    } else {
                        self.mod_change_tgt((proc.cur_s.y..=proc.cur_s.y + strings.len() - 1).collect::<BTreeSet<usize>>());
                    }
                }
                // Insert box
                E_Cmd::InsertStr(_) | E_Cmd::InsertBox(_) | E_Cmd::DelBox(_) => {
                    let first_sel = proc.box_sel_vec.first().unwrap().0;
                    let last_sel = proc.box_sel_vec.last().unwrap().0;
                    self.mod_change_tgt((first_sel.sy..=last_sel.sy).collect::<BTreeSet<usize>>());
                }
                E_Cmd::ReplaceExec(search_str, replace_str, idx_set) => {
                    let tgt_idx_set = self.get_idx_set(search_str, replace_str, idx_set);
                    let set = tgt_idx_set.iter().map(|x| self.buf.char_to_row(*x)).collect::<BTreeSet<usize>>();
                    self.mod_change_tgt(set);
                }
                _ => {}
            }
        };
    }

    pub fn new_change_tgt(&mut self, idxs: BTreeSet<usize>) {
        Log::debug_key("new_change_tgt");
        Log::debug("idxs", &idxs);

        self.change_info.new_row.extend(&idxs);

        Log::debug_key("1111111111111111111");

        /*
        if self.scrl_h.row_width_chars_vec.is_empty() {
            self.scrl_h.row_width_chars_vec.resize(1, (0, 0))
        }
         */
        Log::debug_key("2222222222222222222222");

        for idx in idxs {
            self.scrl_h.row_width_chars_vec.insert(idx, (0, 0));
            //  self.change_info.new_row.insert(idx);
            Log::debug_key("333333333333333333333333");
            self.input_comple.analysis_new(idx, &self.buf.char_vec_row(idx));
            Log::debug_key("44444444444444444444");
        }

        Log::debug_key("55555555555555555555");
    }
    pub fn del_change_tgt(&mut self, idxs: BTreeSet<usize>) {
        Log::debug_key("del_change_tgt");
        Log::debug("idxs", &idxs);
        self.input_comple.analysis_del(&idxs);

        self.change_info.del_row_set.extend(&idxs);
        for (i, idx) in idxs.iter().enumerate() {
            self.scrl_h.row_width_chars_vec.remove(idx - i);
        }
    }

    pub fn mod_change_tgt(&mut self, idxs: BTreeSet<usize>) {
        Log::debug_key("mod_change_tgt");
        Log::debug("idxs", &idxs);
        self.change_info.restayle_row_set.extend(&idxs);
        for idx in &idxs {
            self.input_comple.analysis_mod(*idx, &self.buf.char_vec_row(*idx));
        }
        self.recalc_scrlbar_h(idxs);
    }
}
