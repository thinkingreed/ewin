use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Log, Prompt, PromptCont};
use std::io::Error;
use termion::*;

impl Prompt {
    pub fn new(lang_cfg: LangCfg) -> Self {
        Prompt { lang: lang_cfg, ..Prompt::default() }
    }

    pub fn clear(&mut self) {
        Log::ep_s("★　Prompt clear");

        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.is_save_confirm = false;
        self.is_save_new_file = false;
        self.cont = PromptCont::default();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) -> Result<(), Error> {
        Log::ep_s("★　Prompt draw");
        if self.cont.desc.len() > 0 {
            let cont_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.cont.desc.clone());
            str_vec.push(cont_desc);

            let input_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 1) as u16), clear::CurrentLine, self.cont.input_desc.clone());
            str_vec.push(input_desc);

            let input = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont.buf.iter().collect::<String>());
            if self.is_save_new_file || self.cont.buf.len() > 0 {
                str_vec.push(input);
            }
        }
        return Ok(());
    }

    pub fn set_save_confirm_str(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_save_confirm();
        self.cont = cont;
    }
    pub fn save_new_file(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_new_file_name();
        self.cont = cont;
    }
}

impl PromptCont {
    pub fn new(lang_cfg: LangCfg) -> Self {
        PromptCont { lang: lang_cfg, ..PromptCont::default() }
    }
    pub fn set_save_confirm(&mut self) {
        self.desc = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.save_confirmation_to_close.clone(), "\n");
        self.input_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.yes.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.no.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.cancel.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }

    pub fn set_new_file_name(&mut self) {
        self.desc = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_new_filenm.clone(), "\n");
        self.input_desc = format!(
            "{}{}:{}Enter  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.fixed.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.cancel.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }
}
