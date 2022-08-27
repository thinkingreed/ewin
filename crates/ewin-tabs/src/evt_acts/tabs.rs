use crate::{tab::Tab, tabs::*};
use ewin_cfg::{lang::lang_cfg::Lang, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, evt::*, file::*},
};
use ewin_help::help::*;
use ewin_job::job::*;
use ewin_key::{global::*, key::cmd::*, model::GrepCancelType};
use ewin_prom::each::grep::Grep;
use ewin_state::term::*;
use ewin_utils::files::file::*;
use ewin_view::view::*;

impl Tabs {
    pub fn ctrl_tabs(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("EvtAct::ctrl_tabs");

        match *cmd_type {
            CmdType::CloseFileCurt(close_type) => return self.close_file(self.idx, close_type),
            CmdType::CloseFileTgt(idx) => return self.close_file(idx, CloseFileType::Normal),
            CmdType::CloseAllFile => return self.close_other_than_this_tab(USIZE_UNDEFINED),
            CmdType::SaveFile(ref save_type) => return self.curt().save(save_type),
            CmdType::OpenNewFile => return self.new_tab(),
            CmdType::ChangeFile(idx) => return self.change_file(idx),
            CmdType::SwapFile(org, dst) => return self.swap_file(org, dst),
            CmdType::CloseOtherThanThisTab(idx) => return self.close_other_than_this_tab(idx),
            CmdType::SaveAllFinish => {
                let act_type = self.save_all_tab();
                Log::debug("act_typeact_typeact_typeact_typeact_type", &act_type);
                if let ActType::Draw(_) = act_type {
                    return act_type;
                } else {
                    return ActType::Exit;
                }
            }

            // Prompt
            CmdType::FindProm | CmdType::ReplaceProm | CmdType::GrepProm | CmdType::GrepResultProm | CmdType::MoveRowProm | CmdType::openFileProm(_) | CmdType::EncodingProm => {
                return self.curt().prom_show_com(cmd_type);
            }

            CmdType::GrepingProm(ref grep_info) => {
                Log::debug("grep_info", &grep_info);
                self.add_tab(&mut Tab::new(), File { name: format!(r#"{} "{}""#, &Lang::get().grep, &grep_info.search_str), ..File::default() }, FileOpenType::Nomal);
                GREP_CANCEL_VEC.get().unwrap().try_lock().unwrap().push(GrepCancelType::Greping);
                State::get().curt_mut_state().grep = grep_info.clone();
                Grep::exe_grep(grep_info.clone());
                return self.curt().prom_show_com(cmd_type);
            }
            /*
             * Menu
             */
            CmdType::OpenMenuFile | CmdType::OpenMenuConvert | CmdType::OpenMenuEdit | CmdType::OpenMenuSearch | CmdType::OpenMenuMacro => {}
            // Help
            CmdType::Help => {
                Help::get().toggle_show();
                self.set_size();
                if Help::get().is_show {
                    // Cursor moves out of help display area

                    self.curt().editor.adjust_cur_posi();
                    /*
                    if term.tabs.curt().editor.win_mgr.curt().cur.y - tab.editor.win_mgr.curt().offset.y > tab.editor.get_curt_row_len() - 1 {
                        term.tabs.curt()..editor.win_mgr.curt().cur.y = tab.editor.win_mgr.curt().offset.y + tab.editor.get_curt_row_len() - 1;
                        term.tabs.curt()..editor.win_mgr.curt().cur.x = 0;
                        term.tabs.curt()..editor.win_mgr.curt().cur.disp_x = 0;
                    }
                     */
                }
                return ActType::Draw(DParts::All);
            }
            // switch_tab
            CmdType::SwitchFile(direction) => return self.switch_file(direction),
            /*
             * Tab
             */
            // Clear
            CmdType::ClearTabState(is_clear_editor_state) => return self.curt().clear_curt_tab(is_clear_editor_state),

            _ => return ActType::None,
        };
        return ActType::None;
    }

    pub fn check_exit_close(&mut self) -> ActType {
        Log::debug_key("EvtAct::check_exit_close");

        if self.vec.len() == 1 {
            return ActType::Exit;
        } else {
            self.del_tab(self.idx);

            if self.state.is_all_close_confirm {
                // TODO TEST
                // TODO TEST
                // TODO TEST
                Job::send_cmd(CmdType::CloseOtherThanThisTab(USIZE_UNDEFINED));
                return ActType::None;
            } else if State::get().tabs.all.close_other_than_this_tab_idx != USIZE_UNDEFINED {
                Job::send_cmd(CmdType::CloseOtherThanThisTab(State::get().tabs.all.close_other_than_this_tab_idx));
                return ActType::None;
            } else {
                return ActType::Draw(DParts::All);
            }
        }
    }
}
