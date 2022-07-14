use crate::{
    ewin_com::{global::*, model::*},
    model::*,
    terms::term::*,
};
use ewin_cfg::log::*;
use ewin_com::_cfg::key::cmd::CmdType;
use std::io::Write;

impl EvtAct {
    pub fn ctrl_prom(term: &mut Terminal) -> ActType {
        Log::debug_key("ctrl_prom");

        if !term.curt().state.is_nomal_and_not_grep_result() {
            let cmd = term.cmd.clone();
            term.curt().prom.set_cmd(&cmd);
            // Resize
            let evt_act = term.curt().prom.resize();
            if evt_act != ActType::Next {
                return evt_act;
            }
            // Cancel
            let evt_act = EvtAct::cancel_prom(term);
            if evt_act != ActType::Next {
                return evt_act;
            }
            // Clear msg
            term.curt().msgbar.clear_mag();

            let evt_act = term.curt().prom.curt.as_mut_base().ctrl_cont();

            Log::debug("evt_act", &evt_act);
            if evt_act != ActType::Next {
                return evt_act;
            }

            return match term.curt().state.prom {
                PromState::Search => term.curt().search(),
                PromState::SaveConfirm => EvtAct::save_confirm(term),
                PromState::Replase => term.curt().replace(),
                PromState::Grep => EvtAct::grep(term),
                PromState::GrepResult => EvtAct::grep_result(term),
                PromState::EncNl => EvtAct::enc_nl(term),
                PromState::MoveRow => EvtAct::move_row(term),
                PromState::SaveNewFile => EvtAct::save_new_filenm(term),
                PromState::SaveForced => EvtAct::save_forced(term),
                PromState::WatchFile => EvtAct::watch_file(term),
                PromState::OpenFile => EvtAct::open_file(term),
                _ => ActType::Draw(DParts::All),
            };
        } else {
            return ActType::Next;
        }
    }

    pub fn cancel_prom(term: &mut Terminal) -> ActType {
        match term.curt().prom.cmd.cmd_type {
            CmdType::CancelProm => {
                if term.state.is_all_close_confirm {
                    term.cancel_close_all_tab();
                    term.curt().clear_curt_tab(true);
                } else if term.state.is_all_save {
                    term.cancel_save_all_tab();
                    term.curt().clear_curt_tab(true);
                } else if term.curt().state.prom == PromState::Greping {
                    GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceling).unwrap();
                } else if term.curt().state.prom == PromState::GrepResult {
                    term.curt().clear_curt_tab(true);
                } else if term.curt().state.prom == PromState::Search {
                    term.curt().editor.search.clear();
                    term.curt().clear_curt_tab(true);
                } else {
                    term.curt().clear_curt_tab(true);
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Next,
        }
    }

    pub fn draw_prompt<T: Write>(term: &mut Terminal, out: &mut T) {
        Log::debug_key("draw_prompt");
        // Hide the cursor at this position to target anything other than mouse move
        Terminal::hide_cur();
        term.set_size();
        term.curt().msgbar.draw_only(out);
        // term.curt().sbar.draw_only(out);
        let state = term.curt().state.clone();
        term.curt().prom.draw_only(out, &state);
        if term.curt().prom.curt.as_mut_base().is_draw_cur() {
            Terminal::show_cur();
        }
    }
}
