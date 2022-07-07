use std::io::Write;

use crate::{
    ewin_com::{model::*, util::*},
    global_term::H_FILE_VEC,
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*};
use ewin_const::def::*;
use std::fmt::Write as _;

impl FileBar {
    const ALLOW_BTN_WITH: usize = 2;
    const MENU_BTN_WITH: usize = 3;
    const FILENM_LEN_LIMMIT: usize = 30;
    // Front and back margins of the file
    const FILENM_MARGIN: usize = 3;

    pub fn draw(term: &Terminal, str_vec: &mut Vec<String>) {
        Log::info_key("HeaderBar.draw");

        let menu_btn = format!(" {} ", "⠇");
        let left_arrow_btn = "< ".to_string();
        let right_arrow_btn = " >".to_string();

        let mut hber_str = format!("{}{}{}", MoveTo(0, term.fbar.row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_default_fg_bg());
        if term.fbar.is_left_arrow_disp {
            let _ = write!(hber_str, "{}{}{}", &Colors::get_hbar_active_fg_bg(), left_arrow_btn, &Colors::get_default_fg_bg());
        }
        for (i, h_file) in H_FILE_VEC.get().unwrap().try_lock().unwrap().iter().enumerate() {
            if !h_file.is_disp {
                continue;
            }
            let state_color = if i == term.tab_idx { Colors::get_hbar_active_fg_bg() } else { Colors::get_hbar_passive_fg_bg() };
            hber_str.push_str(&format!("{}{}{}", &state_color, &h_file.filenm_disp.clone(), &Colors::get_default_fg_bg()));
        }
        hber_str.push_str(&format!("{}{}", &Colors::get_hbar_default_bg(), &" ".repeat(term.fbar.all_filenm_rest)));
        if term.fbar.is_right_arrow_disp {
            hber_str.push_str(&right_arrow_btn);
        }
        hber_str = format!("{}{}{}", hber_str, menu_btn, Colors::get_default_bg(),);
        str_vec.push(hber_str);
    }

    pub fn draw_only<T: Write>(term: &Terminal, out: &mut T) {
        Log::debug_key("HeaderBar::draw_only");
        let mut v: Vec<String> = vec![];
        FileBar::draw(term, &mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn set_posi(&mut self, cols_w: usize) {
        self.col_num = cols_w;
        self.all_filenm_space_w = self.col_num - FileBar::MENU_BTN_WITH;
    }

    pub fn set_filenm(term: &mut Terminal) {
        let mut tmp_all_vec: Vec<(usize, String)> = vec![];
        if H_FILE_VEC.get().unwrap().try_lock().unwrap().len() == 0 {
            return;
        }
        let disp_base_idx = if term.fbar.disp_base_idx == USIZE_UNDEFINED { 0 } else { term.fbar.disp_base_idx };

        term.fbar.init();

        let mut max_len = FileBar::FILENM_LEN_LIMMIT;
        let cols = get_term_size().0;
        Log::debug("cols", &cols);
        let left_allow_len = if H_FILE_VEC.get().unwrap().try_lock().unwrap().len() == 1 { 0 } else { FileBar::ALLOW_BTN_WITH };

        Log::debug("hbar.file_vec.len()", &H_FILE_VEC.get().unwrap().try_lock().unwrap().len());

        let rest_len = cols - FileBar::MENU_BTN_WITH - 1 - left_allow_len - FileBar::FILENM_MARGIN;
        Log::debug("rest_len", &rest_len);
        if max_len > rest_len {
            max_len = rest_len;
        }

        // Temperatures stored in Vec for ascending / descending sorting
        for (idx, h_file) in H_FILE_VEC.get().unwrap().try_lock().unwrap().iter_mut().enumerate() {
            // cut str
            h_file.filenm_disp = if get_str_width(&h_file.filenm) > max_len { cut_str(&h_file.filenm, max_len, true, true) } else { h_file.filenm.clone() };

            let filenm_disp = h_file.filenm_disp.clone();
            h_file.filenm_disp = if term.tabs[idx].editor.state.is_changed { format!("* {} x", filenm_disp) } else { format!(" {} x", filenm_disp) };
            tmp_all_vec.push((idx, h_file.filenm_disp.clone()));
        }

        let mut is_vec_reverse = false;
        if term.fbar.disp_base_idx == USIZE_UNDEFINED {
            // If the reference position (left end) is undecided, calculate from the right end
            tmp_all_vec.reverse();
            is_vec_reverse = true;

            // Judgment whether to display left arrow
            let mut width = 0;
            for (_, disp_str) in tmp_all_vec.iter() {
                let disp_str_w = get_str_width(disp_str);
                if term.fbar.all_filenm_space_w >= width + disp_str_w {
                    width += disp_str_w;
                } else {
                    term.fbar.is_left_arrow_disp = true;
                    break;
                }
            }
        } else if term.fbar.disp_base_idx > 0 {
            term.fbar.is_left_arrow_disp = true;
        }

        let mut disp_vec: Vec<(usize, String)> = vec![];
        let mut width = 0;
        // Judgment of tab to display
        let left_arrow_w = if term.fbar.is_left_arrow_disp { FileBar::ALLOW_BTN_WITH } else { 0 };
        let mut idx_old = 0;
        let file_len = H_FILE_VEC.get().unwrap().try_lock().unwrap().len();
        for (idx, _) in tmp_all_vec[disp_base_idx..].iter() {
            let mut vec = H_FILE_VEC.get().unwrap().try_lock().unwrap();
            let h_file = vec.get_mut(*idx).unwrap();
            let right_arrow_w = if term.fbar.disp_base_idx != USIZE_UNDEFINED && *idx != file_len - 1 { FileBar::ALLOW_BTN_WITH } else { 0 };

            if term.fbar.all_filenm_space_w - left_arrow_w - right_arrow_w >= width + get_str_width(&h_file.filenm_disp) {
                h_file.is_disp = true;

                width += get_str_width(&h_file.filenm_disp);
                disp_vec.push((*idx, h_file.filenm_disp.clone()));
            } else {
                if term.fbar.disp_base_idx == USIZE_UNDEFINED {
                    term.fbar.disp_base_idx = idx_old;
                }
                break;
            }
            idx_old = *idx;
        }

        if is_vec_reverse {
            // Returns Reverse to calculate the range of each tab
            disp_vec.reverse();
        }

        if disp_vec.last().unwrap().0 != H_FILE_VEC.get().unwrap().try_lock().unwrap().len() - 1 {
            term.fbar.is_right_arrow_disp = true;
        }

        let mut width = 0;
        for (_, disp_str) in &disp_vec {
            width += get_str_width(disp_str);
        }
        term.fbar.all_filenm_rest = term.fbar.all_filenm_space_w - width;

        // Width calc on tab area
        let mut width = if term.fbar.is_left_arrow_disp { 2 } else { 0 };
        for (idx, filenm) in disp_vec.iter() {
            let s_w = width;

            width += get_str_width(filenm);
            let e_w = width - 1;
            H_FILE_VEC.get().unwrap().try_lock().unwrap().get_mut(*idx).unwrap().filenm_area = (s_w, e_w);
            H_FILE_VEC.get().unwrap().try_lock().unwrap().get_mut(*idx).unwrap().close_area = (e_w - 1, e_w);
        }

        // Width calc on left_arrow
        if term.fbar.is_left_arrow_disp {
            term.fbar.all_filenm_rest -= FileBar::ALLOW_BTN_WITH;
            term.fbar.left_arrow_area = (0, 1);
        }
        // Width calc on right_arrow
        if term.fbar.is_right_arrow_disp {
            term.fbar.all_filenm_rest -= FileBar::ALLOW_BTN_WITH;
            term.fbar.right_arrow_area = (term.fbar.all_filenm_space_w - 2, term.fbar.all_filenm_space_w - 1);
            term.fbar.all_filenm_rest_area = (term.fbar.all_filenm_space_w - term.fbar.all_filenm_rest - FileBar::ALLOW_BTN_WITH, term.fbar.right_arrow_area.0 - 1);
        } else {
            term.fbar.all_filenm_rest_area = (term.fbar.all_filenm_space_w - term.fbar.all_filenm_rest, term.fbar.all_filenm_space_w - 1);
        }
    }

    pub fn init(&mut self) {
        self.is_left_arrow_disp = false;
        self.is_right_arrow_disp = false;
        self.left_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.right_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);

        for h_file in H_FILE_VEC.get().unwrap().try_lock().unwrap().iter_mut() {
            h_file.filenm_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
            h_file.close_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
            h_file.is_disp = false;
        }
    }

    pub fn new() -> Self {
        FileBar { ..FileBar::default() }
    }
}

#[derive(Debug, Clone)]
pub struct FileBar {
    pub all_filenm_rest: usize,
    pub all_filenm_rest_area: (usize, usize),
    pub all_filenm_space_w: usize,
    pub disp_base_idx: usize,
    // pub file_vec: Vec<HeaderFile>,
    pub menu_btn_area: (usize, usize),
    pub close_btn_area: (usize, usize),
    pub is_left_arrow_disp: bool,
    pub is_right_arrow_disp: bool,
    pub right_arrow_area: (usize, usize),
    pub left_arrow_area: (usize, usize),
    // Position on the terminal
    pub row_num: usize,
    pub row_posi: usize,
    pub col_num: usize,
    pub history: History,
    pub state: HeaderBarState,
}

impl Default for FileBar {
    fn default() -> Self {
        FileBar {
            all_filenm_rest: 0,
            all_filenm_rest_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            all_filenm_space_w: 0,
            disp_base_idx: USIZE_UNDEFINED,
            //  file_vec: vec![],
            menu_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            close_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            is_left_arrow_disp: false,
            is_right_arrow_disp: false,
            right_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            left_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            row_num: FILEBAR_ROW_NUM,
            row_posi: 1,
            col_num: 0,
            history: History::default(),
            state: HeaderBarState::default(),
        }
    }
}
