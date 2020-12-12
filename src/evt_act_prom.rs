use crate::model::PromptBufPosi::*;
use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent};
use std::io::Write;

impl EvtAct {
    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Resize(_, _) => return EvtActType::Next,
            _ => {}
        }
        term.set_disp_size(editor, mbar, prom, sbar);

        EvtAct::init_check_prom(editor, mbar);

        let evt_act = EvtAct::check_prom(out, term, editor, mbar, prom, sbar);

        EvtAct::finalize_check_prom(editor, prom);

        if evt_act == EvtActType::Hold && mbar.msg_org != mbar.msg {
            term.draw(out, editor, mbar, prom, sbar).unwrap();
        }

        return evt_act;
    }

    pub fn check_prom<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        if prom.is_save_new_file == true || prom.is_search == true || prom.is_close_confirm == true || prom.is_replace == true || prom.is_grep == true || prom.is_grep_result == true {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('c') => {
                        if prom.is_grep_result && prom.is_grep_result_cancel == false {
                            prom.is_grep_result_cancel = true;
                        } else {
                            prom.clear();
                            mbar.clear();
                            term.draw(out, editor, mbar, prom, sbar).unwrap();
                        }
                        return EvtActType::Hold;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // edit
        if prom.is_save_new_file == true || prom.is_search == true || prom.is_replace == true || prom.is_grep == true {
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
                                    First => prom.cont_1.insert_char(c.to_ascii_uppercase()),
                                    Second => prom.cont_2.insert_char(c.to_ascii_uppercase()),
                                    Third => prom.cont_3.insert_char(c.to_ascii_uppercase()),
                                }
                                prom.clear_sels();
                            }
                            _ => {}
                        }
                        prom.draw_only(out);
                        return EvtActType::Hold;
                    }
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('v') => {
                        let mut is_all_redrow = false;
                        match prom.buf_posi {
                            First => is_all_redrow = prom.cont_1.paste(&term, editor, mbar),
                            Second => is_all_redrow = prom.cont_2.paste(&term, editor, mbar),
                            Third => is_all_redrow = prom.cont_3.paste(&term, editor, mbar),
                        }
                        if is_all_redrow {
                            term.draw(out, editor, mbar, prom, sbar).unwrap();
                        } else {
                            prom.clear_sels();
                            prom.draw_only(out);
                        }
                        return EvtActType::Hold;
                    }
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
                                First => prom.cont_1.insert_char(c),
                                Second => prom.cont_2.insert_char(c),
                                Third => prom.cont_3.insert_char(c),
                            },
                            _ => {}
                        }
                        prom.clear_sels();
                        prom.draw_only(out);
                        return EvtActType::Hold;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // unable to edit
        if prom.is_grep_result == true || mbar.msg_readonly.len() > 0 {
            match editor.evt {
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    F(4) | Right | Left | Down | Up | Home | End => {
                        return EvtActType::Next;
                    }
                    _ => return EvtActType::Hold,
                },
                Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                    Char('w') | Char('s') | Char('c') | Char('a') | Char('f') | Home | End => {
                        return EvtActType::Next;
                    }
                    _ => return EvtActType::Hold,
                },
                Key(KeyEvent { code, .. }) => match code {
                    PageDown | PageUp | Home | End | Down | Up | Left | Right => {
                        return EvtActType::Next;
                    }
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
                Mouse(MouseEvent::ScrollUp(_, _, _)) => return EvtActType::Next,
                Mouse(MouseEvent::ScrollDown(_, _, _)) => return EvtActType::Next,

                _ => return EvtActType::Hold,
            }
        }

        if prom.is_save_new_file == true {
            return EvtAct::save_new_filenm(out, term, editor, mbar, prom, sbar);
        } else if prom.is_close_confirm == true {
            return EvtAct::close(out, term, editor, mbar, prom, sbar);
        } else if prom.is_search == true {
            return EvtAct::search(out, term, editor, mbar, prom, sbar);
        } else if prom.is_replace == true {
            return EvtAct::replace(out, term, editor, mbar, prom, sbar);
        } else if prom.is_grep == true {
            return EvtAct::grep(out, term, editor, mbar, prom, sbar);
        } else if prom.is_grep_result == true {
            return EvtAct::grep_result(term, editor, mbar);
        } else {
            Log::ep_s("EvtProcess::NextEvtProcess");
            return EvtActType::Next;
        }
    }

    pub fn init_check_prom(editor: &mut Editor, mbar: &mut MsgBar) {
        Log::ep_s("init_check_prom");

        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, .. }) => mbar.clear_mag(),
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, .. }) => mbar.clear_mag(),
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End => {}
                _ => mbar.clear_mag(),
            },

            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
            _ => mbar.clear_mag(),
        }
    }

    pub fn finalize_check_prom(editor: &mut Editor, prom: &mut Prompt) {
        Log::ep_s("finalize_check_prom");

        if prom.is_grep {
            // 選択範囲クリア判定
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
