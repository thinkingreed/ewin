use crate::tabs::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, evt::*};
use ewin_key::{global::*, key::cmd::*, model::*};
use ewin_prom::each::{grep::PromGrep, save_confirm::*, search::PromSearch};
use ewin_state::term::*;
use ewin_view::{view::*, view_traits::view_trait::*};
use std::io::Write;

impl Tabs {
    pub fn ctrl_prom(&mut self, cmd: &Cmd) -> ActType {
        Log::debug_key("ctrl_prom");

        if !State::get().curt_state().is_nomal_and_not_grep_result() {
            self.curt().prom.set_cmd(cmd);

            // Resize
            let evt_act = self.curt().prom.resize();
            if evt_act != ActType::Next {
                return evt_act;
            }
            // Cancel
            let evt_act = self.cancel_prom();
            if evt_act != ActType::Next {
                return evt_act;
            }
            // Clear msg
            //  self.curt().msgbar.clear_mag();

            let evt_act = self.curt().prom.curt.as_mut_base().ctrl_cont();

            Log::debug("evt_act", &evt_act);
            if evt_act != ActType::Next {
                return evt_act;
            }

            // let prom = self.curt().prom.curt;
            let prom_state = State::get().curt_state().prom;
            return match prom_state {
                PromState::Search => self.curt().prom.curt.downcast_mut::<PromSearch>().unwrap().search(),
                PromState::SaveConfirm => self.curt().prom.curt.downcast_mut::<PromSaveConfirm>().unwrap().save_confirm(),
                PromState::Replase => self.curt().replace(),
                PromState::Grep => self.curt().prom.curt.downcast_mut::<PromGrep>().unwrap().grep(),
                PromState::GrepResult => self.grep_result(),
                PromState::EncNl => self.enc_nl(),
                PromState::MoveRow => self.move_row(),
                PromState::SaveNewFile => self.save_new_filenm(),
                PromState::SaveForced => self.save_forced(),
                PromState::WatchFile => self.watch_file(),
                PromState::OpenFile => self.open_file_prom(),
                _ => ActType::Draw(DParts::All),
            };
        } else {
            return ActType::Next;
        }
    }

    pub fn cancel_prom(&mut self) -> ActType {
        match self.curt().prom.cmd.cmd_type {
            CmdType::CancelProm => {
                if self.state.is_all_close_confirm {
                    self.cancel_close_all_tab();
                    self.curt().clear_curt_tab(true);
                } else if self.state.is_all_save {
                    self.cancel_save_all_tab();
                    self.curt().clear_curt_tab(true);
                } else if State::get().curt_state().prom == PromState::Greping {
                    GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceling).unwrap();
                } else if State::get().curt_state().prom == PromState::GrepResult {
                    self.curt().clear_curt_tab(true);
                } else if State::get().curt_state().prom == PromState::Search {
                    self.curt().editor.search.clear();
                    self.curt().clear_curt_tab(true);
                } else {
                    self.curt().clear_curt_tab(true);
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Next,
        }
    }

    pub fn draw_prompt<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("draw_prompt");
        // Hide the cursor at this position to target anything other than mouse move
        View::hide_cur();
        self.set_size();
        self.curt().msgbar.draw_only(out);
        // term.self.curt().sbar.draw_only(out);
        self.curt().prom.draw_only(out);
        if self.curt().prom.curt.as_mut_base().is_draw_cur() {
            View::show_cur();
        }
    }
}
