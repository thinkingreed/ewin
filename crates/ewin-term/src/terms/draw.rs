use super::term::*;
use crate::{
    bar::{filebar::*, statusbar::*},
    ewin_com::{global::*, model::*, util::*},
    ewin_editor::model::*,
    global_term::*,
};
use crossterm::{cursor::*, execute, terminal::*};
use ewin_cfg::{
    colors::*,
    lang::lang_cfg::*,
    log::*,
    model::default::{Cfg, CfgSyntax},
};
use ewin_const::def::*;
use ewin_widget::core::*;
use std::{
    fmt,
    io::{stdout, Write},
};

impl Terminal {
    pub fn draw<T: Write>(&mut self, out: &mut T, draw_parts: &DParts) {
        Log::info_key("Terminal.draw start");
        Log::debug("draw_parts", &draw_parts);
        Log::debug("self.curt().editor.draw_range", &self.curt().editor.win_mgr.curt().draw_range);
        self.set_size();

        let mut str_vec: Vec<String> = vec![];

        if (matches!(draw_parts, DParts::All) || matches!(draw_parts, DParts::AllMsgBar(_))) && self.curt().editor.win_mgr.curt().draw_range != E_DrawRange::Init {
            self.curt().editor.win_mgr.curt().draw_range = E_DrawRange::All;
        }

        if let DParts::Editor(e_draw_range) = draw_parts {
            for vec_v in self.curt().editor.win_mgr.win_list.iter_mut() {
                for win in vec_v.iter_mut() {
                    win.draw_range = *e_draw_range;
                }
            }
        }

        if self.curt().editor.win_mgr.curt().draw_range == E_DrawRange::All {
            for vec_v in self.curt().editor.win_mgr.win_list.iter_mut() {
                for win in vec_v.iter_mut() {
                    win.draw_range = E_DrawRange::All;
                }
            }
        }

        // Editor
        match self.curt().editor.win_mgr.curt().draw_range {
            E_DrawRange::Not => {}
            _ => {
                if self.curt().editor.get_curt_row_len() > 0 {
                    let curt_win = self.tabs[self.tab_idx].editor.get_curt_ref_win().clone();
                    if curt_win.draw_range == E_DrawRange::MoveCur {
                        self.tabs[self.tab_idx].editor.draw_move_cur(&mut str_vec, &curt_win);
                        self.curt().msgbar.draw(&mut str_vec);
                        self.draw_flush(out, &mut str_vec);
                        return;
                    }

                    let vec = self.curt().editor.win_mgr.win_list.clone();
                    for (v_idx, vec_v) in vec.iter().enumerate() {
                        for (h_idx, win) in vec_v.iter().enumerate() {
                            self.tabs[self.tab_idx].draw_cache(win);

                            // Clear init
                            let act_type = Editor::clear_init(&mut str_vec, &self.tabs[self.tab_idx].editor, &self.tabs[self.tab_idx].editor_draw_vec[v_idx][h_idx], win);
                            if act_type != ActType::Next {
                                return;
                            }

                            self.tabs[self.tab_idx].editor.draw(&mut str_vec, &self.tabs[self.tab_idx].editor_draw_vec[v_idx][h_idx], win);
                            self.tabs[self.tab_idx].editor_draw_vec[v_idx][h_idx].cells_from = std::mem::take(&mut self.tabs[self.tab_idx].editor_draw_vec[v_idx][h_idx].cells_to);
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
}
