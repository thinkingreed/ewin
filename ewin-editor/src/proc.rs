use crate::{ewin_core::_cfg::key::keycmd::*, model::*};

impl Editor {
    pub fn proc(&mut self) {
        let e_cmd = self.e_cmd.clone();
        match e_cmd {
            // edit
            E_Cmd::InsertStr(str) => self.edit_proc(E_Cmd::InsertStr(str)),
            E_Cmd::InsertLine => self.edit_proc(E_Cmd::InsertLine),
            E_Cmd::DelPrevChar => self.edit_proc(E_Cmd::DelPrevChar),
            E_Cmd::DelNextChar => self.edit_proc(E_Cmd::DelNextChar),
            E_Cmd::Cut => self.edit_proc(E_Cmd::Cut),
            E_Cmd::Copy => self.copy(),
            E_Cmd::Undo => self.undo(),
            E_Cmd::Redo => self.redo(),
            // cursor move
            E_Cmd::CursorUp | E_Cmd::MouseScrollUp | E_Cmd::CursorDown | E_Cmd::MouseScrollDown | E_Cmd::CursorLeft | E_Cmd::CursorRight | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd => self.cur_move_com(),
            E_Cmd::CursorFileHome => self.ctrl_home(),
            E_Cmd::CursorFileEnd => self.ctrl_end(),
            E_Cmd::CursorPageUp => self.page_up(),
            E_Cmd::CursorPageDown => self.page_down(),
            // select
            E_Cmd::CursorUpSelect | E_Cmd::CursorDownSelect | E_Cmd::CursorLeftSelect | E_Cmd::CursorRightSelect | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect => self.shift_move_com(),
            E_Cmd::AllSelect => self.all_select(),
            // Search
            E_Cmd::FindNext => self.search_str(true, false),
            E_Cmd::FindBack => self.search_str(false, false),
            // mouse
            E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeft(_, _) | E_Cmd::MouseDownBoxLeft(_, _) | E_Cmd::MouseDragBoxLeft(_, _) => self.ctrl_mouse(),
            E_Cmd::MouseOpeSwitch => self.ctrl_mouse_capture(),
            // Mode
            E_Cmd::CancelMode => self.cancel_mode(),
            E_Cmd::BoxSelectMode => self.box_select_mode(),
            // empty
            E_Cmd::Null => {}
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ewin_core::{_cfg::cfg::*, clipboard::*, def::*, log::*, model::*};

    #[test]
    fn test_editor_proc_base_edit() {
        Log::set_logger(&Some(CfgLog { level: Some("test".to_string()) }));
        let mut e = Editor::new();
        e.buf.insert_end(&EOF_MARK.to_string());

        // InsertStr
        e.e_cmd = E_Cmd::InsertStr("a".to_string());
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Copy
        e.sel.set_s(0, 0, 0);
        e.sel.set_e(0, 1, 1);
        e.e_cmd = E_Cmd::Copy;
        e.proc();
        let clipboard = get_clipboard().unwrap_or("".to_string());
        assert_eq!(clipboard, "a");

        // Paste
        e.e_cmd = E_Cmd::InsertStr("".to_string());
        e.proc();
        assert_eq!(e.buf.text.to_string(), "aa▚");
        assert_eq!(e.cur, Cur { y: 0, x: 2, disp_x: 2 });

        // Cut
        e.sel.set_s(0, 1, 1);
        e.sel.set_e(0, 2, 2);
        e.e_cmd = E_Cmd::Cut;
        e.proc();
        let clipboard = get_clipboard().unwrap_or("".to_string());
        assert_eq!(clipboard, "a");
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });
        e.sel.clear();

        // InsertLine
        e.e_cmd = E_Cmd::InsertLine;
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a\n▚");
        assert_eq!(e.cur, Cur { y: 1, x: 0, disp_x: 0 });

        // DelPrevChar
        e.e_cmd = E_Cmd::DelPrevChar;
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // DelNextChar
        e.e_cmd = E_Cmd::InsertStr("b".to_string());
        e.proc();
        assert_eq!(e.buf.text.to_string(), "ab▚");
        assert_eq!(e.cur, Cur { y: 0, x: 2, disp_x: 2 });
        e.e_cmd = E_Cmd::CursorLeft;
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });
        e.e_cmd = E_Cmd::DelNextChar;
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Undo
        e.e_cmd = E_Cmd::Undo;
        e.proc();
        assert_eq!(e.buf.text.to_string(), "ab▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Redo
        e.e_cmd = E_Cmd::Redo;
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });
    }

    #[test]
    fn test_editor_proc_base_cur_move() {
        Log::set_logger(&Some(CfgLog { level: Some("test".to_string()) }));
        let mut e = Editor::new();
        e.buf.insert_end(EOF_MARK_STR);

        // CursorLeft
        e.e_cmd = E_Cmd::InsertStr("a".to_string());
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        e.e_cmd = E_Cmd::CursorLeft;
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 0, disp_x: 0 });
        // CursorRight
        e.e_cmd = E_Cmd::CursorRight;
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // E_Cmd::CursorUp
        e.e_cmd = E_Cmd::InsertLine;
        e.proc();
        e.e_cmd = E_Cmd::CursorUp;
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 0, disp_x: 0 });
        // E_Cmd::CursorDown
        e.e_cmd = E_Cmd::CursorDown;
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 0, disp_x: 0 });

        // CursorRowHome
        e.e_cmd = E_Cmd::InsertStr("abc".to_string());
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 3, disp_x: 3 });
        e.e_cmd = E_Cmd::CursorRowHome;
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 0, disp_x: 0 });
        // CursorRowEnd
        e.e_cmd = E_Cmd::CursorRowEnd;
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 3, disp_x: 3 });
    }

    #[test]
    fn test_editor_proc_base_select() {
        Log::set_logger(&Some(CfgLog { level: Some("test".to_string()) }));
        let mut e = Editor::new();
        e.buf.insert_end(&EOF_MARK.to_string());
        e.e_cmd = E_Cmd::InsertStr("123\nabc\nABC".to_string());
        e.proc();

        // CursorUpSelect
        e.e_cmd = E_Cmd::CursorUpSelect;
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { sy: 1, sx: 3, ey: 2, ex: 3, s_disp_x: 3, e_disp_x: 3, ..SelRange::default() });

        // CursorLeftSelect
        e.e_cmd = E_Cmd::CursorLeftSelect;
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { sy: 1, sx: 2, ey: 2, ex: 3, s_disp_x: 2, e_disp_x: 3, ..SelRange::default() });

        // CursorRightSelect
        e.e_cmd = E_Cmd::CursorRightSelect;
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { sy: 1, sx: 3, ey: 2, ex: 3, s_disp_x: 3, e_disp_x: 3, ..SelRange::default() });

        // CursorDownSelect
        e.e_cmd = E_Cmd::CursorDownSelect;
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { ..SelRange::default() });
    }

    #[test]
    fn test_editor_proc_base_find_next_back() {
        Log::set_logger(&Some(CfgLog { level: Some("test".to_string()) }));
        Cfg::init(&Args { ..Args::default() });

        let mut e = Editor::new();
        e.buf.insert_end(&EOF_MARK.to_string());
        e.e_cmd = E_Cmd::InsertStr("123\nabc\nABC\nabc".to_string());
        e.proc();

        println!("{}", e.cur);

        // FindNext
        e.search.str = "b".to_string();
        e.e_cmd = E_Cmd::FindNext;
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 1, disp_x: 1 });
        assert_eq!(e.search, Search { idx: 0, ranges: vec![SearchRange { y: 1, sx: 1, ex: 2 }, SearchRange { y: 3, sx: 1, ex: 2 }], str: "b".to_string(), ..Search::default() });

        // FindBack
        e.e_cmd = E_Cmd::FindNext;
        e.proc();
        assert_eq!(e.cur, Cur { y: 3, x: 1, disp_x: 2 });
        assert_eq!(e.search, Search { idx: 1, ranges: vec![SearchRange { y: 1, sx: 1, ex: 2 }, SearchRange { y: 3, sx: 1, ex: 2 }], str: "b".to_string(), ..Search::default() });
    }
}
