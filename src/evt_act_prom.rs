use crate::{def::NEW_LINE, global::*, log::*, model::*, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_EventKind, MouseEventKind as M_Kind};
use std::io::Write;

impl EvtAct {
    pub fn check_prom<T: Write>(out: &mut T, term: &mut Terminal) -> EvtActType {
        Log::ep_s("　　　　　　　　check_prom");

        // Close・End
        if term.curt().state.is_save_new_file || term.curt().state.is_search || term.curt().state.is_close_confirm || term.curt().state.is_replace || term.curt().state.grep_info.is_grep || term.curt().state.grep_info.is_result || term.curt().state.is_move_line || term.curt().state.is_read_only {
            match term.curt().editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('w') => {
                        if term.curt().state.grep_info.is_result && !(term.curt().state.grep_info.is_stdout_end && term.curt().state.grep_info.is_stderr_end) && !term.curt().state.grep_info.is_cancel {
                            return EvtActType::Hold;
                        } else {
                            return EvtActType::Next;
                        }
                    }
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Esc => {
                        if term.curt().state.grep_info.is_result {
                            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = true).unwrap();
                        } else if term.curt().state.is_read_only {
                        } else {
                            term.curt().prom.clear();
                            term.curt().state.clear();
                            term.curt().mbar.clear();
                            term.curt().editor.d_range.draw_type = DrawType::All;
                        }
                        return EvtActType::DrawOnly;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // contents operation
        if term.curt().state.is_save_new_file || term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep_info.is_grep || term.curt().state.is_move_line {
            let state = &term.curt().state.clone();
            match term.curt().editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    Left | Right | BackTab | Home | End | Char(_) => {
                        match code {
                            Right => term.curt().prom.shift_right(),
                            Left => term.curt().prom.shift_left(),
                            Home => term.curt().prom.shift_home(),
                            End => term.curt().prom.shift_end(),
                            BackTab => {
                                term.curt().prom.tab(false, state);
                            }
                            Char(c) => {
                                let rnw = term.curt().editor.get_rnw();
                                term.curt().prom.insert_char(c.to_ascii_uppercase(), rnw, state);
                                term.curt().prom.clear_sels();
                            }
                            _ => {}
                        }
                        // For incremental search
                        if !term.curt().state.is_search {
                            term.curt().prom.draw_only(out, state);
                            return EvtActType::Hold;
                        }
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => {
                        let clipboard = term.curt().editor.get_clipboard().unwrap_or("".to_string());
                        let is_all_redrow = term.curt().prom.paste(&clipboard);
                        if is_all_redrow {
                            term.draw(out);
                        } else {
                            term.curt().prom.clear_sels();
                            term.curt().prom.draw_only(out, state);
                        }
                        // For incremental search
                        if !term.curt().state.is_search {
                            return EvtActType::Hold;
                        }
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Left | Right | Char(_) | Delete | Backspace | Home | End | Up | Down | Tab => {
                        match code {
                            Left | Right | Delete | Backspace | Home | End => term.curt().prom.operation(code),
                            Up => term.curt().prom.cursor_up(state),
                            Down => term.curt().prom.cursor_down(state),
                            Tab => term.curt().prom.tab(true, state),
                            Char(c) => {
                                let rnw = term.curt().editor.get_rnw();
                                term.curt().prom.insert_char(c, rnw, state);
                            }
                            _ => {}
                        }
                        // For incremental search
                        if !term.curt().state.is_search {
                            term.curt().prom.clear_sels();
                            term.curt().prom.draw_only(out, state);
                            return EvtActType::Hold;
                        }
                    }
                    _ => {}
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => term.curt().prom.ctrl_mouse(x, y, true),
                Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => term.curt().prom.ctrl_mouse(x, y, false),
                _ => {}
            }
        }

        // Search・replace・grep option
        if term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep_info.is_grep {
            match term.curt().editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                    Char('c') => {
                        term.curt().prom.cont_1.change_opt_case_sens();
                        return EvtActType::Hold;
                    }
                    Char('r') => {
                        term.curt().prom.cont_1.change_opt_regex();
                        return EvtActType::Hold;
                    }
                    _ => return EvtActType::Hold,
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                    if term.curt().prom.cont_1.opt_row_posi == y {
                        if term.curt().prom.cont_1.opt_1.mouse_area.0 <= x && x <= term.curt().prom.cont_1.opt_1.mouse_area.1 {
                            term.curt().prom.cont_1.change_opt_case_sens();
                        } else if term.curt().prom.cont_1.opt_2.mouse_area.0 <= x && x <= term.curt().prom.cont_1.opt_2.mouse_area.1 {
                            term.curt().prom.cont_1.change_opt_regex();
                        }
                    }
                }
                _ => {}
            }
        }

        Log::ep("term.curt().state.is_read_only", &term.curt().state.is_read_only);

        // unable to edit
        if term.curt().state.grep_info.is_result || term.curt().state.is_read_only {
            Log::ep_s("unable to edit");

            match term.curt().editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    F(4) | Right | Left | Down | Up | Home | End => return EvtActType::Next,
                    _ => return EvtActType::Hold,
                },
                Key(KeyEvent { code, .. }) => match code {
                    PageDown | PageUp | Home | End | Down | Up | Left | Right => return EvtActType::Next,
                    Enter => {
                        if !term.curt().state.grep_info.is_result {
                            return EvtActType::Hold;
                        }
                    }
                    F(3) => {}
                    _ => {
                        if !term.curt().state.is_close_confirm == true {
                            return EvtActType::Hold;
                        }
                    }
                },
                Mouse(M_Event { kind: M_EventKind::ScrollUp, .. }) => return EvtActType::Next,
                Mouse(M_Event { kind: M_EventKind::ScrollDown, .. }) => return EvtActType::Next,
                _ => return EvtActType::Hold,
            }
        }

        if term.curt().state.is_save_new_file == true {
            return EvtAct::save_new_filenm(term);
        } else if term.curt().state.is_close_confirm == true {
            return EvtAct::close(term);
        } else if term.curt().state.is_search == true {
            return EvtAct::search(term);
        } else if term.curt().state.is_replace == true {
            return EvtAct::replace(term);
        } else if term.curt().state.grep_info.is_grep == true {
            return EvtAct::grep(term);
        } else if term.curt().state.grep_info.is_result == true {
            return EvtAct::grep_result(term);
        } else if term.curt().state.is_move_line == true {
            return EvtAct::move_row(out, term);
        } else {
            Log::ep_s("EvtProcess::NextEvtProcess");
            return EvtActType::Next;
        }
    }

    pub fn clear_mag(tab: &mut Tab) {
        tab.mbar.msg_org = tab.mbar.msg.clone();

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

    pub fn clear_grep_tab_comp(tab: &mut Tab) {
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
    pub fn check_err_prompt(term: &mut Terminal) -> bool {
        // Check if sel range is set
        match term.curt().editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('v') => {
                    if EvtAct::check_clipboard(term) {
                        return true;
                    }
                }
                _ => return false,
            },
            _ => return false,
        }
        return false;
    }

    pub fn check_clipboard(term: &mut Terminal) -> bool {
        let clipboard = term.curt().editor.get_clipboard().unwrap_or("".to_string());

        if clipboard.len() == 0 {
            term.curt().mbar.set_err(&LANG.no_value_in_clipboard.to_string());
            return true;
        }
        // Do not paste multiple lines for Prompt
        if term.curt().state.is_save_new_file || term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep_info.is_grep || term.curt().state.is_move_line {
            if clipboard.match_indices(NEW_LINE).count() > 0 {
                term.curt().mbar.set_err(&LANG.cannot_paste_multi_rows.clone());
                return true;
            };
        }
        return false;
    }
}
