use ewin_cfg::{colors::*, log::*, model::default::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_file_bar::filebar::*;
use ewin_menulist::menubar::*;
use ewin_state::term::*;
use ewin_view::{menulists::core::*, view::*};
use std::io::{stdout, Write};

use crate::term::*;

impl Term {
    pub fn draw<T: Write>(&mut self, out: &mut T, act_type: &ActType) {
        Log::debug("EvtAct::draw.evt_act_type", &act_type);

        if let ActType::Draw(draw_parts) = act_type {
            match &draw_parts {
                DParts::MsgBar(msg) | DParts::AllMsgBar(msg) => self.tabs.curt().msgbar.set_err(msg),
                _ => {}
            };
            Log::debug("self.keywhen", &self.place);
            Log::debug("self.keywhen_org", &self.place_org);

            let draw_parts = if self.place == self.place_org { draw_parts } else { &DParts::All };

            Log::debug("draw_parts", &draw_parts);

            match &draw_parts {
                DParts::None => {}
                DParts::MsgBar(_) => self.tabs.curt().msgbar.draw_only(out),
                DParts::AllMsgBar(_) => self.tabs.draw(out, &DParts::All),
                DParts::MenuBar => MenuBar::get().draw_only(out),
                DParts::FileBar => FileBar::draw_only(out),
                DParts::Prompt => self.tabs.draw_prompt(out),
                DParts::MenuBarMenuList => MenuBar::get().menulist.draw_only(out),
                DParts::InputComple => self.tabs.curt().editor.input_comple.draw_only(out),
                DParts::CtxMenu => CtxMenu::get().draw_only(out),
                DParts::Dialog => Dialog::draw_only(out),
                DParts::All | DParts::ScrollUpDown(_) => self.tabs.draw(out, draw_parts),
                DParts::Editor(_) => self.tabs.draw(out, draw_parts),
                DParts::StatusBar => self.tabs.curt().draw_sbar_only(out),
                DParts::Absolute(range) => {
                    // Menubar
                    if range.contains(&MenuBar::get().row_posi) {
                        MenuBar::get().draw_only(out);
                    };
                    // FileBar
                    if range.contains(&FileBar::get().row_posi) {
                        FileBar::draw_only(out);
                    };

                    // Editor
                    if self.tabs.curt().editor.is_disp_range_absolute(range) {
                        let win_list = self.tabs.curt().editor.win_mgr.win_list.clone();
                        for vec in win_list.iter() {
                            for win in vec {
                                let sy = if range.start < win.area_v.0 { 0 } else { range.start - win.area_v.0 + win.offset.y };
                                let ey = if range.end > win.area_v.0 + win.height() { win.offset.y + win.height() - 1 } else { range.end - win.area_v.0 + win.offset.y };
                                self.tabs.curt().editor.draw_range = E_DrawRange::TargetRange(sy, ey);
                                self.tabs.curt().draw_editor_only(out);
                            }
                        }
                        // let row_posi = self.tabs.curt().editor.get_curt_row_posi();
                        // let offset_y = self.tabs.curt().editor.win_mgr.curt().offset.y;
                    }

                    // MsgBar
                    if range.contains(&self.tabs.curt().msgbar.row_posi) {
                        self.tabs.curt().msgbar.draw_only(out);
                    };
                    // StatusBar
                    if range.contains(&self.tabs.curt().sbar.row_posi) {
                        self.tabs.curt().draw_sbar_only(out);
                    };
                    // Menubar menulist
                    let sy = MenuBar::get().menulist.curt.disp_sy;
                    let ey = MenuBar::get().menulist.curt.disp_ey;
                    if range.contains(&sy) || range.contains(&ey) {
                        MenuBar::get().menulist.draw_only(out);
                    };
                    // InputComple
                    let sy = self.tabs.curt().editor.input_comple.menulist.disp_sy;
                    let ey = self.tabs.curt().editor.input_comple.menulist.disp_ey;
                    if range.contains(&sy) || range.contains(&ey) {
                        self.tabs.curt().editor.input_comple.draw_only(out);
                    };
                    // CtxMenu
                    let ctx_menu_range = CtxMenu::get().menulist.get_disp_range_y();
                    if range.contains(&ctx_menu_range.start) || range.contains(&ctx_menu_range.end) {
                        // self.ctx_menu.draw_only(out);
                        CtxMenu::get().draw_only(out);
                    }
                    // Dialog
                    if Dialog::contain_absolute_range(range) {
                        Dialog::draw_only(out);
                    }
                }
            };
        }
    }

    pub fn draw_cur<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("draw_cur");

        if !State::get().term.is_displayable {
            View::hide_cur();
        } else {
            self.tabs.draw_cur(out);
        }
    }

    pub fn set_bg_color(&mut self) {
        let color_string = if CfgSyntax::get().syntax.theme.settings.background.is_some() {
            if self.tabs.curt().editor.is_enable_syntax_highlight && Cfg::get().general.colors.theme.theme_bg_enable {
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
        Log::info_key("Term.init_draw");
        self.set_bg_color();
        self.tabs.draw(out, &DParts::All);
        self.draw_cur(out);
    }
}
