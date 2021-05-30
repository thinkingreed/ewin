use crate::{colors::*, def::CLOSE, global::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};

impl EvtAct {
    pub fn close(term: &mut Terminal) -> EvtActType {
        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    // save成否判定
                    if Tab::save(term) {
                        return EvtAct::check_exit_close(term);
                    } else {
                        term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                        return EvtActType::DrawOnly;
                    }
                } else if c == 'n' {
                    return EvtAct::check_exit_close(term);
                } else {
                    return EvtActType::Hold;
                }
            }
            _ => return EvtActType::Hold,
        }
    }
    pub fn check_exit_close(term: &mut Terminal) -> EvtActType {
        if term.tabs.len() == 1 {
            return EvtActType::Exit;
        } else {
            let tmp_idx = term.idx;
            term.idx = if term.idx == term.tabs.len() - 1 { term.idx - 1 } else { term.idx };
            term.tabs.remove(tmp_idx);
            term.hbar.file_vec.remove(tmp_idx);
            return EvtActType::Next;
        }
    }
}

impl Prompt {
    pub fn close(term: &mut Terminal) -> bool {
        if term.hbar.file_vec[term.idx].is_changed == true {
            // tab.prom.save_confirm_str(term);
            term.tabs[term.idx].prom.disp_row_num = 2;
            term.set_disp_size();
            let mut cont = PromptCont::new_not_edit_type(term.tabs[term.idx].prom.disp_row_posi as u16);
            cont.set_save_confirm();
            term.tabs[term.idx].prom.cont_1 = cont;
            term.tabs[term.idx].state.is_close_confirm = true;
            return false;
        };
        if term.tabs.len() == 1 {
            return true;
        } else {
            let tab_idx = term.idx;
            term.idx = if term.idx == term.hbar.file_vec.len() - 1 { term.idx - 1 } else { term.idx };
            term.del_tab(tab_idx);
            // Redraw the previous tab
            term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
            term.tabs[term.idx].editor.evt = CLOSE;
            return false;
        }
    }
}

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.save_confirmation_to_close);
        self.key_desc = format!("{}{}:{}Y  {}{}:{}N  {}{}:{}Esc{}", Colors::get_default_fg(), &LANG.yes, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.no, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.cancel, Colors::get_msg_highlight_fg(), Colors::get_default_fg(),);

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
}
