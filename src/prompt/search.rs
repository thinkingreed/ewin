use crate::{_cfg::keys::*, colors::*, def::*, global::*, log::*, model::*, prompt::cont::promptcont::*, prompt::prompt::prompt::*, terminal::*};
use std::cmp::min;

impl EvtAct {
    pub fn search(term: &mut Terminal) -> EvtActType {
        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                Prompt::search(term);
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(_) | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Undo | KeyCmd::Redo => {
                EvtAct::exec_search_incremental(term);
                return EvtActType::DrawOnly;
            }
            KeyCmd::FindNext | KeyCmd::FindBack => return EvtAct::exec_search_confirm(term),
            _ => return EvtActType::Hold,
        };
    }

    pub fn exec_search_confirm(term: &mut Terminal) -> EvtActType {
        Log::debug_s("exec_search_confirm");
        let tab = term.tabs.get_mut(term.idx).unwrap();

        let search_str = tab.prom.cont_1.buf.iter().collect::<String>();
        if &search_str.len() == &0 {
            tab.mbar.set_err(&LANG.not_entered_search_str);
            return EvtActType::DrawOnly;
        }

        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
        let search_vec = tab.editor.search(&search_str, cfg_search);

        if search_vec.len() == 0 {
            tab.mbar.set_err(&LANG.cannot_find_char_search_for);
            return EvtActType::DrawOnly;
        } else {
            tab.editor.search.clear();
            tab.editor.search.ranges = search_vec;
            tab.editor.search.str = search_str;

            // Set index to initial value
            tab.editor.search.idx = USIZE_UNDEFINED;
            term.clear_curt_tab();
            return EvtActType::Next;
        }
    }
}

impl EvtAct {
    pub fn exec_search_incremental(term: &mut Terminal) {
        Log::debug_s("exec_search_incremental");
        term.curt().editor.search.str = term.curt().prom.cont_1.buf.iter().collect::<String>();
        let regex = CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex;

        let s_row_idx = if regex { term.tabs[term.idx].editor.buf.line_to_byte(term.tabs[term.idx].editor.offset_y) } else { term.tabs[term.idx].editor.buf.line_to_char(term.tabs[term.idx].editor.offset_y) };
        let ey = min(term.curt().editor.offset_y + term.curt().editor.disp_row_num, term.curt().editor.buf.len_lines());
        let e_row_idx = if regex { term.tabs[term.idx].editor.buf.line_to_byte(ey) } else { term.tabs[term.idx].editor.buf.line_to_char(ey) };
        let search_org = term.curt().editor.search.clone();

        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
        term.curt().editor.search.ranges = if term.curt().editor.search.str.len() == 0 { vec![] } else { term.tabs[term.idx].editor.get_search_ranges(&term.tabs[term.idx].editor.search.str, s_row_idx, e_row_idx, 0, cfg_search) };
        if !search_org.ranges.is_empty() || !term.curt().editor.search.ranges.is_empty() {
            // Search in advance for drawing
            if !term.curt().editor.search.ranges.is_empty() {
                term.curt().editor.search_str(true, true);
            }
            term.curt().editor.draw_type = DrawType::After(term.curt().editor.offset_y);
        }
    }
}

impl Prompt {
    pub fn search(term: &mut Terminal) {
        term.curt().state.is_search = true;
        term.curt().prom.disp_row_num = 4;
        term.set_disp_size();
        let mut cont = PromptCont::new_edit_type(term.curt(), PromptContPosi::First);
        cont.set_search();
        term.curt().prom.cont_1 = cont;
    }
    pub fn draw_search(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_search);
        self.key_desc = format!(
            "{}{}:{}{}  {}{}:{}{}  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &LANG.search_bottom,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::FindNext),
            Colors::get_default_fg(),
            &LANG.search_top,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::FindBack),
            Colors::get_default_fg(),
            &LANG.close,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::EscPrompt),
            Colors::get_default_fg(),
        );

        self.set_opt_case_sens();
        self.set_opt_regex();

        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.opt_row_posi = base_posi + 2;
        self.buf_row_posi = base_posi + 3;
    }
}
