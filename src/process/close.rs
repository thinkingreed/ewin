use crate::model::{Editor, Process, Prompt, StatusBar, Terminal};
use crate::process::process::EvtProcess;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;

impl Process {
    pub fn close<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, prompt: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                Char('c') => {
                    prompt.clear();
                    terminal.draw(out, editor, prompt, sbar).unwrap();

                    return EvtProcess::Next;
                }
                _ => return EvtProcess::Hold,
            },
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    editor.save(prompt);
                    return EvtProcess::Exit;
                } else if c == 'n' {
                    return EvtProcess::Exit;
                } else {
                    return EvtProcess::Hold;
                }
            }
            _ => return EvtProcess::Hold,
        }
    }
}
