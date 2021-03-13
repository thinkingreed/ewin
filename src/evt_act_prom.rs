use crate::{bar::headerbar::*, bar::msgbar::*, bar::statusbar::*, help::*, log::*, model::*, prompt::prompt::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_EventKind, MouseEventKind as M_Kind};
use std::io::Write;

impl EvtAct {
    pub fn check_prom<T: Write>(out: &mut T, hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        // Close・End
        if prom.is_save_new_file || prom.is_search || prom.is_close_confirm || prom.is_replace || prom.is_grep || prom.is_grep_result || prom.is_move_line {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('w') => {
                        Terminal::init_draw(out, hbar, editor, mbar, prom, help, sbar);
                        return EvtActType::Next;
                    }
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Esc => {
                        if prom.is_grep_result && prom.is_grep_result_cancel == false {
                            prom.is_grep_result_cancel = true;
                        } else {
                            prom.clear();
                            mbar.clear();
                            editor.d_range.draw_type = DrawType::All;
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
        if prom.is_save_new_file || prom.is_search || prom.is_replace || prom.is_grep || prom.is_move_line {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    Left | Right | BackTab | Home | End | Char(_) => {
                        match code {
                            Right => prom.shift_right(),
                            Left => prom.shift_left(),
                            Home => prom.shift_home(),
                            End => prom.shift_end(),
                            BackTab => prom.tab(false),
                            Char(c) => {
                                prom.insert_char(c.to_ascii_uppercase(), prom.is_move_line, editor);
                                prom.clear_sels();
                            }
                            _ => {}
                        }
                        prom.draw_only(out);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => {
                        let is_all_redrow = prom.paste(editor, mbar);
                        if is_all_redrow {
                            Terminal::draw(out, hbar, editor, mbar, prom, help, sbar).unwrap();
                        } else {
                            prom.clear_sels();
                            prom.draw_only(out);
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
                            Left | Right | Delete | Backspace | Home | End => prom.operation(code),
                            Up => prom.cursor_up(),
                            Down => prom.cursor_down(),
                            Tab => prom.tab(true),
                            Char(c) => prom.insert_char(c, prom.is_move_line, editor),
                            _ => {}
                        }
                        prom.clear_sels();
                        prom.draw_only(out);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => prom.ctrl_mouse(x, y, true, out),
                Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: _, row: _, .. }) => {}
                Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => prom.ctrl_mouse(x, y, false, out),
                _ => {}
            }
        }
        // incremental search
        if prom.is_search {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, .. }) => {}
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => EvtAct::exec_search_incremental(out, hbar, editor, mbar, prom, help, sbar),
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) | Key(KeyEvent { code, .. }) => match code {
                    Char(_) | Delete | Backspace => EvtAct::exec_search_incremental(out, hbar, editor, mbar, prom, help, sbar),
                    _ => {}
                },
                _ => {}
            }
        }
        // Search・replace・grep option
        if prom.is_search || prom.is_replace || prom.is_grep {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                    Char('c') => {
                        prom.cont_1.change_opt_case_sens();
                        return EvtActType::Hold;
                    }
                    Char('r') => {
                        prom.cont_1.change_opt_regex();
                        return EvtActType::Hold;
                    }
                    _ => return EvtActType::Hold,
                },
                Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                    if prom.cont_1.opt_row_posi == y {
                        if prom.cont_1.opt_1.mouse_area.0 <= x && x <= prom.cont_1.opt_1.mouse_area.1 {
                            prom.cont_1.change_opt_case_sens();
                        } else if prom.cont_1.opt_2.mouse_area.0 <= x && x <= prom.cont_1.opt_2.mouse_area.1 {
                            prom.cont_1.change_opt_regex();
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
        if prom.is_grep_result == true || mbar.msg_readonly.len() > 0 {
            match editor.evt {
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
                        if !prom.is_grep_result {
                            return EvtActType::Hold;
                        }
                    }
                    F(3) => {}
                    _ => {
                        if !prom.is_close_confirm == true {
                            return EvtActType::Hold;
                        }
                    }
                },
                Mouse(M_Event { kind: M_EventKind::ScrollUp, .. }) => return EvtActType::Next,
                Mouse(M_Event { kind: M_EventKind::ScrollDown, .. }) => return EvtActType::Next,
                _ => return EvtActType::Hold,
            }
        }

        if prom.is_save_new_file == true {
            return EvtAct::save_new_filenm(hbar, editor, mbar, prom, help, sbar);
        } else if prom.is_close_confirm == true {
            return EvtAct::close(hbar, editor, mbar, prom, help, sbar);
        } else if prom.is_search == true {
            return EvtAct::search(editor, mbar, prom);
        } else if prom.is_replace == true {
            return EvtAct::replace(editor, mbar, prom);
        } else if prom.is_grep == true {
            return EvtAct::grep(editor, mbar, prom);
        } else if prom.is_grep_result == true {
            return EvtAct::grep_result(editor);
        } else if prom.is_move_line == true {
            return EvtAct::move_row(out, hbar, editor, mbar, prom, help, sbar);
        } else {
            Log::ep_s("EvtProcess::NextEvtProcess");
            return EvtActType::Next;
        }
    }

    pub fn check_clear_mag(editor: &mut Editor, mbar: &mut MsgBar) {
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => mbar.clear_mag(),
            },

            Mouse(M_Event { kind: M_EventKind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_EventKind::ScrollDown, .. }) => {}
            _ => mbar.clear_mag(),
        }
    }

    pub fn check_grep_clear_tab_comp(editor: &mut Editor, prom: &mut Prompt) {
        Log::ep_s("check_grep_clear_tab_comp");

        if prom.is_grep {
            // Check clear tab candidate
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    Left | Right | Home | End => prom.clear_tab_comp(),
                    _ => {}
                },
                Key(KeyEvent { code, .. }) => match code {
                    Char(_) | Left | Right | Home | End | Backspace | Delete => prom.clear_tab_comp(),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
