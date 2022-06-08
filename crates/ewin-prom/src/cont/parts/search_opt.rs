use crate::{model::*, prom_trait::cont_trait::*};
use ewin_cfg::{colors::*, global::*, lang::lang_cfg::*, model::default::*};
use ewin_com::{_cfg::key::keycmd::*, model::*, util::*};

impl PromContSearchOpt {
    pub fn proc_search_opt(&mut self) -> ActType {
        match self.base.p_cmd {
            P_Cmd::FindCaseSensitive => {
                self.change_opt_case_sens();
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::FindRegex => {
                self.change_opt_regex();
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::MouseDownLeft(y, x) => {
                if self.base.row_posi_range.start == y {
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
        let case_sens = PromptContOpt::get_opt(P_Cmd::FindCaseSensitive, Lang::get().case_sens.to_string(), cfg_search.case_sensitive, 0);
        let regex = PromptContOpt::get_opt(P_Cmd::FindRegex, Lang::get().regex.to_string(), cfg_search.regex, case_sens.mouse_area.start);
        return PromContSearchOpt { case_sens, regex, ..PromContSearchOpt::default() };
    }
}

impl PromPluginBase {
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
    pub fn get_opt(p_cmd: P_Cmd, key_desc_str: String, is_check: bool, sx: usize) -> PromptContOpt {
        let key_str = Keybind::get_key_str(KeyCmd::Prom(p_cmd));
        let x = get_str_width(&format!("{}:{}", &key_str, key_str));

        let opt = PromptContOpt { key_str, key_desc_str, is_check, mouse_area: (sx..sx + x + 2) };
        return opt;
    }
}

impl PromContPluginTrait for PromContSearchOpt {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, _: bool) {
        let mut s = "".to_string();
        s.push_str(&format!("{}{}:{}{}{}  ", Colors::get_default_fg(), &self.case_sens.key_desc_str, Colors::get_msg_warning_fg(), &self.case_sens.key_str, self.case_sens.get_check_str()));
        s.push_str(&format!("{}{}:{}{}{}  ", Colors::get_default_fg(), &self.regex.key_desc_str, Colors::get_msg_warning_fg(), &self.regex.key_str, self.regex.get_check_str()));
        str_vec.push(s);
        str_vec.push(Colors::get_default_fg());
    }

    fn check_allow_p_cmd(&self) -> bool {
        return matches!(self.as_base().p_cmd, P_Cmd::FindCaseSensitive | P_Cmd::FindRegex | P_Cmd::MouseDownLeft(_, _));
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromContSearchOpt {
    pub base: PromptContBase,
    pub case_sens: PromptContOpt,
    pub regex: PromptContOpt,
}
