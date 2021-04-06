use crate::{
    bar::headerbar::*,
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
    io::{stdout, Write},
    path::Path,
    rc::Rc,
};
#[derive(Debug, Clone)]

pub struct Terminal {
    pub hbar: HeaderBar,
    pub help: Help,
    pub tabs: Vec<Rc<RefCell<Tab>>>,
    pub tab_idx: usize,
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
            tabs: vec![Rc::new(RefCell::new(Tab::new()))],
            tab_idx: 0,
            help: Help::new(),
        }
    }
}

pub struct Args {
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

        HeaderBar::draw(out, &self);
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
            /*
            Log::ep("cur", &tab.editor.cur);
            Log::ep("editor.get_rnw()", &tab.editor.get_rnw());
            Log::ep("editor.cur.disp_x", &tab.editor.cur.disp_x);
            Log::ep("cur.y", &tab.editor.cur.y);
            Log::ep("offset_disp_x", &tab.editor.offset_disp_x);
            Log::ep("offset_y", &tab.editor.offset_y);
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

        HeaderBar::set_header_filenm(self);

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

        /*Log::ep("editor.disp_row_num", &tab.editor.disp_row_num);

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
    pub fn init() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    }
    pub fn exit() {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor, Show).unwrap();
    }

    pub fn init_args(file_path: &String) -> Args {
        let mut args = Args::default();

        if !file_path.is_empty() {
            args.filenm = file_path.to_string();
            args.ext = Path::new(&args.filenm).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
        }

        return args;
    }

    pub fn activate<T: Write>(&mut self, args: &Args, out: &mut T) {
        let rc = Rc::clone(&self.tabs[self.tab_idx]);
        let mut tab = rc.borrow_mut();

        let _ = GREP_INFO_VEC.set(tokio::sync::Mutex::new(vec![GrepInfo::default()]));
        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![]));

        let mut h_file = HeaderFile::default();
        h_file.ext = Path::new(&args.filenm).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
        h_file.filenm = if args.filenm.is_empty() { LANG.new_file.clone() } else { args.filenm.clone() };
        self.hbar.file_vec.push(h_file);

        tab.open(&self, &args.filenm);
        self.draw(out, &mut tab);
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
