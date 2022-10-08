use super::model::*;
use crossterm::cursor::*;
use ewin_cfg::log::*;
use ewin_const::{models::view::get_space, term::get_term_size};
use ewin_state::term::*;
use std::{io::Write, u16};

impl Prom {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::info_key("Prompt.draw");
        Log::debug("self.curt.as_base().curt_cont_idx", &self.curt.as_base().curt_cont_idx);

        if !State::get().curt_ref_state().is_nomal() {
            let cols = get_term_size().0;
            for (i, cont) in self.curt.as_base().cont_vec.iter().enumerate() {
                Log::debug("iiiii", &i);
                Log::debug("cont.as_base().row_posi_range", &cont.as_base().row_posi_range);
                for i in cont.as_base().row_posi_range.start..=cont.as_base().row_posi_range.end {
                    str_vec.push(format!("{}{}", MoveTo(self.view.x as u16, (i) as u16), get_space(cols - self.view.x)));
                }
                str_vec.push(MoveTo(self.view.x as u16, cont.as_base().row_posi_range.start as u16).to_string());

                let is_curt = i == self.curt.as_base().curt_cont_idx;
                cont.draw(str_vec, is_curt);
            }
        }
    }
    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("Prompt.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        self.draw_cur(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>) -> bool {
        if let Some((y, x)) = self.curt.as_mut_base().get_cur_posi() {
            str_vec.push(MoveTo((self.view.x + x) as u16, y as u16).to_string());
            return true;
        }
        return false;
    }
}
