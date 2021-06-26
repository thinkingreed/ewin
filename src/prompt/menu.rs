use crate::_cfg::keys::{KeyCmd, Keybind};
use crate::{colors::*, def::*, global::*, log::*, model::*, prompt::choice::*, prompt::cont::promptcont::*, prompt::prompt::prompt::*, terminal::*, util::*};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::collections::{BTreeMap, HashMap};
use std::io::stdout;

impl EvtAct {
    pub fn menu(term: &mut Terminal) -> EvtActType {
        let state = term.curt().state.clone();

        match term.curt().editor.keycmd {
            KeyCmd::MouseDownLeft(y, x) => {
                term.curt().prom.left_down_choice_menu(y as u16, x as u16);
                return EvtActType::Hold;
            }
            KeyCmd::BackTab => {
                term.curt().prom.tab(false, &state);
                return EvtActType::Hold;
            }
            KeyCmd::CursorUp => {
                term.curt().prom.change_choice_vec_menu(CurDirection::Up);
                return EvtActType::Hold;
            }
            KeyCmd::CursorDown => {
                term.curt().prom.change_choice_vec_menu(CurDirection::Down);
                return EvtActType::Hold;
            }
            KeyCmd::Tab => {
                term.curt().prom.tab(true, &state);
                return EvtActType::Hold;
            }
            KeyCmd::CursorLeft | KeyCmd::CursorRight => {
                if term.curt().editor.keycmd == KeyCmd::CursorRight {
                    term.curt().prom.change_choice_vec_menu(CurDirection::Right);
                } else {
                    term.curt().prom.change_choice_vec_menu(CurDirection::Left);
                }

                match term.curt().prom.prom_cont_posi {
                    PromptContPosi::First => {
                        let (y, x) = Choices::get_y_x(&term.curt().prom.cont_1);
                        Choices::set_show_choice(y, x, &mut term.curt().prom.cont_2.choices_map);
                    }
                    _ => {}
                };
                return EvtActType::Hold;
            }
            KeyCmd::InsertLine => {
                let map = term.curt().prom.cont_1.choices_map.clone();
                term.curt().prom.prom_menu.choices_map_cache.insert(PromptContPosi::First, map);
                let map = term.curt().prom.cont_2.choices_map.clone();
                term.curt().prom.prom_menu.choices_map_cache.insert(PromptContPosi::Second, map);
                term.curt().prom.prom_menu.prompt_cont_posi_cache = term.curt().prom.prom_cont_posi;

                let choice_1 = term.curt().prom.cont_1.get_choice();
                let choice_2 = term.curt().prom.cont_2.get_choice();

                // file
                if choice_1.name.contains(&LANG.file) {
                    term.clear_curt_tab();
                    if choice_2.name.contains(&LANG.encode) {
                        EvtAct::match_event(Keybind::get_keys(KeyCmd::Encoding), &mut stdout(), term);
                    } else if choice_2.name.contains(&LANG.create_new) {
                        EvtAct::match_event(Keybind::get_keys(KeyCmd::NewTab), &mut stdout(), term);
                    } else if choice_2.name.contains(&LANG.open) {
                        EvtAct::match_event(Keybind::get_keys(KeyCmd::OpenFile), &mut stdout(), term);
                    } else if choice_2.name.contains(&LANG.save_as) {
                        Prompt::save_new_file(term);
                    } else if choice_2.name.contains(&LANG.end_of_all_save) {
                        if term.save_all_tab() {
                            return EvtActType::Exit;
                        }
                    }
                    // convert
                } else if choice_1.name.contains(&LANG.convert) {
                    if term.curt().editor.sel.is_selected() {
                        term.curt().editor.convert(&choice_2.name);

                        term.clear_curt_tab();
                    } else {
                        term.curt().mbar.set_err(&LANG.no_sel_range)
                    }
                }
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn menu(term: &mut Terminal) {
        Log::debug_key("Prompt.menu");

        term.curt().state.is_menu = true;
        term.curt().prom.disp_row_num = 8;
        let is_disp = term.set_disp_size();
        if !is_disp {
            term.clear_curt_tab();
            term.curt().mbar.set_err(&LANG.increase_height_width_terminal);
            return;
        }

        let mut cont_1 = PromptCont::new_edit_type(term.curt(), PromptContPosi::First);
        cont_1.set_menu(term);
        term.curt().prom.cont_1 = cont_1;
        let mut cont_2 = PromptCont::new_edit_type(term.curt(), PromptContPosi::Second);
        cont_2.set_menu(term);
        term.curt().prom.cont_2 = cont_2;
    }

    pub fn left_down_choice_menu(&mut self, y: u16, x: u16) {
        if self.cont_1.buf_row_posi == y || (self.cont_2.buf_row_posi <= y && y <= self.cont_2.buf_row_posi + self.cont_2.row_len) {
            match y {
                y if self.cont_1.buf_row_posi == y => {
                    self.cont_1.left_down_choice(y, x);
                    let (p_y, p_x) = Choices::get_y_x(&self.cont_1);
                    Choices::set_show_choice(p_y, p_x, &mut self.cont_2.choices_map);
                    self.prom_cont_posi = PromptContPosi::First;
                }
                y if self.cont_2.buf_row_posi <= y && y <= self.cont_2.buf_row_posi + self.cont_2.row_len => {
                    self.cont_2.left_down_choice(y, x);
                    self.prom_cont_posi = PromptContPosi::Second;
                }
                _ => {}
            }
        }
    }

    pub fn draw_menu(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_menu");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        self.cont_1.draw_choice_menu(str_vec);

        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc.clone());
        self.cont_2.draw_choice_menu(str_vec);
    }

    pub fn draw_cur_menu(&self, str_vec: &mut Vec<String>) {
        match self.prom_cont_posi {
            PromptContPosi::First => self.cont_1.draw_choice_cur(str_vec),
            PromptContPosi::Second => self.cont_2.draw_choice_cur(str_vec),
            _ => {}
        };
    }

    pub fn change_choice_vec_menu(&mut self, cur_direction: CurDirection) {
        match self.prom_cont_posi {
            PromptContPosi::First => {
                if self.cont_1.get_choices().unwrap().set_vec_posi(cur_direction) {
                    self.prom_cont_posi = PromptContPosi::Second;
                }
            }
            PromptContPosi::Second => {
                if self.cont_2.get_choices().unwrap().set_vec_posi(cur_direction) {
                    self.prom_cont_posi = PromptContPosi::First;
                }
            }
            _ => {}
        }
    }
}
impl PromptCont {
    pub fn set_menu(&mut self, term: &mut Terminal) {
        let base_posi = self.disp_row_posi;

        match self.posi {
            PromptContPosi::First => {
                self.guide_row_posi = base_posi;
                self.key_desc_row_posi = base_posi + 1;
                self.buf_desc_row_posi = base_posi + 2;
                self.buf_row_posi = base_posi + 3;

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

                if term.curt().editor.keycmd == KeyCmd::OpenMenu && term.curt().prom.prom_menu.choices_map_cache.get(&PromptContPosi::First).is_some() {
                    self.choices_map = term.curt().prom.prom_menu.choices_map_cache.get(&PromptContPosi::First).unwrap().clone();
                    term.curt().prom.prom_cont_posi = term.curt().prom.prom_menu.prompt_cont_posi_cache;
                } else {
                    let mut choices = Choices::default();
                    let file = Keybind::get_menu_str(&LANG.file, KeyCmd::OpenMenuFile);
                    let convert = Keybind::get_menu_str(&LANG.convert, KeyCmd::OpenMenuConvert);
                    let edit_search = Keybind::get_menu_str(&LANG.edit_search, KeyCmd::OpenMenuEdit);
                    let select = Keybind::get_menu_str(&LANG.menu_select, KeyCmd::OpenMenuSelect);

                    choices.vec = vec![vec![Choice::new(&file), Choice::new(&convert), Choice::new(&edit_search), Choice::new(&select)]];
                    choices.is_show = true;

                    // UNDEFINED because there are no parents
                    self.choices_map.insert((USIZE_UNDEFINED, USIZE_UNDEFINED), choices);
                    Choices::set_shaping_choice_list(&mut self.choices_map);
                    Choices::set_choice_area(self.buf_row_posi, &mut self.choices_map);
                    self.set_default_choice_menu(USIZE_UNDEFINED, USIZE_UNDEFINED);
                }

                self.row_len = 4;
            }
            PromptContPosi::Second => {
                self.row_len = 3;
                self.buf_desc_row_posi = base_posi + 4;
                self.buf_row_posi = base_posi + 5;

                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.contents, Colors::get_default_fg());

                if term.curt().editor.keycmd == KeyCmd::OpenMenu && term.curt().prom.prom_menu.choices_map_cache.get(&PromptContPosi::Second).is_some() {
                    self.choices_map = term.curt().prom.prom_menu.choices_map_cache.get(&PromptContPosi::Second).unwrap().clone();
                } else {
                    let create_new = Keybind::get_menu_str(&LANG.create_new, KeyCmd::NewTab);
                    let open_file = Keybind::get_menu_str(&LANG.open, KeyCmd::OpenFile);
                    let encode = Keybind::get_menu_str(&LANG.encode, KeyCmd::Encoding);

                    let vec_1 = vec![Choice::new(&create_new), Choice::new(&open_file), Choice::new(&LANG.save_as)];
                    let vec_2 = vec![Choice::new(&encode), Choice::new(&LANG.end_of_all_save)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1, vec_2];
                    // choices.is_show = true;
                    self.choices_map.insert((0, 0), choices);

                    let vec_1 = vec![Choice::new(&LANG.to_lowercase), Choice::new(&LANG.to_half_width), Choice::new(&LANG.to_space)];
                    let vec_2 = vec![Choice::new(&LANG.to_uppercase), Choice::new(&LANG.to_full_width), Choice::new(&LANG.to_tab)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1, vec_2];
                    self.choices_map.insert((0, 1), choices);

                    let vec_1 = vec![Choice::new(&LANG.move_row)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1];
                    self.choices_map.insert((0, 2), choices);

                    let vec_1 = vec![Choice::new(&LANG.column_select_mode)];
                    let mut choices = Choices::default();
                    choices.vec = vec![vec_1];
                    self.choices_map.insert((0, 3), choices);

                    Choices::set_shaping_choice_list(&mut self.choices_map);
                    Choices::set_choice_area(self.buf_row_posi, &mut self.choices_map);
                    let (y, x) = Choices::get_y_x(&term.curt().prom.cont_1);
                    self.set_default_choice_menu(y, x);
                }
            }
            _ => {}
        };
    }

    pub fn draw_choice_menu(&self, str_vec: &mut Vec<String>) {
        for i in self.buf_row_posi..self.buf_row_posi + self.row_len {
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

    fn set_default_choice_menu(&mut self, parent_vec_y: usize, parent_vec_x: usize) {
        Log::debug_key("set_default_choice_menu");
        Log::debug("self.keycmds", &self.keycmd);

        for ((y, x), choices) in self.choices_map.iter_mut() {
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
                            KeyCmd::OpenMenuConvert => {
                                if choice.name.contains(&LANG.convert) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            }
                            KeyCmd::OpenMenuEdit => {
                                if choice.name.contains(&LANG.edit_search) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            }
                            KeyCmd::OpenMenuSelect => {
                                if choice.name.contains(&LANG.menu_select) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            }
                            _ => {}
                        },
                        PromptContPosi::Second => {
                            choices.is_show = if y == &parent_vec_y && x == &parent_vec_x { true } else { false };
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PromMenu {
    pub choices_map_cache: BTreeMap<PromptContPosi, HashMap<(usize, usize), Choices>>,
    pub prompt_cont_posi_cache: PromptContPosi,
}

impl Default for PromMenu {
    fn default() -> Self {
        PromMenu { choices_map_cache: BTreeMap::new(), prompt_cont_posi_cache: PromptContPosi::First }
    }
}
