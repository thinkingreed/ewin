use crate::{ewin_com::model::*, model::*};
use ewin_cfg::{global::*, log::*, model::default::*};
use ewin_com::_cfg::key::cmd::CmdType;

impl Editor {
    pub fn proc(&mut self) -> ActType {
        Log::debug_key("Editor.proc");

        let cmd = self.cmd.clone();
        Log::debug("cmd", &cmd);

        let act_type = match cmd.cmd_type {
            // edit
            CmdType::InsertStr(_) => self.edit_proc(cmd),
            CmdType::InsertRow => self.edit_proc(cmd),
            CmdType::DelPrevChar => self.edit_proc(cmd),
            CmdType::DelNextChar => self.edit_proc(cmd),
            CmdType::Cut => self.edit_proc(cmd),
            CmdType::Copy => self.copy(),
            CmdType::Undo => self.undo(),
            CmdType::Redo => self.redo(),
            // Search
            CmdType::FindNext | CmdType::FindBack => self.find_next_back(),
            // If CtxMenu = true and there is no Mouse on CtxMenu
            CmdType::MouseMove(_, _) => return ActType::Cancel,
            // InputComple
            CmdType::InputComple => self.init_input_comple(true),
            /*
             * display
             */
            CmdType::SwitchDispScale => {
                CFG_EDIT.get().unwrap().try_lock().map(|mut cfg| cfg.general.editor.scale.is_enable = !cfg.general.editor.scale.is_enable).unwrap();
                return ActType::Draw(DParts::All);
            }
            CmdType::SwitchDispRowNo => {
                CfgEdit::switch_editor_row_no_enable();
                return ActType::Draw(DParts::All);
            }
            CmdType::Null => return ActType::Draw(DParts::All),
            _ => ActType::Cancel,
        };
        if act_type != ActType::Cancel {
            return act_type;
        }
        let cmd = self.cmd.clone();
        match cmd.cmd_type {
            // cursor move
            CmdType::CursorUp | CmdType::MouseScrollUp | CmdType::CursorDown | CmdType::MouseScrollDown | CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorFileHome | CmdType::CursorFileEnd | CmdType::CursorPageUp | CmdType::CursorPageDown => {
                self.cur_move_com();
            }
            // select
            CmdType::CursorUpSelect | CmdType::CursorDownSelect | CmdType::CursorLeftSelect | CmdType::CursorRightSelect | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => self.shift_move_com(),
            CmdType::AllSelect => self.all_select(),
            // mouse
            CmdType::MouseDownLeft(_, _) | CmdType::MouseUpLeft(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) => self.ctrl_mouse(),
            CmdType::MouseModeSwitch => self.ctrl_mouse_capture(),
            // Mode
            CmdType::BoxSelectMode => self.box_select_mode(),
            // empty
            _ => {}
        };
        Log::debug(" self.sel_org == self.sel", &(self.sel_org == self.sel));
        return ActType::Next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ewin_cfg::model::{
        default::{Cfg, CfgLog},
        modal::AppArgs,
    };
    use ewin_com::{_cfg::key::cmd::Cmd, clipboard::*};

    #[test]
    fn test_editor_proc_base_edit() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        let mut e = Editor::new();

        // InsertStr
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("a".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Copy
        e.sel.set_s(0, 0, 0);
        e.sel.set_e(0, 1, 1);
        e.cmd = Cmd::to_cmd(CmdType::Copy);
        e.proc();
        let clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());
        assert_eq!(clipboard, "a");

        // Paste
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "aa▚");
        assert_eq!(e.cur, Cur { y: 0, x: 2, disp_x: 2 });

        // Cut
        e.sel.set_s(0, 1, 1);
        e.sel.set_e(0, 2, 2);
        e.cmd = Cmd::to_cmd(CmdType::Cut);
        e.proc();
        let clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());
        assert_eq!(clipboard, "a");
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });
        e.sel.clear();

        // InsertLine
        e.cmd = Cmd::to_cmd(CmdType::InsertRow);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a\n▚");
        assert_eq!(e.cur, Cur { y: 1, x: 0, disp_x: 0 });

        // DelPrevChar
        e.cmd = Cmd::to_cmd(CmdType::DelPrevChar);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // DelNextChar
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("b".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "ab▚");
        assert_eq!(e.cur, Cur { y: 0, x: 2, disp_x: 2 });
        e.cmd = Cmd::to_cmd(CmdType::CursorLeft);
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });
        e.cmd = Cmd::to_cmd(CmdType::DelNextChar);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Undo
        e.cmd = Cmd::to_cmd(CmdType::Undo);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "ab▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Redo
        e.cmd = Cmd::to_cmd(CmdType::Redo);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });
    }

    #[test]
    fn test_editor_proc_base_cur_move() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        let mut e = Editor::new();

        // CursorLeft
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("a".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        e.cmd = Cmd::to_cmd(CmdType::CursorLeft);
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 0, disp_x: 0 });
        // CursorRight
        e.cmd = Cmd::to_cmd(CmdType::CursorRight);
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Cmd::to_cmd(CmdType::CursorUp
        e.cmd = Cmd::to_cmd(CmdType::InsertRow);
        e.proc();
        e.cmd = Cmd::to_cmd(CmdType::CursorUp);
        e.proc();
        assert_eq!(e.cur, Cur { y: 0, x: 0, disp_x: 0 });
        // Cmd::to_cmd(CmdType::CursorDown
        e.cmd = Cmd::to_cmd(CmdType::CursorDown);
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 0, disp_x: 0 });

        // CursorRowHome
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("abc".to_string()));
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 3, disp_x: 3 });
        e.cmd = Cmd::to_cmd(CmdType::CursorRowHome);
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 0, disp_x: 0 });
        // CursorRowEnd
        e.cmd = Cmd::to_cmd(CmdType::CursorRowEnd);
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 3, disp_x: 3 });
    }

    #[test]
    fn test_editor_proc_base_select() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        let mut e = Editor::new();
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("123\nabc\nABC".to_string()));
        e.proc();

        // CursorUpSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorUpSelect);
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { sy: 1, sx: 3, ey: 2, ex: 3, s_disp_x: 3, e_disp_x: 3, ..SelRange::default() });

        // CursorLeftSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorLeftSelect);
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { sy: 1, sx: 2, ey: 2, ex: 3, s_disp_x: 2, e_disp_x: 3, ..SelRange::default() });

        // CursorRightSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorRightSelect);
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { sy: 1, sx: 3, ey: 2, ex: 3, s_disp_x: 3, e_disp_x: 3, ..SelRange::default() });

        // CursorDownSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorDownSelect);
        e.proc();
        assert_eq!(e.sel.get_range(), SelRange { ..SelRange::default() });
    }

    #[test]
    fn test_editor_proc_base_find_next_back() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        Cfg::init(&AppArgs { ..AppArgs::default() });

        let mut e = Editor::new();
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("123\nabc\nABC\nabc".to_string()));
        e.proc();

        // FindNext
        e.search.str = "b".to_string();
        e.cmd = Cmd::to_cmd(CmdType::FindNext);
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 1, disp_x: 1 });
        assert_eq!(e.search, Search { idx: 0, ranges: vec![SearchRange { y: 1, sx: 1, ex: 2 }, SearchRange { y: 3, sx: 1, ex: 2 }], str: "b".to_string(), ..Search::default() });

        // FindBack
        e.cmd = Cmd::to_cmd(CmdType::FindBack);
        e.proc();
        assert_eq!(e.cur, Cur { y: 3, x: 1, disp_x: 1 });
        assert_eq!(e.search, Search { idx: 1, ranges: vec![SearchRange { y: 1, sx: 1, ex: 2 }, SearchRange { y: 3, sx: 1, ex: 2 }], str: "b".to_string(), ..Search::default() });
    }

    #[test]
    fn test_editor_proc_base_mouse() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        Cfg::init(&AppArgs { ..AppArgs::default() });

        let mut e = Editor::new();
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("123\nabc\nABC\nabc".to_string()));
        e.proc();

        // MouseDownLeft
        e.cmd = Cmd::to_cmd(CmdType::MouseDownLeft(3, 3));
        e.proc();
        assert_eq!(e.cur, Cur { y: 2, x: 1, disp_x: 1 });

        // MouseDragLeft
        // TODO MouseDragLeftDown, MouseDragLeftUp
        e.cmd = Cmd::to_cmd(CmdType::MouseDragLeftDown(4, 4));
        e.proc();
        assert_eq!(e.cur, Cur { y: 3, x: 2, disp_x: 2 });
        assert_eq!(e.sel.get_range(), SelRange { sy: 2, sx: 1, s_disp_x: 1, ey: 3, ex: 2, e_disp_x: 2, ..SelRange::default() });
        e.sel.clear();

        // MouseDownBoxLeft
        e.cmd = Cmd::to_cmd(CmdType::MouseDownLeftBox(2, 3));
        e.proc();
        assert_eq!(e.cur, Cur { y: 1, x: 1, disp_x: 1 });
        assert_eq!(e.sel.mode, SelMode::BoxSelect);

        // MouseDragBoxLeft
        e.cmd = Cmd::to_cmd(CmdType::MouseDragLeftBox(3, 4));
        e.proc();
        assert_eq!(e.cur, Cur { y: 2, x: 2, disp_x: 2 });
        assert_eq!(e.sel.get_range(), SelRange { mode: SelMode::BoxSelect, sy: 1, sx: 1, s_disp_x: 1, ey: 2, ex: 2, e_disp_x: 2 });

        // MouseModeSwitch
        e.cmd = Cmd::to_cmd(CmdType::MouseModeSwitch);
        e.proc();
        assert_eq!(e.state.mouse, Mouse::Disable);
        assert_eq!(e.rnw, 0);
        e.cmd = Cmd::to_cmd(CmdType::MouseModeSwitch);
        e.proc();
        assert_eq!(e.state.mouse, Mouse::Enable);
        assert_eq!(e.rnw, e.buf.len_rows().to_string().len());

        // BoxSelectMode
        e.cmd = Cmd::to_cmd(CmdType::BoxSelectMode);
        e.proc();
        assert_eq!(e.sel.mode, SelMode::Normal);
        e.cmd = Cmd::to_cmd(CmdType::BoxSelectMode);
        e.proc();
        assert_eq!(e.sel.mode, SelMode::BoxSelect);
        e.box_insert.mode = BoxInsertMode::Insert;

        // CancelMode
        e.cmd = Cmd::to_cmd(CmdType::CancelEditorState);
        e.proc();
        assert_eq!(e.sel.mode, SelMode::Normal);
        assert_eq!(e.box_insert.mode, BoxInsertMode::Normal);
        //   select
        e.cmd = Cmd::to_cmd(CmdType::CursorLeftSelect);
        e.proc();
        assert_eq!(e.sel, SelRange { sy: 2, sx: 2, s_disp_x: 2, ey: 2, ex: 1, e_disp_x: 1, ..SelRange::default() });
        e.cmd = Cmd::to_cmd(CmdType::CancelEditorState);
        e.proc();
        assert_eq!(e.sel, SelRange { ..SelRange::default() });
    }
}
