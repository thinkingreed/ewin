use ewin_core::{_cfg::cfg::*, model::*};

pub fn setup() {
    Cfg::init(&Args { ..Args::default() });
    // Keybind::init(&Args { ..Args::default() });
}
