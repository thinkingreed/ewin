use crate::{colors::*, global::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};

impl EvtAct {
    pub fn close(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    // save成否判定
                    if editor.save(mbar, prom, sbar) {
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
    pub fn save_confirm_str(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new();
        cont.set_save_confirm();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.save_confirmation_to_close);
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}Ctrl + c{}",
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
    }
}
