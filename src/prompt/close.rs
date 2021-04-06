use crate::{colors::*, global::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};

impl EvtAct {
    pub fn close(term: &mut Terminal, tab: &mut Tab) -> EvtActType {
        match tab.editor.evt {
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    // save成否判定
                    if tab.save(term) {
                        return EvtAct::check_exit_close(term);
                    } else {
                        tab.editor.d_range.draw_type = DrawType::All;
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
            let tab_idx = term.tab_idx;
            term.tab_idx = if term.tab_idx == term.tabs.len() - 1 { term.tab_idx - 1 } else { term.tab_idx };
            term.tabs.remove(tab_idx);
            term.hbar.file_vec.remove(tab_idx);
            // FILE_VEC.get().unwrap().try_lock().unwrap().remove(term.tab_idx);
            //    term.tab_idx = if term.tab_idx == 0 { term.tab_idx } else { term.tab_idx - 1 };
            return EvtActType::DrawOnly;
        }
    }
}

impl Prompt {
    pub fn close(term: &mut Terminal, tab: &mut Tab) -> bool {
        if term.hbar.file_vec[term.tab_idx].is_changed == true {
            // tab.prom.save_confirm_str(term);
            tab.prom.disp_row_num = 2;
            term.set_disp_size(tab);
            let mut cont = PromptCont::new_not_edit(tab.prom.disp_row_posi as u16);
            cont.set_save_confirm();
            tab.prom.cont_1 = cont;
            tab.prom.is_close_confirm = true;
            return false;
        };
        return true;
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
