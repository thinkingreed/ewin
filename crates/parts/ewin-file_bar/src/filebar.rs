use crate::filebar_file::*;
use ewin_cfg::log::*;
use ewin_const::{def::*, term::*};
use ewin_key::model::*;
use ewin_state::term::*;
use ewin_utils::str_edit::*;

impl FileBar {
    const ALLOW_BTN_WITH: usize = 2;
    const MENU_BTN_WITH: usize = 3;
    const FILENM_LEN_LIMMIT: usize = 30;
    // Front and back margins of the file
    const FILENM_MARGIN: usize = 3;

    pub fn set_posi(cols_w: usize) {
        if let Ok(mut fbar) = FileBar::get_result() {
            fbar.col_num = cols_w;
            fbar.all_filenm_space_w = fbar.col_num - FileBar::MENU_BTN_WITH;
        }
    }

    pub fn set_filenm() {
        Log::debug_key("FileBar::set_filenm");
        let mut tmp_all_vec: Vec<(usize, String)> = vec![];
        /*
        if FileBar::get().file_vec.is_empty() {
            return;
        }
         */
        if let Ok(mut fbar) = FileBar::get_result() {
            let disp_base_idx = if fbar.disp_base_idx == USIZE_UNDEFINED { 0 } else { fbar.disp_base_idx };

            fbar.init();

            let mut max_len = FileBar::FILENM_LEN_LIMMIT;
            let cols = get_term_size().0;
            Log::debug("cols", &cols);
            let left_allow_len = if fbar.file_vec.len() == 1 { 0 } else { FileBar::ALLOW_BTN_WITH };

            let rest_len = cols - FileBar::MENU_BTN_WITH - 1 - left_allow_len - FileBar::FILENM_MARGIN;
            Log::debug("rest_len", &rest_len);
            if max_len > rest_len {
                max_len = rest_len;
            }

            // Temperatures stored in Vec for ascending / descending sorting
            for (idx, f_file) in fbar.file_vec.iter_mut().enumerate() {
                // cut str
                let state = &State::get().tabs.vec[idx];
                f_file.filenm_disp = if get_str_width(&state.file.name) > max_len { cut_str(&state.file.name, max_len, true, true) } else { state.file.name.clone() };

                let filenm_disp = f_file.filenm_disp.clone();
                f_file.filenm_disp = if state.editor.is_changed { format!("* {} x", filenm_disp) } else { format!(" {} x", filenm_disp) };
                tmp_all_vec.push((idx, f_file.filenm_disp.clone()));
            }

            let mut is_vec_reverse = false;
            if fbar.disp_base_idx == USIZE_UNDEFINED {
                // If the reference position (left end) is undecided, calculate from the right end
                tmp_all_vec.reverse();
                is_vec_reverse = true;

                // Judgment whether to display left arrow
                let mut width = 0;
                for (_, disp_str) in tmp_all_vec.iter() {
                    let disp_str_w = get_str_width(disp_str);
                    if fbar.all_filenm_space_w >= width + disp_str_w {
                        width += disp_str_w;
                    } else {
                        fbar.is_left_arrow_disp = true;
                        break;
                    }
                }
            } else if fbar.disp_base_idx > 0 {
                fbar.is_left_arrow_disp = true;
            }

            let mut disp_vec: Vec<(usize, String)> = vec![];
            let mut width = 0;
            // Judgment of tab to display
            let left_arrow_w = if fbar.is_left_arrow_disp { FileBar::ALLOW_BTN_WITH } else { 0 };
            let mut idx_old = 0;
            let file_len = fbar.file_vec.len();
            for (idx, _) in tmp_all_vec[disp_base_idx..].iter() {
                let h_file = fbar.file_vec.get(*idx).unwrap().clone();
                let right_arrow_w = if fbar.disp_base_idx != USIZE_UNDEFINED && *idx != file_len - 1 { FileBar::ALLOW_BTN_WITH } else { 0 };

                if fbar.all_filenm_space_w - left_arrow_w - right_arrow_w >= width + get_str_width(&h_file.filenm_disp) {
                    fbar.file_vec.get_mut(*idx).unwrap().is_disp = true;

                    width += get_str_width(&h_file.filenm_disp);
                    disp_vec.push((*idx, h_file.filenm_disp.clone()));
                } else {
                    if fbar.disp_base_idx == USIZE_UNDEFINED {
                        fbar.disp_base_idx = idx_old;
                    }
                    break;
                }
                idx_old = *idx;
            }

            if is_vec_reverse {
                // Returns Reverse to calculate the range of each tab
                disp_vec.reverse();
            }

            if disp_vec.last().unwrap().0 != fbar.file_vec.len() - 1 {
                fbar.is_right_arrow_disp = true;
            }

            let mut width = 0;
            for (_, disp_str) in &disp_vec {
                width += get_str_width(disp_str);
            }
            fbar.all_filenm_rest = fbar.all_filenm_space_w - width;

            // Width calc on tab area
            let mut width = if fbar.is_left_arrow_disp { 2 } else { 0 };
            for (idx, filenm) in disp_vec.iter() {
                let s_w = width;

                width += get_str_width(filenm);
                let e_w = width - 1;

                fbar.file_vec[*idx].filenm_area = (s_w, e_w);
                fbar.file_vec[*idx].close_area = (e_w - 1, e_w);
            }

            // Width calc on left_arrow
            if fbar.is_left_arrow_disp {
                fbar.all_filenm_rest -= FileBar::ALLOW_BTN_WITH;
                fbar.left_arrow_area = (0, 1);
            }
            // Width calc on right_arrow
            if fbar.is_right_arrow_disp {
                fbar.all_filenm_rest -= FileBar::ALLOW_BTN_WITH;
                fbar.right_arrow_area = (fbar.all_filenm_space_w - 2, fbar.all_filenm_space_w - 1);
                fbar.all_filenm_rest_area = (fbar.all_filenm_space_w - fbar.all_filenm_rest - FileBar::ALLOW_BTN_WITH, fbar.right_arrow_area.0 - 1);
            } else {
                fbar.all_filenm_rest_area = (fbar.all_filenm_space_w - fbar.all_filenm_rest, fbar.all_filenm_space_w - 1);
            }
        }
    }

    pub fn init(&mut self) {
        self.is_left_arrow_disp = false;
        self.is_right_arrow_disp = false;
        self.left_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.right_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);

        for f_file in self.file_vec.iter_mut() {
            f_file.filenm_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
            f_file.close_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
            f_file.is_disp = false;
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
    pub menu_btn_area: (usize, usize),
    pub is_left_arrow_disp: bool,
    pub is_right_arrow_disp: bool,
    pub right_arrow_area: (usize, usize),
    pub left_arrow_area: (usize, usize),
    // Position on the terminal
    pub row_num: usize,
    pub row_posi: usize,
    pub col_num: usize,
    pub file_vec: Vec<FilebarFile>,
    pub history: History,
    pub state: FileBarState,
}

impl Default for FileBar {
    fn default() -> Self {
        FileBar {
            all_filenm_rest: 0,
            all_filenm_rest_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            all_filenm_space_w: 0,
            disp_base_idx: USIZE_UNDEFINED,
            menu_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            is_left_arrow_disp: false,
            is_right_arrow_disp: false,
            right_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            left_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            row_num: FILEBAR_ROW_NUM,
            row_posi: 1,
            col_num: 0,
            file_vec: vec![],
            history: History::default(),
            state: FileBarState::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FileBarState {
    pub is_dragging: bool,
}

impl FileBarState {
    pub fn clear(&mut self) {
        self.is_dragging = false;
    }
}
