use crate::{
    bar::headerbar::*,
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*, keywhen::*},
        _cfg::lang::lang_cfg::*,
        global::*,
        log::*,
        model::*,
    },
    model::*,
};
use ewin_window::window::*;
use std::io::Write;

impl EvtAct {
    pub fn draw<T: Write>(term: &mut Terminal, out: &mut T, act_type: &ActType) {
        Log::debug("EvtAct::term.keycmd", &term.keycmd);
        Log::debug("EvtAct::draw.evt_act_type", &act_type);
        Log::debug("EvtAct::draw.term.draw_parts_org", &term.draw_parts_org);

        if let ActType::Render(draw_parts) = act_type {
            // Judge whether to delete ctx_menu
            let draw_parts = if term.state.is_ctx_menu_hide_draw {
                term.state.is_ctx_menu_hide_draw = false;
                &RParts::All
            } else {
                draw_parts
            };

            Log::debug("EvtAct::draw_parts", &draw_parts);

            match &draw_parts {
                RParts::None => {}
                RParts::MsgBar(msg) | RParts::AllMsgBar(msg) => {
                    if msg == &Lang::get().key_recording {
                        term.curt().mbar.set_keyrecord(msg);
                    } else {
                        term.curt().mbar.set_err(msg);
                    }
                    if let RParts::MsgBar(_) = draw_parts {
                        term.curt().mbar.draw_only(out);
                    } else if let RParts::AllMsgBar(_) = draw_parts {
                        term.render(out, &RParts::All);
                    }
                }
                RParts::HeaderBar => HeaderBar::draw_only(term, out),
                RParts::Prompt => EvtAct::draw_prompt(out, term),
                RParts::CtxMenu => term.ctx_menu.draw_only(out),
                RParts::All | RParts::Editor | RParts::ScrollUpDown(_) => {
                    if let RParts::MsgBar(_) | RParts::AllMsgBar(_) = &term.draw_parts_org {
                        term.curt().editor.draw_range = E_DrawRange::All;
                    }
                    term.render(out, draw_parts);
                }
            };
            term.draw_parts_org = draw_parts.clone();
        }
    }
    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal, act_type: ActType) -> Option<bool> {
        return match &act_type {
            ActType::Next => None,
            ActType::Render(_) => {
                EvtAct::draw(term, out, &act_type);
                term.render_cur(out);
                Some(false)
            }
            ActType::Cancel => {
                term.render_cur(out);
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
        Log::debug("term.keycmd", &term.keycmd);
        Terminal::hide_cur();

        // msg
        EvtAct::set_org_msg(term.curt());
        term.curt().mbar.clear_mag();

        let keywhen = term.get_when(&keys);
        Log::info("keywhen", &keywhen);
        let keycmd = term.keycmd.clone();
        match keywhen {
            KeyWhen::CtxMenuFocus => {
                term.ctx_menu.set_ctx_menu_cmd(term.keycmd.clone());
                let act_type = EvtAct::ctrl_ctx_menu(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::HeaderBarFocus => {
                let act_type = EvtAct::ctrl_headerbar(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }

            KeyWhen::EditorFocus => {
                term.curt().editor.set_cmd(keycmd);
                let act_type = EvtAct::ctrl_editor(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
                // statusbar
                let act_type = EvtAct::ctrl_statusbar(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::PromptFocus => {
                term.curt().prom.set_cmd(keycmd);
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
        if matches!(keys, Keys::MouseMove(_, _)) && (!term.state.is_ctx_menu && !term.curt().editor.is_input_imple_mode(true)) || !term.hbar.state.is_dragging && matches!(keys, Keys::MouseUpLeft(_, _)) {
            // Initialized for post-processing
            term.keycmd = KeyCmd::Null;
            return ActType::Cancel;

            // Because the same key occurs multiple times
            // MouseDragLeft: in the case of Windows and Ubuntu.
            // Resize: in the case of Windows.
        } else if (matches!(keys, Keys::MouseDragLeft(_, _)) || matches!(keys, Keys::Resize(_, _))) && keys == term.keys_org {
            return ActType::Cancel;
        } else if matches!(keys, Keys::Resize(_, _)) {
            if Terminal::check_displayable() {
                term.set_bg_color();
                term.state.is_displayable = true;
            } else {
                term.state.is_displayable = false;
                Terminal::clear_display();
                println!("{}", &Lang::get().increase_height_width_terminal);
                return ActType::Cancel;
            }
        }
        // Judg whether keys are CloseFile
        else if KEY_CMD_MAP.get().unwrap().get(&(keys, KeyWhen::AllFocus)).is_some() {
            if let Some(key_cmd) = KEY_CMD_MAP.get().unwrap().get(&(keys, KeyWhen::EditorFocus)) {
                if key_cmd == &KeyCmd::Edit(E_Cmd::CloseFile) || key_cmd == &KeyCmd::Edit(E_Cmd::CancelState) {
                    term.clear_ctx_menu();
                    term.clear_curt_tab(true);
                }
            }
        }
        return ActType::Next;
    }
    pub fn set_keys(keys: Keys, term: &mut Terminal) -> ActType {
        Log::info("Pressed key", &keys);
        term.set_keys(&keys);
        Log::info("term.keycmd", &term.keycmd);
        if term.keycmd == KeyCmd::Unsupported {
            return ActType::Render(RParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }

        return ActType::Next;
    }
}
