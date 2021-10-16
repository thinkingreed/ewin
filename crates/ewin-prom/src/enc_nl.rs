use crate::{
    ewin_com::{_cfg::key::keycmd::*, colors::*, def::*, global::*, log::Log, model::*, util::*},
    model::*,
    prompt::choice::*,
};
use crossterm::cursor::MoveTo;
use crossterm::{terminal::ClearType::*, terminal::*};
use std::usize;

impl Prompt {
    pub fn left_down_choice_enc_nl(&mut self, y: u16, x: u16) {
        match y {
            y if self.cont_1.buf_row_posi == y => {
                self.cont_1.left_down_choice(y, x);
                self.cont_posi = PromptContPosi::First;
            }
            y if self.cont_2.buf_row_posi == y => {
                self.cont_2.left_down_choice(y, x);
                self.enter_enc_ctrl_bom();
                self.cont_posi = PromptContPosi::Second;
            }
            y if self.cont_3.buf_row_posi == y => {
                self.cont_3.left_down_choice(y, x);
                self.cont_posi = PromptContPosi::Third;
            }
            y if self.cont_4.buf_row_posi == y => {
                let item = self.cont_2.get_choice();
                if item.name == Encode::UTF8.to_string() {
                    self.cont_4.left_down_choice(y, x);
                }
                self.cont_posi = PromptContPosi::Fourth;
            }
            _ => {}
        }
    }
    pub fn enc_nl(&mut self) {
        self.disp_row_num = 10;

        self.cont_1 = PromptCont::new(Some(PromptContPosi::First)).get_enc_nl();
        self.cont_2 = PromptCont::new(Some(PromptContPosi::Second)).get_enc_nl();
        self.cont_3 = PromptCont::new(Some(PromptContPosi::Third)).get_enc_nl();
        self.cont_4 = PromptCont::new(Some(PromptContPosi::Fourth)).get_enc_nl();
    }

    pub fn draw_enc_nl(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_open_file");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::draw_choice_enc_nl(self, str_vec, &self.cont_1);

        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc.clone());
        Prompt::draw_choice_enc_nl(self, str_vec, &self.cont_2);

        let item = self.cont_1.get_choice();
        let is_file_reload = *item.name == LANG.file_reload;

        let cont_3_buf_desc = if is_file_reload { "".to_string() } else { self.cont_3.buf_desc.clone() };
        let cont_4_buf_desc = if is_file_reload { "".to_string() } else { self.cont_4.buf_desc.clone() };
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &cont_3_buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_4.buf_desc_row_posi, &cont_4_buf_desc);

        if is_file_reload {
            Prompt::set_draw_vec(str_vec, self.cont_3.buf_row_posi, &"".to_string());
            Prompt::set_draw_vec(str_vec, self.cont_4.buf_row_posi, &"".to_string());
        } else {
            Prompt::draw_choice_enc_nl(self, str_vec, &self.cont_3);
            Prompt::draw_choice_enc_nl(self, str_vec, &self.cont_4);
        }
    }
    pub fn draw_cur_enc_nl(&self, str_vec: &mut Vec<String>) {
        match self.cont_posi {
            PromptContPosi::First => self.cont_1.draw_choice_cur(str_vec),
            PromptContPosi::Second => self.cont_2.draw_choice_cur(str_vec),
            PromptContPosi::Third => self.cont_3.draw_choice_cur(str_vec),
            PromptContPosi::Fourth => self.cont_4.draw_choice_cur(str_vec),
        };
    }

    fn enter_enc_ctrl_bom(&mut self) {
        let item = self.cont_2.get_choice();
        if item.name == Encode::UTF16LE.to_string() || item.name == Encode::UTF16BE.to_string() {
            self.set_bom(true);
        } else if item.name == Encode::UTF8.to_string() {
            // Do nothing for UTF8
        } else {
            self.set_bom(false);
        }
    }
    pub fn move_enc_nl(&mut self, cur_direction: Direction) {
        match self.cont_posi {
            PromptContPosi::First => {
                let is_move_cont = self.cont_1.get_choices().unwrap().set_vec_posi(cur_direction);
                if is_move_cont {
                    if cur_direction == Direction::Down {
                        self.cont_posi = PromptContPosi::Second;
                    } else if cur_direction == Direction::Up {
                        let item = self.cont_1.get_choice();
                        self.cont_posi = if *item.name == LANG.file_reload { PromptContPosi::Second } else { PromptContPosi::Fourth };
                    }
                }
            }
            PromptContPosi::Second => {
                let is_move_cont = self.cont_2.get_choices().unwrap().set_vec_posi(cur_direction);
                if is_move_cont {
                    if cur_direction == Direction::Down {
                        let item = self.cont_1.get_choice();
                        self.cont_posi = if *item.name == LANG.file_reload { PromptContPosi::First } else { PromptContPosi::Third };
                    } else if cur_direction == Direction::Up {
                        self.cont_posi = PromptContPosi::First;
                    }
                }
                self.enter_enc_ctrl_bom();
            }
            PromptContPosi::Third => {
                let is_move_cont = self.cont_3.get_choices().unwrap().set_vec_posi(cur_direction);
                if is_move_cont {
                    if cur_direction == Direction::Down {
                        self.cont_posi = PromptContPosi::Fourth;
                    } else if cur_direction == Direction::Up {
                        self.cont_posi = PromptContPosi::Second;
                    }
                }
            }

            PromptContPosi::Fourth => match cur_direction {
                Direction::Up | Direction::Down => {
                    let is_move_cont = self.cont_4.get_choices().unwrap().set_vec_posi(cur_direction);
                    if is_move_cont {
                        self.cont_posi = if cur_direction == Direction::Down { PromptContPosi::First } else { PromptContPosi::Third };
                    }
                }
                Direction::Left | Direction::Right => {
                    let item = self.cont_2.get_choice();
                    if *item.name == Encode::UTF8.to_string() {
                        self.cont_4.get_choices().unwrap().set_vec_posi(cur_direction);
                    }
                }
            },
        }
    }

    fn set_bom(&mut self, is_check: bool) {
        for (_, choices) in self.cont_4.choices_map.iter_mut() {
            if choices.is_show {
                for (y_idx, v) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in v.iter().enumerate() {
                        if is_check && item.name == format!("BOM{}", &LANG.with) {
                            choices.vec_y = y_idx;
                            choices.vec_x = x_idx;
                        } else if !is_check && item.name == format!("BOM{}", &LANG.without) {
                            choices.vec_y = y_idx;
                            choices.vec_x = x_idx;
                        }
                    }
                }
            }
        }
    }

    fn draw_choice_enc_nl(prom: &Prompt, str_vec: &mut Vec<String>, prom_cont: &PromptCont) {
        for (_, choices) in prom_cont.choices_map.iter() {
            if !choices.is_show {
                continue;
            }
            for (y_idx, vec) in choices.vec.iter().enumerate() {
                let mut row_width = 1;
                str_vec.push(format!("{}{}", MoveTo(0, prom_cont.buf_row_posi + y_idx as u16), Clear(CurrentLine)));
                for (x_idx, item) in vec.iter().enumerate() {
                    let mut enable_choice = choices.vec_y == y_idx && choices.vec_x == x_idx;
                    match prom_cont.posi {
                        PromptContPosi::Third | PromptContPosi::Fourth => {
                            let item = prom.cont_1.get_choice();
                            enable_choice = enable_choice && *item.name == LANG.keep_and_apply_string;
                        }
                        _ => {}
                    }
                    let item_str = if enable_choice { format!("{}{}{}", Colors::get_msg_warning_inversion_fg_bg(), item.name, Colors::get_hbar_fg_bg()) } else { format!("{}{}", Colors::get_hbar_fg_bg(), item.name) };
                    str_vec.push(format!("{}{}", MoveTo(row_width, prom_cont.buf_row_posi + y_idx as u16), &item_str));

                    row_width += (get_str_width(&item.name) + Choices::ITEM_MARGIN) as u16;
                }
            }
        }
    }
}

impl PromptCont {
    fn get_enc_nl(&mut self) -> PromptCont {
        match self.posi {
            PromptContPosi::First => {
                self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_enc_nl);
                self.key_desc = format!(
                    "{}{}:{}Enter  {}{}:{}{}  {}{}:{}↑↓  {}{}:{}←→・Tab",
                    Colors::get_default_fg(),
                    &LANG.fixed,
                    Colors::get_msg_highlight_fg(),
                    Colors::get_default_fg(),
                    &LANG.close,
                    Colors::get_msg_highlight_fg(),
                    Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
                    Colors::get_default_fg(),
                    &LANG.move_setting_location,
                    Colors::get_msg_highlight_fg(),
                    Colors::get_default_fg(),
                    &LANG.candidate_change,
                    Colors::get_msg_highlight_fg(),
                );

                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.method_of_applying, Colors::get_default_fg());

                let mut choices = Choices::default();
                let vec = vec![vec![Choice::new(&LANG.file_reload.clone()), Choice::new(&LANG.keep_and_apply_string)]];
                choices.vec = vec;
                choices.is_show = true;
                self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (USIZE_UNDEFINED, USIZE_UNDEFINED)), choices);
            }
            PromptContPosi::Second => {
                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.encoding, Colors::get_default_fg());

                let mut utf_vec = vec![Choice::new(&Encode::UTF8.to_string()), Choice::new(&Encode::UTF16LE.to_string()), Choice::new(&Encode::UTF16BE.to_string())];
                let mut local_vec = vec![Choice::new(&Encode::SJIS.to_string()), Choice::new(&Encode::JIS.to_string()), Choice::new(&Encode::EucJp.to_string()), Choice::new(&Encode::GBK.to_string())];
                utf_vec.append(&mut local_vec);
                let enc_vec: Vec<Vec<Choice>> = vec![utf_vec];

                let mut choices = Choices::default();
                choices.is_show = true;
                choices.vec = enc_vec;
                self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 0)), choices);
            }
            PromptContPosi::Third => {
                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.new_line_code, Colors::get_default_fg());
                let nl_vec: Vec<Vec<Choice>> = vec![vec![Choice::new(&NEW_LINE_LF_STR.to_string()), Choice::new(&NEW_LINE_CRLF_STR.to_string())]];

                let mut choices = Choices::default();
                choices.is_show = true;
                choices.vec = nl_vec;
                self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 0)), choices);
            }
            PromptContPosi::Fourth => {
                self.buf_desc = format!("{}BOM{}({}){}", Colors::get_msg_highlight_fg(), &LANG.presence_or_absence, &LANG.selectable_only_for_utf8, Colors::get_default_fg());
                let bom_vec: Vec<Vec<Choice>> = vec![vec![Choice::new(&format!("BOM{}", &LANG.with)), Choice::new(&format!("BOM{}", &LANG.without))]];

                let mut choices = Choices::default();
                choices.is_show = true;
                choices.vec = bom_vec;
                self.choices_map.insert(((USIZE_UNDEFINED, USIZE_UNDEFINED), (0, 0)), choices);
            }
        };

        return self.clone();
    }
    pub fn set_default_choice_enc_nl(&mut self, buf_row_posi: u16, h_file: &HeaderFile) {
        for (_, choices) in self.choices_map.iter_mut() {
            for (y_idx, v) in choices.vec.iter_mut().enumerate() {
                let mut row_width = 1;

                for (x_idx, choice) in v.iter_mut().enumerate() {
                    match self.posi {
                        PromptContPosi::First => {}
                        PromptContPosi::Second => {
                            if h_file.enc.to_string() == choice.name {
                                choices.vec_y = y_idx;
                                choices.vec_x = x_idx;
                            }
                        }
                        PromptContPosi::Third => {
                            if h_file.nl.to_string() == choice.name {
                                choices.vec_y = y_idx;
                                choices.vec_x = x_idx;
                            }
                        }
                        PromptContPosi::Fourth => {
                            if None == h_file.bom {
                                if choice.name == format!("BOM{}", &LANG.without) {
                                    choices.vec_y = y_idx;
                                    choices.vec_x = x_idx;
                                }
                            } else if choice.name == format!("BOM{}", &LANG.with) {
                                choices.vec_y = y_idx;
                                choices.vec_x = x_idx;
                            }
                        }
                    }
                    let item_len = get_str_width(&choice.name);
                    choice.area = (buf_row_posi as usize + y_idx, row_width, row_width + item_len - 1);
                    row_width += item_len + Choices::ITEM_MARGIN;
                }
            }
        }
    }
}
