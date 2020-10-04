use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, Log, Prompt, PromptCont, PromptInput};
use std::io::Error;
use termion::*;

impl Prompt {
    pub fn new(lang_cfg: LangCfg) -> Self {
        Prompt { lang: lang_cfg, ..Prompt::default() }
    }

    pub fn clear(&mut self) {
        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.is_save_confirm = false;
        self.prompt_conts = vec![];
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, editor: &mut Editor) -> Result<(), Error> {
        Log::ep_s("★　Prompt draw");

        for (i, cont) in self.prompt_conts.iter().enumerate() {
            let cont_desc = format!("{}{}{}", cursor::Goto((editor.lnw + 1) as u16, (self.disp_row_posi + i) as u16), clear::CurrentLine, cont.desc.clone());
            str_vec.push(cont_desc);
            let input_desc = format!("{}{}{}", cursor::Goto((editor.lnw + 1) as u16, (self.disp_row_posi + 1 + i) as u16), clear::CurrentLine, cont.prompt_input.desc.clone());
            str_vec.push(input_desc);
        }

        return Ok(());
    }

    pub fn set_save_confirm_str(&mut self) {
        self.disp_row_num = 2;
        let mut prompt_cont = PromptCont::new(self.lang.clone());
        prompt_cont.set_save_confirm();
        self.prompt_conts = vec![prompt_cont];
    }
    pub fn set_new_file_name(&mut self) {
        self.disp_row_num = 3;
        let mut prompt_cont = PromptCont::new(self.lang.clone());
        prompt_cont.set_new_file_name();
        self.prompt_conts = vec![prompt_cont];
    }
}

impl PromptCont {
    pub fn new(lang_cfg: LangCfg) -> Self {
        PromptCont { lang: lang_cfg, ..PromptCont::default() }
    }
    pub fn set_save_confirm(&mut self) {
        self.desc = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.save_confirmation_to_close.clone(), "\n");
        let mut prompt_input = PromptInput::new(self.lang.clone());
        prompt_input.set_save_confirm();
        self.prompt_input = prompt_input;
    }
    pub fn set_new_file_name(&mut self) {
        self.desc = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_new_filenm.clone(), "\n");
        let mut prompt_input = PromptInput::new(self.lang.clone());
        prompt_input.set_save_confirm();
        self.prompt_input = prompt_input;
    }
}

impl PromptInput {
    pub fn new(lang_cfg: LangCfg) -> Self {
        PromptInput { lang: lang_cfg, ..PromptInput::default() }
    }

    pub fn set_save_confirm(&mut self) {
        self.desc = format!(
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
        self.desc = format!(
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
