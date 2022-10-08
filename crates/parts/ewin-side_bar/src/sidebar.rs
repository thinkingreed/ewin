use crate::{explorer::explorer::*, traits::traits::*};
use ewin_cfg::{global::*, model::general::default::*};
use ewin_view::{
    model::*,
    scrollbar::{horizontal::*, vertical::*},
    view::*,
};

impl SideBar {
    const DEFAULT_WIDTH: usize = 25;
    pub const MINIMUM_WIDTH: usize = 10;
    // Percentage of maximum term
    pub const MAXIMUM_WIDTH_PER_TERM: f32 = 0.8;

    pub fn set_init_width(&mut self) {
        let width = CfgEdit::get().general.sidebar.width;
        if width == 0 {
            CFG_EDIT.get().unwrap().try_lock().map(|mut cfg| cfg.general.sidebar.width = SideBar::DEFAULT_WIDTH).unwrap();
        }
    }
}

#[derive(Debug, Clone)]
pub struct SideBar {
    pub cont: Box<dyn SideBarContTrait>,
    pub scrl_h: ScrollbarH,
    pub scrl_h_info: ScrlHInfo,
}

#[derive(Debug, Default, Clone)]

pub struct SideBarContBase {
    pub view: View,
    pub offset: Offset,
    pub config: SideBarContConfig,
    pub scrl_v: ScrollbarV,
    pub scrl_h: ScrollbarH,
    pub scrl_h_info: ScrlHInfo,
}
#[derive(Debug, Clone)]
pub struct SideBarCont {}

impl Default for SideBar {
    fn default() -> Self {
        SideBar { cont: Box::new(Explorer::default()), scrl_h: ScrollbarH::default(), scrl_h_info: ScrlHInfo::default() }
    }
}

#[derive(Debug, Default, Clone)]

pub struct SideBarContConfig {}
