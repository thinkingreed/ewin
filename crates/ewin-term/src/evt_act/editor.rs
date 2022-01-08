use crate::{
    ctx_menu::init::*,
    ewin_com::{_cfg::key::keycmd::*, def::*, log::*, model::*},
    help::*,
    model::*,
    tab::Tab,
};

impl EvtAct {
    pub fn ctrl_editor(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::ctrl_editor");

        let act_type = term.curt().editor.editor_check_err();
        if ActType::Next != act_type {
            return act_type;
        }
        term.curt().editor.set_org_state();
        term.curt().editor.init();

        let keycmd = &term.keycmd.clone();
        match &keycmd {
            KeyCmd::CloseFile => {
                if Tab::prom_save_confirm(term) {
                    return ActType::Exit;
                }
            }
            KeyCmd::Resize => term.resize(),
            KeyCmd::Edit(e_cmd) => match &e_cmd {
                E_Cmd::CloseAllFile => {
                    if term.close_tabs(USIZE_UNDEFINED) {
                        return ActType::Exit;
                    }
                }
                E_Cmd::SaveFile => {
                    let act_type = Tab::save(term, false);
                    if let ActType::Draw(_) = act_type {
                        return act_type;
                    }
                }
                // file
                E_Cmd::NewTab => term.new_tab(),

                // format
                E_Cmd::Format(fmt_type) => return EvtAct::evt_editor_format(term, *fmt_type),
                // key record
                E_Cmd::StartEndRecordKey => return term.curt().record_key_macro_start(),
                E_Cmd::ExecRecordKey => Tab::exec_key_macro(term),
                /*
                 * Prompt
                 */
                E_Cmd::ReplacePrompt => term.curt().prom_replace(),
                E_Cmd::OpenFile(open_file_type) => term.curt().prom_open_file(*open_file_type),
                E_Cmd::Find => term.curt().prom_search(),
                E_Cmd::MoveRow => term.curt().prom_move_row(),
                E_Cmd::Grep => term.curt().prom_grep(),
                E_Cmd::OpenMenu | E_Cmd::OpenMenuFile | E_Cmd::OpenMenuConvert | E_Cmd::OpenMenuEdit | E_Cmd::OpenMenuSearch | E_Cmd::OpenMenuMacro => term.curt().prom_menu(),
                E_Cmd::Encoding => term.curt().prom_enc_nl(),
                // Test
                /*
                  E_Cmd::Encoding => {
                    let h_file = term.curt_h_file().clone();
                    term.curt().prom_save_forced(h_file);
                }
                */
                // Help
                E_Cmd::Help => Help::disp_toggle(term),
                /*
                 * ctx_menu
                 */
                // E_Cmd::MouseDownRight(_, _) | E_Cmd::MouseDragRight(_, _) => CtxMenuGroup::show_init(term),
                E_Cmd::CtxtMenu(y, x) => CtxMenuGroup::show_init(term, *y, *x),
                // switch_tab
                E_Cmd::SwitchTabRight => return term.switch_tab(Direction::Right),
                E_Cmd::SwitchTabLeft => return term.switch_tab(Direction::Left),
                //
                // Operation editor
                _ => term.curt().editor.proc(),
            },
            _ => return ActType::Cancel,
        }

        if term.curt().editor.state.key_macro.is_record {
            term.curt().editor.record_key();
            // When key_record is exec running, redraw only at the end
        } else if term.curt().editor.state.key_macro.is_running() {
            return ActType::Cancel;
        }
        term.curt().editor.finalize();
        term.curt().editor.set_draw_range();

        let dparts = term.curt().editor.set_draw_parts(keycmd);
        return ActType::Draw(dparts);
    }

    fn evt_editor_format(term: &mut Terminal, fmt_type: FmtType) -> ActType {
        if let Some(err_str) = term.curt().editor.format(fmt_type) {
            return ActType::Draw(DParts::MsgBar(err_str));
        } else {
            // highlight data reset
            term.editor_draw_vec[term.idx].clear();
            return ActType::Draw(DParts::All);
        }
    }
}
