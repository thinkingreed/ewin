use crate::{
    bar::headerbar::*,
    bar::statusbar::StatusBar,
    def::*,
    global::*,
    help::{Help, HelpMode},
    log::*,
    model::*,
    tab::Tab,
    util::is_enable_syntax_highlight,
};
use chrono::{DateTime, Local};
use clap::ArgMatches;
use crossterm::{
    cursor::*,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::ResetColor,
    terminal::*,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    env,
    ffi::OsStr,
    fs::metadata,
    io::{stdout, ErrorKind, Write},
    path::Path,
    process::exit,
};

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T) {
        Log::info_s("　　　　　　　Terminal.draw start");

        self.set_disp_size();

        let d_range = self.curt().editor.d_range;

        if !(d_range.draw_type == DrawType::Not || d_range.draw_type == DrawType::MoveCur) {
            self.curt().editor.draw_cache();
            self.tabs[self.idx].editor.draw(out, self.mode);
        }

        HeaderBar::draw(out, &self);
        let mut str_vec: Vec<String> = vec![];

        self.help.draw(&mut str_vec);
        self.curt().mbar.draw(&mut str_vec);
        let state = &self.curt().state.clone();
        self.curt().prom.draw(&mut str_vec, state);

        if d_range.draw_type != DrawType::Not {
            StatusBar::draw(&mut str_vec, &self.hbar.file_vec[self.idx], &mut self.tabs[self.idx])
        }
        Terminal::draw_cur(&mut str_vec, &mut self.tabs[self.idx], self.mode);

        Log::debug("cur", &self.curt().editor.cur);
        Log::debug("offset_x", &self.curt().editor.offset_x);
        Log::debug("offset_disp_x", &self.curt().editor.offset_disp_x);
        Log::debug("offset_y", &self.curt().editor.offset_y);
        Log::debug("tab.state", &self.curt().state);
        // Log::debug("", &self.curt().editor.sel);

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();

        Log::info_s("　　　　　　　Terminal.draw end");
    }

    pub fn draw_cur(str_vec: &mut Vec<String>, tab: &mut Tab, term_mode: TermMode) {
        Log::info_s("　　　　　　　draw_cur");

        if tab.state.is_save_new_file || tab.state.is_search || tab.state.is_replace || tab.state.grep_info.is_grep || tab.state.is_move_line || tab.state.is_open_file {
            tab.prom.draw_cur(str_vec);
        } else {
            let rnw_rnwmargin = if term_mode == TermMode::Normal { tab.editor.get_rnw() + Editor::RNW_MARGIN } else { 0 };
            str_vec.push(MoveTo((tab.editor.cur.disp_x - tab.editor.offset_disp_x + rnw_rnwmargin) as u16, (tab.editor.cur.y - tab.editor.offset_y + tab.editor.disp_row_posi) as u16).to_string());
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

    pub fn set_disp_size(&mut self) {
        Log::debug_s("set_disp_size");
        let (cols, rows) = size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        self.hbar.set_posi(cols);
        HeaderBar::set_header_filenm(self);

        self.help.disp_col_num = cols;
        self.help.disp_row_num = if self.help.mode == HelpMode::Show { Help::DISP_ROW_NUM } else { 0 };
        self.help.disp_row_posi = if self.help.mode == HelpMode::Show { rows - self.help.disp_row_num } else { 0 };

        self.curt().sbar.disp_row_num = 1;
        let help_disp_row_num = if self.help.disp_row_num > 0 { self.help.disp_row_num + 1 } else { 0 };
        self.curt().sbar.disp_row_posi = rows - help_disp_row_num;
        self.curt().sbar.disp_col_num = cols;

        self.curt().prom.disp_col_num = cols;
        self.curt().prom.disp_row_posi = rows - self.curt().prom.disp_row_num + 1 - self.help.disp_row_num - self.curt().sbar.disp_row_num;

        self.curt().mbar.disp_col_num = cols;
        self.curt().mbar.disp_readonly_row_num = if self.curt().state.is_read_only { 1 } else { 0 };
        self.curt().mbar.disp_keyrecord_row_num = if self.curt().mbar.msg_keyrecord.is_empty() { 0 } else { 1 };
        self.curt().mbar.disp_row_num = if self.curt().mbar.msg.str.is_empty() { 0 } else { 1 };

        self.curt().mbar.disp_row_posi = rows - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num;
        self.curt().mbar.disp_keyrecord_row_posi = rows - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num;
        self.curt().mbar.disp_readonly_row_posi = rows - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num;

        self.curt().editor.disp_col_num = cols;
        self.curt().editor.disp_row_num = rows - self.hbar.disp_row_num - self.curt().mbar.disp_readonly_row_num - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num;
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
    pub fn finalize() {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor, Show).unwrap();
    }
    pub fn exit() {
        exit(0);
    }

    //  pub fn open(&mut self, filenm: &String, encoding: Encoding, tab: &mut Tab) {
    pub fn open(&mut self, filenm: &String, tab: &mut Tab) {
        Log::info("File open start", &filenm);
        let path = Path::new(&filenm);
        self.idx = self.tabs.len();

        let mut enc = Encode::Unknown;
        let mut new_line = String::new();
        let mut bom_exsist = None;

        let path_str = &path.to_string_lossy().to_string();
        if path_str.len() > 0 {
            if path.exists() {
                let file_meta = metadata(path).unwrap();
                if file_meta.permissions().readonly() {
                    tab.state.is_read_only = true;
                    tab.mbar.set_readonly(&format!("{}({})", &LANG.unable_to_edit, &LANG.no_write_permission));
                }
                let ext = path.extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();

                tab.editor.draw.syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
                if tab.editor.draw.syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&ext) {
                    tab.editor.is_enable_syntax_highlight = true;
                }
            } else {
                Terminal::finalize();
                println!("{}", LANG.file_not_found.clone());
                Terminal::exit();
            }
        } else {
            let curt_dir = env::current_dir().unwrap();
            let curt_dir = metadata(curt_dir).unwrap();
            if curt_dir.permissions().readonly() {
                Terminal::finalize();
                println!("{}", LANG.no_write_permission.clone());
                Terminal::exit();
            }
        }
        // read
        let result = TextBuffer::from_path(path_str, enc);
        match result {
            Ok((text_buf, _enc, _new_line, _bom_exsist)) => {
                enc = _enc;
                new_line = _new_line;
                bom_exsist = _bom_exsist;
                tab.editor.buf = text_buf;
                tab.editor.buf.text.insert_char(tab.editor.buf.text.len_chars(), EOF_MARK);
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    Terminal::finalize();
                    println!("{}", LANG.no_read_permission.clone());
                    Terminal::exit();
                }
                ErrorKind::NotFound => tab.editor.buf.text.insert_char(tab.editor.buf.text.len_chars(), EOF_MARK),
                /*
                ErrorKind::InvalidData => {
                    enc = Encode::Unknown;
                    let mut decoder = DecodeReaderBytesBuilder::new().utf8_passthru(true).build(std::fs::File::open(path_str).unwrap());
                    let mut dest = vec![];
                    decoder.read_to_end(&mut dest).unwrap();
                    let mut builder = RopeBuilder::new();
                    builder.append(&*String::from_utf8_lossy(&dest[..]));
                    let text = builder.finish();
                    tab.editor.buf = TextBuffer { text };
                    tab.editor.buf.text.insert_char(tab.editor.buf.text.len_chars(), EOF_MARK);
                }
                 */
                _ => {
                    Terminal::finalize();
                    println!("{} {:?}", LANG.file_opening_problem, err);
                    Terminal::exit();
                }
            },
        }

        if enc == Encode::Unknown {
            tab.state.is_unknown_encoding = true;
        }

        let mut h_file = HeaderFile::new(&filenm);
        h_file.enc = enc;
        h_file.new_line = new_line;
        h_file.bom_exsist = bom_exsist;
        self.add_tab(tab.clone(), h_file);
        self.curt().editor.set_cur_default();

        Log::info("File open end", &filenm);
    }

    pub fn activate<T: Write>(&mut self, args: &Args, out: &mut T) {
        Log::info_s("　　　　　　　activate");

        let _ = GREP_INFO_VEC.set(tokio::sync::Mutex::new(vec![GrepInfo::default()]));
        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![]));

        self.open(&args.filenm, &mut Tab::new());
        self.draw(out);
    }

    pub fn add_tab(&mut self, tab: Tab, h_file: HeaderFile) {
        self.idx = self.tabs.len();
        self.tabs.push(tab);

        self.hbar.file_vec.push(h_file);
        self.hbar.disp_base_idx = USIZE_UNDEFINED;
        self.set_disp_size();
        HeaderBar::set_header_filenm(self);
    }

    pub fn del_tab(&mut self, tab_idx: usize) {
        self.tabs.remove(tab_idx);
        self.hbar.file_vec.remove(tab_idx);
        self.hbar.disp_base_idx = USIZE_UNDEFINED;

        if let Some(Ok(mut grep_info_vec)) = GREP_INFO_VEC.get().map(|vec| vec.try_lock()) {
            if grep_info_vec.len() > tab_idx {
                grep_info_vec.remove(tab_idx);
            }
        }
        if let Some(Ok(mut grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
            if grep_cancel_vec.len() > tab_idx {
                grep_cancel_vec.remove(tab_idx);
            }
        }
    }

    pub fn ctrl_mouse_capture(&mut self) {
        match self.mode {
            TermMode::Normal => {
                for tab in self.tabs.iter_mut() {
                    tab.editor.rnw = 0;
                    tab.editor.mode = TermMode::Mouse;
                }
                self.mode = TermMode::Mouse;
                execute!(stdout(), DisableMouseCapture).unwrap();
            }
            TermMode::Mouse => {
                for tab in self.tabs.iter_mut() {
                    tab.editor.rnw = tab.editor.buf.len_lines().to_string().len();
                    tab.editor.mode = TermMode::Normal;
                }
                self.mode = TermMode::Normal;
                execute!(stdout(), EnableMouseCapture).unwrap();
            }
        };
    }
    pub fn new_tab(&mut self) {
        // Disable the event in case of the next display
        self.curt().editor.evt = KEY_NULL;

        let mut new_tab = Tab::new();
        new_tab.editor.set_cur_default();
        new_tab.editor.buf.text.insert_char(new_tab.editor.buf.text.len_chars(), EOF_MARK);
        new_tab.editor.d_range.draw_type = DrawType::All;

        let dt: DateTime<Local> = Local::now();
        self.add_tab(new_tab, HeaderFile::new(&dt.format("%M:%S").to_string()));

        // self.add_tab(new_tab, &LANG.new_file);
    }

    pub fn next_tab(&mut self) {
        self.idx = if self.tabs.len() - 1 == self.idx { 0 } else { self.idx + 1 };
        self.curt().editor.evt = KEY_NULL;
        //  self.curt().editor.d_range.draw_type = DrawType::All;
    }

    pub fn resize(&mut self) {
        self.curt().editor.d_range.draw_type = DrawType::None;
        self.hbar.disp_base_idx = USIZE_UNDEFINED;
    }

    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.idx).unwrap();
    }
}
#[derive(Debug, Clone)]
pub struct Terminal {
    pub mode: TermMode,
    pub hbar: HeaderBar,
    pub help: Help,
    pub tabs: Vec<Tab>,
    // tab index
    pub idx: usize,
}

impl Terminal {
    pub fn new() -> Self {
        return Terminal { ..Terminal::default() };
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal { mode: TermMode::Normal, hbar: HeaderBar::new(), tabs: vec![], idx: 0, help: Help::new() }
    }
}

pub struct Args {
    pub filenm: String,
    pub encoding: Encode,
}
impl Default for Args {
    fn default() -> Self {
        Args { filenm: String::new(), encoding: Encode::UTF8 }
    }
}
impl Args {
    pub fn new(matches: &ArgMatches) -> Self {
        let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
        let args_encoding = matches.value_of_os("encodhing").unwrap_or(OsStr::new("UTF8")).to_string_lossy().to_string();

        let encoding = match args_encoding.as_str() {
            "sjis" => Encode::SJIS,
            _ => Encode::UTF8,
        };

        return Args { filenm: file_path, encoding };
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
