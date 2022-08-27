use crate::tabs::Tabs;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_key::key::cmd::*;
use ewin_prom::{cont::parts::choice::*, each::enc_nl::*};
use ewin_state::term::*;
use ewin_utils::files::{bom::*, encode::*};
use std::io::*;

impl Tabs {
    pub fn enc_nl(&mut self) -> ActType {
        Log::debug_key("EvtAct::enc_nl");

        match self.curt().prom.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                self.curt().prom.curt.downcast_mut::<PromEncNl>().unwrap().click_choice(y as u16, x as u16);
                return ActType::Draw(DParts::Prompt);
            }
            CmdType::CursorLeft | CmdType::CursorRight => {
                let cmd = self.curt().prom.cmd.clone();
                let choice = self.curt().prom.curt.as_mut_base().get_curt_cont_mut().unwrap().downcast_mut::<PromContChoice>().unwrap();
                choice.move_left_right(&cmd.cmd_type);
                self.curt().prom.curt.downcast_mut::<PromEncNl>().unwrap().ctrl_parts();
                return ActType::Draw(DParts::Prompt);
            }
            CmdType::CursorUp | CmdType::CursorDown => {
                let choice = self.curt().prom.curt.as_mut_base().get_curt_cont_mut().unwrap().downcast_mut::<PromContChoice>().unwrap();
                if choice.set_cont_posi_or_is_up_down_cont_posi() {
                    self.curt().prom.curt.as_mut_base().set_next_back_cont_idx();
                }
                self.curt().prom.curt.downcast_mut::<PromEncNl>().unwrap().ctrl_parts();

                return ActType::Draw(DParts::Prompt);
            }
            CmdType::Confirm => {
                Log::debug_s("EvtAct::enc_nl::P_Cmd::Confirm");
                let method_of_apply = self.curt().prom.curt.as_mut_base().get_tgt_cont(2).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let encoding = self.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let nl = self.curt().prom.curt.as_mut_base().get_tgt_cont(4).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let bom = self.curt().prom.curt.as_mut_base().get_tgt_cont(5).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();

                let file_org = State::get().curt_state().file.clone();

                if method_of_apply.name == Lang::get().file_reload {
                    let result = self.vec[self.idx].editor.reload_with_specify_encoding(&mut State::get().curt_mut_state().file, &encoding.name);

                    match result {
                        Ok(had_errors) => {
                            if had_errors {
                                return ActType::Draw(DParts::MsgBar(Lang::get().reading_cannot_convert_encoding.to_string()));
                            }
                            let file = State::get().curt_state().file.clone();
                            if file.nl != file_org.nl {
                                self.curt().editor.change_nl(&file_org.nl, &file.nl);
                            }
                            if file.enc != file_org.enc || file.nl != file_org.nl {
                                // self.curt().editor.state.is_changed = true;
                                State::get().curt_mut_state().editor.is_changed = true;
                            }
                        }
                        Err(err) => {
                            let err_str = match err.kind() {
                                ErrorKind::PermissionDenied => &Lang::get().no_read_permission,
                                ErrorKind::NotFound => &Lang::get().file_not_found,
                                _ => &Lang::get().file_opening_problem,
                            };
                            return ActType::Draw(DParts::MsgBar(err_str.to_string()));
                        }
                    }
                } else {
                    let encode = Encode::from_name(&encoding.name);
                    State::get().curt_mut_state().file.enc = encode;
                    State::get().curt_mut_state().file.bom = Bom::get_select_item_bom(&encode, &bom.name);
                    State::get().curt_mut_state().file.nl = nl.name;
                }
                self.curt().clear_curt_tab(true);
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
