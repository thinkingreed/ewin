use crate::{
    cont::promptcont::*,
    ewin_core::{_cfg::keys::*, colors::*, def::*, global::*, log::*, model::*, util::*},
    prompt::{choice::*, prompt::*},
};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::collections::*;

impl Prompt {
    pub fn menu(&mut self) {
        Log::debug_key("Prompt.menu");

        self.disp_row_num = 11;

        let mut cont_1 = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::First);
        cont_1.set_menu(self);
        self.cont_1 = cont_1;
        let mut cont_2 = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::Second);
        cont_2.set_menu(self);
        self.cont_2 = cont_2;
        let mut cont_3 = PromptCont::new_edit_type(self.keycmd.clone(), PromptContPosi::Third);
        cont_3.set_menu(self);
        self.cont_3 = cont_3;
    }

    pub fn left_down_choice_menu(&mut self, y: u16, x: u16) -> bool {
        let is_menu_select = match y {
            y if self.cont_1.buf_row_posi <= y && y <= self.cont_1.buf_row_posi + self.cont_1.buf_row_len => {
                if self.cont_1.left_down_choice(y, x) {
                    self.cont_posi = PromptContPosi::First;
                    true
                } else {
                    false
                }
            }
            y if self.cont_2.buf_row_posi <= y && y <= self.cont_2.buf_row_posi + self.cont_2.buf_row_len => {
                if self.cont_2.left_down_choice(y, x) {
                    self.cont_posi = PromptContPosi::Second;
                    true
                } else {
                    false
                }
            }
            y if self.cont_3.buf_row_posi <= y && y <= self.cont_3.buf_row_posi + self.cont_3.buf_row_len => {
                if self.cont_3.left_down_choice(y, x) {
                    self.cont_posi = PromptContPosi::Third;
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        if is_menu_select {
            Choices::change_show_choice(self);
        }
        return is_menu_select;
    }
    pub fn cache_menu(&mut self) {
        let map = self.cont_1.choices_map.clone();
        self.prom_menu.choices_map_cache.insert(PromptContPosi::First, map);
        let map = self.cont_2.choices_map.clone();
        self.prom_menu.choices_map_cache.insert(PromptContPosi::Second, map);
        let map = self.cont_3.choices_map.clone();
        self.prom_menu.choices_map_cache.insert(PromptContPosi::Third, map);

        self.prom_menu.cont_posi_cache = self.cont_posi;
    }

    pub fn draw_menu(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_menu");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        self.cont_1.draw_choice_menu(str_vec);
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc.clone());
        self.cont_2.draw_choice_menu(str_vec);
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &self.cont_3.buf_desc.clone());
        self.cont_3.draw_choice_menu(str_vec);
    }

    pub fn draw_cur_menu(&self, str_vec: &mut Vec<String>) {
        match self.cont_posi {
            PromptContPosi::First => self.cont_1.draw_choice_cur(str_vec),
            PromptContPosi::Second => self.cont_2.draw_choice_cur(str_vec),
            PromptContPosi::Third => self.cont_3.draw_choice_cur(str_vec),
            _ => {}
        };
    }

    pub fn change_choice_vec_menu(&mut self, cur_direction: CurDirection) {
        Log::debug_key("Prompt.change_choice_vec_menu");
        match self.cont_posi {
            PromptContPosi::First => {
                if self.cont_1.get_choices().unwrap().set_vec_posi(cur_direction) {
                    if cur_direction == CurDirection::Down {
                        self.cont_posi = PromptContPosi::Second;
                    } else if cur_direction == CurDirection::Up {
                        Log::debug("self.cont_3.choices_map", &self.cont_3.choices_map);
                        self.cont_posi = if self.cont_3.is_show_choices_map() { PromptContPosi::Third } else { PromptContPosi::Second }
                    }
                }
            }
            PromptContPosi::Second => {
                if self.cont_2.get_choices().unwrap().set_vec_posi(cur_direction) {
                    if cur_direction == CurDirection::Down {
                        self.cont_posi = if self.cont_3.is_show_choices_map() { PromptContPosi::Third } else { PromptContPosi::First }
                    } else if cur_direction == CurDirection::Up {
                        self.cont_posi = PromptContPosi::First;
                    }
                }
            }
            PromptContPosi::Third => {
                if self.cont_3.get_choices().unwrap().set_vec_posi(cur_direction) {
                    if cur_direction == CurDirection::Down {
                        self.cont_posi = PromptContPosi::First;
                    } else if cur_direction == CurDirection::Up {
                        self.cont_posi = PromptContPosi::Second;
                    }
                }
            }
            _ => {}
        }
    }
}
impl PromptCont {
    pub fn set_menu(&mut self, prom: &mut Prompt) {
        match self.posi {
            PromptContPosi::First => {
                self.buf_row_len = 2;

                self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.select_menu);
                self.key_desc = format!(
                    "{}{}:{}{}・Click  {}{}:{}{}  {}{}:{}Tab  {}{}:{}↑↓←→",
                    Colors::get_default_fg(),
                    &LANG.fixed,
                    Colors::get_msg_highlight_fg(),
                    Keybind::get_key_str(KeyCmd::ConfirmPrompt),
                    Colors::get_default_fg(),
                    &LANG.close,
                    Colors::get_msg_highlight_fg(),
                    Keybind::get_key_str(KeyCmd::EscPrompt),
                    Colors::get_default_fg(),
                    &LANG.move_setting_location,
                    Colors::get_msg_highlight_fg(),
                    Colors::get_default_fg(),
                    &LANG.candidate_change,
                    Colors::get_msg_highlight_fg(),
                );

                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.menu, Colors::get_default_fg());

                if prom.keycmd == KeyCmd::OpenMenu && prom.prom_menu.choices_map_cache.get(&PromptContPosi::First).is_some() {
                    self.choices_map = prom.prom_menu.choices_map_cache.get(&PromptContPosi::First).unwrap().clone();
                    prom.cont_posi = prom.prom_menu.cont_posi_cache;
                } else {
                    let mut choices = Choices::default();
                    let file = Keybind::get_menu_str(&LANG.file, KeyCmd::OpenMenuFile);
                    let edit = Keybind::get_menu_str(&LANG.edit, KeyCmd::OpenMenuEdit);
                    let search = Keybind::get_menu_str(&LANG.search, KeyCmd::OpenMenuSearch);
                    let macros = Keybind::get_menu_str(&LANG.macros, KeyCmd::OpenMenuMacro);

                    choices.vec = vec![vec![Choice::new(&file), Choice::new(&edit), Choice::new(&search), Choice::new(&macros)]];
                    choices.is_show = true;

                    // UNDEFINED because there are no parents
                    self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (USIZE_UNDEFINED, USIZE_UNDEFINED)), choices);
                    Choices::set_shaping_choice_list(&mut self.choices_map);
                    Choices::set_choice_area(self.buf_row_posi, &mut self.choices_map);
                    self.set_default_choice_menu(USIZE_UNDEFINED, USIZE_UNDEFINED, USIZE_UNDEFINED, USIZE_UNDEFINED);
                }
            }
            PromptContPosi::Second => {
                self.buf_row_len = 2;

                self.buf_desc = format!("{}{} 1{}", Colors::get_msg_highlight_fg(), &LANG.contents, Colors::get_default_fg());

                if prom.keycmd == KeyCmd::OpenMenu && prom.prom_menu.choices_map_cache.get(&PromptContPosi::Second).is_some() {
                    self.choices_map = prom.prom_menu.choices_map_cache.get(&PromptContPosi::Second).unwrap().clone();
                } else {
                    // file
                    let create_new = Keybind::get_menu_str(&LANG.create_new, KeyCmd::NewTab);
                    let open_file = Keybind::get_menu_str(&LANG.open, KeyCmd::OpenFile(OpenFileType::Normal));
                    let encode = Keybind::get_menu_str(&LANG.encode, KeyCmd::Encoding);
                    let vec_1 = vec![Choice::new(&create_new), Choice::new(&open_file), Choice::new(&LANG.save_as)];
                    let vec_2 = vec![Choice::new(&encode), Choice::new(&LANG.end_of_all_save)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1, vec_2];
                    self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 0)), choices);

                    // edit
                    let convert = Keybind::get_menu_str(&LANG.convert, KeyCmd::OpenMenuConvert);
                    let format = &LANG.format;
                    let vec_1 = vec![Choice::new(&convert), Choice::new(&format)];
                    let select = Keybind::get_menu_str(&LANG.box_select, KeyCmd::BoxSelectMode);
                    let vec_2 = vec![Choice::new(&select)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1, vec_2];
                    self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 1)), choices);
                    // search
                    let move_row = Keybind::get_menu_str(&LANG.move_row, KeyCmd::MoveRow);
                    let vec_1 = vec![Choice::new(&move_row)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1];
                    self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 2)), choices);
                    // macros
                    let vec_1 = vec![Choice::new(&LANG.specify_file_and_exec_macro)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1];
                    self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 3)), choices);

                    Choices::set_shaping_choice_list(&mut self.choices_map);
                    Choices::set_choice_area(self.buf_row_posi, &mut self.choices_map);
                    let (y, x) = Choices::get_y_x(&prom.cont_1);
                    self.set_default_choice_menu(USIZE_UNDEFINED, USIZE_UNDEFINED, y, x);
                }
            }
            PromptContPosi::Third => {
                self.buf_row_len = 2;

                self.buf_desc = format!("{}{} 2{}", Colors::get_msg_highlight_fg(), &LANG.contents, Colors::get_default_fg());

                if prom.keycmd == KeyCmd::OpenMenu && prom.prom_menu.choices_map_cache.get(&PromptContPosi::Third).is_some() {
                    self.choices_map = prom.prom_menu.choices_map_cache.get(&PromptContPosi::Third).unwrap().clone();
                } else {
                    let vec_1 = vec![Choice::new(&LANG.to_lowercase), Choice::new(&LANG.to_half_width), Choice::new(&LANG.to_space)];
                    let vec_2 = vec![Choice::new(&LANG.to_uppercase), Choice::new(&LANG.to_full_width), Choice::new(&LANG.to_tab)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1, vec_2];
                    self.choices_map.insert(((0, 1), (0, 0)), choices);

                    let format_json = Keybind::get_menu_str(&LANG.json, KeyCmd::Format(FmtType::JSON));
                    let format_xml = Keybind::get_menu_str(&LANG.xml, KeyCmd::Format(FmtType::XML));
                    let vec_1 = vec![Choice::new(&format_json), Choice::new(&format_xml)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1];
                    self.choices_map.insert(((0, 1), (0, 1)), choices);

                    Choices::set_shaping_choice_list(&mut self.choices_map);
                    Choices::set_choice_area(self.buf_row_posi, &mut self.choices_map);
                    let (first_y, first_x) = Choices::get_y_x(&prom.cont_1);
                    let (second_y, second_x) = Choices::get_y_x(&prom.cont_2);
                    self.set_default_choice_menu(first_y, first_x, second_y, second_x);
                }
            }
            _ => {}
        };
    }

    pub fn draw_choice_menu(&self, str_vec: &mut Vec<String>) {
        for i in self.buf_row_posi..self.buf_row_posi + self.buf_row_len {
            str_vec.push(format!("{}{}", MoveTo(0, i as u16), Clear(CurrentLine)));
        }

        for (_, choices) in self.choices_map.iter() {
            if choices.is_show {
                for (y_idx, vec) in choices.vec.iter().enumerate() {
                    let mut row_width = 1;
                    for (x_idx, item) in vec.iter().enumerate() {
                        let item_str = if choices.is_show && choices.vec_y == y_idx && choices.vec_x == x_idx { format!("{}{}{}", Colors::get_msg_warning_inversion_fg_bg(), item.disp_name, Colors::get_hbar_fg_bg()) } else { format!("{}{}", Colors::get_hbar_fg_bg(), item.disp_name) };
                        str_vec.push(format!("{}{}", MoveTo(row_width, self.buf_row_posi + y_idx as u16), &item_str));

                        row_width += (get_str_width(&item.disp_name) + Choices::ITEM_MARGIN) as u16;
                    }
                }
            }
        }
    }

    fn set_default_choice_menu(&mut self, first_y: usize, first_x: usize, second_y: usize, second_x: usize) {
        Log::debug_key("set_default_choice_menu");
        Log::debug("self.keycmds", &self.keycmd);
        Log::debug("parent_vec_y", &first_y);
        Log::debug("parent_vec_x", &first_x);

        for (((grandparentst_y, grandparentst_x), (parent_y, parent_x)), choices) in self.choices_map.iter_mut() {
            for (y_idx, v) in choices.vec.iter_mut().enumerate() {
                for (x_idx, choice) in v.iter_mut().enumerate() {
                    match self.posi {
                        PromptContPosi::First => match self.keycmd {
                            KeyCmd::OpenMenuFile => {
                                if choice.name.contains(&LANG.file) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            }
                            KeyCmd::OpenMenuEdit => {
                                if choice.name.contains(&LANG.edit) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            }
                            KeyCmd::OpenMenuSearch => {
                                if choice.name.contains(&LANG.search) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            }
                            _ => {}
                        },
                        PromptContPosi::Second => {
                            choices.is_show = if parent_y == &second_y && parent_x == &second_x { true } else { false };
                        }
                        PromptContPosi::Third => {
                            choices.is_show = if grandparentst_y == &first_y && grandparentst_x == &first_x && parent_y == &second_y && parent_x == &second_y { true } else { false };
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    pub fn is_show_choices_map(&mut self) -> bool {
        for (_, choices) in self.choices_map.iter() {
            if choices.is_show {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct PromMenu {
    pub choices_map_cache: BTreeMap<PromptContPosi, HashMap<((usize, usize), (usize, usize)), Choices>>,
    pub cont_posi_cache: PromptContPosi,
}

impl Default for PromMenu {
    fn default() -> Self {
        PromMenu { choices_map_cache: BTreeMap::new(), cont_posi_cache: PromptContPosi::First }
    }
}
