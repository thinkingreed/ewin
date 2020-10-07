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
                // 初期

                prom.cont.buf.insert(prom.cur.x, c);
                prom.cur.disp_x += c.width().unwrap_or(0);
                prom.cur.x += 1;
                write!(out, "{}", cursor::Goto(prom.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                out.flush().unwrap();
                //
                //
                // 部分再描画
                //
                //
                terminal.draw(out, editor, prom, sbar).unwrap();

                return EvtProcess::Hold;
            }
            Key(KeyEvent { code, .. }) => match code {
                Left => {
                    Log::ep("prom.cont.buf", prom.cont.buf.iter().collect::<String>());
                    Log::ep("prom.cur.x", prom.cur.x);
                    Log::ep("prom.cur.disp_x", prom.cur.disp_x);

                    if prom.cur.x != 0 {
                        prom.cur.x -= 1;
                        Log::ep("prom.cont.buf[prom.cur.x]", prom.cont.buf[prom.cur.x].to_string());
                        Log::ep("prom.cont.buf[prom.cur.x].width()", prom.cont.buf[prom.cur.x].width().unwrap_or(0));
                        prom.cur.disp_x -= prom.cont.buf[prom.cur.x].width().unwrap_or(0);
                        Log::ep("prom.cur.disp_x", prom.cur.disp_x);
                        write!(out, "{}", cursor::Goto(prom.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                        out.flush().unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Right => {
                    if prom.cur.x < prom.cont.buf.len() {
                        prom.cur.x += 1;

                        if prom.cur.x == prom.cont.buf.len() {
                            prom.cur.disp_x += 1;
                        } else {
                            prom.cur.disp_x += prom.cont.buf[prom.cur.x].width().unwrap_or(0);
                        }

                        write!(out, "{}", cursor::Goto(prom.cur.disp_x as u16, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                        out.flush().unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Backspace => {
                    if prom.cont.buf.len() > 0 {
                        //    prompt.cont.input = c.to_string();
                        write!(out, "{}", cursor::Goto(1, (prom.disp_row_posi + prom.disp_row_num - 1) as u16)).unwrap();
                    }
                    return EvtProcess::Hold;
                }
                Enter => {
                    /*
                    // 文字列チェック
                    prompt.clear();
                    terminal.draw(out, editor, prompt, sbar).unwrap();
                    */
                    return EvtProcess::Hold;
                }
                _ => return EvtProcess::Hold,
            },
            _ => return EvtProcess::Hold,
        }
    }
}
