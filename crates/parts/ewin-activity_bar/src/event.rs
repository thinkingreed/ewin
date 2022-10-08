use crate::activitybar::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::key::{cmd::*, keys::*};
use ewin_state::term::*;
use ewin_tooltip::tooltip::*;

impl ActivityBar {
    pub fn ctrl_activitybar(cmd_type: &CmdType) -> ActType {
        if let Some(mut activitybar) = ActivityBar::get_result() {
            Log::debug_key("ActivityBar::ctrl_activitybar");
            Log::debug("cmd_type", &cmd_type);

            match cmd_type {
                CmdType::SwitchDispActivityBar => {
                    let is_show = State::get().activitybar.is_show;
                    if !is_show {
                        activitybar.init();
                    } else {
                        State::get().activitybar.is_show = false;
                    }
                    return ActType::Draw(DrawParts::All);
                }
                CmdType::MouseDownLeft(y, x) => {
                    for cont in activitybar.cont_vec.iter_mut() {
                        if cont.as_base().view.is_range(*y, *x) {
                            if cont.as_base().is_select {
                                cont.as_mut_base().is_select = false;
                                return Job::send_cmd(CmdType::HideSideBar);
                            } else {
                                cont.as_mut_base().is_select = true;
                                return Job::send_cmd(CmdType::ShowSideBar);
                            }
                        } else {
                            cont.as_mut_base().is_select = false;
                        }
                    }
                    return ActType::Draw(DrawParts::ActivityBar);
                }
                CmdType::ToolTip(y, x) => {
                    for cont in activitybar.cont_vec.iter() {
                        if cont.as_base().view.is_range(*y, *x) {
                            ToolTip::get().set_msg(&cont.as_base().view);
                            break;
                        }
                    }
                    if ToolTip::get().tgt_view_opt.is_some() {
                        if ToolTip::get().tgt_view_org_opt.is_some() {
                            return ActType::Draw(DrawParts::All);
                        } else {
                            return ActType::Draw(DrawParts::ToolTip);
                        }
                    }
                }
                _ => return ActType::None,
            };
        }
        return ActType::None;
    }

    pub fn judge_when_activitybar(keys: Keys) -> bool {
        Log::debug_key("ActivityBar::judge_when_activitybar");
        Log::debug("keys", &keys);

        if State::get().activitybar.is_show {
            match keys {
                Keys::MouseDownLeft(y, x) | Keys::MouseMove(y, x) => return ActivityBar::get().view.is_range(y as usize, x as usize),
                _ => return false,
            }
        }
        return false;
    }
}
