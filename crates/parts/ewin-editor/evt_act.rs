use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::models::evt::*;
use ewin_key::key::cmd::*;

impl Editor {
    pub fn ctrl_editor(&mut self, cmd: Cmd) -> ActType {
        Log::debug_key("EvtAct::ctrl_editor");

        let evt_act = self.exec_editor(cmd);
        if self.cmd.config.is_recalc_scrl {
            self.calc_editor_scrlbar_h();
            self.calc_editor_scrlbar_v();
        }

        if evt_act != ActType::Next {
            return evt_act;
        }
        return ActType::Draw(self.get_draw_parts());
    }

    pub fn exec_editor(&mut self, cmd: Cmd) -> ActType {
        Log::debug_key("EvtAct::exec_editor");
        self.set_cmd(cmd);
        if self.state.is_read_only && self.cmd.config.is_edit {
            return ActType::Cancel;
        }
        self.set_org_state();
        self.init();
        self.set_tgt_window();

        let cmd = &self.cmd.clone();
        Log::debug("cmd", &cmd);

        match cmd.cmd_type {
            CmdType::Test => {}
            /*
             * editor
             */
            _ => {
                let evt_act = self.proc();
                if evt_act != ActType::Next {
                    return evt_act;
                }
            }
        }

        self.record_key();

        return ActType::Next;
    }
}
