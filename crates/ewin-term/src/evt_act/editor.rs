use crate::{
    ewin_com::{_cfg::key::keycmd::*, model::*},
    help::*,
    model::*,
};
use ewin_cfg::{log::*, model::modal::*};
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
        let keycmd = term.keycmd.clone();
        term.curt().editor.set_keycmd(keycmd);
        if term.curt().editor.state.is_read_only && term.curt().editor.cmd_config.is_edit {
            return ActType::Cancel;
        }
        term.curt().editor.set_org_state();
        term.curt().editor.init();

        let e_cmd = &term.curt().editor.e_cmd.clone();
        Log::debug("e_cmd", &e_cmd);

        match e_cmd {
            E_Cmd::CancelState => term.curt().editor.cancel_state(),
            _ if term.curt().editor.is_input_imple_mode(true) => {
                let act_type = term.curt().editor.ctrl_input_comple();
                if act_type != ActType::Next {
                    return act_type;
                }
            }
            E_Cmd::CloseFile => {
                if Tab::prom_save_confirm(term) {
                    return ActType::Exit;
                }
            }
            E_Cmd::CloseAllFile => {
                if term.close_tabs(USIZE_UNDEFINED) {
                    return ActType::Exit;
                }
            }
            E_Cmd::Resize(_, _) => term.resize(),
            E_Cmd::SaveFile => {
                let act_type = Tab::save(term, SaveType::Normal);
                if let ActType::Draw(_) = act_type {
                    return act_type;
                }
            }
            // file
            E_Cmd::CreateNewFile => term.new_tab(),

            // format
            E_Cmd::Format(fmt_type) => return EvtAct::evt_editor_format(term, *fmt_type),
            // key record
            E_Cmd::StartEndRecordKey => return term.curt().record_key_macro_start(),
            E_Cmd::ExecRecordKey => return Tab::exec_key_macro(term),
            /*
             * Prompt
             */
            E_Cmd::ReplacePrompt => return term.curt().prom_replace(),
            E_Cmd::OpenFile(open_file_type) => return term.curt().prom_open_file(*open_file_type),
            E_Cmd::Find => return term.curt().prom_search(),
            E_Cmd::MoveRow => return term.curt().prom_move_row(),
            E_Cmd::Grep => return term.curt().prom_grep(),
            E_Cmd::OpenMenuFile | E_Cmd::OpenMenuConvert | E_Cmd::OpenMenuEdit | E_Cmd::OpenMenuSearch | E_Cmd::OpenMenuMacro => {}
            E_Cmd::Encoding => {
                let h_file = &term.curt_h_file().clone();
                return term.curt().prom_enc_nl(h_file);
            }
            // Help
            E_Cmd::Help => Help::disp_toggle(term),

            /*
             * ctx_menu
             */
            E_Cmd::CtxtMenu(y, x) => {
                // let editor_row_posi = term.curt_mut().editor.row_posi;
                term.init_ctx_menu(*y, *x);
            } // switch_tab
            E_Cmd::SwitchTabRight => return term.switch_tab(Direction::Right),
            E_Cmd::SwitchTabLeft => return term.switch_tab(Direction::Left),

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
