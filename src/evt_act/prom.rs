use crate::{_cfg::keys::*, clipboard::*, global::*, log::*, model::*, prompt::cont::promptcont::*, tab::Tab, terminal::*};
impl EvtAct {
    pub fn check_prom(term: &mut Terminal) -> EvtActType {
        Log::debug_key("check_prom");

        // Close・Esc
        if !term.curt().state.is_nomal() {
            match term.curt().prom.keycmd {
                KeyCmd::CloseFile => {
                    if term.curt().state.grep_state.is_result && !(term.curt().state.grep_state.is_stdout_end && term.curt().state.grep_state.is_stderr_end) && !term.curt().state.grep_state.is_cancel {
                        return EvtActType::Hold;
                    } else {
                        return EvtActType::Next;
                    }
                }
                KeyCmd::EscPrompt => {
                    if term.state.is_all_close_confirm {
                        term.cancel_close_all_tab();
                    } else if term.state.is_all_save {
                        term.cancel_save_all_tab();
                    } else if term.curt().state.grep_state.is_result {
                        if (term.curt().state.grep_state.is_stdout_end && term.curt().state.grep_state.is_stderr_end) || term.curt().state.grep_state.is_cancel {
                            if term.curt().state.is_search {
                                term.curt().prom.clear();
                                term.curt().state.is_search = false;
                                term.curt().editor.draw_type = DrawType::All;
                            }
                        } else {
                            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = true).unwrap();
                        }
                    } else if term.curt().state.is_read_only {
                    } else if term.curt().state.is_search {
                        term.curt().editor.search.clear();
                        term.curt().prom.clear();
                    } else {
                        term.clear_curt_tab();
                        term.curt().state.grep_state.is_grep = false;
                    }
                    term.clear_curt_tab();
                    return EvtActType::DrawOnly;
                }
                _ => {}
            }
        }

        // contents operation
        if term.curt().state.is_exists_input_field() {
            let state = &term.curt().state.clone();
            match term.curt().prom.keycmd {
                KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => term.curt().prom.shift_move_com(),
                KeyCmd::BackTab => {
                    term.curt().prom.tab(false, state);
                    term.curt().prom.clear_sels()
                }
                KeyCmd::InsertStr(_) => {
                    if !state.is_move_row {
                        term.curt().prom.operation()
                    }
                }
                KeyCmd::Copy => term.curt().prom.copy(),
                KeyCmd::Cut => term.curt().prom.operation(),
                KeyCmd::Undo => term.curt().prom.undo(),
                KeyCmd::Redo => term.curt().prom.redo(),
                KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => term.curt().prom.operation(),
                KeyCmd::CursorUp => term.curt().prom.cursor_up(state),
                KeyCmd::CursorDown => term.curt().prom.cursor_down(state),
                KeyCmd::Tab => term.curt().prom.tab(true, state),
                KeyCmd::MouseDownLeft(y, x) => term.curt().prom.ctrl_mouse(&state, y, x, true),
                KeyCmd::MouseDragLeft(y, x) => term.curt().prom.ctrl_mouse(&state, y, x, false),
                _ => {}
            }
            // clear_sels
            match term.curt().prom.keycmd {
                KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::Tab => term.curt().prom.clear_sels(),
                _ => {}
            }
        }

        // Search・replace・grep option
        if term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep_state.is_grep {
            match term.curt().prom.keycmd {
                KeyCmd::FindCaseSensitive => {
                    term.curt().prom.cont_1.change_opt_case_sens();
                    return EvtActType::Hold;
                }
                KeyCmd::FindRegex => {
                    term.curt().prom.cont_1.change_opt_regex();
                    return EvtActType::Hold;
                }
                KeyCmd::MouseDownLeft(y, x) => {
                    let (y, x) = (y as u16, x as u16);
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

        // unable to edit
        if term.curt().state.grep_state.is_result || term.curt().state.is_read_only {
            if term.curt().state.is_search || term.curt().state.is_move_row {
            } else {
                if Keybind::is_edit(&term.curt().prom.keycmd, true) {
                    /*
                    if term.curt().state.grep_state.is_result {
                        if term.curt().prom.keycmd != KeyCmd::ConfirmPrompt {
                            return EvtActType::Hold;
                        }
                    } else {
                        return EvtActType::Hold;
                    }
                     */
                    return EvtActType::Hold;
                }
                match term.curt().prom.keycmd {
                    // Ctrl
                    KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd | KeyCmd::AllSelect | KeyCmd::Copy | KeyCmd::Find | KeyCmd::NewTab | KeyCmd::OpenFile(_) | KeyCmd::MoveRow => return EvtActType::Next,
                    // Shift
                    KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect | KeyCmd::FindBack => return EvtActType::Next,
                    //
                    KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::CursorPageUp | KeyCmd::CursorPageDown | KeyCmd::Help => return EvtActType::Next,
                    //  KeyCmd::ConfirmPrompt => return EvtActType::Hold,
                    KeyCmd::ConfirmPrompt | KeyCmd::FindNext | KeyCmd::EscPrompt => {}
                    // mouse
                    KeyCmd::MouseScrollUp | KeyCmd::MouseScrollDown | KeyCmd::MouseDownLeft(_, _) | KeyCmd::MouseDragLeft(_, _) => return EvtActType::Next,
                    KeyCmd::Resize => {}
                    _ => return EvtActType::Hold,
                }
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
        } else if term.curt().state.grep_state.is_grep == true {
            return EvtAct::grep(term);
        } else if term.curt().state.grep_state.is_result == true {
            return EvtAct::grep_result(term);
        } else if term.curt().state.is_move_row == true {
            return EvtAct::move_row(term);
        } else if term.curt().state.is_open_file == true {
            return EvtAct::open_file(term);
        } else if term.curt().state.is_enc_nl == true {
            return EvtAct::enc_nl(term);
        } else if term.curt().state.is_menu == true {
            return EvtAct::menu(term);
        } else {
            return EvtActType::Next;
        }
    }

    pub fn clear_mag(tab: &mut Tab) {
        tab.mbar.msg_org = tab.mbar.msg.clone();

        /*
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
        } */
    }

    pub fn clear_tab_comp(tab: &mut Tab) {
        Log::debug_s("check_grep_clear_tab_comp");

        if tab.state.grep_state.is_grep || tab.state.is_open_file || tab.state.is_save_new_file {
            // Check clear tab candidate
            match tab.prom.keycmd {
                KeyCmd::InsertStr(_) | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => {
                    if tab.state.grep_state.is_grep {
                        tab.prom.prom_grep.tab_comp.clear_tab_comp()
                    } else if tab.state.is_open_file {
                        tab.prom.prom_open_file.tab_comp.clear_tab_comp()
                    } else if tab.state.is_save_new_file {
                        tab.prom.prom_save_new_file.tab_comp.clear_tab_comp()
                    }
                }
                _ => {}
            }
        }
    }
    pub fn check_err_prompt(term: &mut Terminal) -> bool {
        // Check if sel range is set
        if term.curt().state.is_exists_input_field() {
            match &term.curt().prom.keycmd {
                KeyCmd::Copy | KeyCmd::Cut => {
                    let is_selected = match term.curt().prom.cont_posi {
                        PromptContPosi::First => term.curt().prom.cont_1.sel.is_selected(),
                        PromptContPosi::Second => term.curt().prom.cont_2.sel.is_selected(),
                        PromptContPosi::Third => term.curt().prom.cont_3.sel.is_selected(),
                        PromptContPosi::Fourth => term.curt().prom.cont_4.sel.is_selected(),
                    };
                    if !is_selected {
                        term.curt().mbar.set_err(&LANG.no_sel_range.to_string());
                        return true;
                    }
                }
                KeyCmd::InsertStr(str) => {
                    if str.is_empty() {
                        if EvtAct::check_clipboard(term) {
                            term.curt().mbar.set_err(&LANG.cannot_paste_multi_rows.clone());
                            return true;
                        }
                    }
                }
                KeyCmd::Undo => {
                    let is_empty_undo = match term.curt().prom.cont_posi {
                        PromptContPosi::First => term.curt().prom.cont_1.history.undo_vec.is_empty(),
                        PromptContPosi::Second => term.curt().prom.cont_2.history.undo_vec.is_empty(),
                        PromptContPosi::Third => term.curt().prom.cont_3.history.undo_vec.is_empty(),
                        PromptContPosi::Fourth => term.curt().prom.cont_4.history.undo_vec.is_empty(),
                    };
                    if is_empty_undo {
                        term.curt().mbar.set_err(&LANG.no_undo_operation.to_string());
                        return true;
                    }
                }
                KeyCmd::Redo => {
                    let is_empty_redo = match term.curt().prom.cont_posi {
                        PromptContPosi::First => term.curt().prom.cont_1.history.redo_vec.is_empty(),
                        PromptContPosi::Second => term.curt().prom.cont_2.history.redo_vec.is_empty(),
                        PromptContPosi::Third => term.curt().prom.cont_3.history.redo_vec.is_empty(),
                        PromptContPosi::Fourth => term.curt().prom.cont_4.history.redo_vec.is_empty(),
                    };
                    if is_empty_redo {
                        term.curt().mbar.set_err(&LANG.no_redo_operation.to_string());
                        return true;
                    }
                }
                _ => return false,
            }
        }
        return false;
    }

    pub fn check_clipboard(term: &mut Terminal) -> bool {
        let clipboard = get_clipboard().unwrap_or("".to_string());

        if clipboard.len() == 0 {
            term.curt().mbar.set_err(&LANG.no_value_in_clipboard.to_string());
            return true;
        }
        // Do not paste multiple lines for Prompt
        if term.curt().state.is_save_new_file || term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep_state.is_grep || term.curt().state.is_move_row {
            if clipboard.match_indices(&NL::get_nl(&term.curt().editor.h_file.nl)).count() > 0 {
                return true;
            };
        }
        return false;
    }
}
