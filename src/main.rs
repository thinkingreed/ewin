#[macro_use]
extern crate clap;
use clap::{App, Arg};
use crossterm::event::{Event, EventStream};
use crossterm::ErrorKind;
use ewin::{_cfg::lang::cfg::LangCfg, global::*, model::*};
use futures::{future::FutureExt, select, StreamExt};
use std::ffi::OsStr;
use std::io::{stdout, BufWriter, Write};
use std::path::{Path, PathBuf};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() {
    let matches = App::new("ewin").version(crate_version!()).bin_name("ewin").arg(Arg::with_name("file").required(false)).get_matches();
    let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();

    let mut editor = Editor::default();
    let lang_cfg = LangCfg::read_lang_cfg();

    let mut term = Terminal::default();
    // ターミナルサイズが小さい場合に処理終了
    if !term.check_displayable(&lang_cfg) {
        return;
    }

    let mut sbar = StatusBar::new(lang_cfg.clone());
    let mut mbar = MsgBar::new();
    let mut prom = Prompt::new(lang_cfg.clone());

    term.set_disp_size(&mut editor, &mut mbar, &mut prom, &mut sbar);

    // grep_result
    if file_path.match_indices("search_str").count() > 0 && file_path.match_indices("search_file").count() > 0 {
        let v: Vec<&str> = file_path.split_ascii_whitespace().collect();
        let search_strs: Vec<&str> = v[0].split("=").collect();
        editor.search.str = search_strs[1].to_string();
        let search_files: Vec<&str> = v[1].split("=").collect();
        editor.search.file = search_files[1].to_string();

        let path = PathBuf::from(&editor.search.file);
        let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
        let path_str = path.to_string_lossy().to_string();
        editor.search.folder = path_str.replace(&filenm, "");
        editor.search.filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();

        if file_path.match_indices("search_row_num").count() == 0 {
            sbar.filenm = format!("grep \"{}\" {}", &editor.search.str, &editor.search.file);
            prom.is_grep_result = true;
            prom.is_grep_stdout = true;
            prom.is_grep_stderr = true;
            prom.grep_result();
            mbar.set_info(&LANG.searching);
        } else {
            sbar.filenm = editor.search.file.clone();
            let search_row_nums: Vec<&str> = v[2].split("=").collect();
            editor.search.row_num = search_row_nums[1].to_string();
            Log::ep("search_row_num", editor.search.row_num.clone());
            editor.open(Path::new(&sbar.filenm), &mut mbar);
            editor.search_str(true);
            //   editor.d_range = DRnage { d_type: DType::All, ..DRnage::default() };
        }
    } else {
        if file_path.len() == 0 {
            sbar.filenm_tmp = lang_cfg.new_file.clone();
        } else {
            sbar.filenm = file_path.to_string();
        }
        editor.open(Path::new(&file_path), &mut mbar);
    }

    let stdout = MouseTerminal::from(AlternateScreen::from(stdout()).into_raw_mode().unwrap());
    let mut out = BufWriter::new(stdout.lock());

    term.draw(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).unwrap();

    if let Err(err) = exec_events(&mut out, &mut term, &mut editor, &mut mbar, &mut prom, &mut sbar).await {
        Log::ep("err", err.to_string());
    }
    term.show_cur(&mut out);
}

async fn exec_events<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> anyhow::Result<()> {
    let mut reader = EventStream::new();

    let mut child = EvtAct::exec_cmd(editor);

    // TODO command実行回避
    let mut reader_stdout = FramedRead::new(child.stdout.take().unwrap(), LinesCodec::new());
    let mut reader_stderr = FramedRead::new(child.stderr.take().unwrap(), LinesCodec::new());
    let mut is_exit = false;

    loop {
        let mut event_next = reader.next().fuse();

        if prom.is_grep_stdout || prom.is_grep_stderr {
            let mut std_out_str = reader_stdout.next().fuse();
            let mut std_err_str = reader_stderr.next().fuse();

            select! {
                std_out_event = std_out_str => {
                    EvtAct::draw_grep_result(out, term, editor, mbar, prom, sbar, std_out_event, true,&mut child);
                },
                std_err_event = std_err_str => {
                    EvtAct::draw_grep_result(out, term, editor, mbar, prom, sbar, std_err_event, false, &mut child);
                },
                maybe_event = event_next => {
                     is_exit =  run_events(out,  term,  editor, mbar, prom,  sbar, maybe_event);
                    }
            }
        } else {
            select! {
                maybe_event = event_next => {
                    is_exit =  run_events(out,  term,  editor, mbar, prom,  sbar, maybe_event);
                }
            }
        }
        if is_exit {
            break;
        }
    }
    Ok(())
}

fn run_events<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, maybe_event: Option<Result<Event, ErrorKind>>) -> bool {
    let mut is_exit = false;

    if let Some(Ok(event)) = maybe_event {
        editor.evt = event.clone();

        is_exit = EvtAct::match_event(out, term, editor, mbar, prom, sbar);
    }
    return is_exit;
}
