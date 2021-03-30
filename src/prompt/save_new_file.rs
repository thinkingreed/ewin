use crate::{colors::*, global::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::path::Path;

impl EvtAct {
    pub fn save_new_filenm(tab: &mut Tab) -> EvtActType {
        match tab.editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    if tab.prom.cont_1.buf.len() == 0 {
                        tab.mbar.set_err(&LANG.not_entered_filenm);
                    } else {
                        let filenm = tab.prom.cont_1.buf.iter().collect::<String>();
                        if Path::new(&filenm).exists() {
                            tab.mbar.set_err(&LANG.file_already_exists);
                            return EvtActType::Hold;
                        }
                        FILE.get().unwrap().try_lock().map(|mut file| file.filenm = filenm).unwrap();
                        tab.save();
                    }
                    tab.editor.d_range.draw_type = DrawType::All;
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
