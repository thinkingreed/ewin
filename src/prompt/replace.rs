use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use termion::color;

impl EvtAct {
    pub fn replace<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.curt_evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Char(c) => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.insert_char(c.to_ascii_uppercase()),
                        PromptBufPosi::Second => prom.cont_2.insert_char(c.to_ascii_uppercase()),
                        _ => {}
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                Char(c) => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.insert_char(c),
                        PromptBufPosi::Second => prom.cont_2.insert_char(c),
                        _ => {}
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Down => {
                    prom.cursor_down();
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Up => {
                    prom.cursor_up();
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Left => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.cursor_left(),
                        PromptBufPosi::Second => prom.cont_2.cursor_left(),
                        _ => {}
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Right => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.cursor_right(),
                        PromptBufPosi::Second => prom.cont_2.cursor_right(),
                        _ => {}
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Delete => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.delete(),
                        PromptBufPosi::Second => prom.cont_2.delete(),
                        _ => {}
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Backspace => {
                    match prom.buf_posi {
                        PromptBufPosi::First => prom.cont_1.backspace(),
                        PromptBufPosi::Second => prom.cont_2.backspace(),
                        _ => {}
                    }
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }
                Tab => {
                    prom.tab(true);
                    prom.draw_only(out);
                    return EvtActType::Hold;
                }

                Enter => {
                    let search_str = prom.cont_1.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_str.clone());
                    } else if prom.cont_2.buf.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_replace_str.clone());
                    } else if editor.get_search_ranges(search_str.clone()).len() == 0 {
                        mbar.set_err(mbar.lang.cannot_find_char_search_for.clone());
                    } else {
                        editor.replace(prom);
                        mbar.clear();
                        prom.clear();
                        prom.is_change = true;
                    }
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                    return EvtActType::Hold;
                }

                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn replace(&mut self) {
        self.disp_row_num = 6;
        let mut cont_1 = PromptCont::new(self.lang.clone());
        let mut cont_2 = PromptCont::new(self.lang.clone());
        cont_1.set_replace(PromptBufPosi::First);
        cont_2.set_replace(PromptBufPosi::Second);
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self, cont_type: PromptBufPosi) {
        if cont_type == PromptBufPosi::First {
            self.guide = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_replace.clone(), "\n");
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c",
                &color::Fg(color::White).to_string(),
                self.lang.all_replace.clone(),
                &color::Fg(color::LightGreen).to_string(),
                &color::Fg(color::White).to_string(),
                self.lang.move_input_field.clone(),
                &color::Fg(color::LightGreen).to_string(),
                &color::Fg(color::White).to_string(),
                self.lang.close.clone(),
                &color::Fg(color::LightGreen).to_string(),
            );
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.search_str.clone(),);
        } else {
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.replace_char.clone(),);
        }
    }
}
