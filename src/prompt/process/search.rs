use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptCont, Search, StatusBar, Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;
use termion::color;

impl Process {
    pub fn search<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        Log::ep_s("Process.search");

        match editor.curt_evt {
            Key(KeyEvent { code, .. }) => match code {
                F(3) => {
                    Log::ep_s("search.F3");

                    let search_str = prom.cont.buf.iter().collect::<String>();
                    if search_str.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_search_str.clone());
                        mbar.draw(out);
                    } else if editor.get_search_ranges(search_str.clone()).len() == 0 {
                        mbar.set_err(mbar.lang.cannot_find_char_search_for.clone());
                        mbar.draw(out);
                    } else {
                        mbar.clear();
                        prom.clear();
                        editor.search.clear();
                        editor.search.str = search_str;
                        // all redrowの為に検索処理実施
                        editor.search_str(true);
                        // indexを初期値に戻す
                        editor.search.index = Search::INDEX_UNDEFINED;
                        terminal.draw(out, editor, mbar, prom, sbar).unwrap();
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
