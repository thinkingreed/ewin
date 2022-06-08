use crate::{model::*, prom_trait::cont_trait::*};
use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*};
use ewin_com::_cfg::key::keycmd::*;

impl PromContPluginTrait for PromContKeyDesc {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, _: bool) {
        for vec in &self.desc_vecs {
            let mut s = "".to_string();
            for cont in vec {
                match &cont.key {
                    PromContKeyMenuType::PCmd(p_cmd) => {
                        s.push_str(&format!("{}{}:{}{} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), PromContKeyMenu::get_str(p_cmd)));
                    }
                    PromContKeyMenuType::ECmd(e_cmd) => {
                        s.push_str(&format!("{}{}:{}{} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::Edit(e_cmd.clone()))));
                    }
                    PromContKeyMenuType::PCmdAndStr(p_cmd, string) => {
                        s.push_str(&format!("{}{}:{}{} {} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), PromContKeyMenu::get_str(p_cmd), string));
                    }
                    PromContKeyMenuType::PCmds { show_cmd: show_cmd_vec, all_cmd: _ } => {
                        s.push_str(&format!("{}{}:{}", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(),));

                        for p_cmd in show_cmd_vec {
                            s.push_str(&format!(" {}", PromContKeyMenu::get_str(p_cmd)));
                        }
                        s.push_str(" ");
                    }
                    PromContKeyMenuType::OneChar(c) => {
                        s.push_str(&format!("{}{}:{}{} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), c));
                    }
                }
            }
            str_vec.push(s);
            str_vec.push(Colors::get_default_fg_bg());
        }
    }

    fn check_allow_p_cmd(&self) -> bool {
        Log::debug_key("check_allow_p_cmd");
        for vec in &self.desc_vecs {
            for menu in vec {
                match &menu.key {
                    PromContKeyMenuType::PCmd(p_cmd) | PromContKeyMenuType::PCmdAndStr(p_cmd, _) => {
                        if &self.as_base().p_cmd == p_cmd {
                            return true;
                        }
                    }
                    PromContKeyMenuType::PCmds { show_cmd: _, all_cmd: all_cmd_vec } => {
                        for p_cmd in all_cmd_vec {
                            if &self.as_base().p_cmd == p_cmd {
                                return true;
                            }
                        }
                    }
                    PromContKeyMenuType::OneChar(c) => {
                        match &self.as_base().p_cmd {
                            P_Cmd::InsertStr(s) if s.to_lowercase() == c.to_lowercase() => return true,
                            _ => {}
                        };
                    }
                    PromContKeyMenuType::ECmd(_) => {}
                };
            }
        }
        return false;
    }
}
impl PromContKeyMenu {
    pub fn get_str(p_cmd: &P_Cmd) -> String {
        // for MouseDownLeft
        return if matches!(p_cmd, P_Cmd::MouseDownLeft(_, _)) { Lang::get().mouse_down_left.to_string() } else { Keybind::get_key_str(KeyCmd::Prom(p_cmd.clone())) };
    }
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContKeyDesc {
    pub base: PromptContBase,
    pub desc_vecs: Vec<Vec<PromContKeyMenu>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PromContKeyMenu {
    pub disp_str: String,
    pub key: PromContKeyMenuType,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PromContKeyMenuType {
    PCmd(P_Cmd),
    ECmd(E_Cmd),
    OneChar(String),
    PCmds { show_cmd: Vec<P_Cmd>, all_cmd: Vec<P_Cmd> },
    PCmdAndStr(P_Cmd, String),
}
impl PromContKeyMenuType {
    pub fn create_cmds(show_cmd: Vec<P_Cmd>, hide_cmd: &mut Vec<P_Cmd>) -> Self {
        let mut all_cmd = show_cmd.clone();
        all_cmd.append(hide_cmd);
        return PromContKeyMenuType::PCmds { show_cmd, all_cmd };
    }
}
