use crate::{colors::*, def::*, global::*, log::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct HeaderBar {
    pub filenm: String,
    pub filenm_disp: String,
    pub filenm_w: usize,
    pub close_btn: char,
    pub close_btn_disp: String,
    pub close_btn_area: (usize, usize),
    pub close_btn_w: usize,
    pub help_btn: String,
    pub help_btn_disp: String,
    pub help_btn_area: (usize, usize),
    pub help_btn_w: usize,
    // Position on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for HeaderBar {
    fn default() -> Self {
        HeaderBar {
            filenm: String::new(),
            filenm_disp: String::new(),
            filenm_w: 0,
            close_btn: '×',
            close_btn_disp: String::new(),
            close_btn_area: (0, 0),
            close_btn_w: 0,
            help_btn: String::new(),
            help_btn_disp: String::new(),
            help_btn_area: (0, 0),
            help_btn_w: 0,
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
    }
}

impl HeaderBar {
    const HELP_BTN_WITH: usize = 7;
    const CLOSE_BTN_WITH: usize = 3;

    pub fn new() -> Self {
        HeaderBar { ..HeaderBar::default() }
    }

    pub fn draw<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("　　　　　　　　HeaderBar.draw");

        let help_btn = format!("{}:{}", KEY_HELP, LANG.help);
        let close_btn = format!(" {} ", self.close_btn);

        let hber_str = format!(
            "{}{}{}{}{}{}{}{} {}{}{}",
            MoveTo(0, self.disp_row_posi as u16),
            Clear(ClearType::CurrentLine),
            Colors::get_default_bg(),
            Colors::get_sber_fg(),
            self.filenm_disp,
            Colors::get_sber_inversion_fg_bg(),
            help_btn,
            Colors::get_default_bg(),
            Colors::get_sber_inversion_fg_bg(),
            close_btn,
            Colors::get_default_bg(),
        );

        let _ = out.write(&hber_str.as_bytes());
        out.flush().unwrap();
    }

    pub fn set_posi(&mut self, cols_w: usize) {
        self.disp_col_num = cols_w;

        self.filenm_w = self.disp_col_num - HeaderBar::HELP_BTN_WITH - 1 - HeaderBar::CLOSE_BTN_WITH - 1;
        self.help_btn_area = (self.filenm_w + 1, self.filenm_w + 1 + HeaderBar::HELP_BTN_WITH - 1);

        // +1 is space between
        let close_btn_area_s = self.filenm_w + HeaderBar::HELP_BTN_WITH + 1;
        self.close_btn_area = (close_btn_area_s, close_btn_area_s + HeaderBar::CLOSE_BTN_WITH - 1);

        self.filenm = FILE.get().unwrap().try_lock().unwrap().filenm.clone();

        if self.filenm.is_empty() {
            self.filenm = LANG.new_file.clone();
        }

        let filenm = cut_str(self.filenm.clone(), self.filenm_w, true);
        self.filenm_disp = format!("{fnm:^width$}", fnm = filenm, width = self.filenm_w - (get_str_width(&filenm) - filenm.chars().count()));
    }
}
