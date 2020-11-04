#[macro_use]
extern crate clap;
use clap::{App, Arg};
use crossterm::event::{read, KeyCode::*, KeyModifiers};
use crossterm::event::{Event::*, KeyEvent, MouseButton, MouseEvent};
use ewin::_cfg::lang::cfg::LangCfg;
use ewin::model::*;
use std::ffi::OsStr;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

#[tokio::main]
async fn main() {
    let matches = App::new("ewin")
        .version(crate_version!())
        .bin_name("ewin")
        .arg(Arg::with_name("file").required(false))
        .arg(Arg::with_name("search_str").long("search_str").value_name("search_str").help("Sets a search target string").takes_value(true))
        .arg(Arg::with_name("search_file").long("search_file").value_name("search_file").help("Sets a search target file name").takes_value(true))
        //.arg(Arg::with_name("-debug").help("debug mode").short("-d").long("-debug"))
        .get_matches();

    let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
    let search_str: String = matches.value_of_os("search_str").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
    let search_file: String = matches.value_of_os("search_file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();

    let mut editor = Editor::default();
    let lang_cfg = LangCfg::read_lang_cfg();

    let mut term = Terminal::default();
    // ターミナルサイズが小さい場合に処理終了
    if !term.check_displayable(&lang_cfg) {
        return;
    }
    term.set_env();

    let mut sbar = StatusBar::new(lang_cfg.clone());
    let mut mbar = MsgBar::new(lang_cfg.clone());
    let mut prom = Prompt::new(lang_cfg.clone());
    term.set_disp_size(&mut editor, &mut mbar, &mut prom, &mut sbar);
    if search_str.len() > 0 && search_file.len() > 0 {
        sbar.filenm = format!("grep \"{}\" {}", search_str, search_file);

        Log::ep("sbar.filenm", sbar.filenm.clone());
        Log::ep("search_str", search_str.clone());
        Log::ep("search_file", search_file.clone());

        if let Err(err) = EvtAct::exec_grep(search_str, search_file).await {
            Log::ep("exec_grep err", err.to_string());
        }

        Log::ep_s("search after");
    //
    //
    //
    //
    } else {
        if file_path.len() == 0 {
            sbar.filenm_tmp = lang_cfg.new_file.clone();
        } else {
            sbar.filenm = file_path.to_string();
        }
        editor.open(Path::new(&file_path));
    }

    let stdout = MouseTerminal::from(AlternateScreen::from(stdout()).into_raw_mode().unwrap());
    let mut out = BufWriter::new(stdout.lock());
    term.draw(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).unwrap();

    loop {
        let event = read();

        editor.curt_evt = event.unwrap().clone();

        // eprintln!("evt {:?}", editor.curt_evt.clone());

        write!(out, "{}", cursor::Hide.to_string()).unwrap();
        out.flush().unwrap();

        let evt_next_process = EvtAct::check_next_process(&mut out, &mut term, &mut editor, &mut mbar, &mut prom, &mut sbar);

        match evt_next_process {
            EvtActType::Exit => return,
            EvtActType::Hold => {}
            EvtActType::Next => {
                EvtAct::init(&mut editor, &mut prom);

                match editor.curt_evt {
                    Resize(_, _) => {
                        write!(out, "{}", clear::All.to_string()).unwrap();
                        term.set_disp_size(&mut editor, &mut mbar, &mut prom, &mut sbar);
                    }
                    Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                        Char('w') => {
                            if editor.close(&mut out, &mut prom) == true {
                                write!(out, "{}", cursor::Show).unwrap();
                                out.flush().unwrap();
                                return;
                            }
                        }
                        Char('s') => {
                            editor.save(&mut mbar, &mut prom, &mut sbar);
                        }
                        Char('c') => editor.copy(&term),
                        Char('x') => editor.cut(&term),
                        Char('v') => editor.paste(&term),
                        Char('a') => editor.all_select(),
                        Char('f') => editor.search_prom(&mut prom),
                        Char('r') => editor.replace_prom(&mut prom),
                        Char('g') => editor.grep_prom(&mut prom),
                        Char('z') => editor.undo(),
                        Char('y') => editor.redo(&term),
                        Home => editor.ctl_home(),
                        End => editor.ctl_end(),
                        _ => {}
                    },
                    Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                        F(4) => editor.move_cursor(&mut out, &mut sbar),
                        Right => editor.shift_right(),
                        Left => editor.shift_left(),
                        Down => editor.shift_down(),
                        Up => editor.shift_up(),
                        Home => editor.shift_home(),
                        End => editor.shift_end(),
                        Char(c) => editor.insert_char(c.to_ascii_uppercase()),
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
                        Home => editor.move_cursor(&mut out, &mut sbar),
                        End => editor.move_cursor(&mut out, &mut sbar),
                        Down => editor.move_cursor(&mut out, &mut sbar),
                        Up => editor.move_cursor(&mut out, &mut sbar),
                        Left => editor.move_cursor(&mut out, &mut sbar),
                        Right => editor.move_cursor(&mut out, &mut sbar),
                        F(3) => editor.move_cursor(&mut out, &mut sbar),
                        _ => {
                            Log::ep_s("Un Supported no modifiers");
                        }
                    },
                    Mouse(MouseEvent::ScrollUp(_, _, _)) => editor.move_cursor(&mut out, &mut sbar),
                    Mouse(MouseEvent::ScrollDown(_, _, _)) => editor.move_cursor(&mut out, &mut sbar),
                    Mouse(MouseEvent::Down(MouseButton::Left, x, y, _)) => editor.mouse_left_press((x + 1) as usize, y as usize),
                    Mouse(MouseEvent::Down(_, _, _, _)) => {}
                    Mouse(MouseEvent::Up(_, x, y, _)) => editor.mouse_release((x + 1) as usize, y as usize),
                    Mouse(MouseEvent::Drag(_, x, y, _)) => editor.mouse_hold((x + 1) as usize, y as usize),
                }

                // EvtAct::finalize(&mut editor);
                if editor.is_redraw == true {
                    term.draw(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).unwrap();
                }
            }
        }
        write!(out, "{}", cursor::Show).unwrap();
        out.flush().unwrap();
    }
}
