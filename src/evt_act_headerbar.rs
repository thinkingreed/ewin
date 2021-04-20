use crate::{def::*, log::*, model::*, terminal::Terminal};
use crossterm::event::{Event::*, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};

impl EvtAct {
    pub fn check_headerbar(event: Event, term: &mut Terminal) -> EvtActType {
        Log::ep_s("　　　　　　　　check_headerbar");
        Log::ep("event", &event.clone());

        term.curt().editor.evt = event;

        match event {
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);

                if y != term.hbar.disp_row_posi {
                    return EvtActType::Hold;
                }

                for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                    if !h_file.is_disp {
                        continue;
                    }

                    if h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                        if term.hbar.file_vec[idx].is_changed {
                            term.idx = idx;
                            term.curt().editor.evt = CLOSE;
                            return EvtActType::Next;
                        } else {
                            if term.tabs.len() == 1 {
                                term.curt().editor.evt = CLOSE;
                                return EvtActType::Next;
                            } else {
                                term.idx = if idx == term.hbar.file_vec.len() - 1 { idx - 1 } else { idx };
                                term.del_tab(idx);
                                return EvtActType::DrawOnly;
                            }
                        }
                    }
                    if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                        term.idx = idx;
                        term.curt().editor.evt = KEY_NULL;
                        return EvtActType::Next;
                    }
                }
                if term.hbar.is_left_arrow_disp {
                    if term.hbar.left_arrow_area.0 <= x && x <= term.hbar.left_arrow_area.1 {
                        term.hbar.disp_base_idx -= 1;
                    }
                }
                if term.hbar.is_right_arrow_disp {
                    if term.hbar.right_arrow_area.0 <= x && x <= term.hbar.right_arrow_area.1 {
                        term.hbar.disp_base_idx += 1;
                    }
                }
                if term.hbar.plus_btn_area.0 <= x && x <= term.hbar.plus_btn_area.1 {
                    term.new_tab();

                    return EvtActType::Next;
                } else if term.hbar.help_btn_area.0 <= x && x <= term.hbar.help_btn_area.1 {
                    term.curt().editor.evt = HELP;
                    return EvtActType::Next;
                } else if term.hbar.close_btn_area.0 <= x && x <= term.hbar.close_btn_area.1 {
                    let vec = &term.hbar.file_vec.clone();
                    let mut vec_idx = vec.len();
                    for h_file in vec.iter().rev() {
                        vec_idx -= 1;
                        if h_file.is_changed {
                            term.tabs[vec_idx].editor.evt = CLOSE;
                            term.tabs[vec_idx].state.is_close_confirm = true;
                        } else {
                            term.del_tab(vec_idx);
                        }
                    }
                    if term.tabs.is_empty() {
                        return EvtActType::Exit;
                    } else {
                        term.idx = if term.idx > term.tabs.len() - 1 { term.tabs.len() - 1 } else { term.idx };
                        term.hbar.disp_base_idx = 0;
                    }
                    return EvtActType::Next;
                }
                return EvtActType::Hold;
            }
            _ => return EvtActType::Hold,
        }
    }
}
