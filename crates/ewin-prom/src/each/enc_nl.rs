use crate::{
    cont::parts::{choice::*, info::*, key_desc::*},
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_com::{_cfg::key::keycmd::*, model::*, util::*};
use ewin_const::def::*;

impl PromPluginEncNl {
    pub fn new() -> Self {
        let mut plugin = PromPluginEncNl { base: PromPluginBase { config: PromptPluginConfig { is_updown_valid: true, ..PromptPluginConfig::default() }, ..PromPluginBase::default() } };
        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_enc_nl.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let fixed = PromContKeyMenu { disp_str: Lang::get().fixed.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Confirm) };

        let switch_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![P_Cmd::NextContent, P_Cmd::CursorUp, P_Cmd::CursorDown, P_Cmd::CursorLeft, P_Cmd::CursorRight], &mut vec![P_Cmd::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Cancel) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![fixed, switch_area, cancel]], ..PromContKeyDesc::default() }));

        let mut cont_choice = PromContChoice { is_disp: true, desc_str_vec: vec![Lang::get().method_of_apply.to_string()], vec: vec![vec![Choice::new(&Lang::get().file_reload), Choice::new(&Lang::get().keep_and_apply_string)]], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        plugin.base.cont_vec.push(Box::new(cont_choice));
        plugin.base.curt_cont_idx = plugin.base.cont_vec.len() - 1;

        let utf_vec = vec![Choice::new(&Encode::UTF8.to_string()), Choice::new(&Encode::UTF16LE.to_string()), Choice::new(&Encode::UTF16BE.to_string())];
        let local_vec = vec![Choice::new(&Encode::SJIS.to_string()), Choice::new(&Encode::JIS.to_string()), Choice::new(&Encode::EucJp.to_string()), Choice::new(&Encode::GBK.to_string())];

        let mut cont_choice = PromContChoice { is_disp: true, config: PromContChoiceConfig { is_multi_row: true }, desc_str_vec: vec![Lang::get().encoding.to_string()], vec: vec![utf_vec, local_vec], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        plugin.base.cont_vec.push(Box::new(cont_choice));

        let nl_vec = vec![Choice::new(NEW_LINE_LF_STR), Choice::new(NEW_LINE_CRLF_STR)];

        let mut cont_choice = PromContChoice { desc_str_vec: vec![Lang::get().new_line_code.to_string()], vec: vec![nl_vec], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        plugin.base.cont_vec.push(Box::new(cont_choice));

        let bom_vec = vec![Choice::new(&Lang::get().with), Choice::new(&Lang::get().without)];
        let mut cont_choice = PromContChoice { desc_str_vec: vec![format!("{}{}", "BOM".to_string(), Lang::get().presence_or_absence.to_string())], vec: vec![bom_vec], vec_y: 0, vec_x: 0, ..PromContChoice::default() };
        cont_choice.set_shaping_choice_list();
        plugin.base.cont_vec.push(Box::new(cont_choice));

        return plugin;
    }
    pub fn set_default_choice_enc_nl(&mut self, h_file: &HeaderFile) {
        for prom_cont in self.base.cont_vec.iter_mut() {
            if let Ok(prom) = prom_cont.downcast_mut::<PromContChoice>() {
                // let prom = prom_cont.downcast_mut::<PromContChoice>().unwrap();

                for (y_idx, v) in prom.vec.iter_mut().enumerate() {
                    let mut row_width = 1;

                    for (x_idx, choice) in v.iter_mut().enumerate() {
                        if prom.desc_str_vec[0] == Lang::get().encoding && h_file.enc.to_string() == choice.name {
                            prom.vec_y = y_idx;
                            prom.vec_x = x_idx;
                        }
                        if prom.desc_str_vec[0] == Lang::get().new_line_code && h_file.nl.to_string() == choice.name {
                            prom.vec_y = y_idx;
                            prom.vec_x = x_idx;
                        }
                        if prom.desc_str_vec[0] == format!("{}{}", "BOM".to_string(), Lang::get().presence_or_absence.to_string()) {
                            if None == h_file.bom {
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
                if !(choice.desc_str_vec[0] == format!("{}{}", "BOM".to_string(), Lang::get().presence_or_absence.to_string()) && encoding_choice.disp_name != Encode::UTF8.to_string()) {
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
}

#[derive(Default, Debug, Clone)]
pub struct PromPluginEncNl {
    pub base: PromPluginBase,
}
impl PromPluginTrait for PromPluginEncNl {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}
