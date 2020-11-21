use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use termion::color;

impl EvtAct {
    pub fn search<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.search");

        match editor.curt_evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(2) => {
                    Log::ep_s("search.Shift + F2");
                    EvtAct::exec_search(out, term, editor, mbar, prom, sbar, false);
                    return EvtActType::Next;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                F(3) => {
                    Log::ep_s("search.F3");
                    EvtAct::exec_search(out, term, editor, mbar, prom, sbar, true);
                    return EvtActType::Next;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }

    fn exec_search<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, is_asc: bool) {
        let search_str = prom.cont_1.buf.iter().collect::<String>();
        if search_str.len() == 0 {
            mbar.set_err(mbar.lang.not_entered_search_str.clone());
            mbar.draw_only(out);
        } else if editor.get_search_ranges(search_str.clone()).len() == 0 {
            mbar.set_err(mbar.lang.cannot_find_char_search_for.clone());
            mbar.draw_only(out);
        } else {
            mbar.clear();
            prom.clear();
            editor.search.clear();
            editor.search.str = search_str;
            // all redrowの為に検索処理実施
            editor.search_str(is_asc);
            // indexを初期値に戻す
            editor.search.index = Search::INDEX_UNDEFINED;
            term.draw(out, editor, mbar, prom, sbar).unwrap();
        }
    }
}

impl Prompt {
    pub fn search(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_search();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_search.clone());
        self.key_desc = format!(
            "{}{}:{}F3  {}{}:{}Shift + F2  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.search_bottom.clone(),
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