use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptCont, Search};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use termion::color;

impl Process {
    pub fn search(editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt) -> EvtProcess {
        Log::ep_s("Process.search");

        match editor.curt_evt {
            Key(KeyEvent { code, .. }) => match code {
                F(3) => {
                    Log::ep_s("search.F3");

                    if prom.cont.buf.len() == 0 {
                        mbar.set_not_entered_serach_str();
                    } else {
                        editor.search.str = prom.cont.buf.iter().collect::<String>();
                        mbar.clear();
                        prom.clear();
                        editor.search.index = Search::INDEX_UNDEFINED;
                    }
                    return EvtProcess::Next;
                }
                _ => return EvtProcess::Hold,
            },
            _ => return EvtProcess::Hold,
        }
    }
}

impl Prompt {
    pub fn search(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_search();
        self.cont = cont;
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_search_str.clone());
        self.key_desc = format!(
            "{}{}:{}F3  {}{}:{}Shift + F2  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.search_bottom_start.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.search_top.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.close.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }
}

impl MsgBar {
    pub fn set_not_entered_serach_str(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.not_entered_search_str.clone(),);
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }

    /*
    pub fn set_no_search_str_bottom(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.no_search_str_bottom.clone(),);
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }
    */
}
