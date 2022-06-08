use crate::{model::*, prom_trait::cont_trait::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_com::_cfg::key::keycmd::*;
use ewin_widget::widget::pulldown::*;

impl PromContPluginTrait for PromContPulldown {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }

    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }

    fn draw(&self, str_vec: &mut Vec<String>, is_curt: bool) {
        Log::debug_key("PromContPulldown.draw");
        Log::debug("self.base.row_posi_range.start", &self.base.row_posi_range.start);
        Log::debug("is_curt", &is_curt);

        for disp_str in &self.desc_str_vec {
            str_vec.push(format!("{}{}{}{}", MoveTo(0, self.base.row_posi_range.start as u16), if is_curt { Colors::get_msg_highlight_inversion_fg_bg() } else { Colors::get_msg_highlight_fg() }, &disp_str, Colors::get_default_fg_bg()));
        }
        str_vec.push(format!("{}{}{}{}", MoveTo(Pulldown::MARGIN as u16, (self.base.row_posi_range.start + self.desc_str_vec.len()) as u16), Colors::get_ctx_menu_fg_bg_non_sel(), self.pulldown.sel_str, Colors::get_default_fg_bg()));
        if self.pulldown.is_disp {
            self.pulldown.widget.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
        }
    }

    fn check_allow_p_cmd(&self) -> bool {
        Log::debug_key("PromContPulldown.check_allow_p_cmd");
        Log::debug("self.pulldown.is_disp", &self.pulldown.is_disp);
        // Log::debug("self.is_mouse_within_area(y, x)", &self.is_mouse_within_area(y, x));

        return match self.as_base().p_cmd {
            P_Cmd::InsertStr(_) | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut | P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect | P_Cmd::Copy | P_Cmd::Undo | P_Cmd::Redo => true,
            //  P_Cmd::MouseDownLeft(y, _) if self.base.row_posi_range.start <= y && y <= self.base.row_posi_range.end || self.pulldown.is_disp => true,
            P_Cmd::MouseDownLeft(_, _) => true,
            P_Cmd::MouseMove(_, _) if self.pulldown.is_disp => true,
            _ => false,
        };
    }
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContPulldown {
    pub base: PromptContBase,
    pub desc_str_vec: Vec<String>,
    // pub config: PromContPulldownConfig,
    pub pulldown: Pulldown,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContPulldownConfig {}
