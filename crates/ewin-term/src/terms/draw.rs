use super::term::*;
use crate::{bar::filebar::*, global_term::*};
use crossterm::{cursor::*, execute, terminal::*};
use ewin_cfg::{colors::*, log::*, model::default::*};
use ewin_const::{def::*, model::*, term::*};
use ewin_dialog::dialog::*;
use ewin_key::model::*;
use ewin_menulist::core::*;
use std::{
    fmt,
    io::{stdout, Write},
};

impl Term {
    pub fn draw<T: Write>(&mut self, out: &mut T, draw_parts: &DParts) {
        Log::info_key("Terminal.draw start");
        Log::debug("draw_parts", &draw_parts);
        Log::debug("self.curt().editor.draw_range 111", &self.curt().editor.draw_range);
        self.set_size();

        let mut str_vec: Vec<String> = vec![];

        if (matches!(draw_parts, DParts::All) || matches!(draw_parts, DParts::AllMsgBar(_))) && self.curt().editor.draw_range != E_DrawRange::Init {
            self.curt().editor.draw_range = E_DrawRange::All;
        }
        if let DParts::Editor(e_draw_range) = draw_parts {
            self.curt().editor.draw_range = *e_draw_range;
        }

        Log::debug("self.curt().editor.draw_range 222", &self.curt().editor.draw_range);

        self.curt().draw_editor(&mut str_vec);
        if self.curt().editor.draw_range == E_DrawRange::MoveCur {
            self.curt().msgbar.draw(&mut str_vec, false);
            self.draw_flush(out, &mut str_vec);
            return;
        }

        self.curt().msgbar.draw(&mut str_vec, false);
        self.curt().draw_sbar(&mut str_vec);

        if &DParts::All == draw_parts || matches!(draw_parts, &DParts::ScrollUpDown(_)) {
            FileBar::draw(self, &mut str_vec);
            self.menubar.draw(&mut str_vec);
            HELP_DISP.get().unwrap().try_lock().unwrap().draw(&mut str_vec);
            let state = &self.curt().state.clone();
            self.curt().prom.draw(&mut str_vec, state);
        }

        if draw_parts == &DParts::All || matches!(draw_parts, DParts::Editor(_)) {
            Log::info("self.state.is_ctx_menu", &self.state.is_ctx_menu);
            if self.state.is_ctx_menu {
                self.ctx_menu.draw(&mut str_vec);
            }
            if self.state.is_menubar_menulist {
                self.menubar.menulist.draw(&mut str_vec);
            }
            if self.curt().editor.is_input_imple_mode(true) {
                self.curt().editor.input_comple.draw(&mut str_vec);
            }
            Dialog::get().draw(&mut str_vec);
        }

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
        // Log::debug("self.curt().editor.win", &self.curt().editor.win_mgr);

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
        if !self.state.is_displayable || self.state.is_ctx_menu || self.state.is_menubar_menulist {
        } else if Dialog::get().is_show {
            Term::hide_cur();
        } else if self.curt().state.is_nomal() && self.curt().editor.is_cur_y_in_screen() {
            self.curt().editor.draw_cur(&mut str_vec);
            Term::show_cur();
        } else if self.curt().state.prom != PromState::None && self.curt().prom.curt.as_mut_base().is_draw_cur() {
            self.curt().prom.draw_cur(&mut str_vec);
            Term::show_cur();
        } else if self.curt().state.prom != PromState::None && self.curt().prom.curt.as_mut_base().is_draw_cur() {
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
