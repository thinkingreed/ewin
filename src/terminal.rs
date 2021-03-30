use crate::{
    bar::headerbar::HeaderBar,
    bar::statusbar::StatusBar,
    global::*,
    help::{Help, HelpMode},
    log::*,
    model::*,
    tab::Tab,
};
use crossterm::{
    cursor::*,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::ResetColor,
    terminal::*,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    cell::RefCell,
    ffi::OsStr,
    io::{self, stdout, Write},
    path::Path,
    path::PathBuf,
    process,
    process::Command,
    rc::Rc,
    sync::Arc,
};
use tokio::sync::Mutex;
#[derive(Debug, Clone)]

pub struct Terminal {
    pub hbar: HeaderBar,
    pub help: Help,
    pub tabs: Tabs,
    pub tabs_idx: usize,
}

impl Terminal {
    pub fn new() -> Self {
        return Terminal { ..Terminal::default() };
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal {
            hbar: HeaderBar::new(),
            tabs: Tabs::default(),
            tabs_idx: 0,
            help: Help::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tabs {
    pub tab_vec: Vec<Arc<Mutex<Tab>>>,
}

impl Default for Tabs {
    fn default() -> Self {
        Tabs { tab_vec: vec![Arc::new(Mutex::new(Tab::new()))] }
    }
}
impl Tabs {}

pub struct Args {
    pub mode: ActivatMode,
    pub filenm: String,
    pub ext: String,
    pub search_str: String,
    // full path
    pub search_file: String,
    pub search_folder: String,
    pub search_filenm: String,
    pub search_case_sens: bool,
    pub search_regex: bool,
    pub search_row_num: String,
}
impl Default for Args {
    fn default() -> Self {
        Args {
            mode: ActivatMode::Nomal,
            filenm: String::new(),
            ext: String::new(),
            search_str: String::new(),
            search_file: String::new(),
            search_folder: String::new(),
            search_filenm: String::new(),
            search_case_sens: true,
            search_regex: false,
            search_row_num: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivatMode {
    Nomal,
    Grep,
    GrepResult,
}

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, tab: &mut Tab) {
        Log::ep_s("　　　　　　　　All draw");

        self.set_disp_size(tab);

        let d_range = tab.editor.d_range;
        Log::ep("d_range", &d_range);

        if !(d_range.draw_type == DrawType::Not || d_range.draw_type == DrawType::MoveCur) {
            tab.editor.draw_cache();
            tab.editor.draw(out);
        }

        self.hbar.draw(out);
        let mut str_vec: Vec<String> = vec![];

        self.help.draw(&mut str_vec);
        tab.mbar.draw(&mut str_vec);
        tab.prom.draw(&mut str_vec, &tab.state);
        if d_range.draw_type != DrawType::Not {
            StatusBar::draw(&mut str_vec, tab)
        }
        Terminal::draw_cur(&mut str_vec, tab);

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_cur(str_vec: &mut Vec<String>, tab: &mut Tab) {
        Log::ep_s("　　　　　　　set_cur_str");

        if tab.prom.is_save_new_file || tab.state.is_search || tab.state.is_replace || tab.state.grep_info.is_grep || tab.prom.is_move_line {
            tab.prom.draw_cur(str_vec);
        } else {
            Log::ep("cur", &tab.editor.cur);
            Log::ep("editor.get_rnw()", &tab.editor.get_rnw());
            Log::ep("editor.cur.disp_x - 1 - editor.offset_disp_x", &(tab.editor.cur.disp_x - 1 - tab.editor.offset_disp_x));
            Log::ep("editor.cur.disp_x", &tab.editor.cur.disp_x);

            /*
            Log::ep("cur.disp_x - 1 - editor.offset_disp_x", &(editor.cur.disp_x - 1 - editor.offset_disp_x));
            Log::ep("cur.disp_x ", &editor.cur.disp_x);
            Log::ep("offset_disp_x", &editor.offset_disp_x);
            Log::ep("cur.y", &editor.cur.y);
            Log::ep("offset_y", &editor.offset_y);
            */
            str_vec.push(MoveTo((tab.editor.cur.disp_x - tab.editor.offset_disp_x) as u16, (tab.editor.cur.y - tab.editor.offset_y + tab.editor.disp_row_posi) as u16).to_string());
        }
    }

    pub fn check_displayable() -> bool {
        let (cols, rows) = size().unwrap();
        if cols <= 20 || rows <= 10 {
            println!("{:?}", &LANG.terminal_size_small);
            return false;
        }
        return true;
    }

    pub fn set_disp_size(&mut self, tab: &mut Tab) {
        Log::ep_s("set_disp_size");

        let (cols, rows) = size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);

        Log::ep("rows", &rows);
        Log::ep("cols", &cols);

        self.hbar.set_posi(cols);
        self.help.disp_row_num = if self.help.mode == HelpMode::Show { Help::DISP_ROW_NUM } else { 0 };
        self.help.disp_row_posi = if self.help.mode == HelpMode::Show { rows - self.help.disp_row_num } else { 0 };

        tab.sbar.disp_row_num = 1;
        let help_disp_row_num = if self.help.disp_row_num > 0 { self.help.disp_row_num + 1 } else { 0 };
        tab.sbar.disp_row_posi = rows - help_disp_row_num;
        tab.sbar.disp_col_num = cols;

        Log::ep("self.help.mode", &self.help.mode);

        tab.prom.disp_col_num = cols;
        tab.prom.disp_row_posi = rows - tab.prom.disp_row_num + 1 - self.help.disp_row_num - tab.sbar.disp_row_num;

        tab.mbar.disp_col_num = cols;
        tab.mbar.disp_readonly_row_num = if tab.mbar.msg_readonly.is_empty() { 0 } else { 1 };
        tab.mbar.disp_keyrecord_row_num = if tab.mbar.msg_keyrecord.is_empty() { 0 } else { 1 };
        tab.mbar.disp_row_num = if tab.mbar.msg.str.is_empty() { 0 } else { 1 };

        tab.mbar.disp_row_posi = rows - tab.prom.disp_row_num - self.help.disp_row_num - tab.sbar.disp_row_num;
        tab.mbar.disp_keyrecord_row_posi = rows - tab.mbar.disp_row_num - tab.prom.disp_row_num - self.help.disp_row_num - tab.sbar.disp_row_num;
        tab.mbar.disp_readonly_row_posi = rows - tab.mbar.disp_keyrecord_row_num - tab.mbar.disp_row_num - tab.prom.disp_row_num - self.help.disp_row_num - tab.sbar.disp_row_num;

        tab.editor.disp_col_num = cols;
        tab.editor.disp_row_num = rows - self.hbar.disp_row_num - tab.mbar.disp_readonly_row_num - tab.mbar.disp_keyrecord_row_num - tab.mbar.disp_row_num - tab.prom.disp_row_num - self.help.disp_row_num - tab.sbar.disp_row_num;

        Log::ep("editor.disp_row_num", &tab.editor.disp_row_num);
        /*
           Log::ep("mbar.disp_keyrecord_row_num", &mbar.disp_keyrecord_row_num);
           Log::ep("mbar.disp_readonly_row_num", &mbar.disp_readonly_row_num);
           Log::ep("mbar.disp_row_num", &mbar.disp_row_num);
           Log::ep("prom.disp_row_num", &prom.disp_row_num);
           Log::ep("help.disp_row_num", &help.disp_row_num);
           Log::ep("help.disp_row_posi", &help.disp_row_posi);
           Log::ep("sbar.disp_row_num", &sbar.disp_row_num);
        */
    }

    pub fn init_draw<T: Write>(&mut self, out: &mut T, tab: &mut Tab) {
        tab.prom.clear();
        tab.state.clear();
        tab.mbar.clear();
        tab.editor.d_range.draw_type = DrawType::All;
        self.draw(out, tab);
    }

    pub fn show_cur() {
        execute!(stdout(), Show).unwrap();
    }
    pub fn hide_cur() {
        execute!(stdout(), Hide).unwrap();
    }
    pub fn startup_terminal(args: String) {
        Log::ep("args", &args);

        let exe_path = if !cfg!(debug_assertions) && Path::new("/usr/bin/ewin").exists() { "/usr/bin/ewin" } else { "/home/hi/rust/ewin/target/release/ewin" };

        if *ENV == Env::WSL {
            if let Err(err) = Command::new("/mnt/c/windows/system32/cmd.exe")
                .arg("/c")
                .arg("start")
                .arg("wsl")
                .arg("-e")
                .arg(exe_path)
                .arg(args)
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn()
            {
                Log::ep_s("WSL");
                Log::ep("startup_terminal err", &err.to_string());
            }
        } else {
            // gnome-terminal
            if let Err(err) = Command::new("gnome-terminal").arg("--").arg(exe_path).arg(args).spawn() {
                Log::ep_s("gnome");
                Log::ep("startup_terminal err", &err.to_string());
            }
        };
    }
    pub fn init() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    }
    pub fn exit() {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor, Show).unwrap();
    }

    pub fn init_args(file_path: &String) -> Args {
        let activate_mode = Terminal::check_activate_mode(&file_path);
        let mut args = Args::default();
        args.mode = activate_mode;

        if activate_mode == ActivatMode::GrepResult || activate_mode == ActivatMode::Grep {
            let v: Vec<&str> = file_path.split_ascii_whitespace().collect();
            args.search_str = v[0].split("=").nth(1).unwrap().to_string();
            args.search_file = v[1].split("=").nth(1).unwrap().to_string();
            args.search_case_sens = if v[2].split("=").nth(1).unwrap() == "true" { true } else { false };
            args.search_regex = if v[3].split("=").nth(1).unwrap() == "true" { true } else { false };

            let path = PathBuf::from(&args.search_file);
            let filenm = path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
            let path_str = path.to_string_lossy().to_string();
            args.search_folder = path_str.replace(&filenm, "");
            args.ext = Path::new(&filenm).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
            args.search_filenm = filenm;

            if activate_mode == ActivatMode::GrepResult {
                let search_row_nums: Vec<&str> = v[4].split("=").collect();
                args.search_row_num = search_row_nums[1].to_string();
            }
        // Normal
        } else {
            if !file_path.is_empty() {
                args.filenm = file_path.to_string();
                args.ext = Path::new(&args.filenm).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
            }
        }
        return args;
    }

    pub fn activate<T: Write>(&mut self, args: &Args, out: &mut T) {
        let arc = Arc::clone(&self.tabs.tab_vec[self.tabs_idx]);
        let mut tab = arc.try_lock().unwrap();

        // grep_result

        /*
         if args.mode == ActivatMode::GrepResult || args.mode == ActivatMode::Grep {
             editor.search.str = args.search_str.clone();
             editor.search.file = args.search_file.clone();
             CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.case_sens = args.search_case_sens).unwrap();
             CFG.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.regex = args.search_regex).unwrap();
             editor.search.folder = args.search_folder.clone();
             editor.search.filenm = args.search_filenm.clone();

             if args.mode == ActivatMode::Grep {
                 FILE.get().unwrap().try_lock().map(|mut file| file.filenm = format!("grep \"{}\" {}", &editor.search.str, &editor.search.file)).unwrap();

                 prom.is_grep_result = true;
                 prom.is_grep_stdout = true;
                 prom.is_grep_stderr = true;

                 prom.grep_result(self);
                 editor.set_cur_default();
                 editor.scroll();
                 editor.scroll_horizontal();
                 mbar.set_info(&LANG.searching);
             } else {
                 let filenm = args.search_file.clone();
                 FILE.get().unwrap().try_lock().map(|mut file| file.filenm = filenm.clone()).unwrap();

                 editor.search.row_num = args.search_row_num.clone();

                 Log::ep("editor.search", &editor.search);

                 editor.open(Path::new(&filenm), &mut *mbar);
                 editor.search_str(true, false);
             }

         // Normal
         } else {
        */
        let filenm = args.filenm.clone();
        FILE.get().unwrap().try_lock().map(|mut file| file.filenm = filenm.clone()).unwrap();

        tab.open(Path::new(&filenm));
        self.draw(out, &mut tab);
        //  }
    }

    pub fn check_activate_mode(file_path: &String) -> ActivatMode {
        if file_path.match_indices("search_str").count() > 0 {
            if file_path.match_indices("search_row_num").count() == 0 {
                return ActivatMode::Grep;
            } else {
                return ActivatMode::GrepResult;
            }
        } else {
            return ActivatMode::Nomal;
        }
    }
}
impl UT {
    /*
    pub fn init_ut() -> (Editor, MsgBar) {
        let mut e = Editor::default();
        e.buf = vec![vec![]];
        e.buf[0] = vec![EOF_MARK];
        e.disp_row_num = 5;
        e.set_cur_default();
        e.d_range = DRnage::default();

        let mbar = MsgBar::new();

        return (e, mbar);
    }

    pub fn insert_str(e: &mut Editor, str: &str) {
        for c in str.chars() {
            e.insert_char(c);
        }
    }
    pub fn undo_all(e: &mut Editor, mbar: &mut MsgBar) {
        let vec = e.undo_vec.clone();
        for evt_proc in vec.iter().rev() {
            Log::ep("undo_all.evt_proc.do_type", evt_proc.do_type);
            e.undo(mbar);
        }
    }
    pub fn get_buf_str(e: &mut Editor) -> String {
        let mut s = String::new();
        for vec in &e.buf {
            s.push_str(&vec.iter().collect::<String>());
        }
        return s;
    }
    */
}
