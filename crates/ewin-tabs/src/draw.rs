use crate::tabs::*;

use ewin_cfg::log::*;
use ewin_const::models::draw::*;
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_editor::{editor_gr::*, model::*};
use ewin_file_bar::filebar::*;
use ewin_menu_bar::menubar::*;
use ewin_prom::model::*;
use ewin_state::term::*;
use ewin_view::view::*;
use std::io::Write;

impl Tabs {
    pub fn draw<T: Write>(&mut self, out: &mut T, draw_parts: &DrawParts) {
        Log::info_key("Tabs.draw start");
        Log::debug("draw_parts", &draw_parts);
        Log::debug("State::get().curt_state()", &State::get().curt_ref_state());

        let mut str_vec: Vec<String> = vec![];

        Editor::draw(&mut str_vec, &mut self.curt().draw_cache_vecs, draw_parts);
        if let DrawParts::Editor(e_draw_raneg) = draw_parts {
            if e_draw_raneg == &E_DrawRange::MoveCur {
                self.draw_flush(out, &mut str_vec);
                return;
            }
        }

        if &DrawParts::TabsAll == draw_parts || matches!(draw_parts, DrawParts::ScrollUpDown(_)) {
            FileBar::draw(&mut str_vec);
            Prom::get().draw(&mut str_vec);
        }

        if draw_parts == &DrawParts::TabsAll || matches!(draw_parts, DrawParts::Editor(_)) {
            CtxMenu::get().draw(&mut str_vec);

            EditorGr::get().curt_mut().draw_input_comple(&mut str_vec);
            Dialog::get().draw(&mut str_vec);
        }

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
        Log::debug("State::get().curt_state().prom", &State::get().curt_ref_state().prom);
        Log::debug("State::get().curt_state().prom", &State::get().curt_ref_state().prom);

        let mut str_vec: Vec<String> = vec![];

        let is_cur_y_in_screen = EditorGr::get().curt_ref().is_cur_y_in_screen();
        if CtxMenu::get().is_show || MenuBar::get().menulist.is_show || Dialog::get().is_show {
            View::hide_cur();
        } else if State::get().curt_ref_state().is_nomal_or_grep_result() && is_cur_y_in_screen {
            View::show_cur();
            EditorGr::get().curt_ref().draw_cur(&mut str_vec);

            View::show_cur();
        } else if !State::get().curt_ref_state().is_nomal() && Prom::get().curt.as_mut_base().is_draw_cur() {
            Prom::get().draw_cur(&mut str_vec);
            View::show_cur();
        }
        if !str_vec.is_empty() {
            let _ = out.write(str_vec.concat().as_bytes());
            out.flush().unwrap();
        }
    }

    pub fn draw_all<T: Write>(&mut self, out: &mut T, draw_parts: DrawParts) {
        self.draw(out, &draw_parts);
        self.draw_cur(out);
    }
}
