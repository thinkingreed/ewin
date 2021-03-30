use crate::{global::*, log::*, model::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_EventKind, MouseEventKind as M_Kind};
use std::io::Write;

impl EvtAct {
    pub fn check_prom<T: Write>(out: &mut T, term: &mut Terminal, tab: &mut Tab) -> EvtActType {
        // Close・End
        if tab.prom.is_save_new_file || tab.state.is_search || tab.prom.is_close_confirm || tab.state.is_replace || tab.state.grep_info.is_grep || tab.state.grep_info.is_result_continue || tab.prom.is_move_line {
            match tab.editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('w') => {
                        // term.init_draw(out);
                        return EvtActType::Next;
                    }
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Esc => {
                        Log::ep("tab.state.grep_info.is_grep_result", &tab.state.grep_info.is_result_continue);
                        if tab.state.grep_info.is_result_continue {
                            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| vec[term.tabs_idx] = true).unwrap();
                            Log::ep_s("grep_result_cancel grep_result_cancel grep_result_cancel grep_result_cancel grep_result_cancel");
                        } else {
                            tab.prom.clear();
                            tab.state.clear();
                            tab.mbar.clear();
                            tab.editor.d_range.draw_type = DrawType::All;
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
        if tab.prom.is_save_new_file || tab.state.is_search || tab.state.is_replace || tab.state.grep_info.is_grep || tab.prom.is_move_line {
            match tab.editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    Left | Right | BackTab | Home | End | Char(_) => {
                        match code {
                            Right => tab.prom.shift_right(),
                            Left => tab.prom.shift_left(),
                            Home => tab.prom.shift_home(),
                            End => tab.prom.shift_end(),
                            BackTab => tab.prom.tab(false, &tab.state),
                            Char(c) => {
                                let rnw = tab.editor.get_rnw();
                                tab.prom.insert_char(c.to_ascii_uppercase(), rnw);
                                tab.prom.clear_sels();
                            }
                            _ => {}
                        }
                        tab.prom.draw_only(out, &tab.state);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => {
                        let clipboard = tab.editor.get_clipboard().unwrap_or("".to_string());
                        let is_all_redrow = tab.prom.paste(&clipboard);
                        if is_all_redrow {
                            term.draw(out, tab);
                        } else {
                            tab.prom.clear_sels();
                            tab.prom.draw_only(out, &tab.state);
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
                            Left | Right | Delete | Backspace | Home | End => tab.prom.operation(code),
                            Up => tab.prom.cursor_up(&tab.state),
                            Down => tab.prom.cursor_down(&tab.state),
                            Tab => tab.prom.tab(true, &tab.state),
                            Char(c) => {
                                let rnw = tab.editor.get_rnw();
                                tab.prom.insert_char(c, rnw);
                            }
                            _ => {}
                        }
                        tab.prom.clear_sels();
                        tab.prom.draw_only(out, &tab.state);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => tab.prom.ctrl_mouse(x, y, true, out),
                Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: _, row: _, .. }) => {}
                Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => tab.prom.ctrl_mouse(x, y, false, out),
                _ => {}
            }
        }
        // incremental search
        if tab.state.is_search {
            match tab.editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, .. }) => {}
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => EvtAct::exec_search_incremental(out, term, tab),
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) | Key(KeyEvent { code, .. }) => match code {
                    Char(_) | Delete | Backspace => EvtAct::exec_search_incremental(out, term, tab),
                    _ => {}
                },
                _ => {}
            }
        }
        // Search・replace・grep option
        if tab.state.is_search || tab.state.is_replace || tab.state.grep_info.is_grep {
            match tab.editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                    Char('c') => {
                        tab.prom.cont_1.change_opt_case_sens();
                        return EvtActType::Hold;
                    }
                    Char('r') => {
                        tab.prom.cont_1.change_opt_regex();
                        return EvtActType::Hold;
                    }
                    _ => return EvtActType::Hold,
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                    if tab.prom.cont_1.opt_row_posi == y {
                        if tab.prom.cont_1.opt_1.mouse_area.0 <= x && x <= tab.prom.cont_1.opt_1.mouse_area.1 {
                            tab.prom.cont_1.change_opt_case_sens();
                        } else if tab.prom.cont_1.opt_2.mouse_area.0 <= x && x <= tab.prom.cont_1.opt_2.mouse_area.1 {
                            tab.prom.cont_1.change_opt_regex();
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
        if tab.state.grep_info.is_result_continue == true || tab.mbar.msg_readonly.len() > 0 {
            match tab.editor.evt {
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
                        if !tab.state.grep_info.is_result_continue {
                            return EvtActType::Hold;
                        }
                    }
                    F(3) => {}
                    _ => {
                        if !tab.prom.is_close_confirm == true {
                            return EvtActType::Hold;
                        }
                    }
                },
                Mouse(M_Event { kind: M_EventKind::ScrollUp, .. }) => return EvtActType::Next,
                Mouse(M_Event { kind: M_EventKind::ScrollDown, .. }) => return EvtActType::Next,
                _ => return EvtActType::Hold,
            }
        }

        if tab.prom.is_save_new_file == true {
            return EvtAct::save_new_filenm(tab);
        } else if tab.prom.is_close_confirm == true {
            return EvtAct::close(tab);
        } else if tab.state.is_search == true {
            return EvtAct::search(tab);
        } else if tab.state.is_replace == true {
            return EvtAct::replace(tab);
        } else if tab.state.grep_info.is_grep == true {
            return EvtAct::grep(term, tab);
        } else if tab.state.grep_info.is_result_continue == true {
            return EvtAct::grep_result(&mut tab.editor);
        } else if tab.prom.is_move_line == true {
            return EvtAct::move_row(out, tab);
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
