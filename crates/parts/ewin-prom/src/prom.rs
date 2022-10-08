use crate::{model::*, traits::main_trait::*};
use ewin_activity_bar::activitybar::*;
use ewin_cfg::log::*;
use ewin_const::term::*;
use ewin_key::key::cmd::*;
use ewin_side_bar::sidebar::*;
use ewin_state::term::*;
use ewin_view::view::*;

impl Prom {
    pub fn init(&mut self, plugin: Box<dyn PromTrait>) {
        self.curt = plugin;
        self.set_size();
    }

    pub fn curt<T: PromTrait>(&mut self) -> &mut T {
        return self.curt.downcast_mut::<T>().unwrap();
    }

    pub fn set_size(&mut self) {
        if !State::get().curt_ref_state().is_nomal() {
            self.view.height = self.curt.as_mut_base().get_disp_all_row_num(self.row_bottom_posi);

            let (cols, _) = get_term_size();

            let tabs_cols = cols - ActivityBar::get().get_width() - SideBar::get().get_width_all();

            self.view.width = tabs_cols;
            self.view.y = self.row_bottom_posi - self.view.height;
            self.view.x = ActivityBar::get().get_width() + SideBar::get().get_width_all();
            self.curt.as_mut_base().set_cont_item_disp_posi(self.view.y);
        }
    }

    pub fn clear(&mut self) {
        Log::debug_key("Prompt.clear");
        self.view = View::default();
    }

    pub fn set_cmd(&mut self, cmd: &Cmd) {
        Log::debug_key("Prompt::set_keys");

        self.cmd = cmd.clone();
        self.curt.as_mut_base().set_key_info(cmd.clone());
    }
}
