use crate::{_cfg::keys::*, bar::headerbar::*, bar::statusbar::*, ctx_menu::init::*, def::*, editor::buf::edit::*, global::*, help::*, log::*, model::*, prompt::prompt::prompt::*, tab::Tab, util::*};
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

        let draw_type = self.curt().editor.draw_type;
        Log::debug("d_range", &draw_type);

        if !(draw_type == DrawType::Not || draw_type == DrawType::MoveCur) {
            self.editor_draw_vec[self.idx].draw_cache(&mut self.tabs[self.idx].editor);
            self.tabs[self.idx].editor.draw(out, &self.editor_draw_vec[self.idx], &self.mouse_mode)
        }
        HeaderBar::draw(out, self);
        let mut str_vec: Vec<String> = vec![];
        let state = &self.curt().state.clone();

        self.help.draw(&mut str_vec);
        self.curt().mbar.draw(&mut str_vec);
        let is_msg_changed = self.curt().mbar.is_msg_changed();
        self.curt().prom.draw(&mut str_vec, state, is_msg_changed);

        if draw_type != DrawType::Not {
            StatusBar::draw(&mut str_vec, &self.hbar.file_vec[self.idx], &mut self.tabs[self.idx])
        }

        self.draw_init_info(&mut str_vec);

        if self.state.is_ctx_menu {
            self.ctx_menu_group.draw(&mut str_vec);
        }
        // for Right click menu

        Log::debug("cur", &self.curt().editor.cur);
        Log::debug("offset_x", &self.curt().editor.offset_x);
        Log::debug("offset_disp_x", &self.curt().editor.offset_disp_x);
        Log::debug("offset_y", &self.curt().editor.offset_y);
        Log::debug("offset_y_org", &self.curt().editor.offset_y_org);
        Log::debug("history.undo_vec", &self.curt().editor.history.undo_vec);
        // Log::debug("self.curt().state.key_record_state", &self.curt().state.key_record_state);
        Log::debug("self.curt().state", &self.curt().state);
        Log::debug("sel_range", &self.curt().editor.sel);
        // Log::debug("box_sel.mode", &self.curt().editor.box_insert.mode);

        let _ = out.write(str_vec.concat().as_bytes());
        out.flush().unwrap();

        Log::info_key("Terminal.draw end");
    }

    pub fn draw_cur<T: Write>(out: &mut T, term: &mut Terminal) {
        let mut str_vec: Vec<String> = vec![];

        if term.state.is_ctx_menu {
            Terminal::hide_cur();
        } else if term.curt().state.is_editor_cur() {
            let rnw_margin = if term.mouse_mode == MouseMode::Normal { term.curt().editor.get_rnw_and_margin() } else { 0 };
            let editor = &term.curt().editor;
            str_vec.push(MoveTo((editor.cur.disp_x - editor.offset_disp_x + rnw_margin) as u16, (editor.cur.y - editor.offset_y + editor.disp_row_posi) as u16).to_string());
            Terminal::show_cur();
        } else if term.curt().state.is_prom_show_cur() {
            Terminal::show_cur();
            term.tabs[term.idx].prom.draw_cur(&mut str_vec, &term.tabs[term.idx]);
        } else {
            Terminal::hide_cur();
        }
        let _ = out.write(str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_init_info(&mut self, str_vec: &mut Vec<String>) {
        // Information display in the center when a new file is created
        if self.curt().editor.buf.len_chars() == 1 && self.curt().editor.draw_type == DrawType::None && self.idx == 0 && self.curt().state.is_nomal() && !self.curt().editor.is_changed {
            let cols = size().unwrap().0 as usize;
            let pkg_name = env!("CARGO_PKG_NAME");
            str_vec.push(format!("{}{}{}", MoveTo(0, 3), Clear(ClearType::CurrentLine), format!("{name:^w$}", name = pkg_name, w = cols - (get_str_width(pkg_name) - pkg_name.chars().count()))));
            let ver_name = format!("{}: {}", "Version", env!("CARGO_PKG_VERSION"));
            str_vec.push(format!("{}{}{}", MoveTo(0, 4), Clear(ClearType::CurrentLine), format!("{ver:^w$}", ver = ver_name, w = cols - (get_str_width(&ver_name) - ver_name.chars().count()))));

            let simple_help = LANG.simple_help_desc.clone();
            str_vec.push(format!("{}{}{}", MoveTo(0, 6), Clear(ClearType::CurrentLine), format!("{s_help:^w$}", s_help = simple_help, w = cols - (get_str_width(&simple_help) - simple_help.chars().count()))));
            let detailed_help = LANG.detailed_help_desc.clone();
            str_vec.push(format!("{}{}{}", MoveTo(0, 7), Clear(ClearType::CurrentLine), format!("{d_help:^w$}", d_help = detailed_help, w = cols - (get_str_width(&detailed_help) - detailed_help.chars().count()))));
        }
    }

    pub fn check_displayable() -> bool {
        let (cols, rows) = size().unwrap();
        // rows 12 is prompt.open_file
        if cols <= 40 || rows <= 12 {
            return false;
        }
        true
    }
    pub fn clear_display() {
        let string = format!("{}{}", Clear(ClearType::All), MoveTo(0, 0).to_string());
        let _ = stdout().write(string.as_bytes());
        stdout().flush().unwrap();
    }

    pub fn set_disp_size(&mut self) -> bool {
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
        if rows < self.hbar.disp_row_num + self.curt().prom.disp_row_num + self.help.disp_row_num + self.curt().sbar.disp_row_num {
            return false;
        }
        self.curt().prom.disp_row_posi = (rows - self.curt().prom.disp_row_num + 1 - self.help.disp_row_num - self.curt().sbar.disp_row_num) as u16 - 1;

        self.curt().mbar.disp_col_num = cols;
        self.curt().mbar.disp_readonly_row_num = if self.curt().state.is_read_only { 1 } else { 0 };
        self.curt().mbar.disp_keyrecord_row_num = if self.curt().mbar.msg_keyrecord.is_empty() { 0 } else { 1 };
        self.curt().mbar.disp_row_num = if self.curt().mbar.msg.str.is_empty() { 0 } else { 1 };

        self.curt().mbar.disp_row_posi = rows - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num - 1;
        self.curt().mbar.disp_keyrecord_row_posi = rows - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num - 1;
        self.curt().mbar.disp_readonly_row_posi = rows - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num - 1;

        self.curt().editor.disp_col_num = if self.mouse_mode == MouseMode::Normal { cols - self.curt().editor.get_rnw_and_margin() } else { cols };
        self.curt().editor.disp_row_num = rows - self.hbar.disp_row_num - self.curt().mbar.disp_readonly_row_num - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.disp_row_num;

        true
    }

    pub fn show_cur() {
        execute!(stdout(), Show).unwrap();
    }
    pub fn hide_cur() {
        execute!(stdout(), Hide).unwrap();
    }
    pub fn init() {
        Macros::init_js_engine();

        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    }
    pub fn finalize_initial() {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, Show).unwrap();
    }
    pub fn finalize() {
        Macros::exit_js_engine();

        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, ResetColor, Show).unwrap();
    }

    pub fn exit() {
        Terminal::finalize();
        exit(0);
    }

    //  pub fn open(&mut self, filenm: &String, encoding: Encoding, tab: &mut Tab) {
    pub fn open(&mut self, filenm: &str, tab: &mut Tab) -> bool {
        Log::info("File open start", &filenm);
        let path = Path::new(&filenm);

        let (is_readable, is_writable) = File::is_readable_writable(filenm);
        if !filenm.is_empty() && !path.exists() {
            Terminal::exit_file_open(&LANG.file_not_found);
        }
        // read
        let result = TextBuffer::from_path(filenm);
        // Default config
        let mut enc = Encode::UTF8;
        let mut new_line = NEW_LINE_LF_STR.to_string();
        let mut bom_exsist = None;
        match result {
            Ok((text_buf, _enc, _new_line, _bom_exsist)) => {
                enc = _enc;
                new_line = _new_line;
                bom_exsist = _bom_exsist;
                tab.editor.buf = text_buf;

                if !is_writable {
                    tab.state.is_read_only = true;
                    tab.mbar.set_readonly(&format!("{}({})", &LANG.unable_to_edit, &LANG.no_write_permission));
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    if !is_readable {
                        if self.tabs.is_empty() {
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
            let mut h_file = HeaderFile::new(filenm);
            h_file.enc = enc;
            h_file.nl = new_line.clone();
            h_file.nl_org = new_line;
            h_file.bom = bom_exsist;

            Log::info("File info", &h_file);

            self.add_tab(tab.clone(), h_file);
            self.idx = self.tabs.len() - 1;
            //  Terminal::enable_syntax_highlight(&path, tab);
            if !filenm.is_empty() {
                self.enable_syntax_highlight(path);
            }
            self.curt().editor.set_cur_default();
            Log::info_s("File open end");
            return true;
        }
        false
    }

    pub fn enable_syntax_highlight(&mut self, path: &Path) {
        let file_meta = metadata(path).unwrap();
        let ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
        //  self.editor_draw_vec[self.idx].syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
        self.editor_draw_vec[self.idx].syntax_reference = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext).map(|sr| sr.clone());

        if self.editor_draw_vec[self.idx].syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&ext) {
            self.curt().editor.is_enable_syntax_highlight = true;
        }
    }

    pub fn exit_file_open(msg: &str) {
        Terminal::finalize_initial();
        println!("{}", msg);
        Terminal::exit();
    }

    pub fn activate<T: Write>(&mut self, args: &Args, out: &mut T) {
        Log::info_key("activate");

        let _ = GREP_INFO_VEC.set(tokio::sync::Mutex::new(vec![GrepState::default()]));
        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![]));
        let _ = TAB.set(tokio::sync::Mutex::new(Tab::new()));

        #[cfg(target_os = "windows")]
        change_output_encoding();

        Log::info("Platform", &*ENV);
        if *ENV == Env::WSL {
            Log::info("Powershell enable", &*IS_POWERSHELL_ENABLE);
        }

        self.ctx_menu_group.init();

        self.open(&args.filenm, &mut Tab::new());
        self.draw(out);
        Terminal::draw_cur(out, self);
    }

    pub fn add_tab(&mut self, tab: Tab, h_file: HeaderFile) {
        self.idx = self.tabs.len();
        self.tabs.push(tab);
        self.editor_draw_vec.push(EditorDraw::default());

        self.hbar.file_vec.push(h_file.clone());
        self.hbar.disp_base_idx = USIZE_UNDEFINED;
        self.set_disp_size();

        self.curt().editor.h_file = h_file;
    }

    pub fn del_tab(&mut self, tab_idx: usize) {
        Log::debug_key("del_tab");
        Log::debug("tab_idx", &tab_idx);
        self.idx = if tab_idx == self.hbar.file_vec.len() - 1 {
            if tab_idx == 0 {
                0
            } else {
                tab_idx - 1
            }
        } else {
            self.idx
        };
        self.tabs.remove(tab_idx);
        self.editor_draw_vec.remove(tab_idx);
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
    pub fn close_tabs(&mut self, leave_tab_idx: usize) -> bool {
        Log::debug_key("close_tabs");

        self.state.close_other_than_this_tab_idx = leave_tab_idx;
        if leave_tab_idx == USIZE_UNDEFINED {
            self.state.is_all_close_confirm = true;
        } else {
            self.state.close_other_than_this_tab_idx = leave_tab_idx;
        }
        // let vec = &self.hbar.file_vec.clone();
        let mut idx = self.tabs.len();
        for _ in 0..self.tabs.len() {
            idx -= 1;
            Log::debug("idx", &idx);

            if idx == self.state.close_other_than_this_tab_idx {
                continue;
            }
            self.idx = idx;
            if self.tabs[idx].editor.is_changed {
                if !Prompt::close(self) {
                    return false;
                }
            } else {
                self.del_tab(idx);
                if self.state.close_other_than_this_tab_idx != 0 {
                    self.state.close_other_than_this_tab_idx -= 1;
                }
            }
            if self.tabs.is_empty() {
                break;
            }
        }

        if !self.tabs.is_empty() {
            Log::debug("self.idx 111", &self.idx);

            if self.state.close_other_than_this_tab_idx != USIZE_UNDEFINED {
                self.idx = self.tabs.len() - 1;
                if self.tabs.len() != 1 && self.idx == leave_tab_idx {
                    self.idx -= 1;
                }
                if self.tabs.len() == 1 {
                    self.state.close_other_than_this_tab_idx = USIZE_UNDEFINED;
                }
            }
            Log::debug("self.idx 222", &self.idx);
            Log::debug("self.curt().editor.keys", &self.curt().editor.keys);
        }

        self.tabs.is_empty()
    }

    pub fn save_all_tab(&mut self) -> bool {
        Log::debug_key("save_all_tab");
        self.state.is_all_save = true;
        let len = self.tabs.len() - 1;
        for idx in (0..=len).rev() {
            self.idx = idx;
            if Tab::save(self) {
                self.del_tab(idx);
            }
        }
        self.tabs.is_empty()
    }

    pub fn cancel_close_all_tab(&mut self) {
        self.state.is_all_close_confirm = false;
        for tab in self.tabs.iter_mut() {
            if tab.editor.is_changed {
                tab.editor.set_keys(&Keys::Null);
                tab.state.clear();
            }
        }
    }
    pub fn cancel_save_all_tab(&mut self) {
        self.state.is_all_save = false;
    }

    pub fn ctrl_mouse_capture(&mut self) {
        match self.mouse_mode {
            MouseMode::Normal => {
                for tab in self.tabs.iter_mut() {
                    tab.editor.rnw = 0;
                    tab.editor.mouse_mode = MouseMode::Mouse;
                }
                self.mouse_mode = MouseMode::Mouse;
                execute!(stdout(), DisableMouseCapture).unwrap();
            }
            MouseMode::Mouse => {
                for tab in self.tabs.iter_mut() {
                    tab.editor.rnw = tab.editor.buf.len_lines().to_string().len();
                    tab.editor.mouse_mode = MouseMode::Normal;
                }
                self.mouse_mode = MouseMode::Normal;
                execute!(stdout(), EnableMouseCapture).unwrap();
            }
        };
    }

    pub fn new_tab(&mut self) {
        // Disable the event in case of the next display
        self.curt().editor.set_keys(&Keys::Null);

        let mut new_tab = Tab::new();
        new_tab.editor.set_cur_default();
        new_tab.editor.buf.text.insert_char(new_tab.editor.buf.text.len_chars(), EOF_MARK);
        new_tab.editor.draw_type = DrawType::All;

        // let dt: DateTime<Local> = Local::now();
        // self.add_tab(new_tab, HeaderFile::new(&dt.format("%M:%S").to_string()));

        self.add_tab(new_tab, HeaderFile::new(&LANG.new_file));
    }

    pub fn next_tab(&mut self) {
        self.idx = if self.tabs.len() - 1 == self.idx { 0 } else { self.idx + 1 };
        self.curt().editor.set_keys(&Keys::Null);
        self.curt().editor.draw_type = DrawType::All;
    }

    pub fn resize(&mut self) {
        // self.set_disp_size();
        self.curt().editor.draw_type = DrawType::All;
    }

    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.idx).unwrap();
    }

    pub fn curt_h_file(&mut self) -> &mut HeaderFile {
        return self.hbar.file_vec.get_mut(self.idx).unwrap();
    }

    pub fn clear_curt_tab(&mut self) {
        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().mbar.clear();
        self.set_disp_size();
        self.curt().editor.draw_type = DrawType::All;
    }

    pub fn clear_pre_tab_status(&mut self) {
        self.idx -= 1;
        self.curt().editor.draw_type = DrawType::All;

        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().mbar.clear();
        self.set_disp_size();
        self.curt().editor.draw_type = DrawType::All;
        self.idx += 1;
    }

    pub fn clear_ctx_menu(&mut self) {
        self.state.is_ctx_menu = false;
        self.ctx_menu_group.clear();
    }

    pub fn set_draw_range_ctx_menu(&mut self) {
        Log::debug_key("set_draw_range_ctx_menu");
        match self.keycmd {
            // TODO area check
            KeyCmd::MouseDownLeft(y, x) => {
                if self.state.is_ctx_menu && !self.ctx_menu_group.is_mouse_within_range(y, x) {
                    self.state.is_ctx_menu = false;
                    self.ctx_menu_group.clear();
                    self.curt().editor.draw_type = DrawType::All;
                }
            }
            KeyCmd::MouseMove(y, x) => {
                if self.state.is_ctx_menu && self.ctx_menu_group.is_mouse_within_range(y, x) {
                    let offset_y = self.curt().editor.offset_y;
                    let hbar_disp_row_num = self.hbar.disp_row_num;
                    let editor_disp_row_num = if self.curt().editor.offset_y > 0 { self.curt().editor.disp_row_num + self.curt().editor.offset_y - hbar_disp_row_num } else { self.curt().editor.disp_row_num - hbar_disp_row_num };

                    if let Some((sy, ey)) = self.ctx_menu_group.get_draw_range(offset_y, hbar_disp_row_num, editor_disp_row_num) {
                        self.curt().editor.draw_type = DrawType::Target(sy, ey);
                    } else {
                        self.curt().editor.draw_type = DrawType::Not;
                    }
                }
            }
            _ => {}
        }
    }
    pub fn set_keys(&mut self, keys: &Keys) {
        let keywhen = if self.curt().state.is_nomal() { KeyWhen::EditorFocus } else { KeyWhen::PromptFocus };
        self.keycmd = Keybind::keys_to_keycmd(keys, keywhen);
        self.keys = *keys;
    }
}

#[derive(Debug, Clone)]
pub struct Terminal {
    pub keycmd: KeyCmd,
    pub keys: Keys,
    pub mouse_mode: MouseMode,
    pub hbar: HeaderBar,
    pub help: Help,
    pub tabs: Vec<Tab>,
    pub editor_draw_vec: Vec<EditorDraw>,
    // tab index
    pub idx: usize,
    pub state: TerminalState,
    pub ctx_menu_group: CtxMenuGroup,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal { ..Terminal::default() }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal { keycmd: KeyCmd::Null, keys: Keys::Null, mouse_mode: MouseMode::Normal, hbar: HeaderBar::new(), tabs: vec![], editor_draw_vec: vec![], idx: 0, help: Help::new(), state: TerminalState::default(), ctx_menu_group: CtxMenuGroup::default() }
    }
}
#[derive(Debug, Clone)]
pub struct TerminalState {
    pub is_all_close_confirm: bool,
    pub is_all_save: bool,
    pub close_other_than_this_tab_idx: usize,
    pub is_displayable: bool,
    pub is_ctx_menu: bool,
}

impl Default for TerminalState {
    fn default() -> Self {
        TerminalState { is_all_close_confirm: false, is_all_save: false, close_other_than_this_tab_idx: USIZE_UNDEFINED, is_displayable: true, is_ctx_menu: false }
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub filenm: String,
    pub out_config_flg: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args { filenm: String::new(), out_config_flg: false }
    }
}
impl Args {
    pub fn new(matches: &ArgMatches) -> Self {
        let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
        Args { filenm: file_path, out_config_flg: if matches.is_present("output-config") { true } else { false } }
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
