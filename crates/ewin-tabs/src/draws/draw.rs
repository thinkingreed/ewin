use crate::tabs::*;

use ewin_cfg::log::*;
use ewin_const::models::draw::*;
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_file_bar::filebar::*;
use ewin_help::help::*;
use ewin_menulist::menubar::*;
use ewin_state::term::*;
use ewin_view::{menulists::core::*, view::View};
use std::io::Write;

impl Tabs {
    pub fn draw<T: Write>(&mut self, out: &mut T, draw_parts: &DParts) {
        Log::info_key("Terminal.draw start");
        Log::debug("draw_parts", &draw_parts);
        Log::debug("self.curt().editor.draw_range 111", &self.curt().editor.draw_range);
        self.set_size();

        let mut str_vec: Vec<String> = vec![];

        if (matches!(draw_parts, DParts::All) || matches!(draw_parts, DParts::AllMsgBar(_))) {
            self.curt().editor.draw_range = E_DrawRange::All;
        } else if let DParts::Editor(e_draw_range) = draw_parts {
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

        if &DParts::All == draw_parts || matches!(draw_parts, DParts::ScrollUpDown(_)) {
            FileBar::draw(&mut str_vec);
            // MenuBar::get().draw_menulist(&mut str_vec);
            MenuBar::get().draw(&mut str_vec);
            Help::get().draw(&mut str_vec);
            self.curt().prom.draw(&mut str_vec);
        }

        if draw_parts == &DParts::All || matches!(draw_parts, DParts::Editor(_)) {
            CtxMenu::get().draw(&mut str_vec);

            MenuBar::get().draw_menulist(&mut str_vec);

            if self.curt().editor.is_input_imple_mode(true) {
                self.curt().editor.input_comple.draw(&mut str_vec);
            }
            Dialog::get().draw(&mut str_vec);
        }

        Log::debug("cur", &self.curt().editor.win_mgr.curt().cur);
        Log::debug("offset", &self.curt().editor.win_mgr.curt().offset);
        // Log::debug("history.undo_vec", &self.curt().editor.history.undo_vec);
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
        Log::debug("self.curt().state.is_nomal()", &State::get().curt_state().is_nomal());
        Log::debug("self.curt().editor.is_cur_y_in_screen()", &self.curt().editor.is_cur_y_in_screen());
        Log::debug("self.curt().editor.win_mgr.curt()", &self.curt().editor.win_mgr.curt());

        let mut str_vec: Vec<String> = vec![];
        if CtxMenu::get().is_show || MenuBar::get().menulist.is_show || Dialog::get().is_show {
            View::hide_cur();
        } else if State::get().curt_state().is_nomal() && self.curt().editor.is_cur_y_in_screen() {
            self.curt().editor.draw_cur(&mut str_vec);
            View::show_cur();
        } else if !State::get().curt_state().is_nomal() && self.curt().prom.curt.as_mut_base().is_draw_cur() {
            self.curt().prom.draw_cur(&mut str_vec);
            View::show_cur();
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
}
