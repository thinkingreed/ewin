use crate::{colors::*, def::*, global::*, log::*, terminal::Terminal, util::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct HeaderBar {
    pub all_filenm_rest: usize,
    pub all_filenm_w: usize,
    pub file_vec: Vec<HeaderFile>,
    pub plus_btn_area: (usize, usize),
    pub close_btn: char,
    pub close_btn_area: (usize, usize),
    pub help_btn: String,
    pub help_btn_area: (usize, usize),
    // Position on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for HeaderBar {
    fn default() -> Self {
        HeaderBar {
            all_filenm_rest: 0,
            all_filenm_w: 0,
            file_vec: vec![],
            plus_btn_area: (0, 0),
            close_btn: '×',
            close_btn_area: (0, 0),
            help_btn: String::new(),
            help_btn_area: (0, 0),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeaderFile {
    pub filenm: String,
    pub filenm_disp: String,
    pub ext: String,
    pub is_changed: bool,
    pub filenm_area: (usize, usize),
    pub close_area: (usize, usize),
}

impl Default for HeaderFile {
    fn default() -> Self {
        HeaderFile {
            filenm: String::new(),
            filenm_disp: String::new(),
            ext: String::new(),
            is_changed: false,
            filenm_area: (0, 0),
            close_area: (0, 0),
        }
    }
}

impl HeaderBar {
    const HELP_BTN_WITH: usize = 7;
    const CLOSE_BTN_WITH: usize = 3;
    const FILENM_LEN_LIMMIT: usize = 15;

    pub fn new() -> Self {
        HeaderBar { ..HeaderBar::default() }
    }

    pub fn draw<T: Write>(out: &mut T, term: &Terminal) {
        Log::ep_s("　　　　　　　　HeaderBar.draw");

        let help_btn = format!("{}:{}", KEY_HELP, LANG.help);
        let close_btn = format!(" {} ", term.hbar.close_btn);

        let mut hber_str = format!("{}{}{}{}", MoveTo(0, term.hbar.disp_row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_default_bg(), Colors::get_sber_fg(),);
        for (i, file) in term.hbar.file_vec.iter().enumerate() {
            if i == term.idx {
                hber_str.push_str(&format!("{}{}{}{}{}{}", &Colors::get_sber_inversion_fg_bg(), &file.filenm_disp.clone(), &Colors::get_default_fg_bg(), &Colors::get_sber_fg(), &"|", &Colors::get_default_fg_bg()));
            } else {
                hber_str.push_str(&format!("{}{}{}", &Colors::get_sber_fg(), &file.filenm_disp.clone(), &"|"));
            }
            if i == term.hbar.file_vec.len() - 1 {
                hber_str.push_str(&format!("{}{}{}{}", &Colors::get_sber_fg(), &" + ", &Colors::get_default_fg_bg(), &" ".repeat(term.hbar.all_filenm_rest)));
            }
        }
        hber_str = format!("{}{}{}{} {}{}{}", hber_str, Colors::get_sber_inversion_fg_bg(), help_btn, Colors::get_default_bg(), Colors::get_sber_inversion_fg_bg(), close_btn, Colors::get_default_bg(),);

        let _ = out.write(&hber_str.as_bytes());
        out.flush().unwrap();
    }

    pub fn set_posi(&mut self, cols_w: usize) {
        self.disp_col_num = cols_w;

        self.all_filenm_w = self.disp_col_num - HeaderBar::HELP_BTN_WITH - 1 - HeaderBar::CLOSE_BTN_WITH;
        self.help_btn_area = (self.all_filenm_w + 1, self.all_filenm_w + 1 + HeaderBar::HELP_BTN_WITH - 1);

        // +1 is space between
        let close_btn_area_s = self.all_filenm_w + HeaderBar::HELP_BTN_WITH + 1;
        self.close_btn_area = (close_btn_area_s, close_btn_area_s + HeaderBar::CLOSE_BTN_WITH - 1);
    }

    pub fn set_header_filenm(term: &mut Terminal) {
        let mut all_filenm = String::new();
        let mut width = 0;

        let vec_len = term.hbar.file_vec.len();
        for (i, header_file) in term.hbar.file_vec.iter_mut().enumerate() {
            let s_w = width;
            header_file.filenm_disp = if header_file.is_changed { format!("● {} ×", header_file.filenm.clone()) } else { format!("{} ×", header_file.filenm.clone()) };

            // cut str
            if get_str_width(&header_file.filenm_disp) > HeaderBar::FILENM_LEN_LIMMIT {
                header_file.filenm_disp = cut_str(header_file.filenm_disp.clone(), HeaderBar::FILENM_LEN_LIMMIT, true);
            }

            width += get_str_width(&header_file.filenm_disp);
            all_filenm = if i == 0 { header_file.filenm_disp.clone() } else { format!("{}|{}", all_filenm, header_file.filenm_disp) };
            let e_w = get_str_width(&all_filenm);

            header_file.filenm_area = (s_w, e_w);
            header_file.close_area = (e_w - 1, e_w);
            // +1 is for "|"
            width += 1;
            if i == vec_len - 1 {
                all_filenm.push_str("|");
                all_filenm.push_str(" + ");
                term.hbar.plus_btn_area = (width, width + 2);
            }
        }

        Log::ep("term.hbar.all_filenm_w", &term.hbar.all_filenm_w);
        Log::ep("get_str_width(&all_filenm)", &get_str_width(&all_filenm));

        term.hbar.all_filenm_rest = term.hbar.all_filenm_w - get_str_width(&all_filenm);
    }
}
