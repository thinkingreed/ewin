use crate::{
    bar::{filebar::*, statusbar::*},
    ewin_com::{
        _cfg::key::{keys::*, keywhen::*},
        global::*,
        model::*,
    },
    global_term::*,
    model::*,
    terms::term::*,
};
use crossterm::{cursor::MoveTo, execute};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::{_cfg::key::cmd::*, util::*};
use ewin_const::def::*;
use ewin_widget::core::*;
use std::io::{stdout, Write};

impl EvtAct {
    pub fn draw<T: Write>(term: &mut Terminal, out: &mut T, act_type: &ActType) {
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
            Log::debug("EvtAct::draw_parts", &draw_parts);

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
                DParts::MenuWidget => term.menubar.widget.draw_only(out),
                DParts::InputComple => term.curt().editor.input_comple.draw_only(out),
                DParts::CtxMenu => term.ctx_widget.draw_only(out),
                DParts::All | DParts::ScrollUpDown(_) => term.draw(out, draw_parts),
                DParts::Editor(_) => term.draw(out, draw_parts),
                DParts::StatusBar => StatusBar::draw_only(out, &mut term.tabs[term.tab_idx], H_FILE_VEC.get().unwrap().try_lock().unwrap().get(term.tab_idx).unwrap()),
                DParts::Absolute(range) => {
                    // FileBar
                    if range.contains(&term.fbar.row_posi) {
                        FileBar::draw_only(term, out);
                    };
                    // Editor
                    if term.curt().editor.get_disp_range_absolute().contains(&range.end) {
                        let row_posi = term.curt().editor.get_curt_row_posi();
                        let offset_y = term.curt().editor.win_mgr.curt().offset.y;
                        let sy = if range.start < row_posi { 0 } else { range.start - row_posi + offset_y };
                        term.draw(out, &DParts::Editor(E_DrawRange::TargetRange(sy, range.end + row_posi + offset_y)));
                    }
                    // Menubar
                    if range.contains(&term.menubar.row_posi) {
                        term.menubar.draw_only(out);
                    };
                    // MenuWidget
                    if range.contains(&term.menubar.widget.curt.disp_sy) {
                        term.menubar.widget.draw_only(out);
                    };
                    // InputComple
                    if range.contains(&term.curt().editor.input_comple.widget.disp_sy) {
                        term.curt().editor.input_comple.draw_only(out);
                    };
                }
            };
            term.draw_parts_org = draw_parts.clone();
        }
    }

    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal, act_type: ActType) -> Option<bool> {
        return match &act_type {
            ActType::Next => None,
            ActType::Draw(_) => {
                EvtAct::draw(term, out, &act_type);
                term.draw_cur(out);
                Some(false)
            }
            ActType::Nothing => Some(false),
            ActType::Cancel => {
                term.draw_cur(out);
                Some(false)
            }
            ActType::Exit => Some(true),
        };
    }

    pub fn match_event<T: Write>(keys: Keys, out: &mut T, term: &mut Terminal) -> bool {
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
        Terminal::hide_cur();

        // msg
        term.curt().msgbar.clear_mag();
        term.menubar.redraw_menubar_on_mouse(&keys);

        match term.keywhen {
            KeyWhen::CtxMenu => {
                term.ctx_widget.set_ctx_menu_cmd(term.cmd.clone());
                let act_type = EvtAct::ctrl_ctx_menu(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::MenuBar => {
                term.menubar.widget.set_menubar_cmd(term.cmd.clone());
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

    pub fn check_keys(keys: Keys, term: &mut Terminal) -> ActType {
        match keys {
            Keys::MouseMove(y, x) if y == term.menubar.row_posi as u16 && term.menubar.is_menubar_displayed_area(y as usize, x as usize).0 => {}
            Keys::MouseMove(y, _) if y > term.menubar.row_posi as u16 && term.menubar.on_mouse_idx != USIZE_UNDEFINED => {}
            Keys::MouseMove(_, _) if term.state.is_menuwidget || term.state.is_ctx_menu || term.curt().editor.is_input_imple_mode(true) => {}
            Keys::MouseMove(_, _) if term.curt().is_prom_pulldown() => {}
            Keys::MouseMove(_, _) => {
                term.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::Nothing;
            }
            Keys::MouseUpLeft(_, _) if term.fbar.state.is_dragging => {}
            Keys::MouseUpLeft(_, _) => {
                term.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::Nothing;
            }
            // Because the same key occurs multiple times
            // MouseDragLeft: in the case of Windows and Ubuntu.
            // Resize: in the case of Windows.
            Keys::MouseDragLeft(_, _) | Keys::Resize(_, _) if keys == term.keys_org => return ActType::Nothing,
            Keys::Resize(_, _) => {
                set_term_size();
                Terminal::clear_all();
                if Terminal::check_displayable() {
                    term.set_bg_color();
                    term.state.is_displayable = true;
                } else {
                    term.state.is_displayable = false;
                    let _ = execute!(stdout(), MoveTo(0, 0));
                    println!("{}", &Lang::get().increase_height_width_terminal);
                    return ActType::Nothing;
                }
            }
            _ => {}
        };
        if !term.state.is_displayable {
            return ActType::Nothing;
        }
        // Judg whether keys are CloseFile
        if let Some(_keys) = CMD_TYPE_MAP.get().unwrap().get(&CmdType::CloseFile) {
            if _keys == &keys {
                term.clear_ctx_menu();
                term.curt().clear_curt_tab(true);
            }
        }
        return ActType::Next;
    }

    pub fn set_keys(keys: Keys, term: &mut Terminal) -> ActType {
        Log::info("Pressed key", &keys);
        term.set_keys(&keys);
        if term.cmd.cmd_type == CmdType::Unsupported {
            return ActType::Draw(DParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }
        return ActType::Next;
    }
}
