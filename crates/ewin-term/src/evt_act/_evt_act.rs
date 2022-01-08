use crate::{
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*, keywhen::*},
        _cfg::lang::lang_cfg::*,
        log::*,
        model::*,
    },
    model::*,
};
use std::io::Write;

impl EvtAct {
    pub fn draw<T: Write>(term: &mut Terminal, out: &mut T, act_type: &ActType) {
        Log::debug("EvtAct::term.keycmd", &term.keycmd);
        Log::debug("EvtAct::draw.evt_act_type", &act_type);
        Log::debug("EvtAct::draw.term.draw_parts_org", &term.draw_parts_org);

        if let ActType::Draw(draw_parts) = act_type {
            if term.state.is_show_init_info {
                let row_posi = term.curt().editor.row_posi;
                term.curt().editor.clear_draw(out, row_posi);
                term.state.is_show_init_info = false;
            }

            // Judge whether to delete ctx_menu
            let draw_parts = if term.state.is_ctx_menu_hide_draw {
                term.state.is_ctx_menu_hide_draw = false;
                &DParts::All
            } else {
                draw_parts
            };

            Log::debug("EvtAct::draw_parts", &draw_parts);

            match &draw_parts {
                DParts::None => {}
                DParts::MsgBar(msg) | DParts::AllMsgBar(msg) => {
                    if msg == &Lang::get().key_recording {
                        term.curt().mbar.set_keyrecord(msg);
                    } else {
                        term.curt().mbar.set_err(msg);
                    }
                    if let DParts::MsgBar(_) = draw_parts {
                        term.curt().mbar.draw_only(out);
                    } else if let DParts::AllMsgBar(_) = draw_parts {
                        term.draw(out, &DParts::All);
                    }
                }

                DParts::CtxMenu => term.ctx_menu_group.draw_only(out),
                DParts::Prompt => EvtAct::draw_prompt(out, term),
                DParts::All | DParts::Editor | DParts::ScrollUpDown(_) => {
                    // If the last time was an err msg, redraw the whole to delete it.
                    if let DParts::MsgBar(_) | DParts::AllMsgBar(_) = &term.draw_parts_org {
                        term.curt().editor.draw_range = E_DrawRange::All;
                    }
                    term.draw(out, draw_parts);
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
            ActType::Cancel => {
                term.draw_cur(out);
                Some(false)
            }
            ActType::Exit => Some(true),
        };
    }

    pub fn match_event<T: Write>(keys: Keys, out: &mut T, term: &mut Terminal) -> bool {
        // Support check for pressed keys
        let act_type = EvtAct::set_keys(keys, term);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        //  term.state.set_org_state();
        Log::debug("term.keycmd", &term.keycmd);

        Terminal::hide_cur();

        // Pressed keys Pre-check
        let act_type = EvtAct::init_event(term);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        // msg
        EvtAct::set_org_msg(term.curt());
        term.curt().mbar.clear_mag();

        let keywhen = term.get_when(&keys);
        Log::info("keywhen", &keywhen);

        match keywhen {
            KeyWhen::CtxMenuFocus => {
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
                // editor
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
                // prom
                let act_type = EvtAct::ctrl_prom(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            _ => {}
        };
        return false;
    }

    pub fn set_keys(keys: Keys, term: &mut Terminal) -> ActType {
        if !term.state.is_ctx_menu {
            match keys {
                Keys::MouseMove(_, _) => {
                    // Initialized for post-processing
                    term.keycmd = KeyCmd::Null;
                    return ActType::Cancel;
                }
                // Because the same key occurs multiple times in the case of Windows.
                #[cfg(target_os = "windows")]
                Keys::MouseDragLeft(_, _) if keys == term.keys_org => return ActType::Cancel,
                _ => Log::info("Pressed key", &keys),
            };
        }
        term.set_keys(&keys);
        if term.keycmd == KeyCmd::Unsupported {
            return ActType::Draw(DParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }
        let keycmd = term.keycmd.clone();
        term.ctx_menu_group.set_cmd(keycmd.clone());
        term.curt().editor.set_cmd(keycmd.clone());
        term.curt().prom.set_cmd(keycmd);

        return ActType::Next;
    }

    pub fn init_event(term: &mut Terminal) -> ActType {
        Log::debug_key("init_event");
        match &term.keycmd {
            KeyCmd::CtxMenu(C_Cmd::MouseMove(_, _)) => return if term.state.is_ctx_menu { ActType::Next } else { ActType::Cancel },
            KeyCmd::Resize => {
                if Terminal::check_displayable() {
                    term.state.is_displayable = true;
                    term.curt().editor.draw_range = E_DrawRange::None;

                    return if term.curt().state.is_nomal() { ActType::Draw(DParts::All) } else { ActType::Next };
                } else {
                    term.state.is_displayable = false;
                    Terminal::clear_display();
                    Terminal::hide_cur();
                    println!("{}", &Lang::get().increase_height_width_terminal);
                    return ActType::Cancel;
                }
            }
            KeyCmd::CloseFile => {
                term.curt().prom.clear();
                term.curt().state.clear();
                term.clear_ctx_menu();
                return ActType::Next;
            }
            _ => return if term.state.is_displayable { ActType::Next } else { ActType::Cancel },
        };
    }
}
