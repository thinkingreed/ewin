use crate::model::Editor;
use ewin_cfg::{lang::lang_cfg::*, log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*, file::SaveFileType, types::*};
use ewin_ctx_menu::view_traits::view_trait::*;
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_state::{tabs::editor::*, term::*};

impl Editor {
    pub fn proc(&mut self) -> ActType {
        Log::debug_key("Editor.proc");

        let cmd = self.cmd.clone();
        Log::debug("cmd", &cmd);

        let act_type = match cmd.cmd_type {
            CmdType::CancelEditorState => return self.cancel_state(),
            // _input_imple
            _ if self.is_input_imple_mode(true) => self.ctrl_input_comple(),
            // edit
            CmdType::InsertStr(_) => self.edit_proc(cmd),
            // GrepResult tgt file confirm
            CmdType::Confirm => self.grep_result(),
            CmdType::InsertRow => self.edit_proc(cmd),
            CmdType::DelPrevChar => self.edit_proc(cmd),
            CmdType::DelNextChar => self.edit_proc(cmd),
            CmdType::Cut => self.edit_proc(cmd),
            CmdType::Copy => self.copy(),
            CmdType::Undo => self.undo(),
            CmdType::Redo => self.redo(),
            CmdType::SaveFile(ref save_type) => {
                let act_type = self.save(save_type);
                if cmd.cmd_type == CmdType::SaveFile(SaveFileType::NewFile) {
                    self.enable_syntax_highlight();
                    State::get().curt_mut_state().clear();
                }
                return act_type;
            }
            // Search
            CmdType::FindNext | CmdType::FindBack => self.find_next_back(),
            // If CtxMenu = true and there is no Mouse on CtxMenu
            CmdType::MouseMove(_, _) => return ActType::Cancel,
            // ctx_menu
            CmdType::CtxMenu(y, x) => self.init_ctx_menu(y, x),
            // InputComple
            CmdType::InputComple => self.init_input_comple(true),
            // format
            CmdType::Format(fmt_type) => return self.format(fmt_type),
            // convert
            CmdType::Convert(conv_type) => return self.convert(conv_type),
            // key record
            CmdType::RecordKeyStartEnd => return self.record_key_macro_start(),
            CmdType::ExecRecordKey => return self.exec_key_macro(),
            CmdType::Search(search_type, search_str) => match search_type {
                SearchType::Incremental => return self.search_incremental(search_str),
                SearchType::Confirm => {
                    let act_type = self.search_confirm(search_str);
                    if act_type != ActType::Next {
                        return act_type;
                    }
                    Job::send_cmd(CmdType::ClearTabState(false));
                    return ActType::None;
                }
            },
            CmdType::ReplaceTryExec(search_str, replace_str) => {
                let cfg_search = CfgEdit::get_search();
                let end_idx = if cfg_search.regex { self.buf.len_bytes() } else { self.buf.len_chars() };

                let idx_set = self.buf.search(&search_str, 0, end_idx, &cfg_search);
                if idx_set.is_empty() {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().cannot_find_search_char.to_string()));
                }
                self.edit_proc_cmd_type(CmdType::ReplaceExec(search_str, replace_str, idx_set));

                State::get().curt_mut_state().clear();
                return ActType::Draw(DrawParts::TabsAll);
            }

            /*
             * display
             */
            CmdType::SwitchDispScale => {
                State::get().curt_mut_state().editor.toggle_state(TabsEditerStateType::Scale);
                return ActType::Draw(DrawParts::TabsAll);
            }
            CmdType::SwitchDispRowNo => {
                State::get().curt_mut_state().editor.toggle_state(TabsEditerStateType::RowNo);
                return ActType::Draw(DrawParts::TabsAll);
            }
            CmdType::WindowSplit(split_type) => {
                self.win_mgr.split_window(split_type);
                self.resize_draw_vec();
                self.set_size_adjust_editor();
                return ActType::Draw(DrawParts::TabsAll);
            }
            CmdType::MoveRow(row_idx) => {
                self.set_cur_target_by_x(row_idx - 1, 0, false);
                self.cmd = Cmd::to_cmd(cmd.cmd_type);
                self.scroll();
                return ActType::Draw(DrawParts::TabsAll);
            }

            CmdType::Null => return ActType::Draw(DrawParts::TabsAll),
            CmdType::Test => return ActType::None,

            _ => ActType::Next,
        };
        if act_type != ActType::Next {
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
            CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) => self.ctrl_mouse(),
            CmdType::MouseModeSwitch => self.ctrl_mouse_capture(),
            // Mode
            CmdType::BoxSelectMode => self.box_select_mode(),
            // empty
            _ => {}
        };
        Log::debug(" self.sel_org == self.sel", &(self.win_mgr.curt_ref().sel_org == self.win_mgr.curt_ref().sel));
        return ActType::Next;
    }
}

#[cfg(test)]
mod tests {
    use crate::model::BoxInsertMode;

    use super::*;
    use ewin_cfg::model::{
        general::default::{Cfg, CfgLog},
        modal::AppArgs,
    };
    use ewin_key::{
        clipboard::*,
        cur::Cur,
        key::cmd::Cmd,
        model::{Search, SearchRange},
        sel_range::{SelMode, SelRange},
    };

    #[test]
    fn test_editor_proc_base_edit() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        let mut e = Editor::new();

        // InsertStr
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("a".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Copy
        e.win_mgr.curt().sel.set_s(0, 0, 0);
        e.win_mgr.curt().sel.set_e(0, 1, 1);
        e.cmd = Cmd::to_cmd(CmdType::Copy);
        e.proc();
        let clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());
        assert_eq!(clipboard, "a");

        // Paste
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "aa▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 2, disp_x: 2 });

        // Cut
        e.win_mgr.curt().sel.set_s(0, 1, 1);
        e.win_mgr.curt().sel.set_e(0, 2, 2);
        e.cmd = Cmd::to_cmd(CmdType::Cut);
        e.proc();
        let clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());
        assert_eq!(clipboard, "a");
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });
        e.win_mgr.curt().sel.clear();

        // InsertLine
        e.cmd = Cmd::to_cmd(CmdType::InsertRow);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a\n▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 0, disp_x: 0 });

        // DelPrevChar
        e.cmd = Cmd::to_cmd(CmdType::DelPrevChar);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });

        // DelNextChar
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("b".to_string()));
        e.proc();
        assert_eq!(e.buf.text.to_string(), "ab▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 2, disp_x: 2 });
        e.cmd = Cmd::to_cmd(CmdType::CursorLeft);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });
        e.cmd = Cmd::to_cmd(CmdType::DelNextChar);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Undo
        e.cmd = Cmd::to_cmd(CmdType::Undo);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "ab▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Redo
        e.cmd = Cmd::to_cmd(CmdType::Redo);
        e.proc();
        assert_eq!(e.buf.text.to_string(), "a▚");
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });
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
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 0, disp_x: 0 });
        // CursorRight
        e.cmd = Cmd::to_cmd(CmdType::CursorRight);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 1, disp_x: 1 });

        // Cmd::to_cmd(CmdType::CursorUp
        e.cmd = Cmd::to_cmd(CmdType::InsertRow);
        e.proc();
        e.cmd = Cmd::to_cmd(CmdType::CursorUp);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 0, x: 0, disp_x: 0 });
        // Cmd::to_cmd(CmdType::CursorDown
        e.cmd = Cmd::to_cmd(CmdType::CursorDown);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 0, disp_x: 0 });

        // CursorRowHome
        e.cmd = Cmd::to_cmd(CmdType::InsertStr("abc".to_string()));
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 3, disp_x: 3 });
        e.cmd = Cmd::to_cmd(CmdType::CursorRowHome);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 0, disp_x: 0 });
        // CursorRowEnd
        e.cmd = Cmd::to_cmd(CmdType::CursorRowEnd);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 3, disp_x: 3 });
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
        assert_eq!(e.win_mgr.curt().sel.get_range(), SelRange { sy: 1, sx: 3, ey: 2, ex: 3, s_disp_x: 3, e_disp_x: 3, ..SelRange::default() });

        // CursorLeftSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorLeftSelect);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel.get_range(), SelRange { sy: 1, sx: 2, ey: 2, ex: 3, s_disp_x: 2, e_disp_x: 3, ..SelRange::default() });

        // CursorRightSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorRightSelect);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel.get_range(), SelRange { sy: 1, sx: 3, ey: 2, ex: 3, s_disp_x: 3, e_disp_x: 3, ..SelRange::default() });

        // CursorDownSelect
        e.cmd = Cmd::to_cmd(CmdType::CursorDownSelect);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel.get_range(), SelRange { ..SelRange::default() });
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
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 1, disp_x: 1 });
        assert_eq!(e.search, Search { idx: 0, ranges: vec![SearchRange { y: 1, sx: 1, ex: 2 }, SearchRange { y: 3, sx: 1, ex: 2 }], str: "b".to_string(), ..Search::default() });

        // FindBack
        e.cmd = Cmd::to_cmd(CmdType::FindBack);
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 3, x: 1, disp_x: 1 });
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
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 2, x: 1, disp_x: 1 });

        // MouseDragLeft
        // TODO MouseDragLeftDown, MouseDragLeftUp
        e.cmd = Cmd::to_cmd(CmdType::MouseDragLeftDown(4, 4));
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 3, x: 2, disp_x: 2 });
        assert_eq!(e.win_mgr.curt().sel.get_range(), SelRange { sy: 2, sx: 1, s_disp_x: 1, ey: 3, ex: 2, e_disp_x: 2, ..SelRange::default() });
        e.win_mgr.curt().sel.clear();

        // MouseDownBoxLeft
        e.cmd = Cmd::to_cmd(CmdType::MouseDownLeftBox(2, 3));
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 1, x: 1, disp_x: 1 });
        assert_eq!(e.win_mgr.curt().sel.mode, SelMode::BoxSelect);

        // MouseDragBoxLeft
        e.cmd = Cmd::to_cmd(CmdType::MouseDragLeftBox(3, 4));
        e.proc();
        assert_eq!(e.win_mgr.curt().cur, Cur { y: 2, x: 2, disp_x: 2 });
        assert_eq!(e.win_mgr.curt().sel.get_range(), SelRange { mode: SelMode::BoxSelect, sy: 1, sx: 1, s_disp_x: 1, ey: 2, ex: 2, e_disp_x: 2 });

        // MouseModeSwitch
        /*
        e.cmd = Cmd::to_cmd(CmdType::MouseModeSwitch);
        e.proc();
        assert_eq!(e.state.mouse, Mouse::Disable);
        assert_eq!(e.rnw, 0);
        e.cmd = Cmd::to_cmd(CmdType::MouseModeSwitch);
        e.proc();
        assert_eq!(e.state.mouse, Mouse::Enable);
        assert_eq!(e.rnw, e.buf.len_rows().to_string().len());
         */
        // BoxSelectMode
        e.cmd = Cmd::to_cmd(CmdType::BoxSelectMode);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel.mode, SelMode::Normal);
        e.cmd = Cmd::to_cmd(CmdType::BoxSelectMode);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel.mode, SelMode::BoxSelect);
        e.box_insert.mode = BoxInsertMode::Insert;

        // CancelMode
        e.cmd = Cmd::to_cmd(CmdType::CancelEditorState);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel.mode, SelMode::Normal);
        assert_eq!(e.box_insert.mode, BoxInsertMode::Normal);
        //   select
        e.cmd = Cmd::to_cmd(CmdType::CursorLeftSelect);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel, SelRange { sy: 2, sx: 2, s_disp_x: 2, ey: 2, ex: 1, e_disp_x: 1, ..SelRange::default() });
        e.cmd = Cmd::to_cmd(CmdType::CancelEditorState);
        e.proc();
        assert_eq!(e.win_mgr.curt().sel, SelRange { ..SelRange::default() });
    }
}
