use crate::{def::*, global::*, help::Help, log::*, model::*, prompt::prompt::Prompt, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};
use std::{
    cmp::{max, min},
    io::Write,
};

impl EvtAct {
    pub fn match_event<T: Write>(event: Event, out: &mut T, term: &mut Terminal) -> bool {
        Terminal::hide_cur();

        let evt_act_type = EvtAct::check_next_process(event, out, term);
        Log::debug("evt_act_type", &evt_act_type);

        match evt_act_type {
            EvtActType::Exit => return true,
            EvtActType::Hold => {}
            EvtActType::DrawOnly | EvtActType::Next => {
                if evt_act_type == EvtActType::DrawOnly {
                    term.curt().editor.d_range.draw_type = DrawType::All;
                }

                if evt_act_type == EvtActType::Next && !EvtAct::check_err(term) {
                    EvtAct::init(term);
                    Editor::set_org(term);

                    let evt = term.curt().editor.evt;

                    match &evt {
                        Resize(_, _) => term.resize(),
                        Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                            Char('w') => {
                                if Prompt::close(term) == true {
                                    Log::debug("Prompt::close(term) true", &true);
                                    return true;
                                }
                            }
                            Char('s') => {
                                let _ = Tab::save(term);
                            }
                            Char('c') => term.curt().editor.copy(),
                            Char('x') => term.curt().editor.exec_edit_proc(EvtType::Cut, "", ""),
                            Char('v') => term.curt().editor.exec_edit_proc(EvtType::Paste, "", ""),
                            Char('a') => term.curt().editor.all_select(),
                            Char('f') => Prompt::search(term),
                            Char('r') => Prompt::replace(term),
                            Char('g') => Prompt::grep(term),
                            Char('z') => term.curt().editor.undo(),
                            Char('y') => term.curt().editor.redo(),
                            Char('l') => Prompt::move_row(term),
                            Char('t') => term.new_tab(),
                            Char('q') => term.next_tab(),
                            Char('o') => Prompt::open_file(term),
                            Home => term.curt().editor.ctrl_home(),
                            End => term.curt().editor.ctrl_end(),

                            _ => term.curt().mbar.set_err(&LANG.unsupported_operation),
                        },

                        Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                            Right => term.curt().editor.shift_right(),
                            Left => term.curt().editor.shift_left(),
                            Down => term.curt().editor.shift_down(),
                            Up => term.curt().editor.shift_up(),
                            Home => term.curt().editor.shift_home(),
                            End => term.curt().editor.shift_end(),
                            Char(c) => term.curt().editor.exec_edit_proc(EvtType::InsertChar, &c.to_ascii_uppercase().to_string(), ""),
                            F(1) => term.curt().record_key_start(),
                            F(2) => Tab::exec_record_key(out, term),
                            F(4) => term.curt().editor.search_str(false, false),
                            _ => term.curt().mbar.set_err(&LANG.unsupported_operation),
                        },
                        Key(KeyEvent { code, .. }) => match code {
                            Char(c) => term.curt().editor.exec_edit_proc(EvtType::InsertChar, &c.to_string(), ""),
                            Tab => term.curt().editor.exec_edit_proc(EvtType::InsertChar, &TAB.to_string(), ""),
                            Enter => term.curt().editor.exec_edit_proc(EvtType::Enter, "", ""),
                            Backspace => term.curt().editor.exec_edit_proc(EvtType::BS, "", ""),
                            Delete => term.curt().editor.exec_edit_proc(EvtType::Del, "", ""),
                            PageDown => term.curt().editor.page_down(),
                            PageUp => term.curt().editor.page_up(),
                            Up => term.curt().editor.cur_up(),
                            Down => term.curt().editor.cur_down(),
                            Left => term.curt().editor.cur_left(),
                            Right => term.curt().editor.cur_right(),
                            Home => term.curt().editor.home(),
                            End => term.curt().editor.end(),
                            F(1) => Help::disp_toggle(term),
                            F(3) => term.curt().editor.search_str(true, false),
                            F(12) => term.ctrl_mouse_capture(),
                            // Initial display
                            Null => term.curt().editor.d_range.draw_type = DrawType::All,
                            _ => term.curt().mbar.set_err(&LANG.unsupported_operation),
                        },

                        Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => term.curt().editor.cur_up(),
                        Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => term.curt().editor.cur_down(),
                        Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => term.curt().editor.ctrl_mouse(*x as usize, *y as usize, true),
                        Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => term.curt().editor.ctrl_mouse(*x as usize, *y as usize, false),

                        _ => term.curt().mbar.set_err(&LANG.unsupported_operation),
                    }

                    if term.curt().state.is_key_record {
                        term.curt().editor.record_key();
                    }
                    EvtAct::finalize(&mut term.curt().editor);
                    term.curt().editor.set_draw_range();
                }

                // Redraw in case of msg change
                if term.tabs[term.idx].mbar.is_msg_changed() {
                    term.curt().editor.d_range.draw_type = DrawType::All;
                }
                // When key_record is executed, redraw only at the end
                if term.curt().editor.d_range.draw_type != DrawType::Not || (term.curt().state.is_key_record_exec == false || term.curt().state.is_key_record_exec == true && term.curt().state.is_key_record_exec_draw == true) {
                    term.draw(out);
                }
            }
        }
        Terminal::show_cur();
        return false;
    }

    pub fn check_next_process<T: Write>(evt: Event, out: &mut T, term: &mut Terminal) -> EvtActType {
        let evt_act = EvtAct::check_headerbar(evt, term);
        Log::debug("EvtAct::check_headerbar", &evt_act);

        if evt_act != EvtActType::Hold {
            return evt_act;
        }

        if EvtAct::check_err_prompt(term) {
            return EvtActType::DrawOnly;
        }

        EvtAct::clear_mag(&mut term.tabs[term.idx]);
        EvtAct::clear_tab_comp(&mut term.tabs[term.idx]);

        let evt_act = EvtAct::check_prom(out, term);
        Log::debug("EvtAct::check_prom", &evt_act);

        if evt_act == EvtActType::Hold && !term.curt().state.grep_info.is_result {
            term.set_disp_size();
            term.curt().mbar.draw_only(out);
            let tab_state = term.curt().state.clone();
            term.curt().prom.draw_only(out, &tab_state);
        }
        return evt_act;
    }

    pub fn init(term: &mut Terminal) {
        Log::debug_s("　　　　　　　EvtAct.init");

        let tab = term.tabs.get_mut(term.idx).unwrap();

        // Initialize of updown_x
        match tab.editor.evt {
            //  Down | Up | ShiftDown | ShiftUp
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up => {}
                _ => tab.editor.updown_x = 0,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {}
                _ => tab.editor.updown_x = 0,
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => tab.editor.updown_x = 0,
        }
        // redraw判定
        tab.editor.d_range.draw_type = DrawType::Not;
        match tab.editor.evt {
            Resize(_, _) => tab.editor.d_range.draw_type = DrawType::All,
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Home | End | Char('c') => tab.editor.d_range.draw_type = DrawType::Not,
                _ => tab.editor.d_range.draw_type = DrawType::All,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End | F(4) => {}
                _ => tab.editor.d_range.draw_type = DrawType::All,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {
                    if tab.editor.sel.is_selected() {
                        let sel = tab.editor.sel.get_range();
                        tab.editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                    } else {
                        if tab.editor.evt == DOWN || tab.editor.evt == UP {
                            let y = tab.editor.cur.y;
                            let y_after = if tab.editor.evt == DOWN {
                                min(y + 1, tab.editor.buf.len_lines() - 1)
                            } else {
                                if y == 0 {
                                    0
                                } else {
                                    y - 1
                                }
                            };

                            tab.editor.d_range = DRange::new(min(y, y_after), max(y, y_after), DrawType::Target);
                        } else {
                            tab.editor.d_range.draw_type = DrawType::MoveCur;
                        }
                    };
                }
                F(1) => tab.editor.d_range.draw_type = DrawType::All,
                F(3) => tab.editor.d_range.draw_type = DrawType::All,
                _ => tab.editor.d_range.draw_type = DrawType::All,
            },

            // for err msg or selected
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {
                if tab.editor.sel.is_selected() {
                    tab.editor.d_range.draw_type = DrawType::Target;
                }
            }
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) | Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {
                if tab.editor.sel.is_selected() {
                    let sel = tab.editor.sel.get_range();
                    tab.editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                }
            }
            _ => tab.editor.d_range.draw_type = DrawType::Not,
        }

        // Edit is_change=true, Clear redo_vec,
        if term.curt().editor.is_edit_evt(false) {
            term.hbar.file_vec[term.idx].is_changed = true;
            // FILE_VEC.get().unwrap().try_lock().unwrap()[term.term.tabs[term.tab_idx]_idx].is_changed = true;
            term.curt().editor.history.clear_redo_vec();
        }

        // clear_redo_vec
        match term.curt().editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('x') | Char('v') => term.curt().editor.history.clear_redo_vec(),
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(_) | Enter | Backspace | Delete => term.curt().editor.history.clear_redo_vec(),
                _ => {}
            },
            _ => {}
        }
        // Msg clear  Other than cursor move
        match term.curt().editor.evt {
            Resize(_, _) => {}
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => term.curt().mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => term.curt().mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => term.curt().mbar.clear_mag(),
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => term.curt().mbar.clear_mag(),
        }
    }

    pub fn finalize(editor: &mut Editor) {
        Log::debug_s("　　　　　　　EvtAct.finalize");

        // set sel draw range, Clear sel range
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End | F(4) => {}
                _ => editor.sel.clear(),
            },
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('a') | Char('c') => {}
                _ => editor.sel.clear(),
            },
            Key(KeyEvent { code, .. }) => match code {
                F(3) => {}
                _ => editor.sel.clear(),
            },
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {}
            _ => editor.sel.clear(),
        }

        // Refresh search results
        if editor.is_edit_evt(true) && editor.search.ranges.len() > 0 {
            editor.search.ranges = editor.get_search_ranges(&editor.search.str, 0, editor.buf.len_chars(), 0);
        }
    }
    pub fn check_err(term: &mut Terminal) -> bool {
        let is_return = false;

        match term.curt().editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                // Check if sel range is set
                Char('x') | Char('c') => {
                    if !term.curt().editor.sel.is_selected() {
                        term.curt().mbar.set_err(&LANG.no_sel_range.to_string());
                        return true;
                    }
                }
                Char('z') => {
                    if term.curt().editor.history.len_undo() == 0 {
                        term.curt().mbar.set_err(&LANG.no_undo_operation.to_string());
                        return true;
                    }
                }
                Char('y') => {
                    if term.curt().editor.history.len_redo() == 0 {
                        term.curt().mbar.set_err(&LANG.no_operation_re_exec.to_string());
                        return true;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        return is_return;
    }
}
