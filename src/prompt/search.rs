use crate::{colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::cmp::min;

impl EvtAct {
    pub fn search(term: &mut Terminal) -> EvtActType {
        match term.curt().editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('v') => {
                    EvtAct::exec_search_incremental(term);
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Char(_) => {
                    EvtAct::exec_search_incremental(term);
                    return EvtActType::DrawOnly;
                }
                F(4) => return EvtAct::exec_search_confirm(term, term.tabs[term.idx].prom.cont_1.buf.iter().collect::<String>()),
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(_) | Delete | Backspace => {
                    EvtAct::exec_search_incremental(term);

                    return EvtActType::DrawOnly;
                }
                F(3) => return EvtAct::exec_search_confirm(term, term.tabs[term.idx].prom.cont_1.buf.iter().collect::<String>()),
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }

    pub fn exec_search_confirm(term: &mut Terminal, search_str: String) -> EvtActType {
        Log::debug_s("exec_search_confirm");
        let tab = term.tabs.get_mut(term.idx).unwrap();

        if search_str.len() == 0 {
            tab.mbar.set_err(&LANG.not_entered_search_str);
            return EvtActType::DrawOnly;
        }

        let search_vec = tab.editor.get_search_ranges(&search_str.clone(), 0, tab.editor.buf.len_chars(), 0);
        if search_vec.len() == 0 {
            tab.mbar.set_err(&LANG.cannot_find_char_search_for);
            return EvtActType::DrawOnly;
        } else {
            tab.editor.search.clear();
            tab.editor.search.ranges = search_vec;
            tab.editor.search.str = search_str;

            // Set index to initial value
            tab.editor.search.idx = USIZE_UNDEFINED;

            tab.prom.clear();
            // tab.state.clear();
            tab.state.clear();
            tab.mbar.clear();
            tab.editor.d_range.draw_type = DrawType::All;
            return EvtActType::Next;
        }
    }
    pub fn exec_search_incremental(term: &mut Terminal) {
        Log::debug_s("exec_search_incremental");
        term.curt().editor.search.str = term.curt().prom.cont_1.buf.iter().collect::<String>();

        let s_idx = term.tabs[term.idx].editor.buf.line_to_char(term.tabs[term.idx].editor.offset_y);
        let ey = min(term.curt().editor.offset_y + term.curt().editor.disp_row_num, term.curt().editor.buf.len_lines());
        let search_org = term.curt().editor.search.clone();

        term.curt().editor.search.ranges = if term.curt().editor.search.str.len() == 0 { vec![] } else { term.tabs[term.idx].editor.get_search_ranges(&term.tabs[term.idx].editor.search.str, s_idx, term.tabs[term.idx].editor.buf.line_to_char(ey), 0) };

        if !search_org.ranges.is_empty() || !term.curt().editor.search.ranges.is_empty() {
            // Search in advance for drawing
            if !term.curt().editor.search.ranges.is_empty() {
                term.curt().editor.search_str(true, true);
            }
            /*
            editor.d_range.draw_type = DrawType::Target;
            let (sy_curt, ey) = editor.search.get_y_range();
            let (sy_org, ey_org) = search_org.get_y_range();
            editor.d_range.sy = min(sy_curt, sy_org);
            editor.d_range.ey = max(ey, ey_org);
            */
            term.curt().editor.d_range.draw_type = DrawType::After;
            term.curt().editor.d_range.sy = term.curt().editor.offset_y;
            //  term.draw(out);
        }
    }
}

impl Prompt {
    pub fn search(term: &mut Terminal) {
        term.curt().state.is_search = true;
        term.curt().prom.disp_row_num = 4;
        term.set_disp_size();
        let mut cont = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::First);
        cont.set_search();
        term.curt().prom.cont_1 = cont;
    }
    pub fn draw_search(&mut self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_search);
        self.key_desc = format!("{}{}:{}F3  {}{}:{}Shift + F4  {}{}:{}Esc{}", Colors::get_default_fg(), &LANG.search_bottom, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.search_top, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(), Colors::get_default_fg(),);

        self.set_opt_case_sens();
        self.set_opt_regex();

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.opt_row_posi = base_posi + 2;
        self.buf_row_posi = base_posi + 3;
    }
}
