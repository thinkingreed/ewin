use crate::model::*;

impl Editor {
    pub fn set_init_scrlbar(&mut self) {
        self.init_editor_scrlbar_h();
        self.set_size_adjust_editor();
        self.calc_editor_scrlbar();
    }

    pub fn calc_editor_scrlbar(&mut self) {
        self.calc_editor_scrlbar_h();
        self.calc_editor_scrlbar_v();
    }
}
