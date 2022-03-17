use crate::{
    bar::{headerbar::*, statusbar::*},
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*, keywhen::*},
        _cfg::lang::lang_cfg::*,
        _cfg::model::default::*,
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
use chrono::{DateTime, Local};
use crossterm::{
    cursor::*,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::ResetColor,
    terminal::*,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ewin_window::{model::CtxMenu, window::WindowTrait};
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
    pub fn render<T: Write>(&mut self, out: &mut T, draw_parts: &RParts) {
        Log::info_key("Terminal.render start");
        Log::debug("draw_parts", &draw_parts);
        Log::debug("self.curt().editor.draw_range", &self.curt().editor.draw_range);
        self.set_disp_size();

        let mut str_vec: Vec<String> = vec![];
        match draw_parts {
            RParts::All | RParts::AllMsgBar(_) => {
                if self.curt().editor.draw_range != E_DrawRange::Init {
                    self.curt().editor.draw_range = E_DrawRange::All;
                }
            }
            _ => {}
        };

        // Editor
        match self.curt().editor.draw_range {
            E_DrawRange::Not => {}
            _ => {
                if self.curt().editor.draw_range == E_DrawRange::MoveCur {
                    StatusBar::render(&mut str_vec, &mut self.tabs[self.tab_idx], &self.hbar.file_vec[self.tab_idx]);
                    self.tabs[self.tab_idx].editor.draw(&mut str_vec, &mut self.editor_draw_vec[self.tab_idx]);
                    self.render_flush(out, &mut str_vec);
                    return;
                } else {
                    self.editor_draw_vec[self.tab_idx].draw_cache(&mut self.tabs[self.tab_idx].editor);
                    self.tabs[self.tab_idx].editor.draw(&mut str_vec, &mut self.editor_draw_vec[self.tab_idx]);
                    StatusBar::render(&mut str_vec, &mut self.tabs[self.tab_idx], &self.hbar.file_vec[self.tab_idx]);
                }
            }
        };

        if &RParts::All == draw_parts || matches!(draw_parts, &RParts::ScrollUpDown(_)) {
            HeaderBar::draw(self, &mut str_vec);
            self.help.render(&mut str_vec);
            self.curt().mbar.render(&mut str_vec);
            let is_msg_changed = self.curt().mbar.is_msg_changed();
            let prom_disp_row_posi = self.curt().prom.disp_row_posi;
            let h_file = &self.curt_h_file().clone();
            let state = &self.curt().state.clone();
            self.curt().prom.render(&mut str_vec, prom_disp_row_posi, state, is_msg_changed, h_file);
        }
        if draw_parts == &RParts::All || draw_parts == &RParts::Editor {
            Log::info("self.state.is_ctx_menu", &self.state.is_ctx_menu);
            if self.state.is_ctx_menu {
                self.ctx_menu.draw(&mut str_vec);
            }
            Log::info("self.curt().editor.state.input_comple_mode", &self.curt().editor.state.input_comple_mode);
            if self.curt().editor.is_input_imple_mode(true) {
                self.curt().editor.input_comple.draw(&mut str_vec);
            }
        }
        self.render_init_info(&mut str_vec);

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
        Log::debug("self.curt().editor.state.input_comple_mode", &self.curt().editor.state.input_comple_mode);

        self.render_flush(out, &mut str_vec);

        Log::info_key("Terminal.draw end");
    }

    // Windows:Suppress the number of flushes due to the following error when trying to flush a large amount of data
    //         Error:Windows stdio in console mode does not support writing non-UTF-8 byte sequences
    // Linux:flickers when written all at once.

    pub fn render_flush<T: Write>(&mut self, out: &mut T, str_vec: &mut Vec<String>) {
        Log::info_key("Terminal.render_flush");
        Log::debug("str_vec.len()", &str_vec.len());

        for string in str_vec.iter() {
            let _ = out.write_all(string.as_bytes());
        }
        out.flush().unwrap();

        str_vec.clear();
    }

    pub fn render_cur<T: Write>(&mut self, out: &mut T) {
        let mut str_vec: Vec<String> = vec![];

        if !self.state.is_displayable || self.state.is_ctx_menu {
            Terminal::hide_cur();
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
            self.tabs[self.tab_idx].prom.draw_cur(&mut str_vec, &self.tabs[self.tab_idx].state);
        } else {
            Terminal::hide_cur();
        }
        if !str_vec.is_empty() {
            let _ = out.write(str_vec.concat().as_bytes());
            out.flush().unwrap();
        }
    }

    pub fn render_all<T: Write>(&mut self, out: &mut T, draw_parts: RParts) {
        self.render(out, &draw_parts);
        self.render_cur(out);
    }

    pub fn render_init_info(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("Terminal.draw_init_info");
        // Information display in the center when a new file is created

        if self.state.is_show_init_info && self.curt().editor.h_file.filenm == Lang::get().new_file && self.tab_idx == 0 && self.curt().editor.buf.len_chars() == 0 && self.curt().state.is_nomal() && !self.curt().editor.state.is_changed {
            self.state.is_show_init_info = false;

            let cols = get_term_size().0;
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
        if cols <= TERM_MINIMUM_WIDTH || rows <= TERM_MINIMUM_HEIGHT {
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

        if self.curt().editor.scrl_h.row_max_width > self.curt().editor.col_len && self.curt().state.is_nomal() {
            self.curt().editor.scrl_h.is_show = true;
            {
                self.curt().editor.row_disp_len -= Cfg::get().general.editor.scrollbar.horizontal.height;
                self.curt().editor.scrl_h.row_posi = self.curt().editor.row_disp_len + 1;
            }
        } else {
            self.curt().editor.scrl_h.is_show = false;
        }
        if self.curt().editor.row_disp_len < self.curt().editor.buf.len_rows() {
            {
                self.curt().editor.scrl_v.is_show = true;
                self.curt().editor.col_len -= Cfg::get().general.editor.scrollbar.vertical.width;
            }
        } else {
            self.curt().editor.scrl_v.is_show = false;
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
                return ActType::Render(RParts::MsgBar(Lang::get().file_not_found.to_string()));
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
                            return ActType::Render(RParts::MsgBar(err_str));
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

            // for input complement

            for i in 0..tab.editor.buf.len_rows() {
                self.curt().editor.input_comple.analysis_new(i, &tab.editor.buf.char_vec_row(i));
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
        self.editor_draw_vec[self.tab_idx].syntax_reference = CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&ext).cloned();

        Log::debug("file_meta.len()", &file_meta.len());
        Log::debug("Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 1024.0 as u64", &(Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 1024.0 as u64));

        if self.editor_draw_vec[self.tab_idx].syntax_reference.is_some() && file_meta.len() < Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 10240000.0 as u64 && is_enable_syntax_highlight(&ext) {
            self.curt().editor.is_enable_syntax_highlight = true;
        }
    }

    pub fn exit_file_open(msg: &str) {
        // Terminal::finalize_initial();
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
        self.ctx_menu.init();
    }

    pub fn set_bg_color(&mut self) {
        let color_string = if CfgSyntax::get().syntax.theme.settings.background.is_some() {
            if self.curt().editor.is_enable_syntax_highlight && Cfg::get().general.colors.theme.theme_bg_enable {
                Colors::bg(Color::from(CfgSyntax::get().syntax.theme.settings.background.unwrap()))
            } else {
                Colors::bg(Cfg::get().colors.editor.bg)
            }
        } else {
            Colors::bg(Cfg::get().colors.editor.bg)
        };
        let _ = stdout().write(color_string.as_bytes());
        stdout().flush().unwrap();
    }
    pub fn init_draw<T: Write>(&mut self, out: &mut T) {
        self.state.is_show_init_info = true;
        Log::debug_s("1111111111111111111111");

        self.set_bg_color();
        Log::debug_s("22222222222222222222222");

        self.render(out, &RParts::All);
        Log::debug_s("3333333333333333333333");

        self.render_cur(out);
    }

    pub fn add_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        self.tab_idx = self.tabs.len();
        self.tabs.push(tab);
        self.editor_draw_vec.push(EditorDraw::default());
        self.hbar.file_vec.push(h_file.clone());
        self.hbar.disp_base_idx = USIZE_UNDEFINED;

        self.init_tab(&h_file, file_open_type);
    }
    pub fn change_tab(&mut self, idx: usize) {
        self.tab_idx = idx;
        let h_file = self.hbar.file_vec[self.tab_idx].clone();
        self.init_tab(&h_file, FileOpenType::Nomal);
    }
    pub fn swap_tab(&mut self, idx_org: usize, idx_dst: usize) {
        let h_file = self.hbar.file_vec.remove(idx_org);
        self.hbar.file_vec.insert(idx_dst, h_file);
        let tab = self.tabs.remove(idx_org);
        self.tabs.insert(idx_dst, tab);
        let editor_draw = self.editor_draw_vec.remove(idx_org);
        self.editor_draw_vec.insert(idx_dst, editor_draw);

        self.change_tab(idx_dst);
    }

    pub fn reopen_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        self.tabs[self.tab_idx] = tab;
        self.editor_draw_vec[self.tab_idx].clear();
        self.hbar.file_vec[self.tab_idx] = h_file.clone();
        self.init_tab(&h_file, file_open_type);
    }

    pub fn init_tab(&mut self, h_file: &HeaderFile, file_open_type: FileOpenType) {
        self.set_disp_size();
        self.curt().editor.calc_scrlbar_h();
        self.curt().editor.calc_editor_scrlbar_v();
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
        self.tab_idx = if tab_idx == self.hbar.file_vec.len() - 1 && tab_idx != 0 { tab_idx - 1 } else { self.tab_idx };
        self.tabs.remove(tab_idx);
        self.editor_draw_vec.remove(tab_idx);
        self.hbar.file_vec.remove(tab_idx);
        self.hbar.disp_base_idx = USIZE_UNDEFINED;
        self.change_tab(self.tab_idx);

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
            self.tab_idx = idx;
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
            Log::debug("self.idx 111", &self.tab_idx);

            if self.state.close_other_than_this_tab_idx != USIZE_UNDEFINED {
                self.tab_idx = self.tabs.len() - 1;
                if self.tabs.len() != 1 && self.tab_idx == leave_tab_idx {
                    self.tab_idx -= 1;
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
            self.tab_idx = idx;
            let act_type = Tab::save(self, SaveType::Normal);
            if let ActType::Render(_) = act_type {
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
        // new_tab.editor.draw_range = E_DrawRange::All;

        let dt: DateTime<Local> = Local::now();
        Log::debug("dt", &dt);
        // self.add_tab(new_tab, HeaderFile::new(&dt.format("%M:%S").to_string()), FileOpenType::Nomal);

        self.add_tab(new_tab, HeaderFile::new(&Lang::get().new_file), FileOpenType::Nomal);
    }

    pub fn switch_tab(&mut self, direction: Direction) -> ActType {
        if self.tabs.len() > 1 {
            let idx = if direction == Direction::Right {
                if self.tabs.len() - 1 == self.tab_idx {
                    0
                } else {
                    self.tab_idx + 1
                }
            } else if self.tab_idx == 0 {
                self.tabs.len() - 1
            } else {
                self.tab_idx - 1
            };
            self.change_tab(idx);
            return ActType::Render(RParts::All);
        } else {
            return ActType::Render(RParts::MsgBar(Lang::get().no_tab_can_be_switched.to_string()));
        }
    }

    pub fn resize(&mut self) {
        self.set_disp_size();
        self.curt().editor.draw_range = E_DrawRange::All;

        self.curt().editor.calc_editor_scrlbar_v();
        self.curt().editor.calc_scrlbar_h();
    }

    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.tab_idx).unwrap();
    }

    pub fn curt_h_file(&mut self) -> &mut HeaderFile {
        return self.hbar.file_vec.get_mut(self.tab_idx).unwrap();
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
        self.curt().editor.cancel_state();
        self.curt().editor.draw_range = E_DrawRange::All;
    }

    pub fn clear_pre_tab_status(&mut self) {
        self.tab_idx -= 1;

        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().mbar.clear();
        self.set_disp_size();
        self.curt().editor.draw_range = E_DrawRange::All;
        self.tab_idx += 1;
    }

    pub fn clear_ctx_menu(&mut self) {
        Log::debug_key("clear_ctx_menu");
        self.state.is_ctx_menu = false;
        self.state.is_ctx_menu_hide_draw = true;
        self.ctx_menu.clear();
    }

    pub fn set_render_range_ctx_menu(&mut self) {
        Log::debug_key("set_render_range_window");
        match self.keycmd {
            KeyCmd::CtxMenu(C_Cmd::MouseDownLeft(y, x)) => {
                if self.state.is_ctx_menu && !self.ctx_menu.window.is_mouse_within_range(y, x, false) {
                    self.clear_ctx_menu();
                    self.curt().editor.draw_range = E_DrawRange::All;
                }
            }
            KeyCmd::CtxMenu(C_Cmd::MouseMove(y, x)) => {
                if self.state.is_ctx_menu && self.ctx_menu.window.is_mouse_within_range(y, x, false) {
                    let (offset_y, editor_row_len) = (self.curt().editor.offset_y, self.curt().editor.row_disp_len);
                    self.curt().editor.draw_range = self.ctx_menu.window.get_draw_range_y(offset_y, HEADERBAR_ROW_NUM, editor_row_len);
                }
            }
            KeyCmd::CtxMenu(C_Cmd::CursorDown) | KeyCmd::CtxMenu(C_Cmd::CursorUp) | KeyCmd::CtxMenu(C_Cmd::CursorRight) | KeyCmd::CtxMenu(C_Cmd::CursorLeft) => {
                let (offset_y, editor_row_len) = (self.curt().editor.offset_y, self.curt().editor.row_disp_len);
                self.curt().editor.draw_range = self.ctx_menu.window.get_draw_range_y(offset_y, HEADERBAR_ROW_NUM, editor_row_len);
            }

            _ => {}
        }
    }

    /*
    pub fn set_editor_render_target_ctx_menu(&mut self) {
        let (offset_y, hbar_disp_row_num, editor_row_len) = (self.curt().editor.offset_y, self.hbar.row_num, self.curt().editor.row_disp_len);
        let draw_range_y_opt = self.ctx_menu.window.get_draw_range_y(offset_y, hbar_disp_row_num, editor_row_len);
        self.curt().editor.draw_range = if let Some((sy, ey)) = draw_range_y_opt { E_DrawRange::TargetRange(sy, ey) } else { E_DrawRange::Not };
    }
     */

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
        Log::debug("self.curt().editor.state.input_comple_mode", &self.curt().editor.state.input_comple_mode);
        let editor_is_dragging = self.curt().editor.state.is_dragging;

        return if self.judge_when_headerbar(keys, self.hbar.row_posi, editor_is_dragging) {
            KeyWhen::HeaderBarFocus
        } else if self.curt().state.judge_when_prompt(keys) {
            KeyWhen::PromptFocus
        } else if self.state.is_ctx_menu {
            if EvtAct::is_ctx_menu_keys(keys, self) {
                KeyWhen::CtxMenuFocus
            } else {
                self.clear_ctx_menu();
                KeyWhen::EditorFocus
            }
        } else {
            let sbar_row_posi = self.curt().sbar.row_posi;
            if self.curt().state.judge_when_statusbar(keys, sbar_row_posi, editor_is_dragging) {
                KeyWhen::StatusBarFocus
                // } else if EvtAct::is_input_comple_keys(keys, self) {
            } else {
                KeyWhen::EditorFocus
            }
        };
    }
    pub fn judge_when_headerbar(&self, keys: &Keys, hbar_row_posi: usize, editor_is_dragging: bool) -> bool {
        match keys {
            Keys::MouseDownLeft(y, _) if y == &(hbar_row_posi as u16) => {
                return true;
            }
            Keys::MouseDragLeft(y, _) if y == &(hbar_row_posi as u16) => return !editor_is_dragging,
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
        Terminal { draw_parts_org: RParts::All, keycmd: KeyCmd::Null, keys: Keys::Null, keys_org: Keys::Null, hbar: HeaderBar::new(), tabs: vec![], editor_draw_vec: vec![], tab_idx: 0, help: Help::new(), state: TerminalState::default(), ctx_menu: CtxMenu::default() }
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
