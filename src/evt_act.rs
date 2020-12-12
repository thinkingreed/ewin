use crate::model::PromptBufPosi::*;
use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use std::io::Write;
use termion::clear;

impl EvtAct {
    pub fn match_event<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> bool {
        term.hide_cur(out);

        let evt_next_process = EvtAct::check_next_process(out, term, editor, mbar, prom, sbar);

        match evt_next_process {
            EvtActType::Exit => return true,
            EvtActType::Hold => {}
            EvtActType::Next => {
                EvtAct::init(editor, prom);

                // eprintln!("editor.curt_evt.clone(){:?}", editor.curt_evt);

                match editor.evt {
                    Resize(_, _) => {
                        write!(out, "{}", clear::All.to_string()).unwrap();
                        term.set_disp_size(editor, mbar, prom, sbar);
                    }

                    Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                        Char('w') => {
                            if editor.close(out, prom) == true {
                                return true;
                            }
                        }
                        Char('s') => {
                            editor.save(mbar, prom, sbar);
                        }
                        Char('c') => editor.copy(&term),
                        Char('x') => editor.cut(&term),
                        Char('v') => editor.paste(&term),
                        Char('a') => editor.all_select(),
                        Char('f') => editor.search(prom),
                        Char('r') => editor.replace_prom(prom),
                        Char('g') => editor.grep_prom(prom),
                        Char('z') => editor.undo(),
                        Char('y') => editor.redo(&term),
                        Home => editor.move_cursor(out, sbar),
                        End => editor.move_cursor(out, sbar),
                        _ => {}
                    },

                    Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                        Right => editor.shift_right(),
                        Left => editor.shift_left(),
                        Down => editor.shift_down(),
                        Up => editor.shift_up(),
                        Home => editor.shift_home(),
                        End => editor.shift_end(),
                        Char(c) => editor.insert_char(c.to_ascii_uppercase()),
                        F(1) => editor.record_key_start(term, mbar, prom, sbar),
                        F(2) => editor.exec_record_key(out, term, mbar, prom, sbar),
                        F(4) => editor.move_cursor(out, sbar),
                        _ => {}
                    },
                    Key(KeyEvent { code, .. }) => match code {
                        Char(c) => editor.insert_char(c),
                        Enter => editor.enter(),
                        Backspace => editor.back_space(),
                        Delete => editor.delete(),
                        PageDown => editor.page_down(),
                        PageUp => editor.page_up(),
                        Home => editor.move_cursor(out, sbar),
                        End => editor.move_cursor(out, sbar),
                        Down => editor.move_cursor(out, sbar),
                        Up => editor.move_cursor(out, sbar),
                        Left => editor.move_cursor(out, sbar),
                        Right => editor.move_cursor(out, sbar),
                        F(3) => editor.move_cursor(out, sbar),

                        _ => {
                            Log::ep_s("Un Supported no modifiers");
                        }
                    },
                    Mouse(MouseEvent::ScrollUp(_, _, _)) => editor.move_cursor(out, sbar),
                    Mouse(MouseEvent::ScrollDown(_, _, _)) => editor.move_cursor(out, sbar),
                    Mouse(MouseEvent::Down(MouseButton::Left, x, y, _)) => editor.mouse_left_press((x + 1) as usize, y as usize),
                    Mouse(MouseEvent::Down(_, _, _, _)) => {}
                    Mouse(MouseEvent::Up(_, x, y, _)) => editor.mouse_release((x + 1) as usize, y as usize),
                    Mouse(MouseEvent::Drag(_, x, y, _)) => editor.mouse_hold((x + 1) as usize, y as usize),
                }

                if prom.is_key_record {
                    editor.record_key();
                }

                EvtAct::finalize(editor);

                // key_record実行時は最終時のみredraw
                if editor.is_redraw == true && prom.is_key_record_exec == false || prom.is_key_record_exec == true && prom.is_key_record_exec_draw == true {
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                }
            }
        }
        term.show_cur(out);
        return false;
    }

    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Resize(_, _) => return EvtActType::Next,
            _ => {}
        }
        term.set_disp_size(editor, mbar, prom, sbar);

        let evt_act = EvtAct::check_prom(out, term, editor, mbar, prom, sbar);

        EvtAct::finalize_check_prom(prom, editor);

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

    pub fn init(editor: &mut Editor, prom: &mut Prompt) {
        // updown_xの初期化
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
            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
            _ => editor.updown_x = 0,
        }
        // all_redraw判定
        editor.is_redraw = false;
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('c') => {}
                _ => editor.is_redraw = true,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End => editor.is_redraw = true,
                F(4) => editor.is_redraw = false,
                _ => editor.is_redraw = true,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End | F(3) => {
                    if editor.sel.is_selected() {
                        editor.is_redraw = true;
                    }
                }
                _ => editor.is_redraw = true,
            },

            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) | Mouse(MouseEvent::Down(MouseButton::Left, _, _, _)) | Mouse(MouseEvent::Up(_, _, _, _)) => {}
            _ => editor.is_redraw = true,
        }

        if editor.is_redraw {
            editor.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
        }

        // Edit    is_change=true, Clear redo_vec,
        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('x') | Char('v') => {
                    prom.is_change = true;
                    editor.redo_vec.clear();
                }
                _ => {}
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Char(_) => {
                    prom.is_change = true;
                    editor.redo_vec.clear();
                }
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(_) | Enter | Backspace | Delete => {
                    prom.is_change = true;
                    editor.redo_vec.clear();
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn finalize(editor: &mut Editor) {
        Log::ep_s("finalize");

        // 選択範囲クリア判定
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
            Mouse(MouseEvent::Down(_, _, _, _)) | Mouse(MouseEvent::Up(_, _, _, _)) | Mouse(MouseEvent::Drag(_, _, _, _)) => {}
            _ => editor.sel.clear(),
        }

        // 検索後に検索対象文字の変更対応で、再検索
        if editor.search.str.len() > 0 {
            editor.search.search_ranges = editor.get_search_ranges(editor.search.str.clone());
        }
    }
    pub fn finalize_check_prom(prom: &mut Prompt, editor: &mut Editor) {
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
