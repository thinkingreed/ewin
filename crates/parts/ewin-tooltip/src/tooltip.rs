use ewin_cfg::log::*;
use ewin_const::{models::event::*, term::*};
use ewin_key::key::keys::*;
use ewin_utils::str_edit::*;
use ewin_view::view::*;

impl ToolTip {
    const MSG_EXTRA: usize = 2;

    pub fn set_msg(&mut self, view: &View) {
        self.is_show = true;
        self.tgt_view_opt = Some(view.clone());
        self.tgt_view_org_opt = self.tgt_view_opt.clone();

        self.vec = view.tooltip_vec.clone();
        let (_, rows) = get_term_size();
        let max_width = get_strs_max_width(&self.vec);

        self.view_org = self.view.clone();
        self.view.width = max_width + ToolTip::MSG_EXTRA;
        self.view.height = self.vec.len();
        self.view.y = if view.y == rows { view.y - self.view.height } else { view.y + 1 };
        self.view.x = if view.x == 0 { view.x_width() } else { view.x_width_middle() };
        self.tgt_view_opt = Some(view.clone());
    }

    pub fn is_tgt_range(&mut self, keys: Keys) -> ActType {
        Log::debug_key("ToolTip.is_tgt_range");
        Log::debug("self.is_show", &self.is_show);
        Log::debug("keys", &keys);
        Log::debug("self.tgt_view", &self.tgt_view_opt);

        if self.is_show {
            match keys {
                Keys::MouseMove(y, x) | Keys::MouseDownLeft(y, x) => {
                    if let Some(ref tgt_view) = self.tgt_view_opt {
                        if tgt_view.is_range(y as usize, x as usize) {
                            return ActType::None;
                        }
                    }
                    self.tgt_view_org_opt = self.tgt_view_opt.clone();
                    self.is_show = false;
                    return ActType::Next;
                }
                _ => {}
            }
            self.clear();
        }
        return ActType::Next;
    }

    pub fn clear(&mut self) {
        self.is_show = false;
        self.tgt_view_opt = None;
        self.tgt_view_org_opt = None;
    }
}

#[derive(Debug, Default, Clone)]
pub struct ToolTip {
    pub is_show: bool,
    pub view: View,
    pub view_org: View,
    pub vec: Vec<String>,
    pub tgt_view_opt: Option<View>,
    pub tgt_view_org_opt: Option<View>,
    pub disp_vec: Vec<String>,
}
