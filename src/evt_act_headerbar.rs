use std::{cell::RefCell, rc::Rc};

use crate::{bar::headerbar::*, def::*, global::*, log::*, model::*, tab::*, terminal::Terminal};
use crossterm::event::{Event::*, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};

impl EvtAct {
    pub fn check_headerbar(event: Event, term: &mut Terminal) -> (Event, EvtActType) {
        Log::ep_s("　　　　　　　　check_headerbar");

        Log::ep("event", &event);

        let mut rtn_event = event.clone();
        match event {
            Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);

                Log::ep("x ", &x);
                Log::ep("y ", &y);
                Log::ep("term.hbar.plus_btn_area ", &term.hbar.plus_btn_area);

                if y != term.hbar.disp_row_posi {
                    return (rtn_event, EvtActType::Hold);
                }

                for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                    Log::ep("h_file.filenm_area", &h_file.filenm_area);
                    Log::ep("h_file.close_area", &h_file.close_area);

                    if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                        if term.hbar.file_vec[idx].is_changed {
                            term.tab_idx = idx;
                            return (CLOSE, EvtActType::Next);
                        } else {
                            if term.tabs.len() == 1 {
                                return (CLOSE, EvtActType::Next);
                            } else {
                                term.tab_idx = if idx == term.hbar.file_vec.len() - 1 { idx - 1 } else { idx };
                                term.tabs.remove(idx);
                                term.hbar.file_vec.remove(idx);
                                return (rtn_event, EvtActType::DrawOnly);
                            }
                        }
                    }
                    if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                        term.tab_idx = idx;
                        return (rtn_event, EvtActType::DrawOnly);
                    }
                }
                if term.hbar.plus_btn_area.0 <= x && x <= term.hbar.plus_btn_area.1 {
                    Log::ep("term.hbar.plus_btn_area", &term.hbar.plus_btn_area);
                    let mut new_tab = Tab::new();
                    new_tab.editor.set_cur_default();

                    term.tab_idx = term.tabs.len();

                    let mut h_file = HeaderFile::default();
                    h_file.filenm = LANG.new_file.clone();
                    term.hbar.file_vec.push(h_file);
                    HeaderBar::set_header_filenm(term);
                    term.tabs.push(Rc::new(RefCell::new(new_tab)));

                    return (rtn_event, EvtActType::DrawOnly);
                } else if term.hbar.close_btn_area.0 <= x && x <= term.hbar.close_btn_area.1 {
                    rtn_event = CLOSE;
                    return (rtn_event, EvtActType::Next);
                } else if term.hbar.help_btn_area.0 <= x && x <= term.hbar.help_btn_area.1 {
                    rtn_event = HELP;
                    return (rtn_event, EvtActType::Next);
                }
                return (rtn_event, EvtActType::Hold);
            }
            _ => return (rtn_event, EvtActType::Hold),
        }
    }
}
