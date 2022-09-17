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
    model::{general::default::*, modal::*},
};
use ewin_const::models::{env::*, event::*};
use ewin_ctx_menu::{ctx_menu::*, global::*};
use ewin_dialog::{dialog::*, global::*};
use ewin_file_bar::{filebar::*, global::*};
use ewin_help::{global::*, help::*};
use ewin_key::{global::*, model::*};
use ewin_menu_bar::{global::*, menubar::*};
use ewin_msg_bar::{global::*, msgbar::*};
use ewin_prom::{global::*, model::*};
use ewin_side_bar::{global::*, sidebar::*};
use ewin_status_bar::{global::*, statusbar::*};
use ewin_tabs::tab::*;
use ewin_utils::{files::file::*, global::*, os::*};

use ewin_plugin::plugin::*;
use ewin_state::{global::*, term::*};
use parking_lot::Mutex;
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

        let _ = GREP_CANCEL_VEC.set(Mutex::new(vec![GrepCancelType::None]));
        let _ = WATCH_INFO.set(Mutex::new(WatchInfo::default()));
        let _ = TABS.set(Mutex::new(State::default()));
        let _ = PROM.set(Mutex::new(Prom::default()));
        let _ = DIALOG.set(Mutex::new(Dialog::default()));
        let _ = CTX_MENU.set(Mutex::new(CtxMenu::default()));
        let _ = FILE_BAR.set(Mutex::new(FileBar::default()));
        let _ = STATUS_BAR.set(Mutex::new(StatusBar::default()));
        let _ = MSG_BAR.set(Mutex::new(MsgBar::default()));
        let _ = HELP.set(Mutex::new(Help::default()));
        let _ = MENU_BAR.set(Mutex::new(MenuBar::default()));
        let _ = SIDE_BAR.set(Mutex::new(SideBar::default()));

        SideBar::get().init(&File::get_absolute_path(&args.filenm), false);
        CtxMenu::get().init();

        let act_typ = self.tabs.open_file(&args.filenm, FileOpenType::Nomal, Tab::new(), None);
        if let ActType::ExitMsg(msg) = act_typ {
            Term::exit_show_msg(&msg)
        };

        MenuBar::get().init();
    }
}
