use crate::statusbar::*;
use ewin_cfg::log::*;
use ewin_const::models::event::*;
use ewin_job::job::*;
use ewin_key::key::{cmd::*, keys::*};

impl StatusBar {
    pub fn ctrl_statusbar(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("ctrl_statusbar");

        match cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                let (x, _) = (*x as usize, *y as usize);
                if self.cur_area.0 <= x && x <= self.cur_area.1 {
                    Job::send_cmd(CmdType::MoveRowProm);
                    return ActType::None;
                } else if self.enc_nl_area.0 <= x && x <= self.enc_nl_area.1 {
                    Job::send_cmd(CmdType::EncodingProm);
                    return ActType::None;
                }
                return ActType::Cancel;
            }
            _ => return ActType::Next,
        }
    }
    pub fn judge_when_statusbar( keys: Keys, editor_is_dragging: bool) -> bool {
        match &keys {
            Keys::MouseDownLeft(y, _) if y == &(StatusBar::get().view.y as u16) => return true,
            Keys::MouseDragLeft(y, _) if y == &(StatusBar::get().view.y as u16) => return !editor_is_dragging,
            _ => return false,
        }
    }
}
