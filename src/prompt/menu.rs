use crate::{colors::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::io::ErrorKind;

impl EvtAct {
    pub fn menu(term: &mut Terminal) -> EvtActType {
        match term.curt().editor.evt {
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let x = x as usize;
                term.tabs[term.idx].prom.left_down_choice_menu(y, x);
                return EvtActType::Hold;
            }

            Key(KeyEvent { code, .. }) => match code {
                Down | Up => {
                    term.curt().prom.up_down_menu(code);
                    return EvtActType::Hold;
                }
                Right => {
                    return EvtActType::Hold;
                }
                Left => {
                    return EvtActType::Hold;
                }
                Enter => {
                    let (apply_item, enc_item, nl_item, bom_item) = (term.curt().prom.cont_1.choices.get_choice(), term.curt().prom.cont_2.choices.get_choice(), term.curt().prom.cont_3.choices.get_choice(), term.curt().prom.cont_4.choices.get_choice());
                    let result = term.tabs[term.idx].editor.buf.set_encoding(&mut term.hbar.file_vec[term.idx], &enc_item, &nl_item, &apply_item, &bom_item);

                    match result {
                        Ok(()) => term.curt().editor.h_file = term.hbar.file_vec[term.idx].clone(),
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
    pub fn menu(term: &mut Terminal) {
        Log::debug_key("Prompt.menud");

        term.curt().state.is_menu = true;
        term.curt().prom.disp_row_num = 8;
        let is_disp = term.set_disp_size();
        if !is_disp {
            term.curt().mbar.set_err(&LANG.increase_height_terminal);
            term.curt().prom.clear();
            term.curt().state.clear();
            return;
        }
        term.curt().prom.cont_1 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::First).get_menu(term);
        term.curt().prom.cont_2 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::Second).get_menu(term);
    }

    pub fn left_down_choice_menu(&mut self, y: u16, x: usize) {
        match y {
            y if self.cont_1.buf_row_posi == y => self.cont_1.left_down_choice(y, x),
            y if self.cont_2.buf_row_posi == y => self.cont_2.left_down_choice(y, x),
            _ => {}
        }
    }

    pub fn draw_menu(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_menu");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::draw_choice_menu(self, str_vec, &self.cont_1);

        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc.clone());
        Prompt::draw_choice_menu(self, str_vec, &self.cont_2);
    }
    pub fn draw_cur_menu(&self, str_vec: &mut Vec<String>) {
        match self.prom_cont_posi {
            PromptContPosi::First => self.cont_1.draw_cur_promcont(str_vec),
            PromptContPosi::Second => self.cont_2.draw_cur_promcont(str_vec),
            _ => {}
        };
    }

    pub fn up_down_menu(&mut self, code: crossterm::event::KeyCode) {
        if code == Up {
            match self.prom_cont_posi {
                PromptContPosi::First => self.prom_cont_posi = PromptContPosi::Second,
                PromptContPosi::Second => self.prom_cont_posi = PromptContPosi::First,
                _ => {}
            }
        } else {
            // code == Down
            match self.prom_cont_posi {
                PromptContPosi::First => self.prom_cont_posi = PromptContPosi::Second,
                PromptContPosi::Second => self.prom_cont_posi = PromptContPosi::First,
                _ => {}
            }
        }
    }

    pub fn draw_choice_menu(prom: &Prompt, str_vec: &mut Vec<String>, prom_cont: &PromptCont) {
        let mut total_idx = 0;
        for (row_idx, vec) in prom_cont.choices.vec.iter().enumerate() {
            let mut row_width = 1;
            for (item_idx, item) in vec.iter().enumerate() {
                if item_idx == 0 {
                    str_vec.push(format!("{}{}", MoveTo(0, prom_cont.buf_row_posi + row_idx as u16), Clear(CurrentLine)));
                }
                let enable_choice = prom_cont.choices.idx == total_idx;
                let item_str = if enable_choice { format!("{}{}{}", Colors::get_msg_warning_inversion_fg_bg(), item.name, Colors::get_hbar_fg_bg()) } else { format!("{}{}", Colors::get_hbar_fg_bg(), item.name) };
                str_vec.push(format!("{}{}", MoveTo(row_width, prom_cont.buf_row_posi + row_idx as u16), &item_str));

                row_width += (get_str_width(&item.name) + Prompt::CHOICE_ITEM_MARGIN) as u16;

                total_idx += 1;
            }
        }
    }
}

impl PromptCont {
    pub fn get_menu(&mut self, term: &mut Terminal) -> PromptCont {
        let base_posi = self.disp_row_posi - 1;

        match self.posi {
            PromptContPosi::First => {
                self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.select_menu_and_work);
                self.key_desc = format!(
                    "{}{}:{}Enter  {}{}:{}Esc  {}{}:{}↑↓  {}{}:{}←→・Tab・Mouse click",
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

                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.menu, Colors::get_default_fg());
                let mut msg_vec: Vec<Vec<Choice>> = vec![vec![Choice::new(&LANG.file.clone()), Choice::new(&LANG.edit)]];
                self.set_default_choice_menu(&mut msg_vec);
                self.choices.vec = msg_vec;

                self.guide_row_posi = base_posi;
                self.key_desc_row_posi = base_posi + 1;
                self.buf_desc_row_posi = base_posi + 2;
                self.buf_row_posi = base_posi + 3;
            }
            PromptContPosi::Second => {
                self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.encoding, Colors::get_default_fg());
                let mut open_vec = vec![Choice::new(&LANG.new_tab), Choice::new(&LANG.open)];
                let mut other_vec = vec![Choice::new(&LANG.save_as)];
                open_vec.append(&mut other_vec);
                let mut file_vec: Vec<Vec<Choice>> = vec![open_vec];
                self.set_default_choice_menu(&mut file_vec);
                self.choices.vec = file_vec;
                self.buf_desc_row_posi = base_posi + 4;
                self.buf_row_posi = base_posi + 5;
            }
            _ => {}
        };

        return self.clone();
    }

    fn set_default_choice_menu(&mut self, vec: &mut Vec<Vec<Choice>>) {
        let mut total_idx = 0;
        for v in vec {
            let mut row_width = 1;

            for item in v {
                match self.posi {
                    PromptContPosi::First => {
                        if item.name == LANG.file.clone() {
                            self.choices.idx = total_idx;
                        }
                    }
                    PromptContPosi::Second => {
                        if item.name == LANG.new_tab.clone() {
                            self.choices.idx = total_idx;
                        }
                    }
                    _ => {}
                }
                let item_len = get_str_width(&item.name);
                item.area = (row_width, row_width + item_len - 1);
                row_width += item_len + Prompt::CHOICE_ITEM_MARGIN;
                total_idx += 1;
            }
        }
    }
}
