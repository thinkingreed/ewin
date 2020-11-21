use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent};
use std::io::Write;

impl EvtAct {
    pub fn check_next_process<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.curt_evt {
            Resize(_, _) => return EvtActType::Next,

            _ => {}
        }
        terminal.set_disp_size(editor, mbar, prom, sbar);

        return prom.check_prom(out, terminal, editor, mbar, sbar);
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

    /*
    pub fn finalize(editor: &mut Editor) {
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                _ => {}
            },
            _ => {}
        }
    }
    */
}
