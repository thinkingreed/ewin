use ewin_cfg::{global::*, model::general::default::*};
use ewin_view::{model::*, scrollbar_v::*, view::*};

use crate::{side_bar_trait::side_bar_trait::*, tree_file_view::tree::*};
impl SideBar {
    const DEFAULT_WIDTH: usize = 25;

    pub fn set_init_width(&mut self) {
        let width = CfgEdit::get().general.sidebar.width;
        if width == 0 {
            //  &mut  CFG.get_mut().unwrap(). general.display.appearance.sidebar.width = Self::DEFAULT_WIDTH;
            //  Cfg::get(). general.display.appearance.sidebar.width = SideBar::DEFAULT_WIDTH;

            CFG_EDIT.get().unwrap().try_lock().map(|mut cfg| cfg.general.sidebar.width = Self::DEFAULT_WIDTH).unwrap();
        }
    }
}
#[derive(Debug, Clone)]
pub struct SideBar {
    pub scrl_v: ScrollbarV,
    pub cont: Box<dyn SideBarContTrait>,
}

#[derive(Debug, Default, Clone)]

pub struct SideBarContBase {
    pub view: View,
    pub offset: Offset,
}
#[derive(Debug, Clone)]
pub struct SideBarCont {}

impl Default for SideBar {
    fn default() -> Self {
        SideBar { cont: Box::new(TreeFileView::new("")), scrl_v: ScrollbarV::default() }
    }
}
