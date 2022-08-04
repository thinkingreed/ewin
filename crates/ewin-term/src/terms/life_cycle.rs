use super::term::*;
use crate::{ewin_editor::model::*, global_term, global_term::*, help::*, model::*, tab::Tab};
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
use ewin_dialog::{dialog::*, global::*};
use ewin_key::{global::*, model::*, util::*};
use ewin_state::{global::*, tabs::*};
use std::{io::stdout, process::exit};

impl Term {
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
        Term::finalize();
        println!("{}", msg);
        Term::exit();
    }

    pub fn exit_proc() {
        Term::finalize();
        Term::exit();
    }

    pub fn activate(&mut self, args: &AppArgs) {
        Log::info_key("activate");

        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![GrepCancelType::None]));
        let _ = global_term::EDITOR.set(tokio::sync::Mutex::new(Editor::new()));
        let _ = WATCH_INFO.set(tokio::sync::Mutex::new(WatchInfo::default()));

        let _ = TABS.set(tokio::sync::Mutex::new(Tabs::get_init_file_info()));

        let _ = HELP_DISP.set(tokio::sync::Mutex::new(Help::default()));
        self.open_file(&args.filenm, FileOpenType::First, Some(&mut Tab::new()), None);
        self.ctx_menu.init();
        self.menubar.init();

        let _ = DIALOG.set(tokio::sync::Mutex::new(Dialog::default()));
    }
}
