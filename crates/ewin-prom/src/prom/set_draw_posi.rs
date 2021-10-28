use crate::{
    ewin_com::{_cfg::key::keycmd::*, def::*, log::*, model::*},
    model::*,
    prom::choice::*,
};
use std::u16;

impl Prompt {
    pub fn draw_set_posi(&mut self, tab_state: &TabState, base_posi: u16, h_file: &HeaderFile) {
        Log::debug_key("draw_set_posi");

        if self.cont_1.guide_row_posi == 0 || self.keycmd == KeyCmd::Resize {
            let mut idx = 0;
            self.cont_1.disp_row_posi = base_posi;
            self.cont_1.guide_row_posi = base_posi;
            idx += 1;
            self.cont_1.key_desc_row_posi = base_posi + idx;
            if tab_state.is_search || (tab_state.is_close_confirm && !tab_state.is_save_new_file) {
                idx += 1;
                self.cont_1.opt_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
            } else if tab_state.is_save_new_file || tab_state.is_move_row {
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
            } else if tab_state.is_replace || tab_state.grep.is_grep {
                idx += 1;
                self.cont_1.opt_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_row_posi = base_posi + idx;
                if tab_state.grep.is_grep {
                    idx += 1;
                    self.cont_3.buf_desc_row_posi = base_posi + idx;
                    idx += 1;
                    self.cont_3.buf_row_posi = base_posi + idx;
                }
            } else if tab_state.is_open_file {
                self.prom_open_file.disp_row_len = self.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM;
                idx += 1;
                self.cont_1.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.disp_row_posi = base_posi + idx;
                self.cont_2.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_row_posi = base_posi + idx;
            } else if tab_state.is_menu {
                idx += 1;
                self.cont_1.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
                Choices::set_choice_area(self.cont_1.buf_row_posi, &mut self.cont_1.choices_map);
                self.cont_1.set_default_choice_menu(USIZE_UNDEFINED, USIZE_UNDEFINED, USIZE_UNDEFINED, USIZE_UNDEFINED);
                idx += 2;
                self.cont_2.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_row_posi = base_posi + idx;
                Choices::set_choice_area(self.cont_2.buf_row_posi, &mut self.cont_2.choices_map);
                let (y, x) = Choices::get_y_x(&self.cont_1);
                self.cont_2.set_default_choice_menu(USIZE_UNDEFINED, USIZE_UNDEFINED, y, x);

                idx += 2;
                self.cont_3.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_3.buf_row_posi = base_posi + idx;
                Choices::set_choice_area(self.cont_3.buf_row_posi, &mut self.cont_3.choices_map);
                let (first_y, first_x) = Choices::get_y_x(&self.cont_1);
                let (second_y, second_x) = Choices::get_y_x(&self.cont_2);
                self.cont_3.set_default_choice_menu(first_y, first_x, second_y, second_x);
            } else if tab_state.is_enc_nl {
                idx += 1;
                self.cont_1.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
                self.cont_1.set_default_choice_enc_nl(self.cont_1.buf_row_posi, h_file);

                idx += 1;
                self.cont_2.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_row_posi = base_posi + idx;
                self.cont_2.set_default_choice_enc_nl(self.cont_2.buf_row_posi, h_file);

                idx += 1;
                self.cont_3.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_3.buf_row_posi = base_posi + idx;
                self.cont_3.set_default_choice_enc_nl(self.cont_3.buf_row_posi, h_file);

                idx += 1;
                self.cont_4.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_4.buf_row_posi = base_posi + idx;
                self.cont_4.set_default_choice_enc_nl(self.cont_4.buf_row_posi, h_file);
            }
        }
    }
}
