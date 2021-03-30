use crate::{colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::{cmp::min, io::Write};

impl EvtAct {
    pub fn search(tab: &mut Tab) -> EvtActType {
        Log::ep_s("Process.search");

        Log::ep("editor.evt", &tab.editor.evt);

        match tab.editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) => return EvtAct::exec_search_confirm(tab),
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                F(3) => return EvtAct::exec_search_confirm(tab),
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }

    fn exec_search_confirm(tab: &mut Tab) -> EvtActType {
        Log::ep_s("exec_search_confirm");
        let search_str = tab.prom.cont_1.buf.iter().collect::<String>();
        if search_str.len() == 0 {
            tab.mbar.set_err(&LANG.not_entered_search_str);
            return EvtActType::Hold;
        }
        let search_vec = tab.editor.get_search_ranges(&search_str.clone(), 0, tab.editor.buf.len_chars(), 0);
        if search_vec.len() == 0 {
            tab.mbar.set_err(&LANG.cannot_find_char_search_for);
            return EvtActType::Hold;
        } else {
            Log::ep_s("exec_search    !!!");

            tab.editor.search.clear();
            tab.editor.search.ranges = search_vec;
            tab.editor.search.str = search_str;
            // Set index to initial value
            tab.editor.search.idx = USIZE_UNDEFINED;

            tab.prom.clear();
            tab.state.clear();
            tab.mbar.clear();
            tab.editor.d_range.draw_type = DrawType::All;
            return EvtActType::Next;
        }
    }
    pub fn exec_search_incremental<T: Write>(out: &mut T, term: &mut Terminal, tab: &mut Tab) {
        Log::ep_s("exec_search_incremental");
        tab.editor.search.str = tab.prom.cont_1.buf.iter().collect::<String>();

        let s_idx = tab.editor.buf.line_to_char(tab.editor.offset_y);
        let ey = min(tab.editor.offset_y + tab.editor.disp_row_num, tab.editor.buf.len_lines());
        let search_org = tab.editor.search.clone();

        Log::ep("s_idx", &s_idx);
        Log::ep("e_idx", &tab.editor.buf.line_to_char(ey));

        tab.editor.search.ranges = if tab.editor.search.str.len() == 0 {
            vec![]
        } else {
            tab.editor.get_search_ranges(&tab.editor.search.str, s_idx, tab.editor.buf.line_to_char(ey), 0)
        };

        if !search_org.ranges.is_empty() || !tab.editor.search.ranges.is_empty() {
            // Search in advance for drawing
            if !tab.editor.search.ranges.is_empty() {
                tab.editor.search_str(true, true);
            }

            /*
            editor.d_range.draw_type = DrawType::Target;
            let (sy_curt, ey) = editor.search.get_y_range();
            let (sy_org, ey_org) = search_org.get_y_range();
            editor.d_range.sy = min(sy_curt, sy_org);
            editor.d_range.ey = max(ey, ey_org);
            */
            tab.editor.d_range.draw_type = DrawType::After;
            tab.editor.d_range.sy = tab.editor.offset_y;
            term.draw(out, tab);
        }
    }
}

impl Prompt {
    pub fn search(term: &mut Terminal, tab: &mut Tab) {
        tab.state.is_search = true;
        tab.prom.disp_row_num = 4;
        term.set_disp_size(tab);
        let mut cont = PromptCont::new_edit(tab.prom.disp_row_posi as u16, PromptContPosi::First);
        cont.set_search();
        tab.prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_search);
        self.key_desc = format!(
            "{}{}:{}F3  {}{}:{}Shift + F4  {}{}:{}Esc{}",
            Colors::get_default_fg(),
            &LANG.search_bottom,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.search_top,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.close,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );

        self.set_opt_case_sens();
        self.set_opt_regex();

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.opt_row_posi = base_posi + 2;
        self.buf_row_posi = base_posi + 3;
    }
}
