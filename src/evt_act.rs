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

                match editor.curt_evt {
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
                        F(1) => editor.record_macro_start(term, mbar, prom, sbar),
                        F(2) => editor.exec_macro(out, term, mbar, prom, sbar),
                        F(4) => editor.move_cursor(out, sbar),
                        _ => {}
                    },
                    // Key(KeyEvent { code: Char(c), .. }) => editor.insert_char(c),
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

                if prom.is_record_macro {
                    editor.record_macro();
                }

                EvtAct::finalize(editor);

                if editor.is_redraw == true {
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                }
            }
        }
        term.show_cur(out);
        return false;
    }

    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.curt_evt {
            Resize(_, _) => return EvtActType::Next,

            _ => {}
        }
        term.set_disp_size(editor, mbar, prom, sbar);

        return prom.check_prom(out, term, editor, mbar, sbar);
    }

    pub fn init(editor: &mut Editor, prom: &mut Prompt) {
        // updown_xの初期化
        match editor.curt_evt {
            //  Down | Up | ShiftDown | ShiftUp
            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {}
                _ => editor.updown_x = 0,
            },
            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
            _ => editor.updown_x = 0,
        }
        // all_redraw判定
        editor.is_redraw = false;
        match editor.curt_evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('c') => {}
                _ => editor.is_redraw = true,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right => editor.is_redraw = true,
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
            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
            _ => editor.is_redraw = true,
        }
        if editor.is_redraw {
            editor.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
        }

        // 選択範囲クリア判定
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::SHIFT }) => match code {
                Down | Up | Left | Right => {}
                _ => editor.sel.clear(),
            },
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                Char('a') | Char('c') | Char('x') => {}
                _ => editor.sel.clear(),
            },
            Key(KeyEvent { code, .. }) => match code {
                Backspace | Delete => {}
                _ => editor.sel.clear(),
            },
            Mouse(MouseEvent::Down(_, _, _, _)) | Mouse(MouseEvent::Up(_, _, _, _)) | Mouse(MouseEvent::Drag(_, _, _, _)) => {}
            _ => editor.sel.clear(),
        }
        // is_change判定
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => {
                if code == Char('x') || code == Char('v') {
                    prom.is_change = true;
                }
            }
            Key(KeyEvent { code: Char(_), .. }) => {
                prom.is_change = true;
            }
            Key(KeyEvent { code, .. }) => {
                if code == Enter || code == Backspace || code == Delete {
                    prom.is_change = true;
                }
            }
            _ => {}
        }
    }

    pub fn finalize(editor: &mut Editor) {
        // 検索後に検索対象文字の変更対応で、再検索
        if editor.search.str.len() > 0 {
            editor.search.search_ranges = editor.get_search_ranges(editor.search.str.clone());
        }
    }
}
