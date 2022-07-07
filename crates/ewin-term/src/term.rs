use crate::{
    bar::{filebar::*, menubar::*, statusbar::*},
    ewin_com::{
        _cfg::key::{keys::*, keywhen::*},
        files::file::*,
        global::*,
        model::*,
        util::*,
    },
    ewin_editor::model::*,
    global_term,
    global_term::*,
    help::*,
    model::*,
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
use ewin_cfg::{
    colors::*,
    lang::lang_cfg::*,
    log::*,
    model::{default::*, modal::*},
};
use ewin_com::_cfg::key::cmd::*;
use ewin_const::{def::*, model::*};
use ewin_widget::{core::*, model::*};
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
        Log::info_key("Terminal.render start");
        Log::debug("draw_parts", &draw_parts);
        Log::debug("self.curt().editor.draw_range", &self.curt().editor.win_mgr.curt().draw_range);
        self.set_disp_size();

        let mut str_vec: Vec<String> = vec![];

        if (matches!(draw_parts, DParts::All) || matches!(draw_parts, DParts::AllMsgBar(_))) && self.curt().editor.win_mgr.curt().draw_range != E_DrawRange::Init {
            self.curt().editor.win_mgr.curt().draw_range = E_DrawRange::All;
        }
        let d_raneg = self.curt().editor.win_mgr.curt().draw_range;

        if self.curt().editor.win_mgr.curt().draw_range == E_DrawRange::All {
            for vec_v in self.curt().editor.win_mgr.win_list.iter_mut() {
                for win in vec_v.iter_mut() {
                    win.draw_range = d_raneg;
                }
            }
        }

        // Editor
        match self.curt().editor.win_mgr.curt().draw_range {
            E_DrawRange::Not => {}
            _ => {
                if self.curt().editor.get_curt_row_len() > 0 {
                    let vec = self.curt().editor.win_mgr.win_list.clone();
                    Log::debug("vec", &vec);
                    Log::debug("vec.len()", &vec.len());

                    // editor_draw_vec window support
                    self.editor_draw_vec[self.tab_idx].resize_with(vec.len(), Vec::new);
                    for (i, v) in vec.iter().enumerate() {
                        let mut add_vec = vec![];
                        for _ in v {
                            add_vec.push(EditorDraw::default());
                        }
                        self.editor_draw_vec[self.tab_idx][i].resize_with(v.len(), EditorDraw::default);
                    }
                    let curt_win = self.tabs[self.tab_idx].editor.get_curt_ref_win().clone();
                    if curt_win.draw_range == E_DrawRange::MoveCur {
                        self.tabs[self.tab_idx].editor.draw_move_cur(&mut str_vec, &curt_win);
                        self.draw_flush(out, &mut str_vec);
                        return;
                    }

                    for (v_idx, vec_v) in vec.iter().enumerate() {
                        Log::debug("vec_v.len()", &vec_v.len());
                        for (h_idx, win) in vec_v.iter().enumerate() {
                            Log::debug("win", &win);
                            Log::debug("h_idx xxx", &h_idx);

                            self.editor_draw_vec[self.tab_idx][v_idx][h_idx].draw_cache(&mut self.tabs[self.tab_idx].editor, win);
                            let draw = &mut self.editor_draw_vec[self.tab_idx][v_idx][h_idx];
                            
                            // Clear init
                            if v_idx == 0 && h_idx == 0 {
                                let act_type = self.tabs[self.tab_idx].editor.clear_init(&mut str_vec, draw);
                                if act_type != ActType::Next {
                                    return;
                                }
                            }

                            self.tabs[self.tab_idx].editor.draw(&mut str_vec, draw, win);
                            self.tabs[self.tab_idx].editor.draw_scale(&mut str_vec, win);
                            self.tabs[self.tab_idx].editor.draw_scrlbar_v(&mut str_vec, win);
                            self.tabs[self.tab_idx].editor.draw_scrlbar_h(&mut str_vec, win);
                        }
                    }
                }
                str_vec.push(Colors::get_default_fg_bg());
            }
        };
        self.tabs[self.tab_idx].editor.draw_window_split_line(&mut str_vec);
        StatusBar::draw(&mut str_vec, &mut self.tabs[self.tab_idx], &H_FILE_VEC.get().unwrap().try_lock().unwrap()[self.tab_idx]);
        self.curt().msgbar.draw(&mut str_vec);

        if &DParts::All == draw_parts || matches!(draw_parts, &DParts::ScrollUpDown(_)) {
            self.menubar.draw(&mut str_vec);
            FileBar::draw(self, &mut str_vec);
            HELP_DISP.get().unwrap().try_lock().unwrap().draw(&mut str_vec);
            let state = &self.curt().state.clone();
            self.curt().prom.draw(&mut str_vec, state);
        }

        if draw_parts == &DParts::All || matches!(draw_parts, DParts::Editor(_)) {
            Log::info("self.state.is_ctx_menu", &self.state.is_ctx_menu);
            if self.state.is_ctx_menu {
                self.ctx_widget.draw(&mut str_vec);
            }
            if self.state.is_menuwidget {
                self.menubar.widget.draw(&mut str_vec);
            }
            if self.curt().editor.is_input_imple_mode(true) {
                self.curt().editor.input_comple.draw(&mut str_vec);
            }
        }
        self.draw_init_info(&mut str_vec);

        Log::debug("cur", &self.curt().editor.win_mgr.curt().cur);
        Log::debug("offset_x", &self.curt().editor.win_mgr.curt().offset.x);
        Log::debug("offset_disp_x", &self.curt().editor.win_mgr.curt().offset.disp_x);
        Log::debug("offset.y", &self.curt().editor.win_mgr.curt().offset.y);
        Log::debug("offset_y_org", &self.curt().editor.win_mgr.curt().offset.y_org);
        Log::debug("history.undo_vec", &self.curt().editor.history.undo_vec);
        // Log::debug("self.curt().state.key_record_state", &self.curt().state.key_record_state);
        //  Log::debug("self.curt().state", &self.curt().state);
        // Log::debug("sel_range", &self.curt().editor.sel);
        //  Log::debug("", &self.curt().editor.search);
        // Log::debug("box_sel.mode", &self.curt().editor.box_insert.mode);
        // Log::debug("scrl_v.is_enable", &self.curt().editor.scrl_v.is_enable);
        // Log::debug("scrl_h.is_enable", &self.curt().editor.scrl_h.is_enable);
        // Log::debug("self.curt().editor.state.input_comple_mode", &self.curt().editor.state.input_comple_mode);
        Log::debug("self.curt().editor.win", &self.curt().editor.win_mgr);

        self.draw_flush(out, &mut str_vec);

        Log::info_key("Terminal.draw end");
    }

    // Windows:Suppress the number of flushes due to the following error when trying to flush a large amount of data
    //         Error:Windows stdio in console mode does not support writing non-UTF-8 byte sequences
    // Linux:flickers when written all at once.
    pub fn draw_flush<T: Write>(&mut self, out: &mut T, str_vec: &mut Vec<String>) {
        Log::info_key("Terminal.draw_flush");
        Log::debug("str_vec.len()", &str_vec.len());

        for string in str_vec.iter() {
            let _ = out.write_all(string.as_bytes());
        }
        out.flush().unwrap();

        str_vec.clear();
    }

    pub fn draw_cur<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("draw_cur");
        Log::debug("self.curt().state.is_nomal()", &self.curt().state.is_nomal());
        Log::debug("self.curt().editor.is_cur_y_in_screen()", &self.curt().editor.is_cur_y_in_screen());
        Log::debug("self.curt().editor.win_mgr.curt()", &self.curt().editor.win_mgr.curt());

        let mut str_vec: Vec<String> = vec![];
        if !self.state.is_displayable || self.state.is_ctx_menu || self.state.is_menuwidget {
        } else if self.curt().state.is_nomal() && self.curt().editor.is_cur_y_in_screen() {
            self.curt().editor.draw_cur(&mut str_vec);
            Terminal::show_cur();
        } else if self.curt().state.prom != PromState::None && self.curt().prom.curt.as_mut_base().is_draw_cur() {
            self.curt().prom.draw_cur(&mut str_vec);
            Terminal::show_cur();
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

    pub fn draw_init_info(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("Terminal.draw_init_info");
        // Information display in the center when a new file is created

        Log::debug("self.curt().state.is_nomal()", &self.curt().state.is_nomal());

        if self.state.is_show_init_info && self.curt().editor.h_file.filenm == Lang::get().new_file && self.tab_idx == 0 && self.curt().editor.buf.len_chars() == 0 && self.curt().state.is_nomal() && !self.curt().editor.state.is_changed {
            self.state.is_show_init_info = false;

            let cols = get_term_size().0;
            let pkg_name = APP_NAME;
            str_vec.push(Colors::get_default_fg_bg());

            let pkg_name = format!("{name:^w$}", name = pkg_name, w = cols - (get_str_width(pkg_name) - pkg_name.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, (self.curt().editor.get_curt_row_posi() + 4) as u16), Clear(ClearType::CurrentLine), pkg_name));

            let ver_name = &format!("{}: {}", "Version", &(*APP_VERSION.get().unwrap().to_string()));
            let ver_name = format!("{ver:^w$}", ver = ver_name, w = cols - (get_str_width(ver_name) - ver_name.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, (self.curt().editor.get_curt_row_posi() + 5) as u16), Clear(ClearType::CurrentLine), ver_name));

            let simple_help = Lang::get().simple_help_desc.clone();
            let simple_help = format!("{s_help:^w$}", s_help = simple_help, w = cols - (get_str_width(&simple_help) - simple_help.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, (self.curt().editor.get_curt_row_posi() + 7) as u16), Clear(ClearType::CurrentLine), simple_help));
            let detailed_help = Lang::get().detailed_help_desc.clone();
            let detailed_help = format!("{d_help:^w$}", d_help = detailed_help, w = cols - (get_str_width(&detailed_help) - detailed_help.chars().count()));
            str_vec.push(format!("{}{}{}", MoveTo(0, (self.curt().editor.get_curt_row_posi() + 8) as u16), Clear(ClearType::CurrentLine), detailed_help));
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

    pub fn set_disp_size(&mut self) -> bool {
        Log::debug_s("set_disp_size");
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));
        Log::debug("self.curt().state", &self.curt().state);

        self.menubar.set_posi(cols);
        self.menubar.set_menunm();

        self.fbar.set_posi(cols);
        FileBar::set_filenm(self);

        let mut hlep = HELP_DISP.get().unwrap().try_lock().unwrap();

        hlep.col_num = cols;
        hlep.row_num = if hlep.is_disp { Help::DISP_ROW_NUM } else { 0 };
        hlep.row_posi = if hlep.is_disp { rows - hlep.row_num } else { 0 };

        let help_disp_row_num = if hlep.row_num > 0 { hlep.row_num + 1 } else { 0 };
        self.curt().sbar.row_posi = if help_disp_row_num == 0 { rows - 1 } else { rows - help_disp_row_num };
        self.curt().sbar.col_num = cols;
        self.curt().prom.col_num = cols;

        Log::debug("self.curt().prom.row_num 111", &self.curt().prom.row_num);

        self.curt().prom.row_posi = rows - self.curt().prom.row_num - hlep.row_num - self.curt().sbar.row_num;
        self.curt().msgbar.col_num = cols;
        self.curt().msgbar.row_num = MSGBAR_ROW_NUM; //if self.curt().mbar.msg.str.is_empty() { 0 } else { 1 };

        self.curt().msgbar.row_posi = rows - self.curt().prom.row_num - hlep.row_num - self.curt().sbar.row_num - 1;
        self.menubar.row_num = if self.curt().state.prom == PromState::OpenFile { 0 } else { MSGBAR_ROW_NUM };

        let rnw_and_margin = self.curt().editor.get_rnw_and_margin();
        let scale_row_num = if CfgEdit::get().general.editor.scale.is_enable { SCALE_ROW_NUM } else { 0 };
        let editor_row = rows - scale_row_num - self.menubar.row_num - self.fbar.row_num - self.curt().msgbar.row_num - self.curt().prom.row_num - hlep.row_num - self.curt().sbar.row_num;
        let is_prom_open_file = self.curt().state.prom == PromState::OpenFile;
        let row_posi = MENUBAR_ROW_NUM + FILEBAR_ROW_NUM + if CfgEdit::get().general.editor.scale.is_enable { 1 } else { 0 };

        self.curt().editor.row_posi = row_posi;
        self.curt().editor.row_num = editor_row;
        if self.curt().editor.win_mgr.win_list.is_empty() {
            self.curt().editor.win_mgr.win_list = vec![vec![Window::new()]];
        }

        let mut height = row_posi;
        let mut width = 0;
        let editor_buf_len_rows = self.curt().editor.buf.len_rows();
        let is_tab_state_normal = self.curt().state.is_nomal();

        let window_height_base = editor_row / self.curt().editor.win_mgr.win_list.len();
        let window_height_rest = editor_row % self.curt().editor.win_mgr.win_list.len();

        let win_v_list_len = self.curt().editor.win_mgr.win_list.get(0).unwrap().len();

        let tgt_h_len = cols - win_v_list_len * rnw_and_margin - (win_v_list_len - 1) * WindowMgr::SPLIT_LINE_V_WIDTH;
        Log::debug("tgt_h_len", &tgt_h_len);
        let window_width_base = tgt_h_len / win_v_list_len;
        let window_width_rest = tgt_h_len % win_v_list_len;

        Log::debug("window_width_base", &window_width_base);
        Log::debug("window_width_rest", &window_width_rest);
        let mut split_line_v = 0;
        for (v_idx, vec_v) in self.curt().editor.win_mgr.win_list.iter_mut().enumerate() {
            for (h_idx, window) in vec_v.iter_mut().enumerate() {
                window.v_idx = v_idx;
                window.h_idx = h_idx;
                let mut window_width = window_width_base;
                let mut window_height = window_height_base;
                if h_idx == 0 {
                    window_width += window_width_rest;
                    window_height += window_height_rest;
                } else {
                    split_line_v = width;
                    width += WindowMgr::SPLIT_LINE_V_WIDTH;
                }
                width += rnw_and_margin;

                Log::debug("window_width", &window_width);
                // scrl_h
                if window.scrl_h.row_max_width > window_width && is_tab_state_normal {
                    window.scrl_h.is_show = true;
                    window.scrl_h.bar_height = Cfg::get().general.editor.scrollbar.horizontal.height;
                    window_height -= window.scrl_h.bar_height;
                } else {
                    window.scrl_h.is_show = false;
                }
                // scrl_v
                if window_height < editor_buf_len_rows {
                    window.scrl_v.is_show = true;
                    window.scrl_v.bar_width = Cfg::get().general.editor.scrollbar.vertical.width;
                    //  window.scrl_v.row_posi = window.area_v.0;
                    window_width -= window.scrl_v.bar_width;
                } else {
                    window.scrl_v.is_show = false;
                }
                window.area_v = if is_prom_open_file { (0, 0) } else { (height, height + window_height) };
                window.area_all_v = if is_prom_open_file { (0, 0) } else { (height - scale_row_num, height + window_height + window.scrl_h.bar_height) };
                window.area_h = (width, width + window_width);
                if window.scrl_h.row_max_width > window_width && is_tab_state_normal {
                    window.scrl_h.row_posi = window.area_v.1;
                }
                Log::debug("width", &width);
                Log::debug("rnw_and_margin", &rnw_and_margin);

                Log::debug_s("1111111111111111111111111111");

                Log::debug("width - rnw_and_margin", &(width - rnw_and_margin));
                Log::debug("window_width", &window_width);
                Log::debug("window.scrl_v.bar_width", &window.scrl_v.bar_width);

                window.area_all_h = (width - rnw_and_margin, width + window_width + window.scrl_v.bar_width);

                Log::debug("window.area_h", &window.area_h);
                Log::debug("window.area_all_h", &window.area_all_h);
                Log::debug("window.area_v", &window.area_v);
                Log::debug("window.area_all_v", &window.area_all_v);

                width += window_width + window.scrl_v.bar_width;
                if v_idx > 0 {
                    height += window_height + window.scrl_h.bar_height;
                }
            }
        }
        if split_line_v > 0 {
            self.curt().editor.win_mgr.split_line_v = split_line_v;
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
                        h_file.mod_time = _modified_time;

                        if !is_writable {
                            tab.editor.state.is_read_only = true;
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
                    self.add_tab(&mut tab.clone(), h_file, file_open_type);
                    self.curt().editor.set_cur_default();
                }
                FileOpenType::Reopen => {
                    self.reopen_tab(tab.clone(), h_file, file_open_type);
                    self.curt().editor.cmd = Cmd::to_cmd(CmdType::ReOpenFile);
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
        self.curt().clear_curt_tab(true);
        self.set_disp_size();
        let h_file = H_FILE_VEC.get().unwrap().try_lock().unwrap().get_mut(self.tab_idx).unwrap().clone();
        self.open_file(&h_file.fullpath, FileOpenType::Reopen, None, Some(&h_file));
    }

    pub fn enable_syntax_highlight(&mut self, path: &Path) {
        let file_meta = metadata(path).unwrap();
        let ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
        //  self.editor_draw_vec[self.idx].syntax_reference = if let Some(sr) = CFG.get().unwrap().try_lock().unwrap().syntax.syntax_set.find_syntax_by_extension(&ext) { Some(sr.clone()) } else { None };
        self.editor_draw_vec[self.tab_idx][self.tabs[self.tab_idx].editor.win_mgr.win_v_idx][self.tabs[self.tab_idx].editor.win_mgr.win_h_idx].syntax_reference = CfgSyntax::get().syntax.syntax_set.find_syntax_by_extension(&ext).cloned();

        Log::debug("file_meta.len()", &file_meta.len());
        Log::debug("Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 1024.0 as u64", &(Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 1024.0 as u64));

        let win_v_idx = self.curt().editor.win_mgr.win_v_idx;
        let win_h_idx = self.curt().editor.win_mgr.win_h_idx;
        if self.editor_draw_vec[self.tab_idx][win_v_idx][win_h_idx].syntax_reference.is_some() && file_meta.len() < Cfg::get().general.colors.theme.disable_syntax_highlight_file_size as u64 * 10240000.0 as u64 && is_enable_syntax_highlight(&ext) {
            self.curt().editor.is_enable_syntax_highlight = true;
        }
    }

    pub fn exit_file_open(msg: &str) {
        // Terminal::finalize_initial();
        println!("{}", msg);
        Terminal::exit();
    }

    pub fn activate(&mut self, args: &AppArgs) {
        Log::info_key("activate");

        let _ = GREP_CANCEL_VEC.set(tokio::sync::Mutex::new(vec![GrepCancelType::None]));
        let _ = global_term::TAB.set(tokio::sync::Mutex::new(Tab::new()));
        let _ = WATCH_INFO.set(tokio::sync::Mutex::new(WatchInfo::default()));
        let _ = H_FILE_VEC.set(tokio::sync::Mutex::new(vec![]));
        let _ = HELP_DISP.set(tokio::sync::Mutex::new(Help::default()));
        self.open_file(&args.filenm, FileOpenType::First, Some(&mut Tab::new()), None);
        self.ctx_widget.init();
        self.menubar.init();
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
        Log::info_key("init_draw");
        self.state.is_show_init_info = true;
        self.set_bg_color();
        self.draw(out, &DParts::All);
        self.draw_cur(out);
    }

    pub fn add_tab(&mut self, tab: &mut Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        self.tab_idx = self.tabs.len();
        tab.idx = self.tabs.len();
        self.tabs.push(tab.clone());
        self.editor_draw_vec.push(vec![vec![EditorDraw::default()]]);

        // FileBar::push_h_file_vec(h_file.clone());
        H_FILE_VEC.get().unwrap().try_lock().map(|mut vec| vec.push(h_file.clone())).unwrap();
        self.fbar.disp_base_idx = USIZE_UNDEFINED;

        self.init_tab(&h_file, file_open_type);
    }
    pub fn change_tab(&mut self, idx: usize) {
        self.tab_idx = idx;
        let h_file = H_FILE_VEC.get().unwrap().try_lock().unwrap()[self.tab_idx].clone();
        self.init_tab(&h_file, FileOpenType::Nomal);
    }
    pub fn swap_tab(&mut self, idx_org: usize, idx_dst: usize) {
        let h_file = H_FILE_VEC.get().unwrap().try_lock().unwrap().remove(idx_org);
        H_FILE_VEC.get().unwrap().try_lock().unwrap().insert(idx_dst, h_file);
        let tab = self.tabs.remove(idx_org);
        self.tabs.insert(idx_dst, tab);
        let editor_draw = self.editor_draw_vec.remove(idx_org);
        self.editor_draw_vec.insert(idx_dst, editor_draw);

        self.change_tab(idx_dst);
    }

    pub fn reopen_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        // tab.idx = self.tab_idx;
        self.tabs[self.tab_idx] = tab;
        self.editor_draw_vec[self.tab_idx].clear();
        H_FILE_VEC.get().unwrap().try_lock().unwrap()[self.tab_idx] = h_file.clone();
        self.init_tab(&h_file, file_open_type);
    }

    pub fn init_tab(&mut self, h_file: &HeaderFile, file_open_type: FileOpenType) {
        self.set_disp_size();
        self.curt().editor.init_editor_scrlbar_h();
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
        self.tab_idx = if tab_idx == H_FILE_VEC.get().unwrap().try_lock().unwrap().len() - 1 && tab_idx != 0 { tab_idx - 1 } else { self.tab_idx };
        self.tabs.remove(tab_idx);
        self.editor_draw_vec.remove(tab_idx);
        H_FILE_VEC.get().unwrap().try_lock().unwrap().remove(tab_idx);
        self.fbar.disp_base_idx = USIZE_UNDEFINED;
        self.change_tab(self.tab_idx);

        if let Some(Ok(mut grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
            if grep_cancel_vec.len() > tab_idx {
                grep_cancel_vec.remove(tab_idx);
            }
        }
    }
    pub fn close_tabs(&mut self, leave_tab_idx: usize) -> ActType {
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
                if self.curt().prom_show_com(&CmdType::CloseFile) == ActType::Exit {
                    return ActType::Exit;
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

        return if self.tabs.is_empty() { ActType::Exit } else { ActType::Draw(DParts::All) };
    }

    pub fn save_all_tab(&mut self) -> ActType {
        Log::debug_key("save_all_tab");
        self.state.is_all_save = true;
        let len = self.tabs.len() - 1;
        for idx in (0..=len).rev() {
            self.tab_idx = idx;
            let act_type = self.curt().save(SaveType::Normal);
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
                tab.editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                tab.state.clear();
            }
        }
    }
    pub fn cancel_save_all_tab(&mut self) {
        self.state.is_all_save = false;
    }

    pub fn new_tab(&mut self) {
        // Disable the event in case of the next display
        self.curt().editor.set_cmd(Cmd::to_cmd(CmdType::Null));

        let mut new_tab = Tab::new();
        //  new_tab.editor.set_cur_default();

        let dt: DateTime<Local> = Local::now();
        Log::debug("dt", &dt);

        self.add_tab(&mut new_tab, HeaderFile::new(&Lang::get().new_file), FileOpenType::Nomal);
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
            return ActType::Draw(DParts::All);
        } else {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_tab_can_be_switched.to_string()));
        }
    }

    pub fn resize(&mut self) {
        self.set_disp_size();
        self.curt().editor.win_mgr.curt().draw_range = E_DrawRange::All;

        // self.curt().editor.calc_editor_scrlbar_v();
        // self.curt().editor.init_editor_scrlbar_h();
        // return ActType::Draw(DParts::All);
    }

    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.tab_idx).unwrap();
    }

    /*
        pub fn curt_h_file(&mut self) -> &mut HeaderFile {
           // return self.fbar.file_vec.get_mut(self.tab_idx).unwrap();
           // let mut vec = H_FILE_VEC.get().unwrap().try_lock().unwrap();
          return   H_FILE_VEC.get_mut().unwrap().try_lock().unwrap().get_mut(self.tab_idx).unwrap();

    }
    */

    pub fn clear_pre_tab_status(&mut self) {
        self.tab_idx -= 1;

        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().msgbar.clear();
        self.set_disp_size();
        self.curt().editor.win_mgr.curt().draw_range = E_DrawRange::All;
        self.tab_idx += 1;
    }

    pub fn set_keys(&mut self, keys: &Keys) {
        self.keywhen = self.get_when(keys);
        Log::debug("keywhen", &self.keywhen);
        self.keys = *keys;
        self.cmd = Cmd::keys_to_cmd(keys, Some(&self.keys_org), self.keywhen.clone());
        Log::debug("self.cmd", &self.cmd);
    }

    pub fn get_when(&mut self, keys: &Keys) -> KeyWhen {
        Log::debug("keys", &keys);

        let editor_is_dragging = self.curt().editor.state.is_dragging;

        if self.is_menuwidget_keys(keys) {
            KeyWhen::MenuBar
        } else if self.state.is_menuwidget {
            self.clear_menuwidget();
            KeyWhen::Editor
        } else if self.judge_when_filebar(keys, self.fbar.row_posi, editor_is_dragging) {
            KeyWhen::FileBar
        } else if self.curt().state.judge_when_prompt(keys) {
            KeyWhen::Prom
        } else if self.state.is_ctx_menu {
            if self.is_ctx_menu_keys(keys) {
                KeyWhen::CtxMenu
            } else {
                self.clear_ctx_menu();
                KeyWhen::Editor
            }
        } else {
            let sbar_row_posi = self.curt().sbar.row_posi;
            if self.curt().state.judge_when_statusbar(keys, sbar_row_posi, editor_is_dragging) {
                KeyWhen::StatusBar
            } else {
                KeyWhen::Editor
            }
        }
    }

    pub fn judge_when_filebar(&self, keys: &Keys, fbar_row_posi: usize, editor_is_dragging: bool) -> bool {
        match keys {
            Keys::MouseDownLeft(y, _) if y == &(fbar_row_posi as u16) => return true,

            Keys::MouseDragLeft(y, _) if y == &(fbar_row_posi as u16) => return !editor_is_dragging,
            _ => return false,
        }
    }
    pub fn close_file(&mut self) -> ActType {
        let act_type = self.curt().prom_show_com(&CmdType::CloseFile);

        if act_type != ActType::Next {
            return act_type;
        }
        if self.tabs.len() == 1 {
            return ActType::Exit;
        } else {
            self.del_tab(self.tab_idx);
            // Redraw the previous tab
            return ActType::Draw(DParts::All);
        }
    }

    pub fn new() -> Self {
        Terminal { ..Terminal::default() }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal { draw_parts_org: DParts::All, cmd: Cmd::default(), keys: Keys::Null, keys_org: Keys::Null, keywhen: KeyWhen::All, fbar: FileBar::new(), menubar: MenuBar::new(), tabs: vec![], editor_draw_vec: vec![], tab_idx: 0, state: TerminalState::default(), ctx_widget: CtxWidget::default() }
    }
}

impl Default for TerminalState {
    fn default() -> Self {
        TerminalState { is_show_init_info: false, is_all_close_confirm: false, is_all_save: false, close_other_than_this_tab_idx: USIZE_UNDEFINED, is_displayable: true, is_ctx_menu: false, is_ctx_menu_hide_draw: false, is_menuwidget: false, is_menuwidget_hide_draw: false }
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
