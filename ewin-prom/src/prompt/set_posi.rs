use super::prompt::*;
use crate::ewin_core::model::*;
use std::u16;

impl Prompt {
    pub fn draw_set_posi(&mut self, tab_state: &TabState, base_posi: u16) {
        if self.cont_1.guide_row_posi == 0 {
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
            } else if tab_state.is_replace || tab_state.grep_state.is_grep {
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
                if tab_state.grep_state.is_grep {
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
                idx += 2;
                self.cont_2.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_row_posi = base_posi + idx;
                idx += 2;
                self.cont_3.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_3.buf_row_posi = base_posi + idx;
            } else if tab_state.is_enc_nl {
                idx += 1;
                self.cont_1.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_1.buf_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_2.buf_row_posi = base_posi + idx;
                idx += 1;
                self.cont_3.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_3.buf_row_posi = base_posi + idx;
                idx += 1;
                self.cont_4.buf_desc_row_posi = base_posi + idx;
                idx += 1;
                self.cont_4.buf_row_posi = base_posi + idx;
            }
        }
    }
}
