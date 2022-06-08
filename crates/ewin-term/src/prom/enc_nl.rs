use crate::{
    ewin_com::{_cfg::key::keycmd::*, model::*},
    model::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::files::bom::*;
use ewin_prom::{cont::parts::choice::*, each::enc_nl::*};
use std::io::*;

impl EvtAct {
    pub fn enc_nl(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::enc_nl");

        match term.curt().prom.p_cmd {
            P_Cmd::MouseDownLeft(y, x) => {
                term.curt().prom.curt.downcast_mut::<PromPluginEncNl>().unwrap().click_choice(y as u16, x as u16);
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::CursorLeft | P_Cmd::CursorRight => {
                let p_cmd = term.curt().prom.p_cmd.clone();
                let choice = term.curt().prom.curt.as_mut_base().get_curt_cont_mut().unwrap().downcast_mut::<PromContChoice>().unwrap();
                choice.move_left_right(&p_cmd);
                term.curt().prom.curt.downcast_mut::<PromPluginEncNl>().unwrap().ctrl_parts();
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::CursorUp | P_Cmd::CursorDown => {
                let choice = term.curt().prom.curt.as_mut_base().get_curt_cont_mut().unwrap().downcast_mut::<PromContChoice>().unwrap();
                if choice.set_cont_posi_or_is_up_down_cont_posi() {
                    term.curt().prom.curt.as_mut_base().set_next_back_cont_idx();
                }
                term.curt().prom.curt.downcast_mut::<PromPluginEncNl>().unwrap().ctrl_parts();

                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::Confirm => {
                Log::debug_s("EvtAct::enc_nl::P_Cmd::Confirm");
                let method_of_apply = term.curt().prom.curt.as_mut_base().get_tgt_cont(2).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let encoding = term.curt().prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let nl = term.curt().prom.curt.as_mut_base().get_tgt_cont(4).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let bom = term.curt().prom.curt.as_mut_base().get_tgt_cont(5).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();

                let h_file_org = term.fbar.file_vec[term.tab_idx].clone();

                if method_of_apply.name == Lang::get().file_reload {
                    let result = term.tabs[term.tab_idx].editor.reload_with_specify_encoding(&mut term.fbar.file_vec[term.tab_idx], &encoding.name);

                    match result {
                        Ok(had_errors) => {
                            if had_errors {
                                return ActType::Draw(DParts::MsgBar(Lang::get().reading_cannot_convert_encoding.to_string()));
                            }
                            let h_file = term.curt_h_file().clone();
                            if term.curt_h_file().nl != h_file_org.nl {
                                term.curt().editor.change_nl(&h_file_org.nl, &h_file.nl);
                            }
                            if h_file_org.enc != term.curt_h_file().enc || h_file_org.nl != term.curt_h_file().nl {
                                term.curt().editor.state.is_changed = true;
                            }
                            term.curt().editor.h_file = term.curt_h_file().clone();
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
                    term.curt_h_file().enc = encode;
                    term.curt_h_file().bom = Bom::get_select_item_bom(&encode, &bom.name);
                    term.curt_h_file().nl = nl.name.to_string();
                    Log::debug("term.curt_h_file()", &term.curt_h_file());
                }
                term.curt().clear_curt_tab(true);
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
