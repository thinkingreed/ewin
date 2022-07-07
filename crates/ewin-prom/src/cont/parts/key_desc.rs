use crate::{model::*, prom_trait::cont_trait::*};
use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*};
use ewin_com::_cfg::key::cmd::{Cmd, CmdType};
use std::fmt::Write as _;
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
                    PromContKeyMenuType::Cmd(cmd_type) => {
                        let _ = write!(s, "{}{}:{}{} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), PromContKeyMenu::get_str(cmd_type));
                    }
                    PromContKeyMenuType::PCmdAndStr(cmd_type, string) => {
                        // s.push_str(&format!("{}{}:{}{} {} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), PromContKeyMenu::get_str(cmd_type), string));
                        let _ = write!(s, "{}{}:{}{} {} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), PromContKeyMenu::get_str(cmd_type), string);
                    }
                    PromContKeyMenuType::PCmds { show_cmd: show_cmd_vec, all_cmd: _ } => {
                        let _ = write!(s, "{}{}:{}", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(),);

                        for cmd_type in show_cmd_vec {
                            let _ = write!(s, " {}", PromContKeyMenu::get_str(cmd_type));
                        }
                        s.push(' ');
                    }
                    PromContKeyMenuType::OneChar(c) => {
                        let _ = write!(s, "{}{}:{}{} ", Colors::get_default_fg(), cont.disp_str, Colors::get_msg_highlight_fg(), c);
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
                    PromContKeyMenuType::Cmd(cmd_type) | PromContKeyMenuType::PCmdAndStr(cmd_type, _) => {
                        if &self.as_base().cmd.cmd_type == cmd_type {
                            return true;
                        }
                    }
                    PromContKeyMenuType::PCmds { show_cmd: _, all_cmd: all_cmd_vec } => {
                        for cmd_type in all_cmd_vec {
                            if &self.as_base().cmd.cmd_type == cmd_type {
                                return true;
                            }
                        }
                    }
                    PromContKeyMenuType::OneChar(c) => {
                        match &self.as_base().cmd.cmd_type {
                            CmdType::InsertStr(s) if s.to_lowercase() == c.to_lowercase() => return true,
                            _ => {}
                        };
                    }
                };
            }
        }
        return false;
    }
}
impl PromContKeyMenu {
    pub fn get_str(cmd_type: &CmdType) -> String {
        // for MouseDownLeft
        return if matches!(cmd_type, CmdType::MouseDownLeft(_, _)) { Lang::get().mouse_down_left.to_string() } else { Cmd::get_cmd_str(cmd_type) };
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
    Cmd(CmdType),
    // ECmd(CmdType),
    OneChar(String),
    PCmds { show_cmd: Vec<CmdType>, all_cmd: Vec<CmdType> },
    PCmdAndStr(CmdType, String),
}
impl PromContKeyMenuType {
    pub fn create_cmds(show_cmd: Vec<CmdType>, hide_cmd: &mut Vec<CmdType>) -> Self {
        let mut all_cmd = show_cmd.clone();
        all_cmd.append(hide_cmd);
        return PromContKeyMenuType::PCmds { show_cmd, all_cmd };
    }
}
