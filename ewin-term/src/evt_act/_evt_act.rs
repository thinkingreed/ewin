use crate::{
    ewin_core::{
        _cfg::key::{keycmd::*, keys::*, keywhen::*},
        global::*,
        log::*,
        model::*,
    },
    model::*,
    terminal::*,
};
use std::io::Write;

impl EvtAct {
    pub fn draw<T: Write>(term: &mut Terminal, out: &mut T, act_type: &ActType) {
        Log::debug("EvtAct::term.keycmd", &term.keycmd);
        Log::debug("EvtAct::draw.evt_act_type", &act_type);
        Log::debug("EvtAct::draw.term.draw_parts_org", &term.draw_parts_org);

        match act_type {
            ActType::Draw(draw_parts) => {
                match &draw_parts {
                    DParts::MsgBar(msg) | DParts::AllMsgBar(msg) => {
                        if msg == &LANG.key_recording {
                            term.curt().mbar.set_keyrecord(&msg);
                        } else {
                            term.curt().mbar.set_err(&msg);
                        }
                        if let DParts::MsgBar(_) = draw_parts {
                            term.curt().mbar.draw_only(out);
                        } else if let DParts::AllMsgBar(_) = draw_parts {
                            term.draw(out, &DParts::All);
                        }
                    }

                    DParts::CtxMenu => {
                        term.set_draw_range_ctx_menu();
                        term.ctx_menu_group.draw_only(out);
                    }
                    DParts::Prompt => EvtAct::draw_prompt(out, term),
                    DParts::All | DParts::Editor | DParts::ScrollUpDown(_) => {
                        // If the last time was an err msg, redraw the whole to delete it.
                        if let DParts::MsgBar(_) | DParts::AllMsgBar(_) = &term.draw_parts_org {
                            term.curt().editor.draw_range = EditorDrawRange::All;
                        }
                        term.draw(out, &draw_parts);
                    }
                };
                term.draw_parts_org = draw_parts.clone();
            }
            _ => {}
        }
    }
    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal, act_type: ActType) -> Option<bool> {
        match term.keycmd {
            // Log at the time of Mouse Move is not output
            KeyCmd::Null => {}
            _ => {
                Log::debug("evt_act_type", &act_type);
                Log::debug("term.keycmd", &term.keycmd);
            }
        }

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
        Terminal::hide_cur();

        Log::info("term.keycmd", &term.keycmd);

        // Pressed keys Pre-check
        let act_type = EvtAct::init_event(term);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }

        // msg
        EvtAct::set_org_msg(&mut term.curt());
        term.curt().mbar.clear_mag();

        let keywhen = term.get_when(&keys);
        Log::info("keywhen", &keywhen);

        match keywhen {
            KeyWhen::CtxMenuFocus => {
                // ctx_menu
                let act_type = EvtAct::ctrl_ctx_menu(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
            }
            KeyWhen::EditorFocus => {
                // headerbar
                let act_type = EvtAct::ctrl_headerbar(term);
                if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
                    return rtn;
                }
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
                _ => Log::info("Pressed key", &keys),
            };
        }
        term.set_keys(&keys);
        if term.keycmd == KeyCmd::Unsupported {
            return ActType::Draw(DParts::MsgBar(LANG.unsupported_operation.to_string()));
        }
        term.ctx_menu_group.set_keys(keys);
        term.curt().prom.set_keys(keys);

        return ActType::Next;
    }

    pub fn init_event(term: &mut Terminal) -> ActType {
        Log::debug_key("init_event");
        match &term.keycmd {
            KeyCmd::CtxMenu(C_Cmd::MouseMove(_, _)) => return if term.state.is_ctx_menu { ActType::Next } else { ActType::Cancel },
            KeyCmd::Resize => {
                if Terminal::check_displayable() {
                    term.state.is_displayable = true;
                    return if term.curt().state.is_nomal() { ActType::Draw(DParts::All) } else { ActType::Next };
                } else {
                    term.state.is_displayable = false;
                    Terminal::clear_display();
                    Terminal::hide_cur();
                    println!("{}", &LANG.increase_height_width_terminal);
                    return ActType::Cancel;
                }
            }
            KeyCmd::CloseFile => {
                term.curt().prom.clear();
                term.curt().state.clear();
                return ActType::Next;
            }
            _ => return if term.state.is_displayable { ActType::Next } else { ActType::Cancel },
        };
    }
}
