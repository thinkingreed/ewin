use crossterm::event::{read, KeyCode::*, KeyModifiers};
use crossterm::event::{Event::*, KeyEvent, MouseButton, MouseEvent};
use ewin::_cfg::args::ARGS;
use ewin::_cfg::lang::cfg::LangCfg;
use ewin::model::{Editor, Log, Prompt, StatusBar, Terminal};
use path::Path;
use std::io::Write;
use std::io::{stdout, BufWriter};
use std::path;
use termion::clear;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let file_path: &str = ARGS.get("file_path").unwrap();
    let mut editor = Editor::default();

    let lang_cfg = LangCfg::read_lang_cfg();

    // ターミナルサイズが小さい場合に処理終了
    if !Terminal::check_displayable(&lang_cfg) {
        return;
    }

    editor.open(Path::new(file_path));

    let mut disp_filenm = file_path;
    if file_path.len() == 0 {
        disp_filenm = &lang_cfg.new_file;
    }

    let mut sbar = StatusBar::new(disp_filenm, lang_cfg.clone());
    let mut prompt = Prompt::new(lang_cfg.clone());
    Terminal::set_disp_size(&mut editor, &mut prompt, &mut sbar);

    let mut terminal = Terminal::default();

    let stdout = MouseTerminal::from(AlternateScreen::from(stdout()).into_raw_mode().unwrap());
    let mut out = BufWriter::new(stdout.lock());
    terminal.draw(&mut out, &mut editor, &mut prompt, &mut sbar).unwrap();

    loop {
        let event = read();

        editor.curt_evt = event.unwrap().clone();
        let evt_next_process = check_next_process(&mut out, &mut terminal, &mut editor, &mut prompt, &mut sbar);

        match evt_next_process {
            EvtProcess::Exit => return,
            EvtProcess::Hold => {}
            EvtProcess::Next => {
                init(&mut editor, &mut prompt);

                match editor.curt_evt {
                    Resize(_, _) => {
                        write!(out, "{}", clear::All.to_string()).unwrap();
                        Terminal::set_disp_size(&mut editor, &mut prompt, &mut sbar);
                    }
                    Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                        Char('w') => {
                            if editor.close(&mut prompt) == true {
                                return;
                            }
                        }
                        Char('s') => {
                            editor.save();
                            prompt.is_change = false;
                        }
                        Char('c') => {
                            editor.copy();
                        }
                        Char('x') => {
                            editor.cut();
                        }
                        Char('v') => {
                            editor.paste();
                        }
                        Char('a') => {
                            editor.all_select();
                        }
                        Home => {
                            editor.ctl_home();
                        }
                        End => {
                            editor.ctl_end();
                        }
                        _ => {
                            //    return;
                            // Log::ep_s("Un Supported");
                        }
                    },
                    Key(KeyEvent { code, modifiers: KeyModifiers::SHIFT }) => match code {
                        Right => {
                            editor.shift_right();
                        }
                        Left => {
                            editor.shift_left();
                        }
                        Down => {
                            editor.shift_down();
                        }
                        Up => {
                            editor.shift_up();
                        }
                        Home => {
                            editor.shift_home();
                        }
                        End => {
                            editor.shift_end();
                        }
                        Char(c) => {
                            editor.insert_char(c.to_ascii_uppercase());
                        }
                        _ => {}
                    },
                    Key(KeyEvent { code: Char(c), .. }) => {
                        editor.insert_char(c);
                    }
                    Key(KeyEvent { code, .. }) => match code {
                        Enter => {
                            editor.enter();
                        }
                        Backspace => {
                            editor.back_space();
                        }
                        Delete => {
                            editor.delete();
                        }
                        Home => {
                            editor.home();
                        }
                        End => {
                            editor.end();
                        }
                        PageDown => {
                            editor.page_down();
                        }
                        PageUp => {
                            editor.page_up();
                        }
                        Down => {
                            editor.move_cursor(Down);
                            editor.draw_cursor(&mut out, &mut sbar).unwrap();
                        }
                        Up => {
                            editor.move_cursor(Up);
                            editor.draw_cursor(&mut out, &mut sbar).unwrap();
                        }
                        Left => {
                            editor.move_cursor(Left);
                            editor.draw_cursor(&mut out, &mut sbar).unwrap();
                        }
                        Right => {
                            editor.move_cursor(Right);
                            editor.draw_cursor(&mut out, &mut sbar).unwrap();
                        }
                        _ => {
                            Log::ep_s("Un Supported no modifiers");
                        }
                    },

                    Mouse(MouseEvent::ScrollUp(_, _, _)) => {
                        editor.move_cursor(Up);
                        editor.draw_cursor(&mut out, &mut sbar).unwrap();
                    }
                    Mouse(MouseEvent::ScrollDown(_, _, _)) => {
                        editor.move_cursor(Down);
                        editor.draw_cursor(&mut out, &mut sbar).unwrap();
                    }
                    Mouse(MouseEvent::Down(MouseButton::Left, x, y, _)) => {
                        editor.mouse_left_press((x + 1) as usize, y as usize);
                    }
                    Mouse(MouseEvent::Down(_, _, _, _)) => {}
                    Mouse(MouseEvent::Up(_, x, y, _)) => {
                        editor.mouse_release((x + 1) as usize, y as usize);
                    }
                    Mouse(MouseEvent::Drag(_, x, y, _)) => {
                        editor.mouse_hold((x + 1) as usize, y as usize);
                    }
                }
                finalize(&mut editor);
                if editor.is_all_redraw == true {
                    terminal.draw(&mut out, &mut editor, &mut prompt, &mut sbar).unwrap();
                }
            }
        }
    }
}

fn init(editor: &mut Editor, prompt: &mut Prompt) {
    // updown_xの初期化
    match editor.curt_evt {
        //  Down | Up | ShiftDown | ShiftUp
        Key(KeyEvent { code, .. }) => match code {
            Down | Up => {}
            _ => {
                editor.cur.updown_x = 0;
            }
        },
        Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
        _ => {
            editor.cur.updown_x = 0;
        }
    }
    // all_redraw判定
    editor.is_all_redraw = false;
    match editor.curt_evt {
        Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
            Char('c') | Char('s') => {}
            _ => {
                editor.is_all_redraw = true;
            }
        },
        Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
            Down | Up | Left | Right => {
                editor.is_all_redraw = true;
            }
            _ => {
                editor.is_all_redraw = true;
            }
        },
        Key(KeyEvent { code, .. }) => match code {
            Down | Up | Left | Right => {
                if !editor.sel.is_unselected() {
                    editor.is_all_redraw = true;
                }
            }
            _ => {
                editor.is_all_redraw = true;
            }
        },
        Mouse(MouseEvent::ScrollUp(_, _, _)) | Mouse(MouseEvent::ScrollDown(_, _, _)) => {}
        _ => {
            editor.is_all_redraw = true;
        }
    }
    // 選択範囲クリア判定
    match editor.curt_evt {
        Key(KeyEvent { code, modifiers: KeyModifiers::SHIFT }) => match code {
            Down | Up | Left | Right => {}
            _ => {
                editor.sel.clear();
            }
        },
        Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
            Char('a') | Char('c') | Char('x') => {}
            _ => {
                editor.sel.clear();
            }
        },
        Key(KeyEvent { code, .. }) => match code {
            Backspace | Delete => {}
            _ => {
                editor.sel.clear();
            }
        },
        Mouse(MouseEvent::Down(_, _, _, _)) | Mouse(MouseEvent::Up(_, _, _, _)) | Mouse(MouseEvent::Drag(_, _, _, _)) => {}
        _ => {
            editor.sel.clear();
        }
    }
    // is_change判定
    match editor.curt_evt {
        Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => {
            if code == Char('x') || code == Char('v') {
                prompt.is_change = true;
            }
        }
        Key(KeyEvent { code: Char(_), .. }) => {
            prompt.is_change = true;
        }
        Key(KeyEvent { code, .. }) => {
            if code == Enter || code == Backspace || code == Delete {
                prompt.is_change = true;
            }
        }

        _ => {}
    }
}

fn finalize(editor: &mut Editor) {
    match editor.curt_evt {
        Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
            Char('x') => {
                editor.sel.clear();
            }
            _ => {}
        },
        Key(KeyEvent { code, .. }) => match code {
            Backspace | Delete => {
                editor.sel.clear();
            }
            _ => {}
        },
        _ => {}
    }
}

fn check_next_process<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, prompt: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
    if prompt.is_save_confirm == true {
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
                    editor.save();
                    return EvtProcess::Exit;
                } else if c == 'n' {
                    return EvtProcess::Exit;
                } else {
                    return EvtProcess::Hold;
                }
            }
            _ => return EvtProcess::Hold,
        }
    } else {
        return EvtProcess::Next;
    }
}

pub enum EvtProcess {
    Hold,
    Next,
    Exit,
}
