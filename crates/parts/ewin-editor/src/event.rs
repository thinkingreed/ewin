use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::models::event::*;
use ewin_key::key::cmd::*;
use ewin_state::term::*;

impl Editor {
    pub fn ctrl_editor(&mut self, cmd: Cmd) -> ActType {
        Log::debug_key("EvtAct::ctrl_editor");

        let evt_act = self.exec_editor(cmd);
        if self.cmd.config.is_recalc_scrl {
            self.calc_editor_scrlbar();
        }

        if evt_act != ActType::Next {
            return evt_act;
        }
        return ActType::Draw(self.get_draw_parts());
    }

    pub fn exec_editor(&mut self, cmd: Cmd) -> ActType {
        Log::debug_key("EvtAct::exec_editor");
        self.set_cmd(cmd);
        if State::get().curt_state().editor.is_read_only && self.cmd.config.is_edit {
            return ActType::Cancel;
        }
        self.set_org_state();
        self.init();
        self.set_tgt_window();

        let cmd = &self.cmd.clone();
        Log::debug("cmd", &cmd);

        let evt_act = self.proc();

        self.record_key();

        if evt_act != ActType::Next {
            return evt_act;
        }

        return ActType::Next;
    }
}
