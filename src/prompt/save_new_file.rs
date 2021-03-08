use crate::{bar::headerbar::*, bar::msgbar::*, bar::statusbar::*, colors::*, global::*, help::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
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
                        FILE.get().unwrap().try_lock().map(|mut file| file.filenm = filenm).unwrap();
                        editor.save(hbar, mbar, prom, help, sbar);
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
    pub fn save_new_file(&mut self, hbar: &mut HeaderBar, editor: &mut Editor, mbar: &mut MsgBar, help: &mut Help, sbar: &mut StatusBar) {
        self.disp_row_num = 3;
        Terminal::set_disp_size(hbar, editor, mbar, self, help, sbar);
        let mut cont = PromptCont::new_not_edit(self.disp_row_posi as u16);
        cont.set_new_file_name();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_new_filenm);
        self.key_desc = format!(
            "{}{}:{}Enter  {}{}:{}Esc{}",
            Colors::get_default_fg(),
            &LANG.fixed,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.cancel,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );
        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.buf_row_posi = base_posi + 2;
    }
}
