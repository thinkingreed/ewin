use crate::model::*;
use crate::{editor, global::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use std::io::Write;
use termion::clear;

impl EvtAct {
    pub fn match_event<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> bool {
        term.hide_cur(out);

        mbar.msg_org = mbar.msg.clone();

        let evt_next_process = EvtAct::check_next_process(out, term, editor, mbar, prom, sbar);

        match evt_next_process {
            EvtActType::Exit => return true,
            EvtActType::Hold => {}
            EvtActType::Next => {
                EvtAct::init(editor, mbar, prom);

                // eprintln!("editor.evt.clone(){:?}", editor.evt.clone());

                let offset_y_org = editor.offset_y;

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
                        Char('c') => editor.copy(&term, mbar),
                        Char('x') => editor.cut(&term, mbar),
                        Char('v') => editor.paste(&term),
                        Char('a') => editor.all_select(),
                        Char('f') => editor.search(prom),
                        Char('r') => editor.replace_prom(prom),
                        Char('g') => editor.grep_prom(prom),
                        Char('z') => editor.undo(mbar),
                        Char('y') => editor.redo(&term, mbar),
                        Home => editor.ctrl_home(),
                        End => editor.ctrl_end(),
                        _ => mbar.set_err(&LANG.lock().unwrap().unsupported_operation),
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
                        F(4) => editor.search_str(false),
                        _ => mbar.set_err(&LANG.lock().unwrap().unsupported_operation),
                    },
                    Key(KeyEvent { code, .. }) => match code {
                        Char(c) => editor.insert_char(c),
                        Enter => editor.enter(),
                        Backspace => editor.back_space(),
                        Delete => editor.delete(),
                        PageDown => editor.page_down(),
                        PageUp => editor.page_up(),
                        Up => editor.cur_up(),
                        Down => editor.cur_down(),
                        Left => editor.cur_left(),
                        Right => editor.cur_right(),
                        Home => editor.home(),
                        End => editor.end(),
                        F(3) => editor.search_str(true),
                        _ => mbar.set_err(&LANG.lock().unwrap().unsupported_operation),
                    },
                    Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => editor.cur_up(),
                    Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => editor.cur_down(),
                    Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => editor.ctrl_mouse((x + 1) as usize, y as usize, true),
                    Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), column: x, row: y, .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), column: x, row: y, .. }) => editor.ctrl_mouse((x + 1) as usize, y as usize, false),
                    _ => mbar.set_err(&LANG.lock().unwrap().unsupported_operation),
                }

                if prom.is_key_record {
                    editor.record_key();
                }
                EvtAct::finalize(editor);

                if offset_y_org != editor.offset_y {
                    editor.d_range.d_type = DType::All;
                }

                // key_record実行時は最終時のみredraw
                if editor.d_range.d_type != DType::Not || (prom.is_key_record_exec == false || prom.is_key_record_exec == true && prom.is_key_record_exec_draw == true) {
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                }
            }
        }
        term.show_cur(out);
        return false;
    }

    pub fn init(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt) {
        Log::ep_s("init");

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
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => {}
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => {}
            _ => editor.updown_x = 0,
        }
        // redraw判定
        match editor.evt {
            Resize(_, _) => editor.d_range.d_type = DType::All,
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Home | End => editor.d_range.d_type = DType::Not,
                _ => editor.d_range.d_type = DType::All,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right | Home | End => editor.d_range.d_type = DType::All,
                F(4) => editor.d_range.d_type = DType::Not,
                _ => editor.d_range.d_type = DType::All,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right | Home | End | F(3) => {
                    if editor.sel.is_selected() {
                        editor.d_range.d_type = DType::All;
                    } else {
                        editor.d_range.d_type = DType::Not;
                    }
                }
                _ => editor.d_range.d_type = DType::All,
            },

            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) | Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => editor.d_range.d_type = DType::Not,

            // for err msg or selected
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {
                if editor.sel.is_selected() {
                    editor.d_range.d_type = DType::All;
                }
            }
            _ => editor.d_range.d_type = DType::Not,
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
        // Msg clear  Other than cursor move
        match editor.evt {
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
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) | Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) | Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), .. }) | Mouse(M_Event { kind: M_Kind::Drag(M_Btn::Left), .. }) => {}
            _ => editor.sel.clear(),
        }
    }
}
