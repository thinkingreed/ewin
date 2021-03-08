use crate::{bar::headerbar::*, bar::msgbar::*, bar::statusbar::*, colors::*, global::*, help::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};

impl EvtAct {
    pub fn close(hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    // save成否判定
                    if editor.save(hbar, mbar, prom, help, sbar) {
                        return EvtActType::Exit;
                    } else {
                        editor.d_range.draw_type = DrawType::All;
                        return EvtActType::DrawOnly;
                    }
                } else if c == 'n' {
                    return EvtActType::Exit;
                } else {
                    return EvtActType::Hold;
                }
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn close(&mut self, hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) -> bool {
        if FILE.get().unwrap().try_lock().unwrap().is_changed == true {
            self.save_confirm_str(hbar, editor, mbar, help, sbar);
            self.is_close_confirm = true;
            return false;
        };
        return true;
    }
    pub fn save_confirm_str(&mut self, hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.disp_row_num = 2;
        Terminal::set_disp_size(hbar, editor, mbar, self, help, sbar);
        let mut cont = PromptCont::new_not_edit(self.disp_row_posi as u16);
        cont.set_save_confirm();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.save_confirmation_to_close);
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}Esc{}",
            Colors::get_default_fg(),
            &LANG.yes,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.no,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.cancel,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );

        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
    }
}
