use crossterm::event::{read, KeyCode::*, KeyModifiers};
use crossterm::event::{Event::*, KeyEvent, MouseButton, MouseEvent};
use ewin::_cfg::args::ARGS;
use ewin::_cfg::lang::cfg::LangCfg;
use ewin::model::{Editor, Log, Process, Prompt, StatusBar, Terminal};
use ewin::process::process::*;

use std::io::Write;
use std::io::{stdout, BufWriter};
use std::path::Path;
use termion::clear;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let file_path: &str = ARGS.get("file_path").unwrap();

    let mut editor = Editor::default();
    let lang_cfg = LangCfg::read_lang_cfg();

    let mut terminal = Terminal::default();
    // ターミナルサイズが小さい場合に処理終了
    if !terminal.check_displayable(&lang_cfg) {
        return;
    }
    let mut disp_filenm = file_path;
    if file_path.len() == 0 {
        disp_filenm = &lang_cfg.new_file;
    }
    let mut sbar = StatusBar::new(disp_filenm, lang_cfg.clone());
    let mut prom = Prompt::new(lang_cfg.clone());
    terminal.set_disp_size(&mut editor, &mut prom, &mut sbar);

    editor.open(Path::new(file_path));

    let stdout = MouseTerminal::from(AlternateScreen::from(stdout()).into_raw_mode().unwrap());
    let mut out = BufWriter::new(stdout.lock());
    terminal.draw(&mut out, &mut editor, &mut prom, &mut sbar).unwrap();

    loop {
        let event = read();

        editor.curt_evt = event.unwrap().clone();
        let evt_next_process = Process::check_next_process(&mut out, &mut terminal, &mut editor, &mut prom, &mut sbar);

        match evt_next_process {
            EvtProcess::Exit => return,
            EvtProcess::Hold => {}
            EvtProcess::Next => {
                Process::init(&mut editor, &mut prom);

                match editor.curt_evt {
                    Resize(_, _) => {
                        write!(out, "{}", clear::All.to_string()).unwrap();
                        terminal.set_disp_size(&mut editor, &mut prom, &mut sbar);
                    }
                    Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                        Char('w') => {
                            if editor.close(&mut prom) == true {
                                return;
                            }
                        }
                        Char('s') => editor.save(&mut prom),
                        Char('c') => editor.copy(),
                        Char('x') => editor.cut(),
                        Char('v') => editor.paste(),
                        Char('a') => editor.all_select(),
                        Home => editor.ctl_home(),
                        End => editor.ctl_end(),
                        _ => {}
                    },
                    Key(KeyEvent { code, modifiers: KeyModifiers::SHIFT }) => match code {
                        Right => editor.shift_right(),
                        Left => editor.shift_left(),
                        Down => editor.shift_down(),
                        Up => editor.shift_up(),
                        Home => editor.shift_home(),
                        End => editor.shift_end(),
                        Char(c) => editor.insert_char(c.to_ascii_uppercase()),
                        _ => {}
                    },
                    Key(KeyEvent { code: Char(c), .. }) => editor.insert_char(c),
                    Key(KeyEvent { code, .. }) => match code {
                        Enter => editor.enter(),
                        Backspace => editor.back_space(),
                        Delete => editor.delete(),
                        Home => editor.home(),
                        End => editor.end(),
                        PageDown => editor.page_down(),
                        PageUp => editor.page_up(),
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
                    Mouse(MouseEvent::Down(MouseButton::Left, x, y, _)) => editor.mouse_left_press((x + 1) as usize, y as usize),
                    Mouse(MouseEvent::Down(_, _, _, _)) => {}
                    Mouse(MouseEvent::Up(_, x, y, _)) => editor.mouse_release((x + 1) as usize, y as usize),
                    Mouse(MouseEvent::Drag(_, x, y, _)) => editor.mouse_hold((x + 1) as usize, y as usize),
                }
                Process::finalize(&mut editor);
                if editor.is_all_redraw == true {
                    terminal.draw(&mut out, &mut editor, &mut prom, &mut sbar).unwrap();
                }
            }
        }
    }
}
