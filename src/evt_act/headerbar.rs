use crate::{def::*, log::*, model::*, prompt::prompt::Prompt, terminal::Terminal};
use crossterm::event::{Event::*, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};

impl EvtAct {
    pub fn check_headerbar(term: &mut Terminal) -> EvtActType {
        Log::debug_key("check_headerbar");

        let evt = term.curt().editor.evt;
        match evt {
            Resize(_, _) => return EvtActType::Hold,
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);
                if y != term.hbar.disp_row_posi {
                    return EvtActType::Hold;
                }

                if term.hbar.all_filenm_space_w >= x {
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
                            return EvtActType::DrawOnly;
                        }
                    }

                    if term.hbar.all_filenm_rest_area.0 <= x && x <= term.hbar.all_filenm_rest_area.1 {
                        match term.curt().editor.evt {
                            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: _, row: _, .. }) => {
                                if term.hbar.history.count_multi_click(&evt) == 2 {
                                    term.new_tab();
                                    return EvtActType::DrawOnly;
                                }
                            }
                            _ => {}
                        }
                        return EvtActType::Hold;
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
                    Prompt::open_file(term);
                    return EvtActType::Next;
                } else if term.hbar.close_btn_area.0 <= x && x <= term.hbar.close_btn_area.1 {
                    term.close_all_tab();

                    if term.tabs.is_empty() {
                        return EvtActType::Exit;
                    }
                    return EvtActType::Next;
                }
                return EvtActType::Hold;
            }
            _ => return EvtActType::Hold,
        }
    }
}
