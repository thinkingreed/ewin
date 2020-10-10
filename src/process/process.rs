use crate::model::{Editor, EvtProcess, MsgBar, Process, Prompt, StatusBar, Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent};
use std::io::Write;

impl Process {
    pub fn check_next_process<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        match editor.curt_evt {
            Resize(_, _) => {
                return EvtProcess::Next;
            }
            _ => {}
        }
        terminal.set_disp_size(editor, mbar, prom, sbar);

        if prom.is_save_new_file == true {
            return Process::save_new_filenm(out, terminal, editor, mbar, prom, sbar);
        } else if prom.is_save_confirm == true {
            return Process::close(out, terminal, editor, mbar, prom, sbar);
        } else if prom.is_search == true {
            return Process::search(out, terminal, editor, mbar, prom, sbar);
        } else {
            return EvtProcess::Next;
        }
    }

    pub fn init(editor: &mut Editor, prom: &mut Prompt) {
        // updown_xの初期化
        match editor.curt_evt {
            //  Down | Up | ShiftDown | ShiftUp
            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {}
                _ => editor.cur.updown_x = 0,
            },
            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
            _ => editor.cur.updown_x = 0,
        }
        // all_redraw判定
        editor.is_all_redraw = false;
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                Char('c') => {}
                _ => editor.is_all_redraw = true,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Down | Up | Left | Right => editor.is_all_redraw = true,
                _ => editor.is_all_redraw = true,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up | Left | Right => {
                    if !editor.sel.is_unselected() {
                        editor.is_all_redraw = true;
                    }
                }
                _ => editor.is_all_redraw = true,
            },
            Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
            _ => editor.is_all_redraw = true,
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
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                Char('x') => editor.sel.clear(),
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Backspace | Delete => editor.sel.clear(),
                _ => {}
            },
            _ => {}
        }
    }
}
