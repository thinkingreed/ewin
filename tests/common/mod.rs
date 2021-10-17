use ewin_com::{_cfg::cfg::*, model::*};

pub fn setup() {
    Cfg::init(&Args { ..Args::default() }, include_str!("../../setting.toml"));
    // Keybind::init(&Args { ..Args::default() });
}
