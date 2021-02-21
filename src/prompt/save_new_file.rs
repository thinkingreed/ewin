use crate::{colors::*, global::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    if prom.cont_1.buf.len() == 0 {
                        mbar.set_err(&LANG.not_entered_filenm);
                    } else {
                        let filenm = prom.cont_1.buf.iter().collect::<String>();
                        if Path::new(&filenm).exists() {
                            mbar.set_err(&LANG.file_already_exists);
                            return EvtActType::Hold;
                        }
                        sbar.filenm = filenm;
                        editor.save(mbar, prom, sbar);
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
    pub fn save_new_file(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new();
        cont.set_new_file_name();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_new_filenm);
        self.key_desc = format!(
            "{}{}:{}Enter  {}{}:{}Ctrl + c{}",
            Colors::get_default_fg(),
            &LANG.fixed,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.cancel,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );
    }
}
