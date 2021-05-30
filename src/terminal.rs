use crate::{
    bar::headerbar::*,
    bar::statusbar::StatusBar,
    def::*,
    global::*,
    help::{Help, HelpMode},
    log::*,
    model::*,
    prompt::prompt::Prompt,
    tab::Tab,
    util::{get_str_width, is_enable_syntax_highlight},
};
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
    ffi::OsStr,
    fs::metadata,
    io::{stdout, ErrorKind, Write},
    path::Path,
    process::exit,
    usize,
};

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T) {
        Log::info_key("Terminal.draw start");

        self.set_disp_size();

        let d_range = self.curt().editor.d_range;

        if !(d_range.draw_type == DrawType::Not || d_range.draw_type == DrawType::MoveCur) {
            self.curt().editor.draw_cache();
            self.tabs[self.idx].editor.draw(out, self.mode);
        }
        HeaderBar::draw(out, &self);
        let mut str_vec: Vec<String> = vec![];
        let state = &self.curt().state.clone();

        self.help.draw(&mut str_vec);
        self.curt().mbar.draw(&mut str_vec);
        self.curt().prom.draw(&mut str_vec, state);

        if d_range.draw_type != DrawType::Not {
            StatusBar::draw(&mut str_vec, &self.hbar.file_vec[self.idx], &mut self.tabs[self.idx])
        }

        // Information display in the center when a new file is created
        if self.curt().editor.buf.len_chars() == 1 && d_range.draw_type == DrawType::None && self.idx == 0 {
            let cols = size().unwrap().0 as usize;
            let pkg_name = env!("CARGO_PKG_NAME");
            str_vec.push(format!("{}{}{}", MoveTo(0, 3), Clear(ClearType::CurrentLine), format!("{name:^w$}", name = pkg_name, w = cols - (get_str_width(&pkg_name) - pkg_name.chars().count()))));
            let ver_name = format!("{}: {}", "Version", env!("CARGO_PKG_VERSION"));
            str_vec.push(format!("{}{}{}", MoveTo(0, 4), Clear(ClearType::CurrentLine), format!("{ver:^w$}", ver = ver_name, w = cols - (get_str_width(&ver_name) - ver_name.chars().count()))));

            let simple_help = LANG.simple_help_desc.clone();
            str_vec.push(format!("{}{}{}", MoveTo(0, 6), Clear(ClearType::CurrentLine), format!("{s_help:^w$}", s_help = simple_help, w = cols - (get_str_width(&simple_help) - simple_help.chars().count()))));
            let detailed_help = LANG.detailed_help_desc.clone();
            str_vec.push(format!("{}{}{}", MoveTo(0, 7), Clear(ClearType::CurrentLine), format!("{d_help:^w$}", d_help = detailed_help, w = cols - (get_str_width(&detailed_help) - detailed_help.chars().count()))));
        }

        // Terminal::draw_cur(&mut str_vec, &mut self.tabs[self.idx], self.mode);

        Log::debug("cur", &self.curt().editor.cur);
        Log::debug("offset_x", &self.curt().editor.offset_x);
        Log::debug("offset_disp_x", &self.curt().editor.offset_disp_x);
        Log::debug("offset_y", &self.curt().editor.offset_y);
        //Log::debug("tab.state", &self.curt().state);
        Log::debug("", &self.curt().editor.sel);

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();

        Log::info_key("Terminal.draw end");
    }

    pub fn draw_cur<T: Write>(out: &mut T, term: &mut Terminal) {
        Log::info_key("draw_cur");
        let mut str_vec: Vec<String> = vec![];

        if term.tabs[term.idx].state.is_normal() {
            let rnw_margin = if &term.mode == &TermMode::Normal { term.tabs[term.idx].editor.get_rnw() + Editor::RNW_MARGIN } else { 0 };
            let editor = &term.tabs[term.idx].editor;
            str_vec.push(MoveTo((editor.cur.disp_x - editor.offset_disp_x + rnw_margin) as u16, (editor.cur.y - editor.offset_y + editor.disp_row_posi) as u16).to_string());
            Terminal::show_cur();
        } else {
            if term.tabs[term.idx].state.is_exists_input_field() || term.tabs[term.idx].state.is_exists_choice() {
                Terminal::show_cur();
                term.tabs[term.idx].prom.draw_cur(&mut str_vec, &term.tabs[term.idx]);
            } else {
                Terminal::hide_cur();
            }
        }
        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn check_displayable() -> bool {
        let (cols, rows) = size().unwrap();
        // rows 12 is prompt.open_file
        if cols <= 40 || rows <= 12 {
            return false;
        }
        return true;
    }
    pub fn clear_display() {
        let string = format!("{}{}", Clear(ClearType::All), MoveTo(0, 0).to_string());
        let _ = stdout().write(&string.as_bytes());
        stdout().flush().unwrap();
    }

    pub fn set_disp_size(&mut self) -> bool {
        Log::debug_s("set_disp_size");
        let (cols, rows) = size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        self.hbar.set_posi(cols);
        HeaderBar::set_header_filenm(&mut self.hbar);

        self.help.disp_col_num = cols;
        self.help.disp_row_num = if self.help.mode == HelpMode::Show { Help::DISP_ROW_NUM } else { 0 };
        self.help.disp_row_posi = if self.help.mode == HelpMode::Show { rows - self.help.disp_row_num } else { 0 };

        self.curt().sbar.disp_row_num = 1;
        let help_disp_row_num = if self.help.disp_row_num > 0 { self.help.disp_row_num + 1 } else { 0 };
        self.curt().sbar.disp_row_posi = if help_disp_row_num == 0 { rows - 1 } else { rows - help_disp_row_num };
        self.curt().sbar.disp_col_num = cols;

        self.curt().prom.disp_col_num = cols;

        if self.curt().state.is_open_file {
            // -1 is MsgBar
            self.curt().prom.disp_row_num = rows - self.hbar.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num - 1;
            if self.curt().prom.disp_row_num < Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM + 1 {
                return false;
            }
        }
        if rows < self.curt().prom.disp_row_num + self.help.disp_row_num + self.curt().sbar.disp_row_num {
            return false;
        }
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

        return true;
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
    pub fn open(&mut self, filenm: &String, tab: &mut Tab) -> bool {
        Log::info("File open start", &filenm);
        let path = Path::new(&filenm);

        let path_str = &path.to_string_lossy().to_string();
        let (is_readable, is_writable) = File::is_readable_writable(&path_str);

        if path_str.len() > 0 && !path.exists() {
            Terminal::exit_file_open(&LANG.file_not_found);
        }
        // read
        let result = TextBuffer::from_path(path_str);
        let mut enc = Encode::UTF8;
        let mut new_line = NEW_LINE_LF_STR.to_string();
        let mut bom_exsist = None;
        match result {
            Ok((text_buf, _enc, _new_line, _bom_exsist)) => {
                enc = _enc;
                new_line = _new_line;
                bom_exsist = _bom_exsist;
                tab.editor.buf = text_buf;
                let file_meta = metadata(path).unwrap();
                let ext = path.extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
                tab.editor.draw.syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
                if tab.editor.draw.syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&ext) {
                    tab.editor.is_enable_syntax_highlight = true;
                }
                if !is_writable {
                    tab.state.is_read_only = true;
                    tab.mbar.set_readonly(&format!("{}({})", &LANG.unable_to_edit, &LANG.no_write_permission));
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    if !is_readable {
                        if self.tabs.len() == 0 {
                            Terminal::exit_file_open(&LANG.no_read_permission);
                        } else {
                            self.curt().mbar.set_err(&LANG.no_read_permission.clone())
                        }
                    }
                }
                ErrorKind::NotFound => tab.editor.buf.text.insert_char(tab.editor.buf.text.len_chars(), EOF_MARK),
                _ => Terminal::exit_file_open(&format!("{} {:?}", LANG.file_opening_problem, err)),
            },
        }

        if is_readable {
            let mut h_file = HeaderFile::new(&filenm);
            h_file.enc = enc;
            h_file.nl = new_line.clone();
            h_file.nl_org = new_line;
            h_file.bom = bom_exsist;

            Log::info("File info", &h_file);

            self.add_tab(tab.clone(), h_file);
            self.idx = self.tabs.len() - 1;
            self.curt().editor.set_cur_default();
            Log::info_s("File open end");
            return true;
        }
        return false;
    }

    pub fn exit_file_open(msg: &String) {
        Terminal::finalize();
        println!("{}", msg);
        Terminal::exit();
    }

    pub fn activate<T: Write>(&mut self, args: &Args, out: &mut T) {
        Log::info_key("activate");

        let _ = GREP_INFO_VEC.set(tokio::sync::Mutex::new(vec![GrepInfo::default()]));
        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![]));

        self.open(&args.filenm, &mut Tab::new());
        self.draw(out);
    }

    pub fn add_tab(&mut self, tab: Tab, h_file: HeaderFile) {
        self.idx = self.tabs.len();
        self.tabs.push(tab);

        self.hbar.file_vec.push(h_file.clone());
        self.hbar.disp_base_idx = USIZE_UNDEFINED;
        self.set_disp_size();

        self.curt().editor.h_file = h_file.clone();
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
    pub fn close_all_tab(&mut self) {
        Log::debug_s("              close_all");

        self.state.is_all_close_confirm = true;
        let vec = &self.hbar.file_vec.clone();
        let mut idx = vec.len() - 1;
        for h_file in vec.iter().rev() {
            if h_file.is_changed {
                self.tabs[idx].editor.evt = CLOSE;
                self.tabs[idx].state.is_close_confirm = true;
                self.tabs[idx].editor.d_range.draw_type = DrawType::All;
            } else {
                self.del_tab(idx);
                if self.tabs.len() == 0 {
                    break;
                }
            }
            idx -= 1;
        }
        if !self.tabs.is_empty() {
            self.idx = if self.idx > self.tabs.len() - 1 { self.tabs.len() - 1 } else { self.idx };
            self.hbar.disp_base_idx = USIZE_UNDEFINED;
            EvtAct::match_event(self.tabs[self.idx].editor.evt, &mut stdout(), self);
        }
    }

    pub fn cancel_close_all_tab(&mut self) {
        self.state.is_all_close_confirm = false;
        let vec = &self.hbar.file_vec.clone();
        for (idx, h_file) in vec.iter().enumerate() {
            if h_file.is_changed {
                self.tabs[idx].editor.evt = KEY_NULL;
                self.tabs[idx].state.clear();
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

        // let dt: DateTime<Local> = Local::now();
        // self.add_tab(new_tab, HeaderFile::new(&dt.format("%M:%S").to_string()));

        self.add_tab(new_tab, HeaderFile::new(&LANG.new_file));
    }

    pub fn next_tab(&mut self) {
        self.idx = if self.tabs.len() - 1 == self.idx { 0 } else { self.idx + 1 };
        self.curt().editor.evt = KEY_NULL;
        self.curt().editor.d_range.draw_type = DrawType::All;
    }

    pub fn resize(&mut self) {
        self.curt().editor.d_range.draw_type = DrawType::None;
    }

    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.idx).unwrap();
    }

    pub fn curt_h_file(&mut self) -> &mut HeaderFile {
        return self.hbar.file_vec.get_mut(self.idx).unwrap();
    }

    pub fn clear_curt_tab_status(&mut self) {
        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().mbar.clear();
        self.set_disp_size();
        self.curt().editor.d_range.draw_type = DrawType::All;
    }

    pub fn clear_pre_tab_status(&mut self) {
        self.idx -= 1;
        self.curt().editor.d_range.draw_type = DrawType::All;

        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().mbar.clear();
        self.set_disp_size();
        self.curt().editor.d_range.draw_type = DrawType::All;
        self.idx += 1;
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
    pub state: TerminalState,
}

impl Terminal {
    pub fn new() -> Self {
        return Terminal { ..Terminal::default() };
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal {
            mode: TermMode::Normal,
            hbar: HeaderBar::new(),
            tabs: vec![],
            idx: 0,
            help: Help::new(),
            state: TerminalState::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerminalState {
    pub is_all_close_confirm: bool,
    pub is_displayable: bool,
}

impl Default for TerminalState {
    fn default() -> Self {
        TerminalState { is_all_close_confirm: false, is_displayable: true }
    }
}

pub struct Args {
    pub filenm: String,
    // pub encoding: Encode,
}
impl Default for Args {
    fn default() -> Self {
        Args {
            filenm: String::new(),
            // encoding: Encode::UTF8
        }
    }
}
impl Args {
    pub fn new(matches: &ArgMatches) -> Self {
        let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
        return Args { filenm: file_path };
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
