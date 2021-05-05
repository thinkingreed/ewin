use crate::{colors::*, global::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::sync::Mutex;

impl EvtAct {
    pub fn replace(term: &mut Terminal) -> EvtActType {
        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let search_str = term.tabs[term.idx].prom.cont_1.buf.iter().collect::<String>();
                    let replace_str = term.tabs[term.idx].prom.cont_2.buf.iter().collect::<String>();

                    if search_str.is_empty() {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_search_str);
                    } else if replace_str.is_empty() {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_replace_str);
                    } else {
                        let search_set = term.tabs[term.idx].editor.buf.search(&search_str.clone(), 0, term.tabs[term.idx].editor.buf.len_chars());
                        if search_set.len() == 0 {
                            term.tabs[term.idx].mbar.set_err(&LANG.cannot_find_char_search_for);
                            return EvtActType::DrawOnly;
                        }

                        let _ = REPLACE_SEARCH_RANGE.set(Mutex::new(search_set));

                        term.tabs[term.idx].editor.exec_edit_proc(EvtType::Replace, &search_str, &replace_str);
                        term.tabs[term.idx].mbar.clear();
                        term.tabs[term.idx].prom.clear();
                        term.tabs[term.idx].state.clear();
                        term.hbar.file_vec[term.idx].is_changed = true;
                    }

                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn replace(term: &mut Terminal) {
        term.tabs[term.idx].state.is_replace = true;
        term.tabs[term.idx].prom.disp_row_num = 7;
        term.set_disp_size();
        let mut cont_1 = PromptCont::new_edit_type(term.tabs[term.idx].prom.disp_row_posi as u16, PromptContPosi::First);
        let mut cont_2 = PromptCont::new_edit_type(term.tabs[term.idx].prom.disp_row_posi as u16, PromptContPosi::Second);
        cont_1.set_replace();
        cont_2.set_replace();
        term.tabs[term.idx].prom.cont_1 = cont_1;
        term.tabs[term.idx].prom.cont_2 = cont_2;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self) {
        let base_posi = self.disp_row_posi - 1;

        if self.prompt_cont_posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_replace);
            self.key_desc = format!("{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Esc", Colors::get_default_fg(), &LANG.all_replace, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.move_input_field, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(),);
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.search_str,);

            self.guide_row_posi = base_posi;
            self.key_desc_row_posi = base_posi + 1;
            self.opt_row_posi = base_posi + 2;
            self.buf_desc_row_posi = base_posi + 3;
            self.buf_row_posi = base_posi + 4;
        } else {
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.replace_char,);

            self.buf_desc_row_posi = base_posi + 5;
            self.buf_row_posi = base_posi + 6;
        }
        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
