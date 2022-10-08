use ewin_const::term::*;
use ewin_view::view::View;

impl Help {
    pub fn toggle_show(&mut self) {
        self.is_show = !self.is_show;
    }

    pub fn new() -> Self {
        Help { ..Help::default() }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Help {
    pub is_show: bool,
    // Number displayed on the terminal
    pub view: View,
    pub key_bind_vec: Vec<Vec<HelpKeybind>>,
}

#[derive(Debug, Clone)]
pub struct HelpKeybind {
    pub key: String,
    pub funcnm: String,
    pub key_bind_len: usize,
    pub mouse_area: (usize, usize),
}
