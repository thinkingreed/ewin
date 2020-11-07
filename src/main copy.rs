#![recursion_limit = "512"]
#[macro_use]
extern crate clap;
use clap::{App, Arg};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use ewin::_cfg::lang::cfg::LangCfg;
use ewin::model::*;
use std::ffi::OsStr;
use std::io::{stdout, BufWriter, Write};
use std::time::SystemTime;
use std::path::{Path, PathBuf};
use termion::clear;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tokio_util::codec::{FramedRead, LinesCodec};

use crossterm::{
    event::{Event, EventStream},
    terminal::disable_raw_mode,
};
use futures::{future::FutureExt, select, StreamExt};

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

    let stdout = MouseTerminal::from(AlternateScreen::from(stdout()).into_raw_mode().unwrap());
    let mut out = BufWriter::new(stdout.lock());

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

    // grep_result
    if search_str.len() > 0 && search_file.len() > 0 {
        sbar.filenm = format!("grep \"{}\" {}", search_str, search_file);

        let path = PathBuf::from(&search_file);
        let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
        let path_str = path.to_string_lossy().to_string();
        editor.search.str = search_str;
        editor.search.file = search_file;
        editor.search.folder = path_str.replace(&filenm, "");
        editor.search.filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
    } else {
        if file_path.len() == 0 {
            sbar.filenm_tmp = lang_cfg.new_file.clone();
        } else {
            sbar.filenm = file_path.to_string();
        }
        editor.open(Path::new(&file_path));
    }

    term.draw(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).unwrap();

    if let Err(err) = exec_events(&mut out, &mut term, &mut editor, &mut mbar, &mut prom, &mut sbar).await {
        eprintln!("err {:?}", err.to_string());
    }

    disable_raw_mode();
}

async fn exec_events<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> anyhow::Result<()> {
    let mut reader = EventStream::new();

    let mut child = EvtAct::exec_cmd(editor);

    // TODO command実行回避
    let mut reader_stdout = FramedRead::new(child.stdout.take().unwrap(), LinesCodec::new());
    let mut reader_stderr = FramedRead::new(child.stderr.take().unwrap(), LinesCodec::new());

    loop {
        let mut event_next = reader.next().fuse();
        //   if prom.is_grep_result {
        let mut std_out_str = reader_stdout.next().fuse();
        let mut std_err_str = reader_stderr.next().fuse();

     

        //   }
        select! {
            std_out_event = std_out_str => {
                match std_out_event {
                          Some(Ok(result_str)) => EvtAct::draw_grep_result(out, term, editor, mbar, prom, sbar, std_out_event),
                          Some(Err(e)) => println!("err {:?}", e),
                          None => {
                              child.kill();
                               eprintln!("stdout end {:?}", SystemTime::now());
                        }
                       }
                  },
            std_err_event = std_err_str => {
                match std_err_event {
                          Some(Ok(result_str)) => EvtAct::draw_grep_result(out, term, editor, mbar, prom, sbar, result_str),
                          Some(Err(e)) => println!("err {:?}", e),
                          None => {
                             child.kill();
                             eprintln!("stderr end {:?}", SystemTime::now());
                        }
                       }
                  },
            maybe_event = event_next => {
                if let Some(Ok( mut event)) = maybe_event{
                  let is_exit =  run_events(out,  term,  editor, mbar, prom,  sbar, &mut  event);
                  if is_exit{ break; }
                }
            }
        }
    }
    Ok(())
}

fn run_events<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, event: &mut Event) -> bool {
    term.hide_cur(out);

    editor.curt_evt = event.clone();

    // eprintln!("evt {:?}", editor.curt_evt);

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
                            Log::ep_s("Char('w')");
                            term.show_cur(out);
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
                    Char('f') => editor.search_prom(prom),
                    Char('r') => editor.replace_prom(prom),
                    Char('g') => editor.grep_prom(prom),
                    Char('z') => editor.undo(),
                    Char('y') => editor.redo(&term),
                    Home => editor.ctl_home(),
                    End => editor.ctl_end(),
                    _ => {}
                },
                Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                    F(4) => editor.move_cursor(out, sbar),
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

            // EvtAct::finalize(&mut editor);
            if editor.is_redraw == true {
                term.draw(out, editor, mbar, prom, sbar).unwrap();
            }
        }
    }
    term.show_cur(out);
    return false;
}
