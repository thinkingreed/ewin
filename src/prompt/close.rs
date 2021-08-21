use crate::{_cfg::keys::*, colors::*, def::*, global::*, model::*, prompt::cont::promptcont::*, prompt::prompt::prompt::*, tab::*, terminal::*};

impl EvtAct {
    pub fn close(term: &mut Terminal) -> EvtActType {
        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                Prompt::close(term);
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(str) => {
                if str == &'y'.to_string() {
                    if Tab::save(term) {
                        return EvtAct::check_exit_close(term);
                    } else {
                        term.curt().editor.draw_type = DrawType::All;
                        return EvtActType::DrawOnly;
                    }
                } else if str == &'n'.to_string() {
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
            term.del_tab(term.idx);
            if term.state.is_all_close_confirm || term.state.close_other_than_this_tab_idx != USIZE_UNDEFINED {
                let is_exit = if term.state.is_all_close_confirm {
                    term.close_tabs(USIZE_UNDEFINED)
                } else if term.tabs.len() == 1 {
                    false
                } else {
                    term.close_tabs(term.state.close_other_than_this_tab_idx)
                };
                if is_exit {
                    return EvtActType::Exit;
                } else {
                    return EvtActType::DrawOnly;
                }
            } else {
                return EvtActType::DrawOnly;
            }
        }
    }
}

impl Prompt {
    pub fn close(term: &mut Terminal) -> bool {
        if term.tabs[term.idx].editor.is_changed == true {
            if !term.curt().state.is_nomal() {
                term.clear_curt_tab();
            }
            term.curt().prom.disp_row_num = 2;
            term.set_disp_size();
            let mut cont = PromptCont::new_not_edit_type(term.curt());
            cont.set_save_confirm();
            term.curt().prom.cont_1 = cont;
            term.curt().state.is_close_confirm = true;
            return false;
        };
        if term.tabs.len() == 1 {
            return true;
        } else {
            term.del_tab(term.idx);
            // Redraw the previous tab
            term.curt().editor.draw_type = DrawType::All;
            return false;
        }
    }
}

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.save_confirmation_to_close);
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}{}{}",
            Colors::get_default_fg(),
            &LANG.yes,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.no,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.cancel,
            Colors::get_msg_highlight_fg(),
            Keybind::get_key_str(KeyCmd::EscPrompt),
            Colors::get_default_fg(),
        );

        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
}
