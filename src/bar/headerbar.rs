use crate::{colors::*, def::*, global::*, log::*, model::*, terminal::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::{io::Write, path::Path};

impl HeaderBar {
    const ALLOW_BTN_WITH: usize = 2;
    const PLUS_BTN_WITH: usize = 3;
    const MENU_BTN_WITH: usize = 3;
    const CLOSE_BTN_WITH: usize = 3;
    const FILENM_LEN_LIMMIT: usize = 30;
    // Front and back margins of the file
    const FILENM_MARGIN: usize = 3;

    pub fn draw<T: Write>(out: &mut T, term: &Terminal) {
        Log::info_key("HeaderBar.draw");

        let plus_btn = format!(" {} ", '+');
        //  let menu_btn = format!(" {} ", "≡");
        let menu_btn = format!(" {} ", "⠇");
        let close_btn = format!(" {} ", 'x');
        let left_arrow_btn = "< ".to_string();
        let right_arrow_btn = " >".to_string();

        let mut hber_str = format!("{}{}{}", MoveTo(0, term.hbar.disp_row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_hbar_fg_bg());
        if term.hbar.is_left_arrow_disp {
            hber_str.push_str(&format!("{}{}{}", &Colors::get_hbar_inversion_fg_bg_active(), left_arrow_btn, &Colors::get_hbar_fg_bg()));
        }
        for (i, h_file) in term.hbar.file_vec.iter().enumerate() {
            if !h_file.is_disp {
                continue;
            }
            let state_color = if i == term.idx { Colors::get_hbar_inversion_fg_bg_active() } else { Colors::get_hbar_inversion_fg_bg_passive() };
            hber_str.push_str(&format!("{}{}{}", &state_color, &h_file.filenm_disp.clone(), &Colors::get_hbar_fg_bg()));
        }
        hber_str.push_str(&format!("{}{}", &Colors::get_default_bg(), &" ".repeat(term.hbar.all_filenm_rest)));
        if term.hbar.is_right_arrow_disp {
            hber_str.push_str(&format!("{}{}{}", &Colors::get_hbar_inversion_fg_bg_active(), right_arrow_btn, &Colors::get_hbar_fg_bg()));
        }
        hber_str = format!("{}{}{}{} {}{}{} {}{}{}", hber_str, Colors::get_hbar_inversion_fg_bg_active(), plus_btn, Colors::get_default_bg(), Colors::get_hbar_inversion_fg_bg_active(), menu_btn, Colors::get_default_bg(), Colors::get_hbar_inversion_fg_bg_active(), close_btn, Colors::get_default_bg());

        let _ = out.write(&hber_str.as_bytes());
        out.flush().unwrap();
    }

    pub fn set_posi(&mut self, cols_w: usize) {
        self.disp_col_num = cols_w;
        self.all_filenm_space_w = self.disp_col_num - HeaderBar::PLUS_BTN_WITH - 1 - HeaderBar::MENU_BTN_WITH - 1 - HeaderBar::CLOSE_BTN_WITH;
        // +1 is space between
        self.plus_btn_area = (self.all_filenm_space_w, self.all_filenm_space_w + HeaderBar::PLUS_BTN_WITH - 1);
        self.menu_btn_area = (self.plus_btn_area.1 + 2, self.plus_btn_area.1 + 2 + HeaderBar::MENU_BTN_WITH - 1);
        self.close_btn_area = (self.menu_btn_area.1 + 2, self.menu_btn_area.1 + 2 + HeaderBar::CLOSE_BTN_WITH - 1);
    }

    pub fn set_header_filenm(hbar: &mut HeaderBar) {
        let mut tmp_all_vec: Vec<(usize, String)> = vec![];
        if hbar.file_vec.len() == 0 {
            return;
        }
        let disp_base_idx = if hbar.disp_base_idx == USIZE_UNDEFINED { 0 } else { hbar.disp_base_idx };

        hbar.init();

        let mut max_len = HeaderBar::FILENM_LEN_LIMMIT;
        let cols = size().unwrap().0 as usize;
        Log::debug("cols", &cols);
        let left_allow_len = if hbar.file_vec.len() == 1 { 0 } else { HeaderBar::ALLOW_BTN_WITH };

        Log::debug("hbar.file_vec.len()", &hbar.file_vec.len());

        let rest_len = cols - HeaderBar::PLUS_BTN_WITH - 1 - HeaderBar::MENU_BTN_WITH - 1 - HeaderBar::CLOSE_BTN_WITH - left_allow_len - HeaderBar::FILENM_MARGIN;
        Log::debug("rest_len", &rest_len);
        if max_len > rest_len {
            max_len = rest_len;
        }

        // Temperatures stored in Vec for ascending / descending sorting
        for (idx, h_file) in hbar.file_vec.iter_mut().enumerate() {
            // cut str
            h_file.filenm_disp = if get_str_width(&h_file.filenm) > max_len { cut_str(h_file.filenm.clone(), max_len, true, true) } else { h_file.filenm.clone() };

            let filenm_disp = h_file.filenm_disp.clone();
            h_file.filenm_disp = if h_file.is_changed { format!("* {} x", filenm_disp) } else { format!(" {} x", filenm_disp) };
            tmp_all_vec.push((idx, h_file.filenm_disp.clone()));
        }

        let mut is_vec_reverse = false;
        if hbar.disp_base_idx == USIZE_UNDEFINED {
            // If the reference position (left end) is undecided, calculate from the right end
            tmp_all_vec.reverse();
            is_vec_reverse = true;

            // Judgment whether to display left arrow
            let mut width = 0;
            for (_, disp_str) in tmp_all_vec.iter() {
                let disp_str_w = get_str_width(&disp_str);
                if hbar.all_filenm_space_w >= width + disp_str_w {
                    width += disp_str_w;
                } else {
                    hbar.is_left_arrow_disp = true;
                    break;
                }
            }
        } else {
            if hbar.disp_base_idx > 0 {
                hbar.is_left_arrow_disp = true;
            }
        }

        let mut disp_vec: Vec<(usize, String)> = vec![];
        let mut width = 0;
        // Judgment of tab to display
        let left_arrow_w = if hbar.is_left_arrow_disp { HeaderBar::ALLOW_BTN_WITH } else { 0 };
        let mut idx_old = 0;
        let file_len = hbar.file_vec.len();
        for (idx, _) in tmp_all_vec[disp_base_idx..].iter() {
            let h_file = hbar.file_vec.get_mut(*idx).unwrap();
            let right_arrow_w = if hbar.disp_base_idx != USIZE_UNDEFINED && *idx != file_len - 1 { HeaderBar::ALLOW_BTN_WITH } else { 0 };

            if hbar.all_filenm_space_w - left_arrow_w - right_arrow_w >= width + get_str_width(&h_file.filenm_disp) {
                h_file.is_disp = true;

                width += get_str_width(&h_file.filenm_disp);
                disp_vec.push((*idx, h_file.filenm_disp.clone()));
            } else {
                if hbar.disp_base_idx == USIZE_UNDEFINED {
                    hbar.disp_base_idx = idx_old;
                }
                break;
            }
            idx_old = *idx;
        }

        if is_vec_reverse {
            // Returns Reverse to calculate the range of each tab
            disp_vec.reverse();
        }

        if disp_vec.last().unwrap().0 != hbar.file_vec.len() - 1 {
            hbar.is_right_arrow_disp = true;
        }

        let mut width = 0;
        for (_, disp_str) in &disp_vec {
            width += get_str_width(&disp_str);
        }
        hbar.all_filenm_rest = hbar.all_filenm_space_w - width;

        // Width calc on tab area
        let mut width = if hbar.is_left_arrow_disp { 2 } else { 0 };
        for (idx, filenm) in disp_vec.iter() {
            let s_w = width;

            width += get_str_width(&filenm);
            let e_w = width - 1;
            hbar.file_vec.get_mut(*idx).unwrap().filenm_area = (s_w, e_w);
            hbar.file_vec.get_mut(*idx).unwrap().close_area = (e_w - 1, e_w);
        }

        // Width calc on left_arrow
        if hbar.is_left_arrow_disp {
            hbar.all_filenm_rest -= HeaderBar::ALLOW_BTN_WITH;
            hbar.left_arrow_area = (0, 1);
        }
        // Width calc on right_arrow
        if hbar.is_right_arrow_disp {
            hbar.all_filenm_rest -= HeaderBar::ALLOW_BTN_WITH;
            hbar.right_arrow_area = (hbar.all_filenm_space_w - 2, hbar.all_filenm_space_w - 1);
            hbar.all_filenm_rest_area = (hbar.all_filenm_space_w - hbar.all_filenm_rest - HeaderBar::ALLOW_BTN_WITH, hbar.right_arrow_area.0 - 1);
        } else {
            hbar.all_filenm_rest_area = (hbar.all_filenm_space_w - hbar.all_filenm_rest, hbar.all_filenm_space_w - 1);
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
    pub menu_btn_area: (usize, usize),
    pub close_btn_area: (usize, usize),
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
            menu_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            close_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
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
    pub fullpath: String,
    pub is_disp: bool,
    pub is_changed: bool,
    pub filenm_area: (usize, usize),
    pub close_area: (usize, usize),
    pub enc: Encode,
    // new line
    pub nl: String,
    pub nl_org: String,
    pub bom: Option<Encode>,
}

impl Default for HeaderFile {
    fn default() -> Self {
        HeaderFile {
            filenm: String::new(),
            filenm_disp: String::new(),
            fullpath: String::new(),
            //  ext: String::new(),
            is_disp: false,
            is_changed: false,
            filenm_area: (0, 0),
            close_area: (0, 0),
            enc: Encode::UTF8,
            nl: NEW_LINE_LF_STR.to_string(),
            nl_org: NEW_LINE_LF_STR.to_string(),
            bom: None,
        }
    }
}

impl HeaderFile {
    pub fn new(filenm: &String) -> Self {
        let path = Path::new(&filenm);
        let setting_filenm;
        let file_fullpath;

        if path.is_absolute() {
            setting_filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string().clone();
            file_fullpath = filenm.clone();
        } else {
            setting_filenm = filenm.clone();
            file_fullpath = Path::new(&*CURT_DIR).join(&filenm).to_string_lossy().to_string();
        }

        return HeaderFile { filenm: if filenm.is_empty() { LANG.new_file.clone() } else { Path::new(&setting_filenm).file_name().unwrap().to_string_lossy().to_string().clone() }, fullpath: file_fullpath, ..HeaderFile::default() };
    }
}
