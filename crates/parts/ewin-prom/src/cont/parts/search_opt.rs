use crate::{model::*, prom_trait::cont_trait::*};
use ewin_cfg::{colors::*, global::*, lang::lang_cfg::*, log::*, model::default::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_key::key::{cmd::*, keys::*};
use ewin_utils::str_edit::*;

use std::fmt::Write as _;

impl PromContSearchOpt {
    const MARGIN: usize = 2;

    pub fn proc_search_opt(&mut self) -> ActType {
        match self.base.cmd.cmd_type {
            CmdType::FindCaseSensitive => {
                self.change_opt_case_sens();
                return ActType::Draw(DParts::Prompt);
            }
            CmdType::FindRegex => {
                self.change_opt_regex();
                return ActType::Draw(DParts::Prompt);
            }
            CmdType::MouseDownLeft(y, x) => {
                Log::debug_s("PromContSearchOpt.MouseDownLeft");
                Log::debug("yyy", &y);
                Log::debug("self.base.row_posi_range.start", &self.base.row_posi_range.start);
                if self.base.row_posi_range.start == y {
                    Log::debug("xxx", &x);
                    Log::debug("self.case_sens.mouse_area", &self.case_sens.mouse_area);
                    if self.case_sens.mouse_area.contains(&x) {
                        self.change_opt_case_sens();
                        return ActType::Draw(DParts::Prompt);
                    } else if self.regex.mouse_area.contains(&x) {
                        self.change_opt_regex();
                        return ActType::Draw(DParts::Prompt);
                    }
                }
            }
            _ => {}
        }
        return ActType::Next;
    }

    pub fn change_opt_case_sens(&mut self) {
        self.case_sens.toggle_check();
        CFG_EDIT.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.case_sensitive = self.case_sens.is_check).unwrap();
    }

    pub fn change_opt_regex(&mut self) {
        self.regex.toggle_check();
        CFG_EDIT.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.search.regex = self.regex.is_check).unwrap();
    }

    pub fn get_searh_opt(cfg_search: &CfgSearch) -> PromContSearchOpt {
        let case_sens = PromptContOpt::get_opt(CmdType::FindCaseSensitive, Lang::get().case_sens.to_string(), cfg_search.case_sensitive, 0);
        let regex = PromptContOpt::get_opt(CmdType::FindRegex, Lang::get().regex.to_string(), cfg_search.regex, case_sens.mouse_area.end + PromContSearchOpt::MARGIN);
        return PromContSearchOpt { case_sens, regex, ..PromContSearchOpt::default() };
    }
}

impl PromBase {
    pub fn get_search_opt(&mut self) -> Option<&mut PromContSearchOpt> {
        for item in self.cont_vec.iter_mut() {
            if let Ok(search_opt) = item.downcast_mut::<PromContSearchOpt>() {
                return Some(search_opt);
            }
        }
        return None;
    }
}

impl PromptContOpt {
    pub fn get_check_str(&self) -> String {
        let str = if self.is_check { "[*]" } else { "[ ]" };
        str.to_string()
    }
    pub fn toggle_check(&mut self) {
        match self.is_check {
            true => self.is_check = false,
            false => self.is_check = true,
        }
    }
    pub fn get_opt(cmd_type: CmdType, key_desc_str: String, is_check: bool, sx: usize) -> PromptContOpt {
        let key_str = Keys::get_key_str(cmd_type);
        let x = get_str_width(&format!("{}:{}", &key_desc_str, key_str));

        let opt = PromptContOpt { key_str, key_desc_str, is_check, mouse_area: (sx + x..sx + x + 3) };
        return opt;
    }
}

impl PromContTrait for PromContSearchOpt {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, _: bool) {
        let mut s = "".to_string();
        let _ = write!(s, "{}{}:{}{}{}  ", Colors::get_default_fg(), &self.case_sens.key_desc_str, Colors::get_msg_warning_fg(), &self.case_sens.key_str, self.case_sens.get_check_str());
        let _ = write!(s, "{}{}:{}{}{}  ", Colors::get_default_fg(), &self.regex.key_desc_str, Colors::get_msg_warning_fg(), &self.regex.key_str, self.regex.get_check_str());
        str_vec.push(s);
        str_vec.push(Colors::get_default_fg());
    }

    fn check_allow_p_cmd(&self) -> bool {
        return matches!(self.as_base().cmd.cmd_type, CmdType::FindCaseSensitive | CmdType::FindRegex | CmdType::MouseDownLeft(_, _));
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromContSearchOpt {
    pub base: PromptContBase,
    pub case_sens: PromptContOpt,
    pub regex: PromptContOpt,
}
