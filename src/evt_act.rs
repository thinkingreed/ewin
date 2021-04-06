use crate::{def::*, global::*, log::*, model::*, prompt::prompt::Prompt, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};
use std::{
    cmp::{max, min},
    io::Write,
    rc::Rc,
};

impl EvtAct {
    pub fn match_event<T: Write>(event: Event, out: &mut T, term: &mut Terminal) -> bool {
        match event {
            Mouse(M_Event { kind: M_Kind::Moved, .. }) => return false,
            _ => {}
        }
        Terminal::hide_cur();

        let evt_act_type = EvtAct::check_next_process(event, out, term);

        Log::ep("evt_act_type", &evt_act_type);

        match evt_act_type {
            EvtActType::Exit => return true,
            EvtActType::Hold => {}
            EvtActType::DrawOnly | EvtActType::Next => {
                Log::ep("term.tab_idx", &term.tab_idx);

                let rc = Rc::clone(&term.tabs[term.tab_idx]);
                let mut tab = rc.borrow_mut();
                Log::ep("editor.evt", &tab.editor.evt);

                if evt_act_type == EvtActType::DrawOnly {
                    tab.editor.d_range.draw_type = DrawType::None;
                }

                if evt_act_type == EvtActType::Next && !EvtAct::check_err(&mut tab) {
                    EvtAct::init(term, &mut tab);

                    tab.editor.cur_y_org = tab.editor.cur.y;
                    let offset_y_org = tab.editor.offset_y;
                    let offset_x_org = tab.editor.offset_x;
                    let rnw_org = tab.editor.get_rnw();
                    tab.editor.sel_org = tab.editor.sel;

                    match tab.editor.evt {
                        Resize(_, _) => tab.editor.d_range.draw_type = DrawType::All,
                        Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                            Char('w') => {
                                if Prompt::close(term, &mut tab) == true {
                                    return true;
                                }
                            }
                            Char('s') => {
                                let _ = tab.save(term);
                            }
                            Char('c') => tab.editor.copy(),
                            Char('x') => tab.editor.exec_edit_proc(EvtType::Cut, "", ""),
                            Char('v') => tab.editor.exec_edit_proc(EvtType::Paste, "", ""),
                            Char('a') => tab.editor.all_select(),
                            Char('f') => Prompt::search(term, &mut tab),
                            Char('r') => Prompt::replace(term, &mut tab),
                            Char('g') => Prompt::grep(term, &mut tab),
                            Char('z') => tab.editor.undo(),
                            Char('y') => tab.editor.redo(),
                            Char('l') => tab.prom.move_row(),
                            Home => tab.editor.ctrl_home(),
                            End => tab.editor.ctrl_end(),
                            _ => tab.mbar.set_err(&LANG.unsupported_operation),
                        },

                        Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                            Right => tab.editor.shift_right(),
                            Left => tab.editor.shift_left(),
                            Down => tab.editor.shift_down(),
                            Up => tab.editor.shift_up(),
                            Home => tab.editor.shift_home(),
                            End => tab.editor.shift_end(),
                            Char(c) => tab.editor.exec_edit_proc(EvtType::InsertChar, &c.to_ascii_uppercase().to_string(), ""),
                            F(1) => tab.record_key_start(),
                            F(2) => tab.exec_record_key(out, term),
                            F(4) => tab.editor.search_str(false, false),
                            _ => tab.mbar.set_err(&LANG.unsupported_operation),
                        },
                        Key(KeyEvent { code, .. }) => match code {
                            Char(c) => tab.editor.exec_edit_proc(EvtType::InsertChar, &c.to_string(), ""),
                            Enter => tab.editor.exec_edit_proc(EvtType::Enter, "", ""),
                            Backspace => tab.editor.exec_edit_proc(EvtType::BS, "", ""),
                            Delete => tab.editor.exec_edit_proc(EvtType::Del, "", ""),
                            PageDown => tab.editor.page_down(),
                            PageUp => tab.editor.page_up(),
                            Up => tab.editor.cur_up(),
                            Down => tab.editor.cur_down(),
                            Left => tab.editor.cur_left(),
                            Right => tab.editor.cur_right(),
                            Home => tab.editor.home(),
                            End => tab.editor.end(),
                            F(1) => term.help.disp_toggle(&mut tab.editor),
                            F(3) => tab.editor.search_str(true, false),
                            _ => tab.mbar.set_err(&LANG.unsupported_operation),
                        },

                        Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => tab.editor.cur_up(),
                        Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => tab.editor.cur_down(),
                        Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => tab.editor.ctrl_mouse((x + 1) as usize, y as usize, true),
                        Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: _, row: _, .. }) => {}
                        Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => tab.editor.ctrl_mouse((x + 1) as usize, y as usize, false),

                        _ => tab.mbar.set_err(&LANG.unsupported_operation),
                    }

                    if tab.prom.is_key_record {
                        tab.editor.record_key();
                    }
                    EvtAct::finalize(&mut tab.editor);
                    let cur_y_org = tab.editor.cur_y_org;
                    tab.editor.set_draw_range(cur_y_org, offset_y_org, offset_x_org, rnw_org);
                }

                Log::ep("offset_y", &tab.editor.offset_y);
                Log::ep("offset_x", &tab.editor.offset_x);
                Log::ep("cur.y", &tab.editor.cur.y);
                Log::ep("cur.x", &tab.editor.cur.x);
                Log::ep("cur.disp_x", &tab.editor.cur.disp_x);
                // Log::ep("", &tab.editor.sel);
                // Log::ep("", &tab.editor.search);
                Log::ep("", &tab.state);

                Log::ep("d_range", &tab.editor.d_range);

                // Redraw in case of msg change
                if tab.mbar.msg_org != tab.mbar.msg {
                    tab.editor.d_range.draw_type = DrawType::All;
                }
                // When key_record is executed, redraw only at the end
                if tab.editor.d_range.draw_type != DrawType::Not || (tab.prom.is_key_record_exec == false || tab.prom.is_key_record_exec == true && tab.prom.is_key_record_exec_draw == true) {
                    term.draw(out, &mut tab);
                }
            }
        }
        Terminal::show_cur();
        return false;
    }

    pub fn check_next_process<T: Write>(evt: Event, out: &mut T, term: &mut Terminal) -> EvtActType {
        let (event, evt_act) = EvtAct::check_headerbar(evt, term);

        term.tabs[term.tab_idx].borrow_mut().editor.evt = event;
        if evt_act != EvtActType::Hold {
            return evt_act;
        }
        Log::ep("check_headerbar", &event.clone());
        let rc = Rc::clone(&term.tabs[term.tab_idx]);
        let mut tab = rc.borrow_mut();

        tab.mbar.msg_org = tab.mbar.msg.clone();

        Log::ep("evt_act", &evt_act);

        if evt_act != EvtActType::Hold {
            return evt_act;
        }

        EvtAct::check_clear_mag(&mut tab);
        EvtAct::check_grep_clear_tab_comp(&mut tab);
        let evt_act = EvtAct::check_prom(out, term, &mut tab);

        Log::ep("evt_act", &evt_act);

        if evt_act == EvtActType::Hold {
            if tab.mbar.msg_org != tab.mbar.msg {
                tab.mbar.draw_only(out);
                tab.prom.draw_cur_only(out);
            }
            let tab_state = tab.state.clone();
            tab.prom.draw_only(out, &tab_state);
        }

        return evt_act;
    }

    pub fn init(term: &mut Terminal, tab: &mut Tab) {
        Log::ep_s("init");

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
                            // let y = tab.editor.cur.y - tab.editor.offset_y;
                            let y = tab.editor.cur.y;

                            let y_after = if tab.editor.evt == DOWN {
                                y + 1
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
                F(3) => {
                    if tab.editor.search.idx == USIZE_UNDEFINED {
                        tab.editor.d_range.draw_type = DrawType::All;
                    }
                }
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

        // Edit    is_change=true, Clear redo_vec,
        if tab.editor.is_edit_evt(false) {
            term.hbar.file_vec[term.tab_idx].is_changed = true;
            // FILE_VEC.get().unwrap().try_lock().unwrap()[term.tab_idx].is_changed = true;
            tab.editor.history.clear_redo_vec();
        }

        // clear_redo_vec
        match tab.editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('x') | Char('v') => tab.editor.history.clear_redo_vec(),
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(_) | Enter | Backspace | Delete => tab.editor.history.clear_redo_vec(),
                _ => {}
            },
            _ => {}
        }
        // Msg clear  Other than cursor move
        match tab.editor.evt {
            Resize(_, _) => {}
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => tab.mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => tab.mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => tab.mbar.clear_mag(),
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => tab.mbar.clear_mag(),
        }
    }

    pub fn finalize(editor: &mut Editor) {
        Log::ep_s("finalize");

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
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), .. }) => {}
            _ => editor.sel.clear(),
        }

        // Refresh search results
        if editor.is_edit_evt(true) && editor.search.ranges.len() > 0 {
            editor.search.ranges = editor.get_search_ranges(&editor.search.str, 0, editor.buf.len_chars(), 0);
        }
    }
    pub fn check_err(tab: &mut Tab) -> bool {
        let is_return = false;

        match tab.editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                // Check if sel range is set
                Char('x') | Char('c') => {
                    if !tab.editor.sel.is_selected() {
                        tab.mbar.set_err(&LANG.no_sel_range.to_string());
                        return true;
                    }
                }
                Char('z') => {
                    if tab.editor.history.len_undo() == 0 {
                        tab.mbar.set_err(&LANG.no_undo_operation.to_string());
                        return true;
                    }
                }
                Char('y') => {
                    if tab.editor.history.len_redo() == 0 {
                        tab.mbar.set_err(&LANG.no_operation_re_exec.to_string());
                        return true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        // Check if sel range is set
        match tab.editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('v') => {
                    let clipboard = tab.editor.get_clipboard().unwrap_or("".to_string());
                    if clipboard.len() == 0 {
                        tab.mbar.set_err(&LANG.no_value_in_clipboard.to_string());
                        return true;
                    }
                    // TODO TEST
                    // Do not paste multiple lines for Prompt
                    if tab.prom.is_save_new_file || tab.state.is_search || tab.state.is_replace || tab.state.grep_info.is_grep || tab.prom.is_move_line {
                        if clipboard.match_indices(NEW_LINE).count() > 0 {
                            tab.mbar.set_err(&LANG.cannot_paste_multi_rows.clone());
                            return true;
                        };
                    }
                }
                _ => {}
            },
            _ => {}
        }
        return is_return;
    }
}
