use crate::{help::*, model::*, tab::Tab, terms::term::*};
use ewin_cfg::{log::*, model::modal::*};
use ewin_const::{def::*, model::*};
use ewin_dialog::{cont::cont::*, dialog::*};
use ewin_key::key::cmd::*;

impl EvtAct {
    pub fn ctrl_editor_cmd_type(cmd_type: CmdType, term: &mut Term) -> ActType {
        Log::debug_key("EvtAct::call_editor");

        term.cmd = Cmd::to_cmd(cmd_type);
        return EvtAct::ctrl_editor(term);
    }

    pub fn ctrl_editor(term: &mut Term) -> ActType {
        Log::debug_key("EvtAct::ctrl_editor");

        let evt_act = EvtAct::exec_editor(term);
        if
        // term.curt().editor.win_mgr.curt().draw_range == E_DrawRange::Init ||
        term.curt().editor.cmd.config.is_recalc_scrl {
            term.curt().editor.calc_editor_scrlbar_h();
            term.curt().editor.calc_editor_scrlbar_v();
        }

        if evt_act != ActType::Next {
            return evt_act;
        }

        return ActType::Draw(term.curt().editor.get_draw_parts());
    }

    pub fn exec_editor(term: &mut Term) -> ActType {
        Log::debug_key("EvtAct::exec_editor");
        let cmd = term.cmd.clone();
        term.curt().editor.set_cmd(cmd);
        if term.curt().editor.state.is_read_only && term.curt().editor.cmd.config.is_edit {
            return ActType::Cancel;
        }
        term.curt().editor.set_org_state();
        term.curt().editor.init();
        term.curt().editor.set_tgt_window();

        let cmd = &term.curt().editor.cmd.clone();
        Log::debug("cmd", &cmd);

        match cmd.cmd_type {
            CmdType::CancelEditorState => term.curt().editor.cancel_state(),

            CmdType::CloseFile => return term.close_file(),
            CmdType::CloseAllFile => return term.close_tabs(USIZE_UNDEFINED),
            CmdType::SaveFile => {
                let act_type = term.curt().save(SaveType::Normal);
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
            CmdType::Help => return Help::disp_toggle(term),
            // ctx_menu
            CmdType::CtxtMenu(y, x) => term.init_ctx_menu(y, x),
            // switch_tab
            CmdType::SwitchTab(direction) => return term.switch_tab(direction),
            // WindowSplit
            CmdType::WindowSplit(split_type) => {
                term.curt().editor.win_mgr.split_window(split_type);
                term.curt().resize_editor_draw_vec();
                term.set_size();
            }
            CmdType::Test => Dialog::init(DialogContType::AboutApp),
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

    fn evt_editor_format(term: &mut Term, fmt_type: FileType) -> ActType {
        if let Some(err_str) = term.curt().editor.format(fmt_type) {
            return ActType::Draw(DParts::MsgBar(err_str));
        } else {
            // highlight data reset
            let tab_idx = term.tab_idx;
            term.curt().editor_draw_vec[tab_idx].clear();
            return ActType::Draw(DParts::All);
        }
    }
}
