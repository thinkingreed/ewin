use crate::{colors::*, def::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;

impl EvtAct {
    pub fn search<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.search");

        match editor.evt {
            Key(KeyEvent { modifiers: KeyModifiers::ALT, code }) => match code {
                Char('c') => {
                    prom.cont_1.opt_1.toggle_check();

                    // let cfg = CFG.get().unwrap().lock();
                    CFG.get().unwrap().lock().map(|mut cfg| cfg.colors.editor.fg = Color { rgb: Rgb { r: 55 as u8, g: 55 as u8, b: 55 as u8 } }).unwrap();
                    Log::ep("cfg.colors.editor.fg", &CFG.get().unwrap().lock().unwrap().colors.editor.fg);

                    return EvtActType::Hold;
                }
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) => return EvtAct::exec_search_confirm(out, editor, mbar, prom, help, sbar),
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                F(3) => return EvtAct::exec_search_confirm(out, editor, mbar, prom, help, sbar),
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }

    fn exec_search_confirm<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("exec_search_confirm");
        let search_str = prom.cont_1.buf.iter().collect::<String>();
        if search_str.len() == 0 {
            mbar.set_err(&LANG.not_entered_search_str);
            return EvtActType::Hold;
        }
        let search_vec = editor.get_search_ranges(&search_str.clone(), 0, editor.buf.len_chars());
        if search_vec.len() == 0 {
            mbar.set_err(&LANG.cannot_find_char_search_for);
            return EvtActType::Hold;
        } else {
            Log::ep_s("exec_search    !!!");

            editor.search.clear();
            editor.search.ranges = search_vec;
            editor.search.str = search_str;
            // Set index to initial value
            editor.search.idx = USIZE_UNDEFINED;

            prom.clear();
            mbar.clear();
            editor.d_range.draw_type = DrawType::All;
            return EvtActType::Next;
        }
    }
    pub fn exec_search_incremental<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) {
        Log::ep_s("exec_search_incremental");
        editor.search.str = prom.cont_1.buf.iter().collect::<String>();

        let s_idx = editor.buf.line_to_char(editor.offset_y);
        let ey = editor.offset_y + editor.disp_row_num;
        let ranges = editor.search.ranges.clone();
        editor.search.ranges = if editor.search.str.len() == 0 { vec![] } else { editor.get_search_ranges(&editor.search.str, s_idx, editor.buf.line_to_char(ey)) };

        if !ranges.is_empty() || !editor.search.ranges.is_empty() {
            // Search in advance for drawing
            editor.search_str(true, true);

            editor.d_range.draw_type = DrawType::All;
            Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
        }
    }
}

impl Prompt {
    pub fn search(&mut self) {
        self.is_search = true;
        self.disp_row_num = 4;
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
        let key = format!("{}{}:{}Alt + c{}", Colors::get_default_fg(), &LANG.case_sens, Colors::get_msg_warning_fg(), Colors::get_default_fg(),);
        let opt = PromptContOpt {
            key: key,
            is_check: CFG.get().unwrap().lock().unwrap().general.editor.search.case_sens,
        };
        self.opt_1 = opt;
    }
}
