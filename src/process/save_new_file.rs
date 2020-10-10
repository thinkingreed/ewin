use crate::model::{Editor, Log, Process, Prompt, StatusBar, Terminal};
use crate::process::process::EvtProcess;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use termion::cursor;
use unicode_width::UnicodeWidthChar;

impl Process {
    pub fn save_new_filenm<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                Char('c') => {
                    prom.clear();
                    terminal.draw(out, editor, prom, sbar).unwrap();
                    return EvtProcess::Next;
                }
                _ => return EvtProcess::Hold,
            },
            Key(KeyEvent { code: Char(c), .. }) => {
                prom.cont.buf.insert(prom.cont.cur.x, c);
                prom.cont.cur.disp_x += c.width().unwrap_or(0);
                prom.cont.cur.x += 1;
                let mut v: Vec<String> = vec![];
                prom.draw(&mut v).unwrap();
                write!(out, "{}{}", v.concat(), cursor::Goto(prom.cont.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                out.flush().unwrap();
                return EvtProcess::Hold;
            }
            Key(KeyEvent { code, .. }) => match code {
                Left => {
                    if prom.cont.cur.x != 0 {
                        prom.cont.cur.x -= 1;
                        prom.cont.cur.disp_x -= prom.cont.buf[prom.cont.cur.x].width().unwrap_or(0);
                        write!(out, "{}", cursor::Goto(prom.cont.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                        out.flush().unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Right => {
                    if prom.cont.cur.x < prom.cont.buf.len() {
                        prom.cont.cur.x += 1;
                        if prom.cont.cur.x == prom.cont.buf.len() {
                            prom.cont.cur.disp_x += 1;
                        } else {
                            prom.cont.cur.disp_x += prom.cont.buf[prom.cont.cur.x].width().unwrap_or(0);
                        }

                        write!(out, "{}", cursor::Goto(prom.cont.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                        out.flush().unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Delete => {
                    Log::ep_s("★　Delete");
                    if prom.cont.cur.x < prom.cont.buf.len() {
                        prom.cont.buf.remove(prom.cont.cur.x);
                        let mut v: Vec<String> = vec![];
                        prom.draw(&mut v).unwrap();
                        write!(out, "{}{}", v.concat(), cursor::Goto(prom.cont.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                        out.flush().unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Backspace => {
                    Log::ep_s("★　Backspace");
                    if prom.cont.cur.x > 0 {
                        prom.cont.cur.x -= 1;
                        prom.cont.cur.disp_x -= prom.cont.buf[prom.cont.cur.x].width().unwrap_or(0);
                        prom.cont.buf.remove(prom.cont.cur.x);
                        let mut v: Vec<String> = vec![];
                        prom.draw(&mut v).unwrap();
                        write!(out, "{}{}", v.concat(), cursor::Goto(prom.cont.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                        out.flush().unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Enter => {
                    if prom.cont.buf.len() == 0 {
                    } else {
                        // TODO 存在するファイル名の対応
                        sbar.filenm = prom.cont.buf.iter().collect::<String>();
                        editor.save(prom, sbar);
                        terminal.draw(out, editor, prom, sbar).unwrap();
                    }
                    return EvtProcess::Hold;
                }
                _ => return EvtProcess::Hold,
            },
            _ => return EvtProcess::Hold,
        }
    }
}
