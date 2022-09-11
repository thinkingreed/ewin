use ewin_cfg::{colors::*, log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*};
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_file_bar::filebar::*;
use ewin_menu_bar::menubar::*;
use ewin_msg_bar::msgbar::*;
use ewin_prom::model::*;
use ewin_side_bar::sidebar::*;
use ewin_state::term::*;
use ewin_status_bar::statusbar::*;
use ewin_view::{menulists::core::*, view::*};
use std::io::{stdout, Write};

use crate::term::*;

impl Term {
    pub fn draw<T: Write>(&mut self, out: &mut T, act_type: &ActType) {
        Log::debug("Term::draw.evt_act_type", &act_type);

        self.set_size();

        if let ActType::Draw(draw_parts) = act_type {
            match &draw_parts {
                DrawParts::MsgBar(msg) | DrawParts::TabsAllMsgBar(msg) => MsgBar::get().set_err(msg),
                _ => {}
            };
            Log::debug("self.keywhen", &self.place);
            Log::debug("self.keywhen_org", &self.place_org);
            let draw_parts = if self.place == self.place_org { draw_parts } else { &DrawParts::All };

            Log::debug("draw_parts", &draw_parts);

            match &draw_parts {
                DrawParts::None => {}
                DrawParts::MsgBar(_) => MsgBar::get().draw_only(out),
                DrawParts::TabsAllMsgBar(_) => self.tabs.draw(out, &DrawParts::TabsAll),
                DrawParts::MenuBar => MenuBar::get().draw_only(out),
                DrawParts::FileBar => FileBar::draw_only(out),
                DrawParts::Prompt => Prom::get().draw_only(out),
                DrawParts::MenuBarMenuList => MenuBar::get().menulist.draw_only(out),
                DrawParts::InputComple => self.tabs.curt().editor.input_comple.draw_only(out),
                DrawParts::CtxMenu => CtxMenu::get().draw_only(out),
                DrawParts::Dialog => Dialog::draw_only(out),
                DrawParts::TabsAll => self.tabs.draw(out, draw_parts),
                DrawParts::Editor(_) => self.tabs.draw(out, draw_parts),
                DrawParts::StatusBar => {
                    let (cur_cont, mut opt_vec) = StatusBar::get_editor_conts(&self.tabs.curt().editor);
                    StatusBar::get().draw_only(out, cur_cont, &mut opt_vec);
                }
                DrawParts::SideBar => SideBar::get().draw_only(out),
                DrawParts::All | DrawParts::ScrollUpDown(_) => {
                    // View::clear_all();
                    self.set_size();

                    MenuBar::get().draw_only(out);
                    if MenuBar::get().menulist.is_show {
                        MenuBar::get().menulist.draw_only(out);
                    }
                    self.tabs.draw(out, &DrawParts::TabsAll);
                    SideBar::get().draw_only(out);
                    MsgBar::get().draw_only(out);
                }

                DrawParts::TabsAbsolute(range) => {
                    // SideBar
                    if range.contains(&SideBar::get().cont.as_base().view.y) {
                        SideBar::get().draw_only(out);
                    };
                    // Menubar
                    if range.contains(&MenuBar::get().view.y) {
                        MenuBar::get().draw_only(out);
                    };
                    // FileBar
                    if range.contains(&FileBar::get().view.y) {
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
                                self.tabs.curt().editor.draw_only(out);
                            }
                        }
                    }

                    // MsgBar
                    if range.contains(&MsgBar::get().view.height) {
                        MsgBar::get().draw_only(out);
                    };
                    // StatusBar
                    if range.contains(&StatusBar::get().view.y) {
                        let (cur_cont, mut opt_vec) = StatusBar::get_editor_conts(&self.tabs.curt().editor);
                        StatusBar::get().draw_only(out, cur_cont, &mut opt_vec);
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
            if self.tabs.curt().editor.is_enable_syntax_highlight && Cfg::get().colors.theme.theme_bg_enable {
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
        self.draw(out, &ActType::Draw(DrawParts::All));
        self.draw_cur(out);
    }
}
