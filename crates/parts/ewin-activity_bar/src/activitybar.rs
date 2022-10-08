use crate::traits::traits::*;
use ewin_view::view::*;

impl ActivityBar {}

#[derive(Debug, Default, Clone)]
pub struct ActivityBar {
    pub view: View,
    pub is_show: bool,
    pub cont_vec: Vec<Box<dyn ActivityBarContTrait>>,
}
