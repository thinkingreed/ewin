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
use ewin_com::_cfg::cfg::{Cfg, CfgSyntax};
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
    pub fn render<T: Write>(&mut self, out: &mut T, draw_parts: &DParts) {
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
                    StatusBar::draw(&mut str_vec, &mut self.tabs[self.idx], &self.hbar.file_vec[self.idx]);
                    self.curt().editor.draw_scrlbar_h(&mut str_vec);
                    self.curt().editor.draw_scrlbar_v(&mut str_vec);
                    self.render_flush(out, str_vec);
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
        self.render_init_info(&mut str_vec, draw_range_org);

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
        Log::debug("scrl_v.is_enable", &self.curt().editor.scrl_v.is_enable);
        Log::debug("scrl_h.is_enable", &self.curt().editor.scrl_h.is_enable);

        self.render_flush(out, str_vec);

        Log::info_key("Terminal.draw end");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn render_flush<T: Write>(&mut self, out: &mut T, str_vec: Vec<String>) {
        let _ = out.write(str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    // Suppress the number of flushes due to the following error when trying to flush a large amount of data in windows
    // Error:Windows stdio in console mode does not support writing non-UTF-8 byte sequences
    #[cfg(target_os = "windows")]
    pub fn render_flush<T: Write>(&mut self, out: &mut T, str_vec: Vec<String>) {
        for string in str_vec {
            let _ = out.write(string.as_bytes());
        }
        out.flush().unwrap();
    }

    pub fn render_cur<T: Write>(&mut self, out: &mut T) {
        let mut str_vec: Vec<String> = vec![];

        if self.state.is_ctx_menu {
            self.ctx_menu_group.render_cur();
        } else if self.curt().state.is_nomal() && self.curt().editor.is_cur_y_in_screen() {
            if self.curt().mbar.is_exsist_msg() && self.hbar.row_num + self.curt().editor.cur.y - self.curt().editor.offset_y == self.curt().mbar.disp_row_posi {
                self.curt().editor.cur_up();
            }
            let rnw_margin = if self.curt().editor.state.mouse_mode == MouseMode::Normal { self.curt().editor.get_rnw_and_margin() } else { 0 };
            let editor = &self.curt().editor;

            if editor.offset_disp_x <= editor.cur.disp_x && editor.cur.disp_x <= editor.offset_disp_x + editor.col_len && editor.offset_y <= editor.cur.y && editor.cur.y <= editor.offset_y + editor.row_disp_len {
                str_vec.push(MoveTo((editor.cur.disp_x - editor.offset_disp_x + rnw_margin) as u16, (editor.cur.y - editor.offset_y + editor.row_posi) as u16).to_string());
            }

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

    pub fn render_all<T: Write>(&mut self, out: &mut T, draw_parts: DParts) {
        self.render(out, &draw_parts);
        self.render_cur(out);
    }

    pub fn render_init_info(&mut self, str_vec: &mut Vec<String>, draw_range_org: E_DrawRange) {
        Log::debug_key("Terminal.draw_init_info");
        // Information display in the center when a new file is created

        Log::debug("self.curt().editor.buf.len_chars()", &self.curt().editor.buf.len_chars());
        Log::debug("draw_type_org", &draw_range_org);
        Log::debug(" self.idx", &self.idx);
        Log::debug("self.curt().state.is_nomal()", &self.curt().state.is_nomal());
        Log::debug("self.curt().editor.state.is_changed", &self.curt().editor.state.is_changed);

        if self.curt().editor.buf.len_chars() == 0 && draw_range_org == E_DrawRange::None && self.idx == 0 && self.curt().state.is_nomal() && !self.curt().editor.state.is_changed {
            self.state.is_show_init_info = true;

            let cols = get_term_size().0 as usize;
            let pkg_name = APP_NAME;
            Colors::set_text_color(str_vec);
            let pkg_name = format!("{name:^w$}", name = pkg_name, w = cols - (get_str_width(pkg_name) - pkg_name.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, 3), Clear(ClearType::CurrentLine), pkg_name));

            let ver_name = &format!("{}: {}", "Version", &(*APP_VERSION.get().unwrap().to_string()));
            let ver_name = format!("{ver:^w$}", ver = ver_name, w = cols - (get_str_width(ver_name) - ver_name.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, 4), Clear(ClearType::CurrentLine), ver_name));

            let simple_help = Lang::get().simple_help_desc.clone();
            let simple_help = format!("{s_help:^w$}", s_help = simple_help, w = cols - (get_str_width(&simple_help) - simple_help.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, 6), Clear(ClearType::CurrentLine), simple_help));
            let detailed_help = Lang::get().detailed_help_desc.clone();
            let detailed_help = format!("{d_help:^w$}", d_help = detailed_help, w = cols - (get_str_width(&detailed_help) - detailed_help.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, 7), Clear(ClearType::CurrentLine), detailed_help));
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
        let string = format!("{}{}", Clear(ClearType::All), MoveTo(0, 0));
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

        self.curt().editor.col_len = if self.curt().editor.state.mouse_mode == MouseMode::Normal { cols - self.curt().editor.get_rnw_and_margin() } else { cols };
        self.curt().editor.row_disp_len = rows - self.hbar.row_num - self.curt().mbar.disp_readonly_row_num - self.curt().mbar.disp_keyrecord_row_num - self.curt().mbar.disp_row_num - self.curt().prom.disp_row_num - self.help.disp_row_num - self.curt().sbar.row_num;

        if self.curt().editor.row_disp_len < self.curt().editor.buf.len_rows() {
            {
                self.curt().editor.scrl_v.is_show = true;
                self.curt().editor.col_len -= Cfg::get().general.editor.scrollbar.vertical.width;
            }
        } else {
            self.curt().editor.scrl_v.is_show = false;
        }

        if self.curt().editor.scrl_h.row_max_width > self.curt().editor.col_len && self.curt().state.is_nomal() {
            self.curt().editor.scrl_h.is_show = true;
            {
                self.curt().editor.row_disp_len -= Cfg::get().general.editor.scrollbar.horizontal.height;
                self.curt().editor.scrl_h.row_posi = self.curt().editor.row_disp_len + 1;
            }
        } else {
            self.curt().editor.scrl_h.is_show = false;
        }

        return true;
    }

    pub fn show_cur() {
        execute!(stdout(), Show).unwrap();
    }

    pub fn hide_cur() {
        execute!(stdout(), Hide).unwrap();
    }

    pub fn set_title<T: fmt::Display>(_f: T) {
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

    pub fn open_file(&mut self, filenm: &str, file_open_type: FileOpenType, tab_opt: Option<&mut Tab>, h_file_opt: Option<&HeaderFile>) -> ActType {
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
            if let Some(h_file_org) = h_file_opt {
                h_file.watch_mode = h_file_org.watch_mode;
            }
            let mut tab = if let Some(tab) = tab_opt { tab.clone() } else { self.curt().clone() };

            if !filenm.is_empty() {
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

            Log::info("File info", &h_file);

            match file_open_type {
                FileOpenType::First | FileOpenType::Nomal => {
                    self.add_tab(tab.clone(), h_file, file_open_type);
                    self.curt().editor.set_cur_default();
                }
                FileOpenType::Reopen => {
                    self.reopen_tab(tab.clone(), h_file, file_open_type);
                    self.curt().editor.e_cmd = E_Cmd::ReOpenFile;
                    self.curt().editor.state.is_changed = false;
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

    pub fn reopen_curt_file(&mut self) {
        self.clear_curt_tab(true);
        self.set_disp_size();
        let h_file = self.curt_h_file().clone();
        self.open_file(&h_file.fullpath, FileOpenType::Reopen, None, Some(&h_file));
    }

    pub fn enable_syntax_highlight(&mut self, path: &Path) {
        let file_meta = metadata(path).unwrap();
        let ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
        //  self.editor_draw_vec[self.idx].syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
        self.editor_draw_vec[self.idx].syntax_reference = CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&ext).cloned();

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
        let _ = WATCH_INFO.set(tokio::sync::Mutex::new(WatchInfo::default()));

        self.open_file(&args.filenm, FileOpenType::First, Some(&mut Tab::new()), None);

        self.ctx_menu_group.init();
    }

    pub fn init_draw<T: Write>(&mut self, out: &mut T) {
        self.render(out, &DParts::All);
        self.render_cur(out);
    }

    pub fn add_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        self.idx = self.tabs.len();
        self.tabs.push(tab);
        self.editor_draw_vec.push(EditorDraw::default());
        self.hbar.file_vec.push(h_file.clone());
        self.hbar.disp_base_idx = USIZE_UNDEFINED;

        self.init_tab(&h_file, file_open_type);
    }
    pub fn change_tab(&mut self, idx: usize) {
        self.idx = idx;
        let h_file = self.hbar.file_vec[self.idx].clone();
        self.init_tab(&h_file, FileOpenType::Nomal);
    }

    pub fn reopen_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        self.tabs[self.idx] = tab;
        self.editor_draw_vec[self.idx].clear();
        self.hbar.file_vec[self.idx] = h_file.clone();
        self.init_tab(&h_file, file_open_type);
    }

    pub fn init_tab(&mut self, h_file: &HeaderFile, file_open_type: FileOpenType) {
        self.curt().editor.calc_scrlbar_h();
        self.curt().editor.calc_scrlbar_v();
        self.set_disp_size();
        self.curt().editor.h_file = h_file.clone();
        if file_open_type != FileOpenType::Reopen && File::is_exist_file(&h_file.fullpath) {
            if let Some(Ok(mut watch_info)) = WATCH_INFO.get().map(|watch_info| watch_info.try_lock()) {
                watch_info.fullpath = h_file.fullpath.clone();
                watch_info.mode = h_file.watch_mode;
            }
        }
        Terminal::set_title(&h_file.filenm);
    }

    pub fn del_tab(&mut self, tab_idx: usize) {
        Log::debug_key("del_tab");
        Log::debug("tab_idx", &tab_idx);
        self.idx = if tab_idx == self.hbar.file_vec.len() - 1 && tab_idx != 0 { tab_idx - 1 } else { self.idx };
        self.tabs.remove(tab_idx);
        self.editor_draw_vec.remove(tab_idx);
        self.hbar.file_vec.remove(tab_idx);
        self.hbar.disp_base_idx = USIZE_UNDEFINED;
        self.change_tab(self.idx);

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
        new_tab.editor.draw_range = E_DrawRange::All;

        // let dt: DateTime<Local> = Local::now();
        // self.add_tab(new_tab, HeaderFile::new(&dt.format("%M:%S").to_string()));

        self.add_tab(new_tab, HeaderFile::new(&Lang::get().new_file), FileOpenType::Nomal);
    }

    pub fn switch_tab(&mut self, direction: Direction) -> ActType {
        if self.tabs.len() > 1 {
            let idx = if direction == Direction::Right {
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
            self.change_tab(idx);
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

    pub fn set_render_range_ctx_menu(&mut self) {
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
                    self.set_editor_render_target_for_ctxmenu_rerender();
                }
            }
            KeyCmd::CtxMenu(C_Cmd::CursorDown) | KeyCmd::CtxMenu(C_Cmd::CursorUp) | KeyCmd::CtxMenu(C_Cmd::CursorRight) | KeyCmd::CtxMenu(C_Cmd::CursorLeft) => {
                self.set_editor_render_target_for_ctxmenu_rerender();
            }
            _ => {}
        }
    }

    pub fn set_editor_render_target_for_ctxmenu_rerender(&mut self) {
        let (offset_y, hbar_disp_row_num, editor_row_len) = (self.curt().editor.offset_y, self.hbar.row_num, self.curt().editor.row_disp_len);
        self.curt().editor.draw_range = if let Some((sy, ey)) = self.ctx_menu_group.get_draw_range(offset_y, hbar_disp_row_num, editor_row_len) { E_DrawRange::Target(sy, ey) } else { E_DrawRange::Not }
    }

    pub fn set_keys(&mut self, keys: &Keys) {
        let keywhen = self.get_when(keys);
        Log::debug("Terminal.set_keys.keywhen", &keywhen);
        self.keycmd = Keybind::keys_to_keycmd_pressed(keys, Some(&self.keys_org), keywhen);
        Log::debug("Terminal.set_keys.keycmd", &self.keycmd);
        self.keys = *keys;
    }

    pub fn get_when(&mut self, keys: &Keys) -> KeyWhen {
        Log::debug("keys", &keys);
        Log::debug("self.state", &self.state);
        Log::debug("self.curt().state", &self.curt().state);
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
