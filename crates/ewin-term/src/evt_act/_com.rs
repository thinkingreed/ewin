use crate::{bar::filebar::*, model::*, terms::term::*};
use crossterm::{cursor::MoveTo, execute};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{def::*, model::*, term::*};
use ewin_dialog::dialog::*;
use ewin_key::{
    global::*,
    key::{cmd::*, keys::*, keywhen::*},
};
use ewin_menulist::core::*;
use ewin_view::view_trait::view_trait::*;
use std::io::{stdout, Write};

impl EvtAct {
    pub fn draw<T: Write>(term: &mut Term, out: &mut T, act_type: &ActType) {
        Log::debug("EvtAct::draw.evt_act_type", &act_type);

        if let ActType::Draw(draw_parts) = act_type {
            // Judge whether to delete ctx_menu

            let draw_parts = if term.state.is_ctx_menu_hide_draw || term.state.is_menuwidget_hide_draw {
                term.state.is_ctx_menu_hide_draw = false;
                term.state.is_menuwidget_hide_draw = false;
                &DParts::All
            } else {
                draw_parts
            };

            match &draw_parts {
                DParts::None => {}
                DParts::MsgBar(msg) | DParts::AllMsgBar(msg) => {
                    term.curt().msgbar.set_err(msg);
                    if let DParts::MsgBar(_) = draw_parts {
                        term.curt().msgbar.draw_only(out);
                    } else if let DParts::AllMsgBar(_) = draw_parts {
                        term.draw(out, &DParts::All);
                    }
                }
                DParts::MenuBar => term.menubar.draw_only(out),
                DParts::FileBar => FileBar::draw_only(term, out),
                DParts::Prompt => EvtAct::draw_prompt(term, out),
                DParts::MenuWidget => term.menubar.menulist.draw_only(out),
                DParts::InputComple => term.curt().editor.input_comple.draw_only(out),
                DParts::CtxMenu => term.ctx_menu.draw_only(out),
                DParts::Dialog => Dialog::draw_only(out),
                DParts::All | DParts::ScrollUpDown(_) => term.draw(out, draw_parts),
                DParts::Editor(_) => term.draw(out, draw_parts),
                DParts::StatusBar => term.curt().draw_sbar_only(out),
                DParts::Absolute(range) => {
                    Log::debug("range", &range);

                    // Menubar
                    if range.contains(&term.menubar.row_posi) {
                        term.menubar.draw_only(out);
                    };
                    // FileBar
                    if range.contains(&term.fbar.row_posi) {
                        FileBar::draw_only(term, out);
                    };
                    // Editor
                    if term.curt().editor.is_disp_range_absolute(range) {
                        let row_posi = term.curt().editor.get_curt_row_posi();
                        let offset_y = term.curt().editor.win_mgr.curt().offset.y;
                        let sy = if range.start < row_posi { 0 } else { range.start - row_posi + offset_y };
                        let ey = if range.end > row_posi + term.curt().editor.get_curt_row_len() { offset_y + term.curt().editor.get_curt_row_len() - 1 } else { range.end - row_posi + offset_y };
                        term.curt().editor.draw_range = E_DrawRange::TargetRange(sy, ey);
                        Log::debug("term.curt().editor.draw_range", &term.curt().editor.draw_range);
                        term.curt().draw_editor_only(out);
                    }
                    // MsgBar
                    if range.contains(&term.curt().msgbar.row_posi) {
                        term.curt().msgbar.draw_only(out);
                    };
                    // StatusBar
                    if range.contains(&term.curt().sbar.row_posi) {
                        term.curt().draw_sbar_only(out);
                    };
                    // MenuWidget
                    if range.contains(&term.menubar.menulist.curt.disp_sy) {
                        term.menubar.menulist.draw_only(out);
                    };
                    // InputComple
                    if range.contains(&term.curt().editor.input_comple.menulist.disp_sy) {
                        term.curt().editor.input_comple.draw_only(out);
                    };
                    // CtxMenu
                    if range.contains(&term.ctx_menu.menulist.get_disp_range_y().start) || range.contains(&term.ctx_menu.menulist.get_disp_range_y().end) {
                        term.ctx_menu.draw_only(out);
                    }
                    // Dialog
                    if Dialog::contain_absolute_range(range) {
                        Dialog::draw_only(out);
                    }
                }
            };
            term.draw_parts_org = draw_parts.clone();
        }
    }

    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Term, act_type: ActType) -> Option<bool> {
        return match &act_type {
            ActType::Next => None,
            ActType::Draw(_) => {
                EvtAct::draw(term, out, &act_type);
                term.draw_cur(out);
                Some(false)
            }
            ActType::None => Some(false),
            ActType::Cancel => {
                term.draw_cur(out);
                Some(false)
            }
            ActType::Exit => Some(true),
        };
    }

    pub fn match_event<T: Write>(keys: Keys, out: &mut T, term: &mut Term) -> bool {
        // Pressed keys check
        let act_type = EvtAct::check_keys(keys, term);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        // Support check for pressed keys
        let act_type = EvtAct::set_keys(keys, term);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        Term::hide_cur();
        // msg
        term.curt().msgbar.clear_mag();
        term.menubar.redraw_menubar_on_mouse(&keys);

        match term.keywhen {
            KeyWhen::Dialog => {
                let act_type = Dialog::ctrl_dialog(&term.cmd.cmd_type);
                Log::info("ctrl_dialog act_type", &act_type);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::CtxMenu => {
                term.ctx_menu.set_ctx_menu_cmd(term.cmd.clone());
                let act_type = EvtAct::ctrl_ctx_menu(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::MenuBar => {
                term.menubar.menulist.set_menubar_cmd(term.cmd.clone());
                let act_type = EvtAct::ctrl_menubar(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::FileBar => {
                let act_type = EvtAct::ctrl_filebar(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::Editor => {
                let act_type = EvtAct::ctrl_editor(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::StatusBar => {
                let act_type = EvtAct::ctrl_statusbar(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::Prom => {
                let act_type = EvtAct::ctrl_prom(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            _ => {}
        };
        return false;
    }

    pub fn check_keys(keys: Keys, term: &mut Term) -> ActType {
        match keys {
            Keys::MouseMove(y, x) if y == term.menubar.row_posi as u16 && term.menubar.is_menubar_displayed_area(y as usize, x as usize).0 => {}
            Keys::MouseMove(y, _) if y > term.menubar.row_posi as u16 && term.menubar.on_mouse_idx != USIZE_UNDEFINED => {}
            Keys::MouseMove(_, _) if term.state.is_menubar_menulist || term.state.is_ctx_menu || term.curt().editor.is_input_imple_mode(true) => {}
            Keys::MouseMove(_, _) if term.curt().is_prom_pulldown() => {}
            Keys::MouseMove(y, x) if Dialog::get().is_tgt_mouse_move(y as usize, x as usize) => {}
            Keys::MouseMove(_, _) => {
                term.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::None;
            }
            Keys::MouseUpLeft(_, _) if term.fbar.state.is_dragging => {}
            Keys::MouseUpLeft(_, _) => {
                term.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::None;
            }
            // Because the same key occurs multiple times
            // MouseDragLeft: in the case of Windows and Ubuntu.
            // Resize: in the case of Windows.
            Keys::MouseDragLeft(_, _) | Keys::Resize(_, _) if keys == term.keys_org => return ActType::None,
            Keys::Resize(_, _) => {
                set_term_size();
                Term::clear_all();
                if Term::check_displayable() {
                    term.set_bg_color();
                    term.state.is_displayable = true;
                } else {
                    term.state.is_displayable = false;
                    let _ = execute!(stdout(), MoveTo(0, 0));
                    println!("{}", &Lang::get().increase_height_width_terminal);
                    return ActType::None;
                }
            }
            _ => {}
        };
        if !term.state.is_displayable {
            return ActType::None;
        }
        // Judg whether keys are CloseFile
        if let Some(_keys) = CMD_TYPE_MAP.get().unwrap().get(&CmdType::CloseFile) {
            if _keys == &keys {
                Dialog::get().clear();
                term.clear_ctx_menu();
                term.curt().clear_curt_tab(true);
            }
        }
        return ActType::Next;
    }

    pub fn set_keys(keys: Keys, term: &mut Term) -> ActType {
        Log::info("Pressed key", &keys);
        term.set_keys(&keys);
        if term.cmd.cmd_type == CmdType::Unsupported {
            return ActType::Draw(DParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }
        return ActType::Next;
    }
}
