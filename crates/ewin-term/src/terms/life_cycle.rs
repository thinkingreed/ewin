use super::term::*;
use crate::{
    ewin_com::{global::*, model::*, util::*},
    ewin_editor::model::*,
    global_term,
    global_term::*,
    help::*,
    model::*,
    tab::Tab,
};
use crossterm::{
    cursor::*,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::ResetColor,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ewin_cfg::{
    log::*,
    model::{default::*, modal::*},
};
use ewin_const::model::*;
use std::{io::stdout, process::exit};

impl Terminal {
    pub fn init() {
        Macros::init_js_engine();
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
        if Cfg::get().general.mouse.mouse_enable {
            execute!(stdout(), EnableMouseCapture).unwrap();
        }
        Log::info("Platform", &*ENV);
        if *ENV == Env::WSL {
            Log::info("Powershell enable", &*IS_POWERSHELL_ENABLE);
        }
        if cfg!(target_os = "windows") && Cfg::get().system.os.windows.change_output_encoding_utf8 {
            change_output_encoding();
        }
    }

    pub fn finalize() {
        Macros::exit_js_engine();

        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor, Show).unwrap();
    }

    fn exit() {
        exit(0);
    }

    pub fn exit_show_msg(msg: &str) {
        Terminal::finalize();
        println!("{}", msg);
        Terminal::exit();
    }

    pub fn exit_proc() {
        Terminal::finalize();
        Terminal::exit();
    }

    pub fn activate(&mut self, args: &AppArgs) {
        Log::info_key("activate");

        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![GrepCancelType::None]));
        let _ = global_term::EDITOR.set(tokio::sync::Mutex::new(Editor::new()));
        let _ = WATCH_INFO.set(tokio::sync::Mutex::new(WatchInfo::default()));
        let _ = H_FILE_VEC.set(tokio::sync::Mutex::new(vec![]));
        let _ = HELP_DISP.set(tokio::sync::Mutex::new(Help::default()));
        self.open_file(&args.filenm, FileOpenType::First, Some(&mut Tab::new()), None);
        self.ctx_widget.init();
        self.menubar.init();
    }
}
