use crate::{colors::*, def::*, global::*, log::*, model::*, terminal::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;

impl HeaderBar {
    const ALLOW_BTN_WITH: usize = 2;
    const PLUS_BTN_WITH: usize = 3;
    const HELP_BTN_WITH: usize = 7;
    const CLOSE_BTN_WITH: usize = 3;
    const FILENM_LEN_LIMMIT: usize = 12;

    pub fn draw<T: Write>(out: &mut T, term: &Terminal) {
        Log::info_s("　　　　　　　HeaderBar.draw");

        let plus_btn = format!(" {} ", '+');
        let help_btn = format!("{}:{}", KEY_HELP, LANG.help);
        let close_btn = format!(" {} ", 'x');
        let left_arrow_btn = "< ".to_string();
        let right_arrow_btn = " >".to_string();

        let mut hber_str = format!("{}{}{}", MoveTo(0, term.hbar.disp_row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_hbar_fg_bg());
        if term.hbar.is_left_arrow_disp {
            hber_str.push_str(&format!("{}{}{}", &Colors::get_hbar_inversion_fg_bg(), left_arrow_btn, &Colors::get_hbar_fg_bg()));
        }
        for (i, h_file) in term.hbar.file_vec.iter().enumerate() {
            if !h_file.is_disp {
                continue;
            }
            if i == term.idx {
                hber_str.push_str(&format!("{}{}{}", &Colors::get_hbar_inversion_fg_bg(), &h_file.filenm_disp.clone(), &Colors::get_hbar_fg_bg()));
            } else {
                hber_str.push_str(&format!("{}", &h_file.filenm_disp.clone()));
            }
            if i != term.hbar.file_vec.len() - 1 && term.hbar.file_vec.get(i + 1).unwrap().is_disp {
                hber_str.push_str(&"|");
            }
        }
        hber_str.push_str(&format!("{}{}", &Colors::get_default_bg(), &" ".repeat(term.hbar.all_filenm_rest)));
        if term.hbar.is_right_arrow_disp {
            hber_str.push_str(&format!("{}{}{}", &Colors::get_hbar_inversion_fg_bg(), right_arrow_btn, &Colors::get_hbar_fg_bg()));
        }
        hber_str = format!("{}{}{}{} {}{}{} {}{}{}", hber_str, Colors::get_hbar_inversion_fg_bg(), plus_btn, Colors::get_default_bg(), Colors::get_hbar_inversion_fg_bg(), help_btn, Colors::get_default_bg(), Colors::get_hbar_inversion_fg_bg(), close_btn, Colors::get_default_bg());

        let _ = out.write(&hber_str.as_bytes());
        out.flush().unwrap();
    }

    pub fn set_posi(&mut self, cols_w: usize) {
        self.disp_col_num = cols_w;
        self.all_filenm_space_w = self.disp_col_num - HeaderBar::PLUS_BTN_WITH - 1 - HeaderBar::HELP_BTN_WITH - 1 - HeaderBar::CLOSE_BTN_WITH;
        // +1 is space between
        self.plus_btn_area = (self.all_filenm_space_w, self.all_filenm_space_w + HeaderBar::PLUS_BTN_WITH - 1);
        self.help_btn_area = (self.plus_btn_area.1 + 2, self.plus_btn_area.1 + 2 + HeaderBar::HELP_BTN_WITH - 1);
        self.close_btn_area = (self.help_btn_area.1 + 2, self.help_btn_area.1 + 2 + HeaderBar::CLOSE_BTN_WITH - 1);
    }

    pub fn set_header_filenm(term: &mut Terminal) {
        let mut tmp_all_vec: Vec<(usize, String)> = vec![];
        let vec_len = *&term.hbar.file_vec.len();
        let disp_base_idx = if term.hbar.disp_base_idx == USIZE_UNDEFINED { 0 } else { term.hbar.disp_base_idx };

        term.hbar.init();

        // Temperatures stored in Vec for ascending / descending sorting
        for (idx, h_file) in term.hbar.file_vec.iter_mut().enumerate() {
            // cut str
            h_file.filenm_disp = if get_str_width(&h_file.filenm) > HeaderBar::FILENM_LEN_LIMMIT { cut_str(h_file.filenm.clone(), HeaderBar::FILENM_LEN_LIMMIT, true) } else { h_file.filenm.clone() };
            h_file.filenm_disp = if h_file.is_changed { format!("* {} x", h_file.filenm_disp.clone()) } else { format!("{} x", h_file.filenm_disp.clone()) };
            tmp_all_vec.push((idx, h_file.filenm_disp.clone()));
        }

        if term.hbar.disp_base_idx == USIZE_UNDEFINED {
            // Reverse to calculate length from right side
            tmp_all_vec.reverse();
        } else {
            if term.hbar.disp_base_idx > 0 {
                term.hbar.is_left_arrow_disp = true;
            }
        }

        let mut all_filenm_vec: Vec<(usize, String)> = vec![];
        let mut width = 0;
        // Judgment of tab to display
        let mut idx_org = 0;
        let mut left_arrow_w = 0;
        if term.hbar.is_left_arrow_disp {
            left_arrow_w += HeaderBar::ALLOW_BTN_WITH;
        }

        for (i, (idx, _)) in tmp_all_vec[disp_base_idx..].iter().enumerate() {
            let h_file = term.hbar.file_vec.get_mut(*idx).unwrap();
            let right_arrow_w = if term.hbar.disp_base_idx != USIZE_UNDEFINED && *idx != vec_len - 1 { HeaderBar::ALLOW_BTN_WITH } else { 0 };

            if term.hbar.all_filenm_space_w - left_arrow_w - right_arrow_w > width + get_str_width(&h_file.filenm_disp) {
                h_file.is_disp = true;

                width += get_str_width(&h_file.filenm_disp);
                all_filenm_vec.push((*idx, h_file.filenm_disp.clone()));

                if vec_len != 1 && i != tmp_all_vec[disp_base_idx..].len() - 1 {
                    all_filenm_vec.push((USIZE_UNDEFINED, "|".to_string()));
                    width += 1;
                }
            } else {
                if term.hbar.disp_base_idx == USIZE_UNDEFINED {
                    term.hbar.disp_base_idx = idx_org;
                }
                // del last "|"
                if i <= tmp_all_vec.len() - 1 {
                    all_filenm_vec.pop();
                }
                break;
            }
            idx_org = *idx;
        }

        if term.hbar.disp_base_idx == USIZE_UNDEFINED {
            // Returns Reverse to calculate the range of each tab
            all_filenm_vec.reverse();
        }

        if all_filenm_vec.last().unwrap().0 != vec_len - 1 {
            term.hbar.is_right_arrow_disp = true;
        }

        let mut width = 0;
        for (_, disp_str) in &all_filenm_vec {
            width += get_str_width(&disp_str);
        }
        term.hbar.all_filenm_rest = term.hbar.all_filenm_space_w - width;

        // Width calc on tab
        let mut width = if term.hbar.is_left_arrow_disp { 2 } else { 0 };
        for (idx, filenm) in all_filenm_vec.iter() {
            let s_w = width;
            if *filenm == "|".to_string() {
                width += 1;
                continue;
            } else {
                width += get_str_width(&filenm);
            }
            let e_w = width - 1;
            term.hbar.file_vec.get_mut(*idx).unwrap().filenm_area = (s_w, e_w);
            term.hbar.file_vec.get_mut(*idx).unwrap().close_area = (e_w - 1, e_w);
        }

        // Width calc on left_arrow
        if term.hbar.is_left_arrow_disp {
            term.hbar.all_filenm_rest -= HeaderBar::ALLOW_BTN_WITH;
            term.hbar.left_arrow_area = (0, 1);
        }
        // Width calc on right_arrow
        if term.hbar.is_right_arrow_disp {
            term.hbar.all_filenm_rest -= HeaderBar::ALLOW_BTN_WITH;
            term.hbar.right_arrow_area = (term.hbar.all_filenm_space_w - 2, term.hbar.all_filenm_space_w - 1);
            term.hbar.all_filenm_rest_area = (term.hbar.all_filenm_space_w - term.hbar.all_filenm_rest - HeaderBar::ALLOW_BTN_WITH, term.hbar.right_arrow_area.0 - 1);
        } else {
            term.hbar.all_filenm_rest_area = (term.hbar.all_filenm_space_w - term.hbar.all_filenm_rest, term.hbar.all_filenm_space_w - 1);
        }
    }

    pub fn init(&mut self) {
        self.is_left_arrow_disp = false;
        self.is_right_arrow_disp = false;
        self.left_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.right_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);

        for h_file in self.file_vec.iter_mut() {
            h_file.filenm_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
            h_file.close_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
            h_file.is_disp = false;
        }
    }
    pub fn new() -> Self {
        HeaderBar { ..HeaderBar::default() }
    }
}

#[derive(Debug, Clone)]
pub struct HeaderBar {
    pub all_filenm_rest: usize,
    pub all_filenm_rest_area: (usize, usize),
    pub all_filenm_space_w: usize,
    pub disp_base_idx: usize,
    pub file_vec: Vec<HeaderFile>,
    pub plus_btn_area: (usize, usize),
    pub help_btn: String,
    pub close_btn_area: (usize, usize),
    pub help_btn_area: (usize, usize),
    pub is_left_arrow_disp: bool,
    pub is_right_arrow_disp: bool,
    pub right_arrow_area: (usize, usize),
    pub left_arrow_area: (usize, usize),

    // Position on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    pub history: History,
}

impl Default for HeaderBar {
    fn default() -> Self {
        HeaderBar {
            all_filenm_rest: 0,
            all_filenm_rest_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            all_filenm_space_w: 0,
            disp_base_idx: USIZE_UNDEFINED,
            file_vec: vec![],
            plus_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            close_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            help_btn: String::new(),
            help_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            is_left_arrow_disp: false,
            is_right_arrow_disp: false,
            right_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            left_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
            history: History::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeaderFile {
    pub filenm: String,
    pub filenm_disp: String,
    // pub ext: String,
    pub is_disp: bool,
    pub is_changed: bool,
    pub filenm_area: (usize, usize),
    pub close_area: (usize, usize),
    pub enc: Encode,
    pub new_line: String,
    pub bom_exsist: Option<Encode>,
}

impl Default for HeaderFile {
    fn default() -> Self {
        HeaderFile {
            filenm: String::new(),
            filenm_disp: String::new(),
            //  ext: String::new(),
            is_disp: false,
            is_changed: false,
            filenm_area: (0, 0),
            close_area: (0, 0),
            enc: Encode::UTF8,
            new_line: NEW_LINE_LF.to_string(),
            bom_exsist: None,
        }
    }
}

impl HeaderFile {
    pub fn new(filenm: &String) -> Self {
        return HeaderFile {
            filenm: if filenm.is_empty() { LANG.new_file.clone() } else { filenm.clone() },
            ..HeaderFile::default()
        };
    }
}
