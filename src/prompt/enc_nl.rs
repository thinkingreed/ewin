use crate::{bar::headerbar::HeaderFile, colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::io::ErrorKind;

impl EvtAct {
    pub fn enc_nl(term: &mut Terminal) -> EvtActType {
        match term.curt().editor.evt {
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let x = x as usize;
                term.tabs[term.idx].prom.left_down_choice(y, x);
                return EvtActType::Hold;
            }
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                BackTab => {
                    term.curt().prom.tab_enc_nl(false);
                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {
                    term.curt().prom.up_down_enc_nl(code);
                    return EvtActType::Hold;
                }
                Tab | Right => {
                    term.curt().prom.tab_enc_nl(true);
                    return EvtActType::Hold;
                }
                Left => {
                    term.curt().prom.tab_enc_nl(false);
                    return EvtActType::Hold;
                }
                Enter => {
                    let (enc_item, apply_item, nl_item, bom_item) = (term.curt().prom.cont_1.choices.get_choice(), term.curt().prom.cont_2.choices.get_choice(), term.curt().prom.cont_3.choices.get_choice(), term.curt().prom.cont_4.choices.get_choice());
                    let result = term.tabs[term.idx].editor.buf.set_encoding(&mut term.hbar.file_vec[term.idx], &enc_item, &nl_item, &apply_item, &bom_item);

                    match result {
                        Ok(()) => {}
                        Err(err) => {
                            match err.kind() {
                                ErrorKind::PermissionDenied => term.curt().mbar.set_err(&LANG.no_read_permission),
                                ErrorKind::NotFound => term.curt().mbar.set_err(&LANG.file_not_found),
                                _ => term.curt().mbar.set_err(&LANG.file_opening_problem),
                            };
                            return EvtActType::DrawOnly;
                        }
                    }
                    term.clear_curt_tab_status();
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    const CHOICE_ITEM_MARGIN: usize = 2;

    pub fn left_down_choice(&mut self, y: u16, x: usize) {
        match y {
            y if self.cont_1.buf_row_posi == y => {
                self.cont_1.left_down_choice(y, x);
                self.enter_enc_ctrl_bom();
            }
            y if self.cont_2.buf_row_posi == y => self.cont_2.left_down_choice(y, x),
            y if self.cont_3.buf_row_posi == y => self.cont_3.left_down_choice(y, x),
            y if self.cont_4.buf_row_posi == y => {
                let item = self.cont_1.choices.get_choice();
                if item.name == Encode::UTF8.to_string() {
                    self.cont_4.left_down_choice(y, x);
                }
            }
            _ => {}
        }
    }
    pub fn enc_nl(term: &mut Terminal) {
        term.curt().state.is_enc_nl = true;
        term.curt().prom.disp_row_num = 10;
        term.set_disp_size();
        term.curt().prom.cont_1 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::First).get_enc_nl(term);
        term.curt().prom.cont_2 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::Second).get_enc_nl(term);
        term.curt().prom.cont_3 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::Third).get_enc_nl(term);
        term.curt().prom.cont_4 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::Fourth).get_enc_nl(term);
    }

    pub fn draw_enc_nl(&self, str_vec: &mut Vec<String>) {
        Log::debug_s("              　　　　　draw_open_file");

        // buf_desc
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &self.cont_3.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_4.buf_desc_row_posi, &self.cont_4.buf_desc.clone());
        // buf
        Prompt::draw_choice(self, str_vec, &self.cont_1);
        Prompt::draw_choice(self, str_vec, &self.cont_2);
        Prompt::draw_choice(self, str_vec, &self.cont_3);
        Prompt::draw_choice(self, str_vec, &self.cont_4);
    }
    pub fn draw_cur_enc_nl(&self, str_vec: &mut Vec<String>) {
        match self.prom_cont_posi {
            PromptContPosi::First => Prompt::draw_cur_enc_nl_promcont(str_vec, &self.cont_1),
            PromptContPosi::Second => Prompt::draw_cur_enc_nl_promcont(str_vec, &self.cont_2),
            PromptContPosi::Third => Prompt::draw_cur_enc_nl_promcont(str_vec, &self.cont_3),
            PromptContPosi::Fourth => Prompt::draw_cur_enc_nl_promcont(str_vec, &self.cont_4),
        };
    }
    fn draw_cur_enc_nl_promcont(str_vec: &mut Vec<String>, promcont: &PromptCont) {
        let (mut y, mut x) = (0, 0);
        let mut total_idx = 0;
        for (row_idx, vec) in promcont.choices.vec.iter().enumerate() {
            for item in vec {
                if promcont.choices.idx == total_idx {
                    y = promcont.buf_row_posi + row_idx as u16;
                    x = item.area.0;
                    break;
                }
                total_idx += 1;
            }
        }
        Log::debug("x", &x);
        Log::debug("y", &y);

        str_vec.push(MoveTo(x as u16, y as u16).to_string());
    }
    pub fn up_down_enc_nl(&mut self, code: crossterm::event::KeyCode) {
        if code == Up {
            match self.prom_cont_posi {
                PromptContPosi::First => {
                    let item = self.cont_2.choices.get_choice();
                    self.prom_cont_posi = if *item.name == LANG.file_reload { PromptContPosi::Second } else { PromptContPosi::Fourth }
                }
                PromptContPosi::Second => self.prom_cont_posi = PromptContPosi::First,
                PromptContPosi::Third => self.prom_cont_posi = PromptContPosi::Second,
                PromptContPosi::Fourth => self.prom_cont_posi = PromptContPosi::Third,
            }
        } else {
            // code == Down
            match self.prom_cont_posi {
                PromptContPosi::First => self.prom_cont_posi = PromptContPosi::Second,
                PromptContPosi::Second => {
                    let item = self.cont_2.choices.get_choice();
                    self.prom_cont_posi = if *item.name == LANG.file_reload { PromptContPosi::First } else { PromptContPosi::Third }
                }
                PromptContPosi::Third => self.prom_cont_posi = PromptContPosi::Fourth,
                PromptContPosi::Fourth => self.prom_cont_posi = PromptContPosi::First,
            }
        }
    }

    pub fn enter_enc_ctrl_bom(&mut self) {
        let item = self.cont_1.choices.get_choice();
        if item.name == Encode::UTF16LE.to_string() || item.name == Encode::UTF16BE.to_string() {
            self.set_bom(true);
        } else if item.name == Encode::UTF8.to_string() {
            // Do nothing for UTF8
        } else {
            self.set_bom(false);
        }
    }
    pub fn tab_enc_nl(&mut self, is_asc: bool) {
        match self.prom_cont_posi {
            PromptContPosi::First => {
                self.cont_1.choices.set_next_back_choice(is_asc);
                self.enter_enc_ctrl_bom();
            }
            PromptContPosi::Second => self.cont_2.choices.set_next_back_choice(is_asc),
            PromptContPosi::Third => self.cont_3.choices.set_next_back_choice(is_asc),
            PromptContPosi::Fourth => {
                // BOM can be changed only in UTF8
                let item = self.cont_1.choices.get_choice();
                if *item.name == Encode::UTF8.to_string() {
                    self.cont_4.choices.set_next_back_choice(is_asc);
                }
            }
        }
    }

    fn set_bom(&mut self, is_check: bool) {
        let mut total_idx = 0;
        for v in self.cont_4.choices.vec.iter_mut() {
            for item in v {
                if is_check && item.name == format!("BOM{}", &LANG.with) {
                    self.cont_4.choices.idx = total_idx;
                } else if !is_check && item.name == format!("BOM{}", &LANG.without) {
                    self.cont_4.choices.idx = total_idx;
                }
                total_idx += 1;
            }
        }
    }

    pub fn draw_choice(prom: &Prompt, str_vec: &mut Vec<String>, prom_cont: &PromptCont) {
        let mut total_idx = 0;
        for (row_idx, vec) in prom_cont.choices.vec.iter().enumerate() {
            let mut row_width = 1;
            for (item_idx, item) in vec.iter().enumerate() {
                if item_idx == 0 {
                    str_vec.push(format!("{}{}", MoveTo(0, prom_cont.buf_row_posi + row_idx as u16), Clear(CurrentLine)));
                }
                let mut enable_choice = prom_cont.choices.idx == total_idx;
                match prom_cont.posi {
                    PromptContPosi::Third | PromptContPosi::Fourth => {
                        let item = prom.cont_2.choices.get_choice();
                        enable_choice = enable_choice && *item.name == LANG.keep_and_apply_string;
                    }
                    _ => {}
                }
                let item_str = if enable_choice { format!("{}{}{}", Colors::get_msg_warning_inversion_fg_bg(), item.name, Colors::get_hbar_fg_bg()) } else { format!("{}{}", Colors::get_hbar_fg_bg(), item.name) };
                str_vec.push(format!("{}{}", MoveTo(row_width, prom_cont.buf_row_posi + row_idx as u16), &item_str));

                row_width += (get_str_width(&item.name) + Prompt::CHOICE_ITEM_MARGIN) as u16;

                total_idx += 1;
            }
        }
    }
}

impl PromptCont {
    pub fn get_enc_nl(&mut self, term: &mut Terminal) -> PromptCont {
        let base_posi = self.disp_row_posi - 1;
        let h_file = &term.curt_h_file();

        match self.posi {
            PromptContPosi::First => {
                self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_enc_nl);
                self.key_desc = format!(
                    "{}{}:{}Enter  {}{}:{}Esc  {}{}:{}↑↓  {}{}:{}←→・Tab・Mouse click  ",
                    Colors::get_default_fg(),
                    &LANG.fixed,
                    Colors::get_msg_highlight_fg(),
                    Colors::get_default_fg(),
                    &LANG.close,
                    Colors::get_msg_highlight_fg(),
                    Colors::get_default_fg(),
                    &LANG.move_setting_location,
                    Colors::get_msg_highlight_fg(),
                    Colors::get_default_fg(),
                    &LANG.candidate_change,
                    Colors::get_msg_highlight_fg(),
                );

                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.encoding, Colors::get_default_fg());
                let mut utf_vec = vec![Choice::new(&Encode::UTF8.to_string()), Choice::new(&Encode::UTF16LE.to_string()), Choice::new(&Encode::UTF16BE.to_string())];
                let mut local_vec = vec![Choice::new(&Encode::SJIS.to_string()), Choice::new(&Encode::JIS.to_string()), Choice::new(&Encode::EucJp.to_string()), Choice::new(&Encode::GBK.to_string())];
                utf_vec.append(&mut local_vec);
                let mut enc_vec: Vec<Vec<Choice>> = vec![utf_vec];
                self.set_default_choice(h_file, &mut enc_vec);
                let choise = self.choices.get_choice();
                if choise.name.is_empty() {
                    // Set UTF8 if encoding is unknown
                    self.choices.idx = 0;
                }
                self.choices.vec = enc_vec;
                self.guide_row_posi = base_posi;
                self.key_desc_row_posi = base_posi + 1;
                self.buf_desc_row_posi = base_posi + 2;
                self.buf_row_posi = base_posi + 3;
            }
            PromptContPosi::Second => {
                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.method_of_applying, Colors::get_default_fg());
                let mut msg_vec: Vec<Vec<Choice>> = vec![vec![Choice::new(&LANG.file_reload.clone()), Choice::new(&LANG.keep_and_apply_string)]];
                self.set_default_choice(h_file, &mut msg_vec);
                self.choices.vec = msg_vec;

                self.buf_desc_row_posi = base_posi + 4;
                self.buf_row_posi = base_posi + 5;
            }
            PromptContPosi::Third => {
                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.new_line_code, Colors::get_default_fg());
                let mut nl_vec: Vec<Vec<Choice>> = vec![vec![Choice::new(&NEW_LINE_LF_STR.to_string()), Choice::new(&NEW_LINE_CRLF_STR.to_string())]];
                self.set_default_choice(h_file, &mut nl_vec);
                self.choices.vec = nl_vec;

                self.buf_desc_row_posi = base_posi + 6;
                self.buf_row_posi = base_posi + 7;
            }
            PromptContPosi::Fourth => {
                self.buf_desc = format!("{}BOM{}({}){}", Colors::get_msg_highlight_fg(), &LANG.presence_or_absence, &LANG.selectable_only_for_utf8, Colors::get_default_fg());
                let mut bom_vec: Vec<Vec<Choice>> = vec![vec![Choice::new(&format!("BOM{}", &LANG.with)), Choice::new(&format!("BOM{}", &LANG.without))]];
                self.set_default_choice(h_file, &mut bom_vec);
                self.choices.vec = bom_vec;

                self.buf_desc_row_posi = base_posi + 8;
                self.buf_row_posi = base_posi + 9;
            }
        };

        return self.clone();
    }
    pub fn left_down_choice(&mut self, y: u16, x: usize) {
        if y == self.buf_row_posi {
            let mut total_idx = 0;
            for vec in &self.choices.vec {
                for item in vec {
                    if item.area.0 <= x && x <= item.area.1 {
                        self.choices.idx = total_idx;
                    }
                    total_idx += 1;
                }
            }
        }
    }

    fn set_default_choice(&mut self, h_file: &HeaderFile, vec: &mut Vec<Vec<Choice>>) {
        let mut total_idx = 0;
        for v in vec {
            let mut row_width = 1;

            for item in v {
                match self.posi {
                    PromptContPosi::First => {
                        if h_file.enc.to_string() == item.name {
                            self.choices.idx = total_idx;
                        }
                    }
                    PromptContPosi::Second => {
                        if item.name == LANG.file_reload.clone() {
                            self.choices.idx = total_idx;
                        }
                    }
                    PromptContPosi::Third => {
                        if h_file.nl.to_string() == item.name {
                            self.choices.idx = total_idx;
                        }
                    }
                    PromptContPosi::Fourth => {
                        if None == h_file.bom {
                            if item.name == format!("BOM{}", &LANG.without) {
                                self.choices.idx = total_idx;
                            }
                        } else {
                            if item.name == format!("BOM{}", &LANG.with) {
                                self.choices.idx = total_idx;
                            }
                        }
                    }
                }
                let item_len = get_str_width(&item.name);
                item.area = (row_width, row_width + item_len - 1);
                row_width += item_len + Prompt::CHOICE_ITEM_MARGIN;
                total_idx += 1;
            }
        }
    }
}
