#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]

extern crate ewin_com;
extern crate ewin_editor;
extern crate ewin_prom;

pub mod global_term {
    use crate::help::*;
    use ewin_com::model::*;
    use ewin_editor::model::Editor;
    use once_cell::sync::OnceCell;
    use tokio::sync::Mutex;
    pub static EDITOR: OnceCell<tokio::sync::Mutex<Editor>> = OnceCell::new();

    pub static HELP_DISP: OnceCell<Mutex<Help>> = OnceCell::new();
    pub static H_FILE_VEC: OnceCell<Mutex<Vec<HeaderFile>>> = OnceCell::new();

    pub fn get_help() -> Help {
        // TODO clone
        return HELP_DISP.get_or_init(|| Mutex::new(Help::default())).try_lock().unwrap().clone();
    }

    pub fn toggle_help_disp() {
        let _ = HELP_DISP.get().unwrap().try_lock().map(|mut help| help.is_disp = !help.is_disp);
        // HELP_DISP.get().unwrap().try_lock().unwrap().ma;
        // get_help().is_disp = !get_help().is_disp;
    }
}
pub mod evt_act {
    pub mod _com;
    pub mod ctx_menu;
    pub mod editor;
    pub mod filebar;
    pub mod menubar;
    pub mod prom;
    pub mod statusbar;
}
pub mod terms {
    pub mod draw;
    pub mod file;
    pub mod life_cycle;
    pub mod size;
    pub mod tabs;
    pub mod term;
}
pub mod bar {
    pub mod filebar;
    pub mod menubar;
    pub mod msgbar;
    pub mod statusbar;
}
pub mod macros {
    pub mod js_func;
    pub mod js_macro;
    pub mod key_macro;
}
pub mod prom {
    pub mod enc_nl;
    pub mod grep;
    pub mod grep_result;
    pub mod move_row;
    pub mod open_file;
    pub mod replace;
    pub mod save_confirm;
    pub mod save_forced;
    pub mod save_new_file;
    pub mod search;
    pub mod watch_file;
}
pub mod help;
pub mod model;
pub mod tab;
