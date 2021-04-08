use crate::{global::*, log::*, model::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_EventKind, MouseEventKind as M_Kind};
use std::io::Write;

impl EvtAct {
    pub fn check_prom<T: Write>(out: &mut T, term: &mut Terminal) -> EvtActType {
        Log::ep_s("　　　　　　　　check_prom");

        // Close・End
        if term.tabs[term.idx].prom.is_save_new_file || term.tabs[term.idx].state.is_search || term.tabs[term.idx].prom.is_close_confirm || term.tabs[term.idx].state.is_replace || term.tabs[term.idx].state.grep_info.is_grep || term.tabs[term.idx].state.grep_info.is_result || term.tabs[term.idx].prom.is_move_line {
            match term.tabs[term.idx].editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('w') => {
                        // term.init_draw(out);
                        return EvtActType::Next;
                    }
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Esc => {
                        Log::ep("term.tabs[term.tab_idx].state.grep_info.is_grep_result", &term.tabs[term.idx].state.grep_info.is_result);
                        if term.tabs[term.idx].state.grep_info.is_result {
                            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| vec[term.idx] = true).unwrap();
                        } else {
                            term.tabs[term.idx].prom.clear();
                            term.tabs[term.idx].state.clear();
                            term.tabs[term.idx].state.clear();
                            term.tabs[term.idx].mbar.clear();
                            term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                        }
                        return EvtActType::DrawOnly;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let mut evt_act_type = None;

        // contents operation
        if term.tabs[term.idx].prom.is_save_new_file || term.tabs[term.idx].state.is_search || term.tabs[term.idx].state.is_replace || term.tabs[term.idx].state.grep_info.is_grep || term.tabs[term.idx].prom.is_move_line {
            let state = &term.tabs[term.idx].state.clone();
            match term.tabs[term.idx].editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    Left | Right | BackTab | Home | End | Char(_) => {
                        match code {
                            Right => term.tabs[term.idx].prom.shift_right(),
                            Left => term.tabs[term.idx].prom.shift_left(),
                            Home => term.tabs[term.idx].prom.shift_home(),
                            End => term.tabs[term.idx].prom.shift_end(),
                            BackTab => {
                                term.tabs[term.idx].prom.tab(false, state);
                            }
                            Char(c) => {
                                let rnw = term.tabs[term.idx].editor.get_rnw();
                                term.tabs[term.idx].prom.insert_char(c.to_ascii_uppercase(), rnw);
                                term.tabs[term.idx].prom.clear_sels();
                            }
                            _ => {}
                        }
                        term.tabs[term.idx].prom.draw_only(out, state);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => {
                        let clipboard = term.tabs[term.idx].editor.get_clipboard().unwrap_or("".to_string());
                        let is_all_redrow = term.tabs[term.idx].prom.paste(&clipboard);
                        if is_all_redrow {
                            term.draw(out);
                        } else {
                            term.tabs[term.idx].prom.clear_sels();
                            term.tabs[term.idx].prom.draw_only(out, state);
                        }
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Left | Right | Char(_) | Delete | Backspace | Home | End | Up | Down | Tab => {
                        match code {
                            Left | Right | Delete | Backspace | Home | End => term.tabs[term.idx].prom.operation(code),
                            Up => term.tabs[term.idx].prom.cursor_up(state),
                            Down => term.tabs[term.idx].prom.cursor_down(state),
                            Tab => term.tabs[term.idx].prom.tab(true, state),
                            Char(c) => {
                                let rnw = term.tabs[term.idx].editor.get_rnw();
                                term.tabs[term.idx].prom.insert_char(c, rnw);
                            }
                            _ => {}
                        }
                        term.tabs[term.idx].prom.clear_sels();
                        term.tabs[term.idx].prom.draw_only(out, state);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => term.tabs[term.idx].prom.ctrl_mouse(x, y, true),
                Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: _, row: _, .. }) => {}
                Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => term.tabs[term.idx].prom.ctrl_mouse(x, y, false),
                _ => {}
            }
        }

        // incremental search
        if term.tabs[term.idx].state.is_search {
            match term.tabs[term.idx].editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, .. }) => {}
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => EvtAct::exec_search_incremental(out, term),
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) | Key(KeyEvent { code, .. }) => match code {
                    Char(_) | Delete | Backspace => EvtAct::exec_search_incremental(out, term),
                    _ => {}
                },
                _ => {}
            }
        }
        // Search・replace・grep option
        if term.tabs[term.idx].state.is_search || term.tabs[term.idx].state.is_replace || term.tabs[term.idx].state.grep_info.is_grep {
            match term.tabs[term.idx].editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                    Char('c') => {
                        term.tabs[term.idx].prom.cont_1.change_opt_case_sens();
                        return EvtActType::Hold;
                    }
                    Char('r') => {
                        term.tabs[term.idx].prom.cont_1.change_opt_regex();
                        return EvtActType::Hold;
                    }
                    _ => return EvtActType::Hold,
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                    if term.tabs[term.idx].prom.cont_1.opt_row_posi == y {
                        if term.tabs[term.idx].prom.cont_1.opt_1.mouse_area.0 <= x && x <= term.tabs[term.idx].prom.cont_1.opt_1.mouse_area.1 {
                            term.tabs[term.idx].prom.cont_1.change_opt_case_sens();
                        } else if term.tabs[term.idx].prom.cont_1.opt_2.mouse_area.0 <= x && x <= term.tabs[term.idx].prom.cont_1.opt_2.mouse_area.1 {
                            term.tabs[term.idx].prom.cont_1.change_opt_regex();
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(act_type) = evt_act_type {
            return act_type;
        }

        // unable to edit
        if term.tabs[term.idx].state.grep_info.is_result == true || term.tabs[term.idx].mbar.msg_readonly.len() > 0 {
            match term.tabs[term.idx].editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    F(4) | Right | Left | Down | Up | Home | End => return EvtActType::Next,
                    _ => return EvtActType::Hold,
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('w') | Home | End => return EvtActType::Next,
                    _ => return EvtActType::Hold,
                },
                Key(KeyEvent { code, .. }) => match code {
                    PageDown | PageUp | Home | End | Down | Up | Left | Right => return EvtActType::Next,
                    Enter => {
                        if !term.tabs[term.idx].state.grep_info.is_result {
                            return EvtActType::Hold;
                        }
                    }
                    F(3) => {}
                    _ => {
                        if !term.tabs[term.idx].prom.is_close_confirm == true {
                            return EvtActType::Hold;
                        }
                    }
                },
                Mouse(M_Event { kind: M_EventKind::ScrollUp, .. }) => return EvtActType::Next,
                Mouse(M_Event { kind: M_EventKind::ScrollDown, .. }) => return EvtActType::Next,
                _ => return EvtActType::Hold,
            }
        }

        if term.tabs[term.idx].prom.is_save_new_file == true {
            return EvtAct::save_new_filenm(term);
        } else if term.tabs[term.idx].prom.is_close_confirm == true {
            return EvtAct::close(term);
        } else if term.tabs[term.idx].state.is_search == true {
            return EvtAct::search(term);
        } else if term.tabs[term.idx].state.is_replace == true {
            return EvtAct::replace(term);
        } else if term.tabs[term.idx].state.grep_info.is_grep == true {
            return EvtAct::grep(term);
        } else if term.tabs[term.idx].state.grep_info.is_result == true {
            return EvtAct::grep_result(term);
        } else if term.tabs[term.idx].prom.is_move_line == true {
            return EvtAct::move_row(out, term);
        } else {
            Log::ep_s("EvtProcess::NextEvtProcess");
            return EvtActType::Next;
        }
    }

    pub fn check_clear_mag(tab: &mut Tab) {
        match tab.editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => tab.mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => tab.mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => tab.mbar.clear_mag(),
            },

            Mouse(M_Event { kind: M_EventKind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_EventKind::ScrollDown, .. }) => {}
            _ => tab.mbar.clear_mag(),
        }
    }

    pub fn check_grep_clear_tab_comp(tab: &mut Tab) {
        Log::ep_s("check_grep_clear_tab_comp");

        if tab.state.grep_info.is_grep {
            // Check clear tab candidate
            match tab.editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    Left | Right | Home | End => tab.prom.clear_tab_comp(),
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Char(_) | Left | Right | Home | End | Backspace | Delete => tab.prom.clear_tab_comp(),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
