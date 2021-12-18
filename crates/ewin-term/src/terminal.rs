use crate::{
    bar::{headerbar::*, statusbar::*},
    ctx_menu::init::*,
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*, keywhen::*},
        _cfg::lang::lang_cfg::*,
        colors::*,
        def::*,
        file::*,
        global::*,
        log::*,
        model::*,
        util::*,
    },
    ewin_editor::model::*,
    ewin_prom::model::*,
    global_term,
    help::*,
    model::*,
    tab::*,
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
    ffi::OsStr,
    fmt,
    fs::metadata,
    io::{stdout, ErrorKind, Write},
    path::Path,
    process::exit,
    usize,
};

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, draw_parts: &DParts) {
        Log::info_key("Terminal.draw start");
        self.set_disp_size();

        let mut str_vec: Vec<String> = vec![];
        match draw_parts {
            DParts::All | DParts::AllMsgBar(_) if self.curt().editor.draw_range != E_DrawRange::None => self.curt().editor.draw_range = E_DrawRange::All,
            _ => {}
        };
        let draw_range_org = self.curt().editor.draw_range;

        // Editor
        match self.curt().editor.draw_range {
            E_DrawRange::Not => {}
            _ => {
                if self.curt().editor.draw_range == E_DrawRange::MoveCur {
                    StatusBar::draw_only(out, &mut self.tabs[self.idx], &self.hbar.file_vec[self.idx]);
                    return;
                } else {
                    self.editor_draw_vec[self.idx].draw_cache(&mut self.tabs[self.idx].editor);
                    self.tabs[self.idx].editor.draw(&mut str_vec, &self.editor_draw_vec[self.idx]);
                    StatusBar::draw(&mut str_vec, &mut self.tabs[self.idx], &self.hbar.file_vec[self.idx]);
                }
            }
        };
        if let &DParts::ScrollUpDown(_) | &DParts::All = draw_parts {
            HeaderBar::draw(self, &mut str_vec);
            self.help.draw(&mut str_vec);
            self.curt().mbar.draw(&mut str_vec);
            let is_msg_changed = self.curt().mbar.is_msg_changed();
            let prom_disp_row_posi = self.curt().prom.disp_row_posi;
            let h_file = &self.curt_h_file().clone();
            let state = &self.curt().state.clone();
            self.curt().prom.draw(&mut str_vec, prom_disp_row_posi, state, is_msg_changed, h_file);
        }
        if draw_parts == &DParts::All || draw_parts == &DParts::Editor {
            Log::info("self.state.is_ctx_menu", &self.state.is_ctx_menu);

            if self.state.is_ctx_menu {
                // self.set_draw_range_ctx_menu();
                self.ctx_menu_group.draw(&mut str_vec);
            }
        }
        self.draw_init_info(&mut str_vec, draw_range_org);

        Log::debug("cur", &self.curt().editor.cur);
        Log::debug("offset_x", &self.curt().editor.offset_x);
        Log::debug("offset_disp_x", &self.curt().editor.offset_disp_x);
        Log::debug("offset_y", &self.curt().editor.offset_y);
        Log::debug("offset_y_org", &self.curt().editor.offset_y_org);
        Log::debug("history.undo_vec", &self.curt().editor.history.undo_vec);
        // Log::debug("self.curt().state.key_record_state", &self.curt().state.key_record_state);
        //  Log::debug("self.curt().state", &self.curt().state);
        Log::debug("sel_range", &self.curt().editor.sel);
        // Log::debug("", &self.curt().editor.search);
        // Log::debug("box_sel.mode", &self.curt().editor.box_insert.mode);

        self.draw_flush(out, str_vec);

        Log::info_key("Terminal.draw end");
    }

    #[cfg(not(target_os = "windows"))]
    pub fn draw_flush<T: Write>(&mut self, out: &mut T, str_vec: Vec<String>) {
        let _ = out.write(str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    // Suppress the number of flushes due to the following error when trying to flush a large amount of data in windows
    // Error:Windows stdio in console mode does not support writing non-UTF-8 byte sequences
    #[cfg(target_os = "windows")]
    pub fn draw_flush<T: Write>(&mut self, out: &mut T, str_vec: Vec<String>) {
        let string = str_vec.concat();
        // NEW_LINE_LF is mark
        let vec = split_inclusive(&string, NEW_LINE_LF);
        for string in vec {
            let _ = out.write(string.as_bytes());
            out.flush().unwrap();
        }
    }

    pub fn draw_cur<T: Write>(&mut self, out: &mut T) {
        let mut str_vec: Vec<String> = vec![];

        if self.state.is_ctx_menu {
            self.ctx_menu_group.draw_cur();
        } else if self.curt().state.is_editor_cur() {
            if self.curt().mbar.is_exsist_msg() && self.hbar.row_num + self.curt().editor.cur.y - self.curt().editor.offset_y == self.curt().mbar.disp_row_posi {
                self.curt().editor.cur_up();
            }
            let rnw_margin = if self.curt().editor.state.mouse_mode == MouseMode::Normal { self.curt().editor.get_rnw_and_margin() } else { 0 };
            let editor = &self.curt().editor;
            str_vec.push(MoveTo((editor.cur.disp_x - editor.offset_disp_x + rnw_margin) as u16, (editor.cur.y - editor.offset_y + editor.row_posi) as u16).to_string());

            Terminal::show_cur();
        } else if self.curt().state.is_prom_show_cur() {
            Terminal::show_cur();
            self.tabs[self.idx].prom.draw_cur(&mut str_vec, &self.tabs[self.idx].state);
        } else {
            Terminal::hide_cur();
        }
        if !str_vec.is_empty() {
            let _ = out.write(str_vec.concat().as_bytes());
            out.flush().unwrap();
        }
    }

    pub fn draw_all<T: Write>(&mut self, out: &mut T, draw_parts: DParts) {
        self.draw(out, &draw_parts);
        self.draw_cur(out);
    }

    pub fn draw_init_info(&mut self, str_vec: &mut Vec<String>, draw_range_org: E_DrawRange) {
        Log::debug_key("Terminal.draw_init_info");
        // Information display in the center when a new file is created

        Log::debug("self.curt().editor.buf.len_chars()", &self.curt().editor.buf.len_chars());
        Log::debug("draw_type_org", &draw_range_org);
        Log::debug(" self.idx", &self.idx);
        Log::debug("self.curt().state.is_nomal()", &self.curt().state.is_nomal());
        Log::debug("self.curt().editor.state.is_changed", &self.curt().editor.state.is_changed);

        if self.curt().editor.buf.len_chars() == 1 && draw_range_org == E_DrawRange::None && self.idx == 0 && self.curt().state.is_nomal() && !self.curt().editor.state.is_changed {
            self.state.is_show_init_info = true;

            let cols = get_term_size().0 as usize;
            let pkg_name = APP_NAME;
            Colors::set_text_color(str_vec);
            str_vec.push(format!("{}{}{}", MoveTo(0, 3), Clear(ClearType::CurrentLine), format!("{name:^w$}", name = pkg_name, w = cols - (get_str_width(pkg_name) - pkg_name.chars().count()))));

            let ver_name = &format!("{}: {}", "Version", &(*APP_VERSION.get().unwrap().to_string()));

            str_vec.push(format!("{}{}{}", MoveTo(0, 4), Clear(ClearType::CurrentLine), format!("{ver:^w$}", ver = ver_name, w = cols - (get_str_width(ver_name) - ver_name.chars().count()))));

            let simple_help = Lang::get().simple_help_desc.clone();
            str_vec.push(format!("{}{}{}", MoveTo(0, 6), Clear(ClearType::CurrentLine), format!("{s_help:^w$}", s_help = simple_help, w = cols - (get_str_width(&simple_help) - simple_help.chars().count()))));
            let detailed_help = Lang::get().detailed_help_desc.clone();
            str_vec.push(format!("{}{}{}", MoveTo(0, 7), Clear(ClearType::CurrentLine), format!("{d_help:^w$}", d_help = detailed_help, w = cols - (get_str_width(&detailed_help) - detailed_help.chars().count()))));
        }
    }

    pub fn check_displayable() -> bool {
        let (cols, rows) = get_term_size();
        // rows 12 is prompt.open_file
        if cols <= TERM_MINIMUM_WIDTH as u16 || rows <= TERM_MINIMUM_HEIGHT as u16 {
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
        let (cols, rows) = get_term_size();
        let (cols, rows) = (cols as usize, rows as usize);
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        self.hbar.set_posi(cols);
        HeaderBar::set_header_filenm(self);

        self.help.disp_col_num = cols;
        self.help.disp_row_num = if self.help.mode == HelpMode::Show { Help::DISP_ROW_NUM } else { 0 };
        self.help.disp_row_posi = if self.help.mode == HelpMode::Show { rows - self.help.disp_row_num } else { 0 };

        self.curt().sbar.row_num = 1;
        let help_disp_row_num = if self.help.disp_row_num > 0 { self.help.disp_row_num + 1 } else { 0 };
        self.curt().sbar.row_posi = if help_disp_row_num == 0 { rows - 1 } else { rows - help_disp_row_num };
        self.curt().sbar.col_num = cols;

        self.curt().prom.disp_col_num = cols;

        if self.curt().state.is_open_file {
            // -1 is MsgBar
            self.curt().prom.disp_row_num = rows - self.hbar.row_num - self.help.disp_row_num - self.curt().sbar.row_num - 1;
            if self.curt().prom.disp_row_num < Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM + 1 {
                return false;
            }
        }
        if rows < self.hbar.row_num + self.curt().prom.disp_row_num + self.help.disp_row_num + self.curt().sbar.row_num {
            return false;
        }
        self.curt().prom.disp_row_posi = (rows - self.curt().prom.disp_row_num + 1 - self.help.disp_row_num - self.curt().sbar.row_num) as u16 - 1;

        self.curt().mbar.disp_col_num = cols;
        self.curt().mbar.disp_readonly_row_num = if self.curt().editor.state.is_read_only { 1 } else { 0 };
        self.curt().mbar.disp_keyrecord_row_num = if self.curt().mbar.msg_keyrecord.is_empty() { 0 } else { 1 };
        self.curt().mbar.disp_row_num = if self.curt().mbar.msg.str.is_empty() { 0 } else { 1 };

        self.curt().mbar.disp_row_posi = rows - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.row_num - 1;
        self.curt().mbar.disp_keyrecord_row_posi = rows - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.row_num - 1;
        self.curt().mbar.disp_readonly_row_posi = rows - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.row_num - 1;

        self.curt().editor.col_num = if self.curt().editor.state.mouse_mode == MouseMode::Normal { cols - self.curt().editor.get_rnw_and_margin() } else { cols };
        self.curt().editor.row_len = rows - self.hbar.row_num - self.curt().mbar.disp_readonly_row_num - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.row_num;

        if self.curt().editor.row_len < self.curt().editor.buf.len_rows() {
            {
                self.curt().editor.col_num -= CFG.get().unwrap().try_lock().unwrap().general.editor.scrollbar.vertical.width;
            }
        }

        Log::debug("self.curt().editor.row_num ", &self.curt().editor.row_len);
        Log::debug("self.curt().editor.row_num ", &CFG.get().unwrap().try_lock().unwrap().general.editor.scrollbar.vertical.width);

        return true;
    }

    pub fn show_cur() {
        execute!(stdout(), Show).unwrap();
    }
    pub fn hide_cur() {
        execute!(stdout(), Hide).unwrap();
    }

    pub fn set_title<T: fmt::Display>(_f: T) {
        #[cfg(target_os = "windows")]
        execute!(stdout(), SetTitle(_f)).unwrap();
    }

    pub fn clear_all() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
    pub fn init() {
        Macros::init_js_engine();

        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();

        #[cfg(target_os = "windows")]
        change_output_encoding();

        Log::info("Platform", &*ENV);
        if *ENV == Env::WSL {
            Log::info("Powershell enable", &*IS_POWERSHELL_ENABLE);
        }
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

    pub fn open_file(&mut self, filenm: &str, tab_opt: Option<&mut Tab>, file_open_type: FileOpenType) -> ActType {
        Log::info("File open start", &filenm);

        let path = Path::new(&filenm);
        let (is_readable, is_writable) = File::is_readable_writable(filenm);

        if !filenm.is_empty() && !path.exists() {
            if file_open_type == FileOpenType::First {
                Terminal::exit_file_open(&Lang::get().file_not_found);
                return ActType::Exit;
            } else {
                return ActType::Draw(DParts::MsgBar(Lang::get().file_not_found.to_string()));
            };
        } else {
            let mut h_file = HeaderFile::new(filenm);
            let mut tab = if let Some(tab) = tab_opt { tab.clone() } else { self.curt().clone() };

            if filenm.is_empty() {
                if file_open_type == FileOpenType::First || file_open_type == FileOpenType::Nomal {
                    tab.editor.buf.text.insert_char(tab.editor.buf.text.len_chars(), EOF_MARK);
                }
            } else {
                // read
                let result = TextBuffer::read_file(filenm);
                match result {
                    Ok((text_buf, _enc, _new_line, _bom_exsist, _modified_time)) => {
                        h_file.enc = _enc;
                        h_file.nl = _new_line;
                        h_file.bom = _bom_exsist;
                        tab.editor.buf = text_buf;
                        h_file.modified_time = _modified_time;

                        if !is_writable {
                            tab.editor.state.is_read_only = true;
                            tab.mbar.set_readonly(&format!("{}({})", &Lang::get().unable_to_edit, &Lang::get().no_write_permission));
                        }
                    }
                    Err(err) => {
                        let err_str = if err.kind() == ErrorKind::PermissionDenied && !is_readable { Lang::get().no_read_permission.clone() } else { format!("{} {:?}", &Lang::get().file_opening_problem, err) };
                        if self.tabs.is_empty() {
                            Terminal::exit_file_open(&err_str);
                        } else {
                            return ActType::Draw(DParts::MsgBar(err_str));
                        }
                    }
                }
            }

            Terminal::set_title(&filenm);
            Log::info("Title", &h_file);
            Log::info("File info", &h_file);

            match file_open_type {
                FileOpenType::First | FileOpenType::Nomal => {
                    self.add_tab(tab.clone(), h_file);
                    self.curt().editor.set_cur_default();
                }
                FileOpenType::Reopen => {
                    self.change_curt_tab(tab.clone(), h_file);
                    self.curt().editor.e_cmd = E_Cmd::ReOpenFile;
                    self.curt().editor.adjust_cur_posi();
                }
            };
            if !filenm.is_empty() {
                self.enable_syntax_highlight(path);
            }
            Log::info_s("File open end");

            return ActType::Next;
        }
    }

    pub fn enable_syntax_highlight(&mut self, path: &Path) {
        let file_meta = metadata(path).unwrap();
        let ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
        //  self.editor_draw_vec[self.idx].syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
        self.editor_draw_vec[self.idx].syntax_reference = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext).cloned();

        if self.editor_draw_vec[self.idx].syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&ext) {
            self.curt().editor.is_enable_syntax_highlight = true;
        }
    }

    pub fn exit_file_open(msg: &str) {
        Terminal::finalize_initial();
        println!("{}", msg);
        Terminal::exit();
    }

    pub fn activate(&mut self, args: &Args) {
        Log::info_key("activate");

        let _ = GREP_INFO_VEC.set(tokio::sync::Mutex::new(vec![GrepState::default()]));
        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![]));
        let _ = global_term::TAB.set(tokio::sync::Mutex::new(Tab::new()));

        self.open_file(&args.filenm, Some(&mut Tab::new()), FileOpenType::First);

        self.ctx_menu_group.init();
    }

    pub fn init_draw<T: Write>(&mut self, out: &mut T) {
        self.draw(out, &DParts::All);
        self.draw_cur(out);
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
    pub fn change_curt_tab(&mut self, tab: Tab, h_file: HeaderFile) {
        self.tabs[self.idx] = tab;
        self.editor_draw_vec[self.idx].clear();

        self.hbar.file_vec[self.idx] = h_file.clone();
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
        }
        let mut idx = self.tabs.len();

        for _ in 0..self.tabs.len() {
            idx -= 1;
            Log::debug("idx", &idx);
            if idx == self.state.close_other_than_this_tab_idx {
                continue;
            }
            self.idx = idx;
            if self.tabs[idx].editor.state.is_changed {
                if !Tab::prom_save_confirm(self) {
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
        }

        self.tabs.is_empty()
    }

    pub fn save_all_tab(&mut self) -> ActType {
        Log::debug_key("save_all_tab");
        self.state.is_all_save = true;
        let len = self.tabs.len() - 1;
        for idx in (0..=len).rev() {
            self.idx = idx;
            let act_type = Tab::save(self, false);
            if let ActType::Draw(_) = act_type {
                return act_type;
            } else {
                self.del_tab(idx);
            }
        }
        return ActType::Next;
    }

    pub fn cancel_close_all_tab(&mut self) {
        self.state.is_all_close_confirm = false;
        for tab in self.tabs.iter_mut() {
            if tab.editor.state.is_changed {
                tab.editor.set_cmd(KeyCmd::Null);
                tab.state.clear();
            }
        }
    }
    pub fn cancel_save_all_tab(&mut self) {
        self.state.is_all_save = false;
    }

    pub fn new_tab(&mut self) {
        // Disable the event in case of the next display
        self.curt().editor.set_cmd(KeyCmd::Null);

        let mut new_tab = Tab::new();
        new_tab.editor.set_cur_default();
        new_tab.editor.buf.insert_end(&EOF_MARK.to_string());
        new_tab.editor.draw_range = E_DrawRange::All;

        // let dt: DateTime<Local> = Local::now();
        // self.add_tab(new_tab, HeaderFile::new(&dt.format("%M:%S").to_string()));

        self.add_tab(new_tab, HeaderFile::new(&Lang::get().new_file));
        Terminal::set_title(&self.curt_h_file().fullpath);
    }

    pub fn switch_tab(&mut self, direction: Direction) -> ActType {
        if self.tabs.len() > 1 {
            self.idx = if direction == Direction::Right {
                if self.tabs.len() - 1 == self.idx {
                    0
                } else {
                    self.idx + 1
                }
            } else if self.idx == 0 {
                self.tabs.len() - 1
            } else {
                self.idx - 1
            };
            Terminal::set_title(&self.curt_h_file().fullpath);
            return ActType::Draw(DParts::All);
        } else {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_tab_can_be_switched.to_string()));
        }
    }

    pub fn resize(&mut self) {
        self.curt().editor.draw_range = E_DrawRange::All;
    }

    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.idx).unwrap();
    }

    pub fn curt_h_file(&mut self) -> &mut HeaderFile {
        return self.hbar.file_vec.get_mut(self.idx).unwrap();
    }

    pub fn clear_curt_tab(&mut self, is_clear_grep_info: bool) {
        Log::debug_key("clear_curt_tab");
        self.curt().prom.clear();
        self.curt().state.clear();
        if is_clear_grep_info {
            self.curt().state.grep.clear();
        }
        self.curt().mbar.clear();
        // self.set_disp_size();
        self.curt().editor.draw_range = E_DrawRange::All;
    }

    pub fn clear_pre_tab_status(&mut self) {
        self.idx -= 1;

        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().mbar.clear();
        self.set_disp_size();
        self.curt().editor.draw_range = E_DrawRange::All;
        self.idx += 1;
    }

    pub fn clear_ctx_menu(&mut self) {
        Log::debug_key("clear_ctx_menu");
        self.state.is_ctx_menu = false;
        self.state.is_ctx_menu_hide_draw = true;
        self.ctx_menu_group.clear();
    }

    pub fn set_draw_range_ctx_menu(&mut self) {
        Log::debug_key("set_draw_range_ctx_menu");
        match self.keycmd {
            KeyCmd::CtxMenu(C_Cmd::MouseDownLeft(y, x)) => {
                if self.state.is_ctx_menu && !self.ctx_menu_group.is_mouse_within_range(y, x, false) {
                    self.state.is_ctx_menu = false;
                    self.ctx_menu_group.clear();
                    self.curt().editor.draw_range = E_DrawRange::All;
                }
            }
            KeyCmd::CtxMenu(C_Cmd::MouseMove(y, x)) => {
                if self.state.is_ctx_menu && self.ctx_menu_group.is_mouse_within_range(y, x, false) {
                    self.set_editor_draw_target_for_ctxmenu_redraw();
                }
            }
            KeyCmd::CtxMenu(C_Cmd::CursorDown) | KeyCmd::CtxMenu(C_Cmd::CursorUp) | KeyCmd::CtxMenu(C_Cmd::CursorRight) | KeyCmd::CtxMenu(C_Cmd::CursorLeft) => {
                self.set_editor_draw_target_for_ctxmenu_redraw();
            }
            _ => {}
        }
    }

    pub fn set_editor_draw_target_for_ctxmenu_redraw(&mut self) {
        let (offset_y, hbar_disp_row_num, editor_row_len) = (self.curt().editor.offset_y, self.hbar.row_num, self.curt().editor.row_len);
        self.curt().editor.draw_range = if let Some((sy, ey)) = self.ctx_menu_group.get_draw_range(offset_y, hbar_disp_row_num, editor_row_len) { E_DrawRange::Target(sy, ey) } else { E_DrawRange::Not }
    }

    pub fn set_keys(&mut self, keys: &Keys) {
        let keywhen = self.get_when(keys);
        Log::debug("Terminal.set_keys.keywhen", &keywhen);

        let hbar_row_posi = self.hbar.row_posi;
        let sbar_row_posi = self.curt().sbar.row_posi;
        self.keycmd = Keybind::keys_to_keycmd_pressed(keys, Some(&self.keys_org), keywhen, hbar_row_posi, sbar_row_posi);
        Log::debug("Terminal.set_keys.keycmd", &self.keycmd);
        self.keys = *keys;
    }

    pub fn get_when(&mut self, keys: &Keys) -> KeyWhen {
        Log::debug("self.state", &self.state);
        Log::debug("self.curt().state", &self.curt().state);
        Log::debug("keys", &keys);
        Log::debug("self.state.is_ctx_menu", &self.state.is_ctx_menu);

        return if self.judge_when_headerbar(keys, self.hbar.row_posi) {
            KeyWhen::HeaderBarFocus
        } else if self.curt().state.judge_when_prompt(keys) {
            KeyWhen::PromptFocus
        } else if self.state.is_ctx_menu {
            // KeyWhen::CtxMenuFocus

            if EvtAct::is_ctrl_ctx_keys(keys, self) {
                KeyWhen::CtxMenuFocus
            } else {
                self.clear_ctx_menu();
                KeyWhen::EditorFocus
            }
        } else {
            let sbar_row_posi = self.curt().sbar.row_posi;
            if self.curt().state.judge_when_statusbar(keys, sbar_row_posi) {
                KeyWhen::StatusBarFocus
            } else {
                KeyWhen::EditorFocus
            }
        };
    }
    pub fn judge_when_headerbar(&self, keys: &Keys, hbar_row_posi: usize) -> bool {
        match keys {
            Keys::MouseDownLeft(y, _)
             // | Keys::MouseDragLeft(y, _)
              => {
                if y == &(hbar_row_posi as u16) {
                    return true;
                }
                return false;
            }
             _ => return false,
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        Terminal { ..Terminal::default() }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal { draw_parts_org: DParts::All, keycmd: KeyCmd::Null, keys: Keys::Null, keys_org: Keys::Null, hbar: HeaderBar::new(), tabs: vec![], editor_draw_vec: vec![], idx: 0, help: Help::new(), state: TerminalState::default(), ctx_menu_group: CtxMenuGroup::default() }
    }
}

impl Default for TerminalState {
    fn default() -> Self {
        TerminalState { is_show_init_info: false, is_all_close_confirm: false, is_all_save: false, close_other_than_this_tab_idx: USIZE_UNDEFINED, is_displayable: true, is_ctx_menu: false, is_ctx_menu_hide_draw: false }
    }
}

/*
impl UT {

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

}
*/
