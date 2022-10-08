use crate::{
    activitybar::*,
    cont::*,
    each::{explorer::*, management::*, search::*},
};
use ewin_cfg::{lang::lang_cfg::*, log::*, model::general::default::*};
use ewin_const::def::*;
use ewin_state::term::*;
use ewin_view::traits::view::*;
use std::collections::HashMap;

impl ActivityBar {
    pub const CONT_EXPLORER: &str = "explorer";
    pub const CONT_SEARCH: &str = "search";
    pub const CONT_MANAGEMENT: &str = "management";

    pub fn init(&mut self) {
        Log::debug_key("ActivityBar.init");

        if CfgEdit::get().general.activitybar.width > 0 || State::get().activitybar.is_show {
            State::get().activitybar.is_show = true;

            self.set_size();

            if self.cont_vec.is_empty() {
                let mut map: Vec<HashMap<String, ActivityContBase>> = serde_json::from_str(&Cfg::get().general.activitybar.content.to_string()).unwrap();
                self.set_internal_struct(&mut map);
            }
        } else {
            State::get().sidebar.is_show = false;
        }
    }

    pub fn set_internal_struct(&mut self, vec: &mut [HashMap<String, ActivityContBase>]) {
        Log::debug_key("ActivityBar.set_internal_struct");

        for map in vec.iter_mut() {
            for (key, cont_base) in map {
                // CONT_FILE_TREE
                if key == ActivityBar::CONT_EXPLORER {
                    cont_base.is_select = true;
                    cont_base.view.tooltip_vec = vec![Lang::get().explorer.to_string()];
                    self.cont_vec.push(Box::new(ActivutyBarExplorer::new(cont_base.clone())));
                } else if key == ActivityBar::CONT_SEARCH {
                    cont_base.view.tooltip_vec = vec![Lang::get().search.to_string()];
                    self.cont_vec.push(Box::new(ActivutyBarSearch::new(cont_base.clone())));
                } else if key == ActivityBar::CONT_MANAGEMENT {
                    cont_base.view.tooltip_vec = vec![Lang::get().management.to_string()];
                    self.cont_vec.push(Box::new(ActivutyBarManagement::new(cont_base.clone())));
                }
            }
        }
        self.set_seize_cont();
    }

    pub fn set_seize_cont(&mut self) {
        // +1 is extra
        let mut row_idx = MENUBAR_HEIGHT + 1;

        for cont in self.cont_vec.iter_mut() {
            cont.as_mut_base().view.x = self.view.x;
            cont.as_mut_base().view.width = self.view.width;
            cont.as_mut_base().view.y = row_idx;
            cont.as_mut_base().view.height = ActivityBar::CONT_HEIGHT;
            row_idx += ActivityBar::CONT_HEIGHT;

            if cont.downcast_ref::<ActivutyBarManagement>().is_ok() {
                cont.as_mut_base().view.y = self.view.y_height() - ActivityBar::CONT_HEIGHT;
            }
        }
    }
}
