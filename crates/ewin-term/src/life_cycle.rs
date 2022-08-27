use super::term::*;
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
use ewin_const::models::{env::*, file::*};
use ewin_ctx_menu::{ctx_menu::CtxMenu, global::*};
use ewin_dialog::{dialog::*, global::*};
use ewin_file_bar::{filebar::*, global::*};
use ewin_help::{global::*, help::*};
use ewin_key::{global::*, model::*};
use ewin_menulist::{global::*, menubar::*};
use ewin_tabs::tab::*;
use ewin_utils::{global::*, os::*};

use ewin_plugin::plugin::*;
use ewin_state::{global::*, term::*};
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

    pub fn exit() {
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
        let _ = WATCH_INFO.set(tokio::sync::Mutex::new(WatchInfo::default()));

        let _ = TABS.set(tokio::sync::Mutex::new(State::get_init_file_info()));

        let _ = DIALOG.set(tokio::sync::Mutex::new(Dialog::default()));
        let _ = CTX_MENU.set(tokio::sync::Mutex::new(CtxMenu::default()));
        let _ = FILE_BAR.set(tokio::sync::Mutex::new(FileBar::default()));
        let _ = HELP.set(tokio::sync::Mutex::new(Help::default()));
        let _ = MENU_BAR.set(tokio::sync::Mutex::new(MenuBar::default()));

        self.set_size_init();
        self.tabs.open_file(&args.filenm, FileOpenType::First, Some(&mut Tab::new()), None);

        CtxMenu::init();
        MenuBar::get().init();

        Log::info("FileBar::get()", &FileBar::get());
    }
}
