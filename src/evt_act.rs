use crate::{def::*, global::*, log::*, model::*, prompt::prompt::Prompt, tab::Tab, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind, *};
use std::{
    cmp::{max, min},
    io::Write,
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
                Log::ep("term.tab_idx", &term.idx);

                // let rc = Rc::clone(&term.tabs[term.tab_idx]);
                // let mut tab = term.tabs[term.tab_idx];
                // Log::ep("editor.evt", &tab.editor.evt);

                if evt_act_type == EvtActType::DrawOnly {
                    //  tab.editor.d_range.draw_type = DrawType::None;
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::None;
                }

                if evt_act_type == EvtActType::Next && !EvtAct::check_err(term) {
                    EvtAct::init(term);
                    Editor::set_org(term);

                    let evt = term.tabs[term.idx].editor.evt;

                    match &evt {
                        Resize(_, _) => {
                            term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                        }
                        Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                            Char('w') => {
                                if Prompt::close(term) == true {
                                    return true;
                                }
                            }
                            Char('s') => {
                                let _ = Tab::save(term);
                            }
                            Char('c') => term.tabs[term.idx].editor.copy(),
                            Char('x') => term.tabs[term.idx].editor.exec_edit_proc(EvtType::Cut, "", ""),
                            Char('v') => term.tabs[term.idx].editor.exec_edit_proc(EvtType::Paste, "", ""),
                            Char('a') => term.tabs[term.idx].editor.all_select(),
                            Char('f') => Prompt::search(term),
                            Char('r') => Prompt::replace(term),
                            Char('g') => Prompt::grep(term),
                            Char('z') => term.tabs[term.idx].editor.undo(),
                            Char('y') => term.tabs[term.idx].editor.redo(),
                            Char('l') => Prompt::move_row(term),
                            Home => term.tabs[term.idx].editor.ctrl_home(),
                            End => term.tabs[term.idx].editor.ctrl_end(),
                            _ => term.tabs[term.idx].mbar.set_err(&LANG.unsupported_operation),
                        },

                        Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                            Right => term.tabs[term.idx].editor.shift_right(),
                            Left => term.tabs[term.idx].editor.shift_left(),
                            Down => term.tabs[term.idx].editor.shift_down(),
                            Up => term.tabs[term.idx].editor.shift_up(),
                            Home => term.tabs[term.idx].editor.shift_home(),
                            End => term.tabs[term.idx].editor.shift_end(),
                            Char(c) => term.tabs[term.idx].editor.exec_edit_proc(EvtType::InsertChar, &c.to_ascii_uppercase().to_string(), ""),
                            F(1) => term.tabs[term.idx].record_key_start(),
                            F(2) => Tab::exec_record_key(out, term),
                            F(4) => term.tabs[term.idx].editor.search_str(false, false),
                            _ => term.tabs[term.idx].mbar.set_err(&LANG.unsupported_operation),
                        },
                        Key(KeyEvent { code, .. }) => match code {
                            Char(c) => term.tabs[term.idx].editor.exec_edit_proc(EvtType::InsertChar, &c.to_string(), ""),
                            Enter => term.tabs[term.idx].editor.exec_edit_proc(EvtType::Enter, "", ""),
                            Backspace => term.tabs[term.idx].editor.exec_edit_proc(EvtType::BS, "", ""),
                            Delete => term.tabs[term.idx].editor.exec_edit_proc(EvtType::Del, "", ""),
                            PageDown => term.tabs[term.idx].editor.page_down(),
                            PageUp => term.tabs[term.idx].editor.page_up(),
                            Up => term.tabs[term.idx].editor.cur_up(),
                            Down => term.tabs[term.idx].editor.cur_down(),
                            Left => term.tabs[term.idx].editor.cur_left(),
                            Right => term.tabs[term.idx].editor.cur_right(),
                            Home => term.tabs[term.idx].editor.home(),
                            End => term.tabs[term.idx].editor.end(),
                            F(1) => term.help.disp_toggle(&mut term.tabs[term.idx].editor),
                            F(3) => term.tabs[term.idx].editor.search_str(true, false),
                            _ => term.tabs[term.idx].mbar.set_err(&LANG.unsupported_operation),
                        },

                        Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => term.tabs[term.idx].editor.cur_up(),
                        Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => term.tabs[term.idx].editor.cur_down(),
                        Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => term.tabs[term.idx].editor.ctrl_mouse((x + 1) as usize, *y as usize, true),
                        Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: _, row: _, .. }) => {}
                        Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => term.tabs[term.idx].editor.ctrl_mouse((x + 1) as usize, *y as usize, false),

                        _ => term.tabs[term.idx].mbar.set_err(&LANG.unsupported_operation),
                    }

                    if term.tabs[term.idx].prom.is_key_record {
                        term.tabs[term.idx].editor.record_key();
                    }
                    EvtAct::finalize(&mut term.tabs[term.idx].editor);
                    term.tabs[term.idx].editor.set_draw_range();
                }

                // Redraw in case of msg change
                if term.tabs[term.idx].mbar.msg_org != term.tabs[term.idx].mbar.msg {
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                }
                // When key_record is executed, redraw only at the end
                if term.tabs[term.idx].editor.d_range.draw_type != DrawType::Not || (term.tabs[term.idx].prom.is_key_record_exec == false || term.tabs[term.idx].prom.is_key_record_exec == true && term.tabs[term.idx].prom.is_key_record_exec_draw == true) {
                    term.draw(out);
                }
            }
        }
        Terminal::show_cur();
        return false;
    }

    pub fn check_next_process<T: Write>(evt: Event, out: &mut T, term: &mut Terminal) -> EvtActType {
        let (event, evt_act) = EvtAct::check_headerbar(evt, term);

        term.tabs[term.idx].editor.evt = event;
        if evt_act != EvtActType::Hold {
            return evt_act;
        }
        Log::ep("check_headerbar", &event.clone());
        // let rc = Rc::clone(&term.tabs[term.tab_idx]);
        // let mut tab = term.tabs[term.tab_idx];

        term.tabs[term.idx].mbar.msg_org = term.tabs[term.idx].mbar.msg.clone();

        Log::ep("evt_act", &evt_act);

        if evt_act != EvtActType::Hold {
            return evt_act;
        }

        EvtAct::check_clear_mag(&mut term.tabs[term.idx]);
        EvtAct::check_grep_clear_tab_comp(&mut term.tabs[term.idx]);
        let evt_act = EvtAct::check_prom(out, term);

        Log::ep("evt_act", &evt_act);

        if evt_act == EvtActType::Hold {
            if term.tabs[term.idx].mbar.msg_org != term.tabs[term.idx].mbar.msg {
                term.tabs[term.idx].mbar.draw_only(out);
                term.tabs[term.idx].prom.draw_cur_only(out);
            }
            let tab_state = term.tabs[term.idx].state.clone();
            term.tabs[term.idx].prom.draw_only(out, &tab_state);
        }

        return evt_act;
    }

    pub fn init(term: &mut Terminal) {
        Log::ep_s("init");
        // let rc = Rc::clone(&term.term.tabs[term.tab_idx]s[term.term.tabs[term.tab_idx]_idx]);
        //  let mut term.tabs[term.tab_idx] = term.term.tabs[term.tab_idx]s[term.term.tabs[term.tab_idx]_idx];

        // Initialize of updown_x
        match term.tabs[term.idx].editor.evt {
            //  Down | Up | ShiftDown | ShiftUp
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up => {}
                _ => term.tabs[term.idx].editor.updown_x = 0,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {}
                _ => term.tabs[term.idx].editor.updown_x = 0,
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => term.tabs[term.idx].editor.updown_x = 0,
        }
        // redraw判定
        term.tabs[term.idx].editor.d_range.draw_type = DrawType::Not;
        match term.tabs[term.idx].editor.evt {
            Resize(_, _) => term.tabs[term.idx].editor.d_range.draw_type = DrawType::All,
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Home | End | Char('c') => term.tabs[term.idx].editor.d_range.draw_type = DrawType::Not,
                _ => term.tabs[term.idx].editor.d_range.draw_type = DrawType::All,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End | F(4) => {}
                _ => term.tabs[term.idx].editor.d_range.draw_type = DrawType::All,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {
                    if term.tabs[term.idx].editor.sel.is_selected() {
                        let sel = term.tabs[term.idx].editor.sel.get_range();
                        term.tabs[term.idx].editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                    } else {
                        if term.tabs[term.idx].editor.evt == DOWN || term.tabs[term.idx].editor.evt == UP {
                            // let y = term.tabs[term.tab_idx].editor.cur.y - term.tabs[term.tab_idx].editor.offset_y;
                            let y = term.tabs[term.idx].editor.cur.y;

                            let y_after = if term.tabs[term.idx].editor.evt == DOWN {
                                y + 1
                            } else {
                                if y == 0 {
                                    0
                                } else {
                                    y - 1
                                }
                            };

                            term.tabs[term.idx].editor.d_range = DRange::new(min(y, y_after), max(y, y_after), DrawType::Target);
                        } else {
                            term.tabs[term.idx].editor.d_range.draw_type = DrawType::MoveCur;
                        }
                    };
                }
                F(1) => term.tabs[term.idx].editor.d_range.draw_type = DrawType::All,
                F(3) => {
                    if term.tabs[term.idx].editor.search.idx == USIZE_UNDEFINED {
                        term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
                    }
                }
                _ => term.tabs[term.idx].editor.d_range.draw_type = DrawType::All,
            },

            // for err msg or selected
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {
                if term.tabs[term.idx].editor.sel.is_selected() {
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::Target;
                }
            }
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) | Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {
                if term.tabs[term.idx].editor.sel.is_selected() {
                    let sel = term.tabs[term.idx].editor.sel.get_range();
                    term.tabs[term.idx].editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                }
            }
            _ => term.tabs[term.idx].editor.d_range.draw_type = DrawType::Not,
        }

        // Edit    is_change=true, Clear redo_vec,
        if term.tabs[term.idx].editor.is_edit_evt(false) {
            term.hbar.file_vec[term.idx].is_changed = true;
            // FILE_VEC.get().unwrap().try_lock().unwrap()[term.term.tabs[term.tab_idx]_idx].is_changed = true;
            term.tabs[term.idx].editor.history.clear_redo_vec();
        }

        // clear_redo_vec
        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('x') | Char('v') => term.tabs[term.idx].editor.history.clear_redo_vec(),
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(_) | Enter | Backspace | Delete => term.tabs[term.idx].editor.history.clear_redo_vec(),
                _ => {}
            },
            _ => {}
        }
        // Msg clear  Other than cursor move
        match term.tabs[term.idx].editor.evt {
            Resize(_, _) => {}
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => term.tabs[term.idx].mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => term.tabs[term.idx].mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => term.tabs[term.idx].mbar.clear_mag(),
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => term.tabs[term.idx].mbar.clear_mag(),
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
    pub fn check_err(term: &mut Terminal) -> bool {
        let is_return = false;

        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                // Check if sel range is set
                Char('x') | Char('c') => {
                    if !term.tabs[term.idx].editor.sel.is_selected() {
                        term.tabs[term.idx].mbar.set_err(&LANG.no_sel_range.to_string());
                        return true;
                    }
                }
                Char('z') => {
                    if term.tabs[term.idx].editor.history.len_undo() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.no_undo_operation.to_string());
                        return true;
                    }
                }
                Char('y') => {
                    if term.tabs[term.idx].editor.history.len_redo() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.no_operation_re_exec.to_string());
                        return true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        // Check if sel range is set
        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('v') => {
                    let clipboard = term.tabs[term.idx].editor.get_clipboard().unwrap_or("".to_string());
                    if clipboard.len() == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.no_value_in_clipboard.to_string());
                        return true;
                    }
                    // TODO TEST
                    // Do not paste multiple lines for Prompt
                    if term.tabs[term.idx].prom.is_save_new_file || term.tabs[term.idx].state.is_search || term.tabs[term.idx].state.is_replace || term.tabs[term.idx].state.grep_info.is_grep || term.tabs[term.idx].prom.is_move_line {
                        if clipboard.match_indices(NEW_LINE).count() > 0 {
                            term.tabs[term.idx].mbar.set_err(&LANG.cannot_paste_multi_rows.clone());
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
