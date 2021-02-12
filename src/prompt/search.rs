use crate::{colors::*, def::*, global::*, help::*, model::*, statusbar::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::cmp::min;
use std::io::Write;

impl EvtAct {
    pub fn search<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.search");

        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) => {
                    Log::ep_s("search.Shift + F4");
                    EvtAct::exec_search_confirm(out, editor, mbar, prom, help, sbar, false);
                    return EvtActType::Next;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                F(3) => {
                    Log::ep_s("search.F3");
                    if EvtAct::exec_search_confirm(out, editor, mbar, prom, help, sbar, true) {
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

    fn exec_search_confirm<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar, is_asc: bool) -> bool {
        Log::ep_s("exec_search_confirm");
        let search_str = prom.cont_1.buf.iter().collect::<String>();
        if search_str.len() == 0 {
            mbar.set_err(&LANG.not_entered_search_str);
            mbar.draw_only(out);
            prom.draw_only(out);
            return false;
        }
        let search_vec = editor.get_search_ranges(&search_str.clone(), 0, editor.buf.len_chars());
        if search_vec.len() == 0 {
            mbar.set_err(&LANG.cannot_find_char_search_for);
            mbar.draw_only(out);
            prom.draw_only(out);
            return false;
        } else {
            Log::ep_s("exec_search    !!!");

            editor.search.clear();
            editor.search.ranges = search_vec;
            editor.search.str = search_str;
            // all redrowの為に検索処理実施
            editor.search_str(is_asc);
            // indexを初期値に戻す
            editor.search.index = USIZE_UNDEFINED;
            Terminal::init_draw(out, editor, mbar, prom, help, sbar);
            return true;
        }
    }
    pub fn exec_search_incremental<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) {
        Log::ep_s("exec_search_incremental");
        editor.search.str = prom.cont_1.buf.iter().collect::<String>();

        let s_idx = editor.buf.line_to_char(editor.offset_y);
        let ey = editor.offset_y + editor.disp_row_num - 1;
        editor.search.ranges = if editor.search.str.len() == 0 { vec![] } else { editor.get_search_ranges(&editor.search.str, s_idx, editor.buf.line_to_char(ey)) };

        // Search in advance for drawing
        editor.search_str(true);

        editor.d_range.draw_type = DrawType::All;
        Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
    }
}

impl Prompt {
    pub fn search(&mut self) {
        self.is_search = true;
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
