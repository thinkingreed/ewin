use crate::prompt::promptcont::promptcont::PromptBufPosi::*;
use crate::{global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, statusbar::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent as M_Event, MouseEventKind as M_EventKind};
use std::io::Write;

impl EvtAct {
    pub fn check_next_process<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Resize(_, _) => return EvtActType::Next,
            _ => {}
        }

        EvtAct::check_clear_mag(editor, mbar);
        let evt_act = EvtAct::check_prom(out, editor, mbar, prom, help, sbar);
        EvtAct::finalize_check_prom(editor, prom);

        if evt_act == EvtActType::Hold {
            if mbar.msg_org != mbar.msg {
                mbar.draw_only(out);
                prom.draw_cur_only(out);
            }
            prom.draw_only(out);
        }

        return evt_act;
    }

    pub fn check_prom<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        // Close・End
        if prom.is_save_new_file || prom.is_search || prom.is_close_confirm || prom.is_replace || prom.is_grep || prom.is_grep_result || prom.is_move_line {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('c') => {
                        if prom.is_grep_result && prom.is_grep_result_cancel == false {
                            prom.is_grep_result_cancel = true;
                        } else {
                            prom.clear();
                            mbar.clear();
                            editor.d_range.draw_type = DrawType::All;
                        }
                        return EvtActType::DrawOnly;
                    }
                    Char('w') => {
                        Terminal::init_draw(out, editor, mbar, prom, help, sbar);
                        return EvtActType::Next;
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
                            Right => match prom.buf_posi {
                                First => prom.cont_1.shift_right(),
                                Second => prom.cont_2.shift_right(),
                                Third => prom.cont_3.shift_right(),
                            },
                            Left => match prom.buf_posi {
                                First => prom.cont_1.shift_left(),
                                Second => prom.cont_2.shift_left(),
                                Third => prom.cont_3.shift_left(),
                            },
                            BackTab => {
                                prom.tab(false);
                                prom.clear_sels();
                            }
                            Home => match prom.buf_posi {
                                First => prom.cont_1.shift_home(),
                                Second => prom.cont_2.shift_home(),
                                Third => prom.cont_3.shift_home(),
                            },
                            End => match prom.buf_posi {
                                First => prom.cont_1.shift_end(),
                                Second => prom.cont_2.shift_end(),
                                Third => prom.cont_3.shift_end(),
                            },
                            Char(c) => {
                                match prom.buf_posi {
                                    First => prom.cont_1.insert_char(c.to_ascii_uppercase(), prom.is_move_line, editor),
                                    Second => prom.cont_2.insert_char(c.to_ascii_uppercase(), prom.is_move_line, editor),
                                    Third => prom.cont_3.insert_char(c.to_ascii_uppercase(), prom.is_move_line, editor),
                                }
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
                        let is_all_redrow = match prom.buf_posi {
                            First => prom.cont_1.paste(editor, mbar),
                            Second => prom.cont_2.paste(editor, mbar),
                            Third => prom.cont_3.paste(editor, mbar),
                        };
                        if is_all_redrow {
                            Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
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
                            Left | Right | Delete | Backspace | Home | End => match prom.buf_posi {
                                First => prom.cont_1.edit(code),
                                Second => prom.cont_2.edit(code),
                                Third => prom.cont_3.edit(code),
                            },
                            Up => prom.cursor_up(),
                            Down => prom.cursor_down(),
                            Tab => prom.tab(true),
                            Char(c) => match prom.buf_posi {
                                First => prom.cont_1.insert_char(c, prom.is_move_line, editor),
                                Second => prom.cont_2.insert_char(c, prom.is_move_line, editor),
                                Third => prom.cont_3.insert_char(c, prom.is_move_line, editor),
                            },
                            _ => {}
                        }
                        prom.clear_sels();
                        prom.draw_only(out);
                        evt_act_type = Some(EvtActType::Hold);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // incremental search
        if prom.is_search {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::ALT, .. }) => {}
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => EvtAct::exec_search_incremental(out, editor, mbar, prom, help, sbar),
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) | Key(KeyEvent { code, .. }) => match code {
                    Char(_) | Delete | Backspace => EvtAct::exec_search_incremental(out, editor, mbar, prom, help, sbar),
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
                        prom.cont_1.opt_1.toggle_check();
                        CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.case_sens = prom.cont_1.opt_1.is_check).unwrap();
                        return EvtActType::Hold;
                    }
                    Char('r') => {
                        prom.cont_1.opt_2.toggle_check();
                        CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.regex = prom.cont_1.opt_2.is_check).unwrap();
                        return EvtActType::Hold;
                    }
                    _ => return EvtActType::Hold,
                },
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
            return EvtAct::save_new_filenm(editor, mbar, prom, sbar);
        } else if prom.is_close_confirm == true {
            return EvtAct::close(editor, mbar, prom, sbar);
        } else if prom.is_search == true {
            return EvtAct::search(editor, mbar, prom);
        } else if prom.is_replace == true {
            return EvtAct::replace(editor, mbar, prom);
        } else if prom.is_grep == true {
            return EvtAct::grep(editor, mbar, prom);
        } else if prom.is_grep_result == true {
            return EvtAct::grep_result(editor);
        } else if prom.is_move_line == true {
            return EvtAct::move_row(out, editor, mbar, prom, help, sbar);
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

    pub fn finalize_check_prom(editor: &mut Editor, prom: &mut Prompt) {
        Log::ep_s("finalize_check_prom");

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
