use crate::{bar::headerbar::*, bar::msgbar::*, bar::statusbar::StatusBar, colors::*, global::*, help::Help, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::sync::Mutex;

impl EvtAct {
    pub fn replace(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let search_str = prom.cont_1.buf.iter().collect::<String>();
                    let replace_str = prom.cont_2.buf.iter().collect::<String>();

                    if search_str.is_empty() {
                        mbar.set_err(&LANG.not_entered_search_str);
                    } else if replace_str.is_empty() {
                        mbar.set_err(&LANG.not_entered_replace_str);
                    } else {
                        let search_set = editor.buf.search(&search_str.clone(), 0, editor.buf.len_chars());
                        if search_set.len() == 0 {
                            mbar.set_err(&LANG.cannot_find_char_search_for);
                            return EvtActType::DrawOnly;
                        }

                        let _ = REPLACE_SEARCH_RANGE.set(Mutex::new(search_set));

                        editor.exec_edit_proc(EvtType::Replace, &search_str, &replace_str);
                        mbar.clear();
                        prom.clear();
                        FILE.get().unwrap().try_lock().map(|mut file| file.is_changed = true).unwrap();
                    }

                    editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn replace(&mut self, hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.is_replace = true;
        self.disp_row_num = 7;
        Terminal::set_disp_size(hbar, editor, mbar, self, help, sbar);
        let mut cont_1 = PromptCont::new_edit(self.disp_row_posi as u16, PromptContPosi::First);
        let mut cont_2 = PromptCont::new_edit(self.disp_row_posi as u16, PromptContPosi::Second);
        cont_1.set_replace();
        cont_2.set_replace();
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self) {
        let base_posi = self.disp_row_posi - 1;

        if self.prompt_cont_posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_replace);
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Esc",
                Colors::get_default_fg(),
                &LANG.all_replace,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.move_input_field,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
            );
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.search_str,);

            self.guide_row_posi = base_posi;
            self.key_desc_row_posi = base_posi + 1;
            self.opt_row_posi = base_posi + 2;
            self.buf_desc_row_posi = base_posi + 3;
            self.buf_row_posi = base_posi + 4;
        } else {
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.replace_char,);

            self.buf_desc_row_posi = base_posi + 5;
            self.buf_row_posi = base_posi + 6;
        }
        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
