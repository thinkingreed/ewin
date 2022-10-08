use crate::{btn_grourp::*, conts::parts::kvs::file_prop::*, traits::traits::*};
use ewin_const::def::*;
use ewin_view::view::*;

#[derive(Debug, Clone)]
pub struct Dialog {
    pub view: View,
    pub is_show: bool,
    pub cont: Box<dyn DialogContTrait>,
    pub base_y: usize,
    pub base_x: usize,
    pub close_btn: View,
    pub btn_group: DialogBtnGrourp,
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog { view: View::default(), is_show: false, cont: Box::new(DialogContFileProp::default()), base_y: USIZE_UNDEFINED, base_x: USIZE_UNDEFINED, close_btn: View::default(), btn_group: DialogBtnGrourp::default() }
    }
}

#[derive(Debug, PartialEq, Default, Eq, Clone, Hash)]
pub struct DialogBtn {
    pub name: String,
    pub name_width: usize,
    pub is_on_mouse: bool,
    pub cfg: DialogBtnCfg,
    pub btn_type: DialogBtnType,
    pub view: View,
}

#[derive(Debug, PartialEq, Default, Eq, Clone, Hash, Copy)]
pub enum DialogBtnType {
    #[default]
    Ok,
}

#[derive(Debug, PartialEq, Default, Eq, Clone, Hash)]
pub struct DialogBtnCfg {
    pub is_close: bool,
}
