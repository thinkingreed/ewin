use crate::{activitybar::*, each::management::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_const::{def::*, models::view::*};
use ewin_state::term::*;
use ewin_view::traits::view::*;

impl ActivityBar {
    pub const CONT_HEIGHT: usize = 2;

    pub fn draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("ActivityBar.draw");

        if State::get().activitybar.is_show {
            str_vec.push(Colors::get_activitybar_bg_default());
            self.clear_all(str_vec);

            let mut idx = MENUBAR_HEIGHT;
            for (i, cont) in self.cont_vec.iter().enumerate() {
                if idx >= self.view.height - ActivityBar::CONT_HEIGHT {
                    break;
                }

                if self.view.height - idx >= ActivityBar::CONT_HEIGHT && cont.downcast_ref::<ActivutyBarManagement>().is_err() {
                    Log::debug("cont.as_base().view", &cont.as_base().view);

                    if i == 0 {
                        idx += 1;
                        str_vec.push(get_space(self.view.width));
                    }
                    cont.draw_row(str_vec, idx);
                    idx += ActivityBar::CONT_HEIGHT;
                    continue;
                }

                loop {
                    str_vec.push(MoveTo(self.view.x as u16, idx as u16).to_string());
                    str_vec.push(Colors::get_activitybar_bg_default());
                    str_vec.push(get_space(self.view.width));
                    idx += 1;
                    if idx >= self.view.y_height() - ActivityBar::CONT_HEIGHT {
                        break;
                    }
                }
            }

            // Management
            for cont in self.cont_vec.iter() {
                if cont.downcast_ref::<ActivutyBarManagement>().is_ok() {
                    cont.draw_row(str_vec, cont.as_base().view.y);

                    str_vec.push(MoveTo(cont.as_base().view.x as u16, (cont.as_base().view.y + 1) as u16).to_string());
                    str_vec.push(get_space(self.view.width));
                }
            }
        }
    }

    pub fn draw_only<T: std::io::Write>(&self, out: &mut T) {
        Log::debug_key("ActivityBar.draw_only");

        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        Log::debug("vvvvvvvvvvvvvvvvvvvvvvvvv", &v);

        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
