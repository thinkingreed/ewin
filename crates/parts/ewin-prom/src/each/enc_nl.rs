use crate::{
    cont::parts::{choice::*, info::*, key_desc::*},
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
};
use ewin_job::job::*;
use ewin_key::{key::cmd::*, model::*};
use ewin_state::term::*;
use ewin_utils::{
    files::{bom::*, encode::*, file::*},
    str_edit::*,
};

impl PromEncNl {
    pub fn enc_nl(&mut self) -> ActType {
        Log::debug_key("EvtAct::enc_nl");

        match self.base.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                self.click_choice(y as u16, x as u16);
                return ActType::Draw(DrawParts::Prompt);
            }
            CmdType::CursorLeft | CmdType::CursorRight => {
                let cmd = self.base.cmd.clone();
                let choice = self.as_mut_base().get_curt_cont_mut().unwrap().downcast_mut::<PromContChoice>().unwrap();
                choice.move_left_right(&cmd.cmd_type);
                self.ctrl_parts();
                return ActType::Draw(DrawParts::Prompt);
            }
            CmdType::CursorUp | CmdType::CursorDown => {
                let choice = self.as_mut_base().get_curt_cont_mut().unwrap().downcast_mut::<PromContChoice>().unwrap();
                if choice.set_cont_posi_or_is_up_down_cont_posi() {
                    self.as_mut_base().set_next_back_cont_idx();
                }
                self.ctrl_parts();

                return ActType::Draw(DrawParts::Prompt);
            }
            CmdType::Confirm => {
                Log::debug_s("EvtAct::enc_nl::P_Cmd::Confirm");
                let method_of_apply = self.as_mut_base().get_tgt_cont(2).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let encode = self.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let nl = self.as_mut_base().get_tgt_cont(4).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();
                let bom = self.as_mut_base().get_tgt_cont(5).unwrap().downcast_mut::<PromContChoice>().unwrap().get_choice();

                // let file_org = State::get().curt_state().file.clone();
                let encode = Encode::from_name(&encode.name);

                if method_of_apply.name == Lang::get().file_reload {
                    // TODO What to do if the file is being edited

                    let result = File::read_file(&State::get().curt_state().file.name);
                    match result {
                        Ok((vec, _, _, _)) => {
                            let (_, _, had_errors) = File::read_bytes(&vec, encode);
                            if had_errors {
                                return ActType::Draw(DrawParts::MsgBar(Lang::get().cannot_convert_encoding.to_string()));
                            } else {
                                Job::send_cmd(CmdType::ReOpenFile(FileOpenType::ReopenEncode(encode)));
                                return ActType::None;
                            }
                        }
                        Err(err) => return ActType::Draw(DrawParts::MsgBar(File::get_io_err_str(err))),
                    }
                } else {
                    State::get().curt_mut_state().file.enc = encode;
                    State::get().curt_mut_state().file.bom = Bom::get_select_item_bom(&encode, &bom.name);
                    State::get().curt_mut_state().file.nl = nl.name;

                    State::get().curt_mut_state().clear();
                    return ActType::Draw(DrawParts::TabsAll);
                }
            }
            _ => return ActType::Cancel,
        }
    }
    pub fn new() -> Self {
        let mut prom = PromEncNl { base: PromBase { cfg: PromptConfig { is_updown_valid: true }, ..PromBase::default() } };
        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_enc_nl.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let fixed = PromContKeyMenu { disp_str: Lang::get().fixed.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };

        let switch_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown, CmdType::CursorLeft, CmdType::CursorRight], &mut vec![CmdType::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![fixed, switch_area, cancel]], ..PromContKeyDesc::default() }));

        let mut cont_choice = PromContChoice { is_disp: true, desc_str_vec: vec![Lang::get().method_of_apply.to_string()], vec: vec![vec![Choice::new(&Lang::get().file_reload), Choice::new(&Lang::get().keep_and_apply_string)]], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        prom.base.cont_vec.push(Box::new(cont_choice));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        let utf_vec = vec![Choice::new(&Encode::UTF8.to_string()), Choice::new(&Encode::UTF16LE.to_string()), Choice::new(&Encode::UTF16BE.to_string())];
        let local_vec = vec![Choice::new(&Encode::SJIS.to_string()), Choice::new(&Encode::JIS.to_string()), Choice::new(&Encode::EucJp.to_string()), Choice::new(&Encode::GBK.to_string())];

        let mut cont_choice = PromContChoice { is_disp: true, config: PromContChoiceConfig { is_multi_row: true }, desc_str_vec: vec![Lang::get().encoding.to_string()], vec: vec![utf_vec, local_vec], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        prom.base.cont_vec.push(Box::new(cont_choice));

        let nl_vec = vec![Choice::new(NEW_LINE_LF_STR), Choice::new(NEW_LINE_CRLF_STR)];

        let mut cont_choice = PromContChoice { desc_str_vec: vec![Lang::get().new_line_code.to_string()], vec: vec![nl_vec], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        prom.base.cont_vec.push(Box::new(cont_choice));

        let bom_vec = vec![Choice::new(&Lang::get().with), Choice::new(&Lang::get().without)];
        let mut cont_choice = PromContChoice { desc_str_vec: vec![format!("{}{}", "BOM", Lang::get().presence_or_absence)], vec: vec![bom_vec], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        prom.base.cont_vec.push(Box::new(cont_choice));

        return prom;
    }
    pub fn set_default_choice_enc_nl(&mut self, file: &File) {
        for prom_cont in self.base.cont_vec.iter_mut() {
            if let Ok(prom) = prom_cont.downcast_mut::<PromContChoice>() {
                for (y_idx, v) in prom.vec.iter_mut().enumerate() {
                    let mut row_width = 1;

                    for (x_idx, choice) in v.iter_mut().enumerate() {
                        if prom.desc_str_vec[0] == Lang::get().encoding && file.enc.to_string() == choice.name {
                            prom.vec_y = y_idx;
                            prom.vec_x = x_idx;
                        }
                        if prom.desc_str_vec[0] == Lang::get().new_line_code && file.nl == choice.name {
                            prom.vec_y = y_idx;
                            prom.vec_x = x_idx;
                        }
                        if prom.desc_str_vec[0] == format!("{}{}", "BOM", Lang::get().presence_or_absence) {
                            if None == file.bom {
                                if choice.name == Lang::get().without {
                                    prom.vec_y = y_idx;
                                    prom.vec_x = x_idx;
                                }
                            } else if choice.name == Lang::get().with {
                                prom.vec_y = y_idx;
                                prom.vec_x = x_idx;
                            }
                        }
                        let item_len = get_str_width(&choice.name);
                        choice.area = (prom.base.row_posi_range.start + prom.desc_str_vec.len() + y_idx, row_width, row_width + item_len - 1);
                        row_width += item_len + PromContChoice::ITEM_MARGIN;
                    }
                }
            }
        }
    }

    pub fn ctrl_parts(&mut self) {
        self.ctrl_method_of_apply();
        self.ctrl_bom();
    }

    pub fn ctrl_method_of_apply(&mut self) {
        Log::debug_key("PromPluginEncNl.ctrl_bom");

        let method_of_apply_choice = &self.as_mut_base().cont_vec[2].clone().downcast::<PromContChoice>().unwrap().get_choice();

        if method_of_apply_choice.name == Lang::get().file_reload {
            // nl_cont
            self.as_mut_base().cont_vec[4].downcast_mut::<PromContChoice>().unwrap().is_disp = false;
            // bom_cont
            self.as_mut_base().cont_vec[5].downcast_mut::<PromContChoice>().unwrap().is_disp = false;
        } else {
            self.as_mut_base().cont_vec[4].downcast_mut::<PromContChoice>().unwrap().is_disp = true;
            self.as_mut_base().cont_vec[5].downcast_mut::<PromContChoice>().unwrap().is_disp = true;
        }
    }

    pub fn ctrl_bom(&mut self) {
        Log::debug_key("PromPluginEncNl.ctrl_bom");

        let encoding_choice = &self.as_mut_base().cont_vec[3].clone().downcast::<PromContChoice>().unwrap().get_choice();
        let bom_cont = self.as_mut_base().cont_vec[5].downcast_mut::<PromContChoice>().unwrap();

        if encoding_choice.name == Encode::UTF16LE.to_string() || encoding_choice.name == Encode::UTF16BE.to_string() {
            bom_cont.set_bom(true);
        } else if encoding_choice.name == Encode::UTF8.to_string() {
            // Do nothing for UTF8
        } else {
            bom_cont.set_bom(false);
        }
    }

    pub fn click_choice(&mut self, y: u16, x: u16) {
        Log::debug_key("PromPluginEncNl.click_choice");
        let encoding_choice = &self.as_base().cont_vec[2].clone().downcast::<PromContChoice>().unwrap().get_choice();
        let mut idx = self.as_mut_base().curt_cont_idx;
        let mut is_enable_click = false;
        for (i, cont) in self.as_mut_base().cont_vec.iter_mut().enumerate() {
            if let Ok(choice) = cont.downcast_mut::<PromContChoice>() {
                if !(choice.desc_str_vec[0] == format!("{}{}", "BOM", Lang::get().presence_or_absence) && encoding_choice.disp_name != Encode::UTF8.to_string()) {
                    is_enable_click = choice.click_choice(y, x);
                    if is_enable_click {
                        idx = i;
                        break;
                    }
                }
            }
        }
        if is_enable_click {
            self.ctrl_bom();
            self.as_mut_base().curt_cont_idx = idx;
        }
    }

    pub fn init() -> ActType {
        Log::debug_key("Tab::prom_enc_nl");
        State::get().curt_mut_state().prom = PromState::EncNl;
        Prom::get().init(Box::new(PromEncNl::new()));
        Prom::get().curt.downcast_mut::<PromEncNl>().unwrap().set_default_choice_enc_nl(&State::get().curt_state().file);
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromEncNl {
    pub base: PromBase,
}
impl PromTrait for PromEncNl {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
