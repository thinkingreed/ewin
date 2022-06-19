use crate::{ewin_com::model::*, help::*, model::*};
use ewin_cfg::{log::*, model::modal::*};
use ewin_com::_cfg::key::cmd::CmdType;
use ewin_const::def::*;

impl EvtAct {
    pub fn ctrl_editor(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::ctrl_editor");

        let evt_act = EvtAct::exec_editor(term);
        if evt_act != ActType::Next {
            return evt_act;
        }
        return ActType::Draw(term.curt().editor.get_draw_parts());
    }

    pub fn exec_editor(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::exec_editor");
        let cmd = term.cmd.clone();
        term.curt().editor.set_cmd(cmd);
        if term.curt().editor.state.is_read_only && term.curt().editor.cmd.config.is_edit {
            return ActType::Cancel;
        }
        term.curt().editor.set_org_state();
        term.curt().editor.init();

        let e_cmd = &term.curt().editor.cmd.clone();
        Log::debug("e_cmd", &e_cmd);
        let cmd = &term.curt().editor.cmd.clone();
        Log::debug("cmd", &cmd);

        match cmd.cmd_type {
            CmdType::CancelEditorState => term.curt().editor.cancel_state(),
            _ if term.curt().editor.is_input_imple_mode(true) => {
                let act_type = term.curt().editor.ctrl_input_comple();
                if act_type != ActType::Next {
                    return act_type;
                }
            }
            CmdType::CloseFile => return term.close_file(),
            CmdType::CloseAllFile => return term.close_tabs(USIZE_UNDEFINED),
            CmdType::Resize(_, _) => term.resize(),
            CmdType::SaveFile => {
                let act_type = Tab::save(term, SaveType::Normal);
                if let ActType::Draw(_) = act_type {
                    return act_type;
                }
            }
            // file
            CmdType::CreateNewFile => term.new_tab(),

            // format
            CmdType::Format(fmt_type) => return EvtAct::evt_editor_format(term, fmt_type),
            // key record
            CmdType::RecordKeyStartEnd => return term.curt().record_key_macro_start(),
            CmdType::ExecRecordKey => return Tab::exec_key_macro(term),
            /*
             * Prompt
             */
            CmdType::FindProm | CmdType::ReplaceProm | CmdType::GrepProm | CmdType::GrepingProm | CmdType::GrepResultProm | CmdType::MoveRowProm | CmdType::openFileProm(_) | CmdType::EncodingProm => {
                return term.curt().prom_show_com(&cmd.cmd_type);
            }
            /*
             * Menu
             */
            CmdType::OpenMenuFile | CmdType::OpenMenuConvert | CmdType::OpenMenuEdit | CmdType::OpenMenuSearch | CmdType::OpenMenuMacro => {}
            // Help
            CmdType::Help => Help::disp_toggle(term),
            /*
             * ctx_menu
             */
            CmdType::CtxtMenu(y, x) => {
                // let editor_row_posi = term.curt_mut().editor.row_posi;
                term.init_ctx_menu(y, x);
            }
            // switch_tab
            CmdType::SwitchTabRight => return term.switch_tab(Direction::Right),
            CmdType::SwitchTabLeft => return term.switch_tab(Direction::Left),

            /*
             * editor
             */
            _ => {
                let evt_act = term.curt().editor.proc();
                if evt_act != ActType::Next {
                    return evt_act;
                }
            }
        }

        term.curt().editor.record_key();

        return ActType::Next;
    }

    fn evt_editor_format(term: &mut Terminal, fmt_type: FileType) -> ActType {
        if let Some(err_str) = term.curt().editor.format(fmt_type) {
            return ActType::Draw(DParts::MsgBar(err_str));
        } else {
            // highlight data reset
            term.editor_draw_vec[term.tab_idx].clear();
            return ActType::Draw(DParts::All);
        }
    }
}
