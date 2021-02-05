use crate::{colors::*, def::*, global::*, model::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;

impl EvtAct {
    pub fn search<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.search");

        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) => {
                    Log::ep_s("search.Shift + F4");
                    EvtAct::exec_search(out, editor, mbar, prom, sbar, false);
                    return EvtActType::Next;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                F(3) => {
                    Log::ep_s("search.F3");
                    if EvtAct::exec_search(out, editor, mbar, prom, sbar, true) {
                        return EvtActType::Next;
                    } else {
                        return EvtActType::Hold;
                    }
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }

    fn exec_search<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, is_asc: bool) -> bool {
        Log::ep_s("exec_search");
        let search_str = prom.cont_1.buf.iter().collect::<String>();
        if search_str.len() == 0 {
            mbar.set_err(&LANG.not_entered_search_str);
            mbar.draw_only(out, editor, prom, sbar);
            prom.draw_only(out);
            return false;
        }
        let search_vec = editor.get_search_ranges(&search_str.clone(), "");
        if search_vec.len() == 0 {
            mbar.set_err(&LANG.cannot_find_char_search_for);
            mbar.draw_only(out, editor, prom, sbar);
            prom.draw_only(out);
            return false;
        } else {
            Log::ep_s("exec_search    !!!");

            mbar.clear();
            prom.clear();
            editor.search.clear();
            editor.search.ranges = search_vec;
            editor.search.str = search_str;
            // all redrowの為に検索処理実施
            editor.search_str(is_asc);
            // indexを初期値に戻す
            editor.search.index = USIZE_UNDEFINED;
            Terminal::draw(out, editor, mbar, prom, sbar).unwrap();
            return true;
        }
    }
}

impl Prompt {
    pub fn search(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new();
        cont.set_search();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_search);
        self.key_desc = format!(
            "{}{}:{}F3  {}{}:{}Shift + F4  {}{}:{}Ctrl + c{}",
            Colors::get_default_fg(),
            &LANG.search_bottom,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.search_top,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.close,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );
    }
}
