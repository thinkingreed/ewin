use crate::{def::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, statusbar::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use std::io::Write;

impl EvtAct {
    pub fn match_event<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> bool {
        match editor.evt {
            Mouse(M_Event { kind: M_Kind::Moved, .. }) => return false,
            _ => {}
        }

        Terminal::hide_cur();
        mbar.msg_org = mbar.msg.clone();
        let evt_next_process = EvtAct::check_next_process(out, editor, mbar, prom, help, sbar);

        match evt_next_process {
            EvtActType::Exit => return true,
            EvtActType::Hold => {}
            EvtActType::DrawOnly | EvtActType::Next => {
                Log::ep("editor.evt", &editor.evt);

                if evt_next_process == EvtActType::Next && !EvtAct::check_err(editor, mbar) {
                    EvtAct::init(editor, mbar, prom);
                    editor.cur_y_org = editor.cur.y;
                    let offset_y_org = editor.offset_y;
                    let offset_x_org = editor.offset_x;
                    let rnw_org = editor.rnw;
                    editor.sel_org = editor.sel;

                    match editor.evt {
                        Resize(_, _) => editor.d_range.draw_type = DrawType::All,
                        Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                            Char('w') => {
                                if prom.close(editor, mbar, help, sbar) == true {
                                    return true;
                                }
                            }
                            Char('s') => {
                                editor.save(mbar, prom, help, sbar);
                            }
                            Char('c') => editor.copy(),
                            Char('x') => editor.exec_edit_proc(EvtType::Cut, ""),
                            Char('v') => editor.exec_edit_proc(EvtType::Paste, ""),
                            Char('a') => editor.all_select(),
                            Char('f') => prom.search(editor, mbar, help, sbar),
                            Char('r') => prom.replace(editor, mbar, help, sbar),
                            Char('g') => prom.grep(editor, mbar, help, sbar),
                            Char('z') => editor.undo(mbar),
                            Char('y') => editor.redo(),
                            Char('l') => prom.move_row(editor, mbar, help, sbar),
                            Home => editor.ctrl_home(),
                            End => editor.ctrl_end(),
                            _ => mbar.set_err(&LANG.unsupported_operation),
                        },

                        Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                            Right => editor.shift_right(),
                            Left => editor.shift_left(),
                            Down => editor.shift_down(),
                            Up => editor.shift_up(),
                            Home => editor.shift_home(),
                            End => editor.shift_end(),
                            Char(c) => editor.exec_edit_proc(EvtType::InsertChar, &c.to_ascii_uppercase().to_string()),
                            F(1) => editor.record_key_start(mbar, prom),
                            F(2) => editor.exec_record_key(out, mbar, prom, help, sbar),
                            F(4) => editor.search_str(false, false),
                            _ => mbar.set_err(&LANG.unsupported_operation),
                        },
                        Key(KeyEvent { code, .. }) => match code {
                            Char(c) => editor.exec_edit_proc(EvtType::InsertChar, &c.to_string()),
                            Enter => editor.exec_edit_proc(EvtType::Enter, ""),
                            Backspace => editor.exec_edit_proc(EvtType::BS, ""),
                            Delete => editor.exec_edit_proc(EvtType::Del, ""),
                            PageDown => editor.page_down(),
                            PageUp => editor.page_up(),
                            Up => editor.cur_up(),
                            Down => editor.cur_down(),
                            Left => editor.cur_left(),
                            Right => editor.cur_right(),
                            Home => editor.home(),
                            End => editor.end(),
                            F(1) => help.disp_toggle(editor, mbar, prom, sbar),
                            F(3) => editor.search_str(true, false),
                            _ => mbar.set_err(&LANG.unsupported_operation),
                        },
                        Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => editor.cur_up(),
                        Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => editor.cur_down(),
                        Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => editor.ctrl_mouse((x + 1) as usize, y as usize, true),
                        Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: _, row: _, .. }) => return false,
                        Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => editor.ctrl_mouse((x + 1) as usize, y as usize, false),
                        _ => mbar.set_err(&LANG.unsupported_operation),
                    }

                    if prom.is_key_record {
                        editor.record_key();
                    }
                    EvtAct::finalize(editor);
                    editor.set_draw_range(editor.cur_y_org, offset_y_org, offset_x_org, rnw_org);
                }
                Log::ep("offset_y", &editor.offset_y);
                Log::ep("offset_x", &editor.offset_x);
                Log::ep("cur.y", &editor.cur.y);
                Log::ep("cur.x", &editor.cur.x);
                Log::ep("cur.disp_x", &editor.cur.disp_x);
                Log::ep("", &editor.sel);
                // Log::ep("", &editor.search);

                // Redraw in case of msg change
                if mbar.msg_org != mbar.msg {
                    editor.d_range.draw_type = DrawType::All;
                }
                // When key_record is executed, redraw only at the end
                if editor.d_range.draw_type != DrawType::Not || (prom.is_key_record_exec == false || prom.is_key_record_exec == true && prom.is_key_record_exec_draw == true) {
                    Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
                }
            }
        }
        Terminal::show_cur();
        return false;
    }

    pub fn init(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt) {
        Log::ep_s("init");

        // Initialize of updown_x
        match editor.evt {
            //  Down | Up | ShiftDown | ShiftUp
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up => {}
                _ => editor.updown_x = 0,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {}
                _ => editor.updown_x = 0,
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => editor.updown_x = 0,
        }
        // redraw判定
        editor.d_range.draw_type = DrawType::Not;
        match editor.evt {
            Resize(_, _) => editor.d_range.draw_type = DrawType::All,
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Home | End | Char('c') => editor.d_range.draw_type = DrawType::Not,
                _ => editor.d_range.draw_type = DrawType::All,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End | F(4) => {}
                _ => editor.d_range.draw_type = DrawType::All,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {
                    if editor.sel.is_selected() {
                        let sel = editor.sel.get_range();
                        editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                    } else {
                        editor.d_range.draw_type = DrawType::MoveCur;
                    };
                }
                F(1) => editor.d_range.draw_type = DrawType::All,
                F(3) => {
                    if editor.search.idx == USIZE_UNDEFINED {
                        editor.d_range.draw_type = DrawType::All;
                    }
                }
                _ => editor.d_range.draw_type = DrawType::All,
            },

            // for err msg or selected
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {
                if editor.sel.is_selected() {
                    editor.d_range.draw_type = DrawType::Target;
                }
            }
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) | Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {
                if editor.sel.is_selected() {
                    let sel = editor.sel.get_range();
                    editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                }
            }
            _ => editor.d_range.draw_type = DrawType::Not,
        }

        // Edit    is_change=true, Clear redo_vec,
        if editor.is_edit_evt(false) {
            prom.is_change = true;
            editor.history.clear_redo_vec();
        }

        // clear_redo_vec
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('x') | Char('v') => {
                    prom.is_change = true;
                    editor.history.clear_redo_vec();
                }
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(_) | Enter | Backspace | Delete => {
                    prom.is_change = true;
                    editor.history.clear_redo_vec();
                }
                _ => {}
            },
            _ => {}
        }
        // Msg clear  Other than cursor move
        match editor.evt {
            Resize(_, _) => {}
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => mbar.clear_mag(),
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => mbar.clear_mag(),
        }
    }

    pub fn finalize(editor: &mut Editor) {
        Log::ep_s("finalize");

        // set sel draw range, Clear sel range
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End => {
                    let sel = editor.sel.get_range();
                    editor.d_range = DRange::new(sel.sy, sel.ey, DrawType::Target);
                }
                F(4) => {}
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
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) => {
                Log::ep("editor.sel_org", &editor.sel_org);
                if editor.sel_org.is_selected() {
                    let sel_org = editor.sel_org.get_range();
                    editor.d_range = DRange::new(sel_org.sy, sel_org.ey, DrawType::Target);
                }
            }
            Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {
                if editor.sel.is_selected() {
                    let sy = editor.sel.get_diff_y_mouse_drag(editor.sel_org, editor.cur.y);
                    editor.d_range = DRange::new(sy, sy + 1, DrawType::Target);
                }
            }
            _ => editor.sel.clear(),
        }

        // Refresh search results
        if editor.is_edit_evt(true) && editor.search.ranges.len() > 0 {
            editor.search.ranges = editor.get_search_ranges(&editor.search.str, 0, editor.buf.len_chars(), 0);
        }
    }
    pub fn check_err(editor: &mut Editor, mbar: &mut MsgBar) -> bool {
        let is_return = false;

        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                // Check if sel range is set
                Char('x') | Char('c') => {
                    if !editor.sel.is_selected() {
                        mbar.set_err(&LANG.no_sel_range.to_string());
                        return true;
                    }
                }
                Char('y') => {
                    if editor.history.len_redo() == 0 {
                        mbar.set_err(&LANG.no_operation_re_exec.to_string());
                        return true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        // Check if sel range is set
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('v') => {
                    let clipboard = editor.get_clipboard().unwrap_or("".to_string());
                    if clipboard.len() == 0 {
                        mbar.set_err(&LANG.no_value_in_clipboard.to_string());
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
