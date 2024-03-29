use crate::menulist::*;
use ewin_cfg::{global::*, lang::lang_cfg::*, log::*};
use ewin_const::def::*;
use ewin_const::term::*;
use ewin_key::key::keys::Keys;
use ewin_key::key_traits::key_trait::*;
use ewin_utils::str_edit::*;
use ewin_utils::util::*;
use ewin_view::view::*;
use std::ops::Range;

impl MenuBar {
    const ALLOW_BTN_WITH: usize = 2;
    const CLOSE_BTN_WITH: usize = 3;

    pub fn set_posi(&mut self, cols_w: usize) {
        self.view.y = 0;
        self.view.width = cols_w;
        self.menu_space_w = self.view.width - MenuBar::CLOSE_BTN_WITH;
        // +1 is space between
        self.close_btn_area = (self.menu_space_w, self.menu_space_w + MenuBar::CLOSE_BTN_WITH - 1);
    }

    pub fn set_menunm(&mut self) {
        let mut tmp_all_vec: Vec<(usize, String)> = vec![];
        if self.menu_vec.is_empty() {
            return;
        }
        let disp_base_idx = if self.disp_base_idx == USIZE_UNDEFINED { 0 } else { self.disp_base_idx };

        let cols = get_term_size().0;
        Log::debug("cols", &cols);
        let left_allow_len = if self.menu_vec.len() == 1 { 0 } else { MenuBar::ALLOW_BTN_WITH };

        Log::debug("self.menu_vec.len()", &self.menu_vec.len());

        // Temperatures stored in Vec for ascending / descending sorting
        for (idx, menu_cont) in self.menu_vec.iter_mut().enumerate() {
            tmp_all_vec.push((idx, menu_cont.menunm.clone()));
        }

        let mut is_vec_reverse = false;
        if self.disp_base_idx == USIZE_UNDEFINED {
            // If the reference position (left end) is undecided, calculate from the right end
            tmp_all_vec.reverse();
            is_vec_reverse = true;

            // Judgment whether to display left arrow
            let mut width = 0;
            for (_, disp_str) in tmp_all_vec.iter() {
                let disp_str_w = get_str_width(disp_str);
                if self.menu_space_w >= width + disp_str_w {
                    width += disp_str_w;
                } else {
                    self.is_left_arrow_disp = true;
                    break;
                }
            }
        } else if self.disp_base_idx > 0 {
            self.is_left_arrow_disp = true;
        }

        let mut disp_vec: Vec<(usize, String)> = vec![];
        let mut width = 0;
        // Judgment of tab to display
        let left_arrow_w = if self.is_left_arrow_disp { MenuBar::ALLOW_BTN_WITH } else { 0 };
        let mut idx_old = 0;
        let file_len = self.menu_vec.len();
        for (idx, _) in tmp_all_vec[disp_base_idx..].iter() {
            let menu_cont = self.menu_vec.get_mut(*idx).unwrap();
            let right_arrow_w = if self.disp_base_idx != USIZE_UNDEFINED && *idx != file_len - 1 { MenuBar::ALLOW_BTN_WITH } else { 0 };

            if self.menu_space_w - left_arrow_w - right_arrow_w >= width + get_str_width(&menu_cont.menunm) {
                menu_cont.is_disp = true;

                width += get_str_width(&menu_cont.dispnm);
                disp_vec.push((*idx, menu_cont.dispnm.clone()));
            } else {
                if self.disp_base_idx == USIZE_UNDEFINED {
                    self.disp_base_idx = idx_old;
                }
                break;
            }
            idx_old = *idx;
        }

        if is_vec_reverse {
            // Returns Reverse to calculate the range of each tab
            disp_vec.reverse();
        }

        if disp_vec.last().unwrap().0 != self.menu_vec.len() - 1 {
            self.is_right_arrow_disp = true;
        }

        let mut width = 0;
        for (_, disp_str) in &disp_vec {
            width += get_str_width(disp_str);
        }
        self.menu_rest = self.menu_space_w - width;

        // Width calc on left_arrow
        if self.is_left_arrow_disp {
            self.menu_rest -= MenuBar::ALLOW_BTN_WITH;
            self.left_arrow_area = (0, 1);
        }
        // Width calc on right_arrow
        if self.is_right_arrow_disp {
            self.menu_rest -= MenuBar::ALLOW_BTN_WITH;
            self.right_arrow_area = (self.menu_space_w - 2, self.menu_space_w - 1);
            self.menu_rest_area = (self.menu_space_w - self.menu_rest - MenuBar::ALLOW_BTN_WITH, self.right_arrow_area.0 - 1);
        } else {
            self.menu_rest_area = (self.menu_space_w - self.menu_rest, self.menu_space_w - 1);
        }
    }

    pub fn init(&mut self) {
        Log::debug_key("MenuBar.init");

        self.set_posi(get_term_size().0);
        self.set_menunm();

        self.is_left_arrow_disp = false;
        self.is_right_arrow_disp = false;
        self.left_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.right_arrow_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);

        self.menulist.init();
        let mut tmp_len = 0;

        self.menulist.menu_map.iter().for_each(|(menunm_str, _)| {
            let dispnm = format!(" {}", get_cfg_lang_name(menunm_str));

            let name_len = get_str_width(&dispnm);
            let range = Range { start: tmp_len, end: tmp_len + name_len };

            let is_always_reset_name = LANG_MAP[menunm_str] == Lang::get().display || LANG_MAP[menunm_str] == Lang::get().window;
            self.menu_vec.push(MenubarCont::new(menunm_str, &dispnm, range, is_always_reset_name));
            tmp_len += name_len;
        });

        Log::debug("self.menu_vec", &self.menu_vec);
        self.set_menunm();
    }

    pub fn is_check_clear(&mut self, keys: Keys) {
        if self.menulist.is_show && !self.is_allow_key(keys) {
            self.clear_menulist_all();
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuBar {
    pub menu_rest: usize,
    pub menu_rest_area: (usize, usize),
    pub menu_space_w: usize,
    pub menu_vec: Vec<MenubarCont>,
    pub disp_base_idx: usize,
    // Select idx
    pub sel_idx: usize,
    pub on_mouse_idx: usize,
    pub on_mouse_idx_org: usize,
    pub close_btn_area: (usize, usize),
    pub is_left_arrow_disp: bool,
    pub is_right_arrow_disp: bool,
    pub right_arrow_area: (usize, usize),
    pub left_arrow_area: (usize, usize),
    pub view: View,
    pub menulist: MenubarMenuList,
}

impl Default for MenuBar {
    fn default() -> Self {
        MenuBar {
            menu_rest: 0,
            menu_rest_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            menu_space_w: 0,
            sel_idx: USIZE_UNDEFINED,
            on_mouse_idx: USIZE_UNDEFINED,
            on_mouse_idx_org: USIZE_UNDEFINED,
            menu_vec: vec![],
            disp_base_idx: USIZE_UNDEFINED,
            close_btn_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            is_left_arrow_disp: false,
            is_right_arrow_disp: false,
            right_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            left_arrow_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            view: View::default(),
            menulist: MenubarMenuList::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenubarCont {
    pub is_disp: bool,
    // Since the display content changes depending on the setting,
    // reset the value every time.
    pub is_always_reset_name: bool,
    pub menunm: String,
    pub dispnm: String,
    pub area: Range<usize>,
}

impl Default for MenubarCont {
    fn default() -> Self {
        Self { is_disp: true, menunm: String::new(), dispnm: String::new(), area: Range::default(), is_always_reset_name: false }
    }
}

impl MenubarCont {
    pub fn new(menunm: &str, dispnm: &str, area: Range<usize>, is_always_reset_name: bool) -> Self {
        MenubarCont { menunm: menunm.to_string(), dispnm: dispnm.to_string(), is_disp: true, area, is_always_reset_name }
    }
}
