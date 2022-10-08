use super::horizontal::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_cfg::log::*;

pub trait ScrlHTrait: DynClone + Any + 'static + std::fmt::Debug {
    fn get_row_chars(&mut self, idx: usize) -> usize;
    fn get_row_width(&mut self, idx: usize) -> usize;
    fn get_scrl_h_info(&mut self) -> &mut ScrlHInfo;
    fn get_vec_len(&mut self) -> usize;
    fn get_row_max_chars(&mut self) -> usize;

    fn init_scrlbar_h(&mut self) {
        Log::debug_key("ScrlHTrait.init_scrlbar_h");

        self.get_scrl_h_info().row_width_chars_vec = vec![(0, 0); self.get_vec_len()];
        for i in 0..self.get_vec_len() {
            self.get_scrl_h_info().row_width_chars_vec[i] = (self.get_row_width(i) + ScrollbarH::SCROLL_BAR_H_END_LINE_MARGIN, self.get_row_chars(i) + ScrollbarH::SCROLL_BAR_H_END_LINE_MARGIN);

            if self.get_scrl_h_info().row_width_chars_vec[i].0 > self.get_scrl_h_info().row_max_width {
                self.get_scrl_h_info().row_max_width_idx = i;
                self.get_scrl_h_info().row_max_width = self.get_scrl_h_info().row_width_chars_vec[i].0;
                self.get_scrl_h_info().row_max_chars = self.get_scrl_h_info().row_width_chars_vec[i].1;

                if self.get_scrl_h_info().row_max_chars > self.get_scrl_h_info().row_max_width {
                    self.get_scrl_h_info().row_max_width = self.get_scrl_h_info().row_max_chars;
                }
            }
        }
    }

    fn calc_row_max(&mut self) {
        if !self.get_scrl_h_info().row_width_chars_vec.is_empty() {
            self.get_scrl_h_info().row_max_width = self.get_scrl_h_info().row_width_chars_vec.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
            let row_max_width = self.get_scrl_h_info().row_max_width;
            self.get_scrl_h_info().row_max_width_idx = self.get_scrl_h_info().row_width_chars_vec.iter().position(|(x, _)| x == &row_max_width).unwrap();
            self.get_scrl_h_info().row_max_chars = self.get_row_max_chars();
        }
    }
}

downcast!(dyn ScrlHTrait);
dyn_clone::clone_trait_object!(ScrlHTrait);
