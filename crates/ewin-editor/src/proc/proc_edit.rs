use crate::{
    ewin_com::{_cfg::key::cmd::*, model::*, util::*},
    model::*,
};
use ewin_cfg::log::*;
use ewin_const::def::*;
use std::cmp::{max, min};

impl Editor {
    pub fn edit_proc(&mut self, cmd: Cmd) -> ActType {
        Log::debug_s("edit_proc");
        Log::debug("cmd", &cmd);

        // Editing Editor from other than KeyWhen::EditorFocus
        if self.cmd.cmd_type == CmdType::Null {
            self.cmd = cmd.clone();
        }
        let mut evt_proc = EvtProc::default();
        let is_selected_org = self.win_mgr.curt().sel.is_selected_width();
        let mut proc_del = Proc::default();

        // selected range delete
        Log::debug("self.sel.is_selected_width()", &self.win_mgr.curt().sel.is_selected_width());
        if self.win_mgr.curt().sel.is_selected_width() && self.cmd.config.is_edit {
            Log::debug("is_selected", &true);
            proc_del = Proc { cmd: if cmd.cmd_type == CmdType::DelNextChar { Cmd::to_cmd(CmdType::DelNextChar) } else { Cmd::to_cmd(CmdType::DelPrevChar) }, ..Proc::default() };
            proc_del.cur_s = Cur { y: self.win_mgr.curt().sel.sy, x: self.win_mgr.curt().sel.sx, disp_x: self.win_mgr.curt().sel.s_disp_x };
            proc_del.cur_e = self.win_mgr.curt().cur;
            match self.win_mgr.curt().sel.mode {
                SelMode::Normal => proc_del.str = self.buf.slice(self.win_mgr.curt().sel.get_range()),
                SelMode::BoxSelect => self.set_box_sel(&mut proc_del),
            }
            proc_del.sel = self.win_mgr.curt().sel;
            self.del_sel_range(&proc_del);
            if cmd.cmd_type != CmdType::Cut {
                self.win_mgr.curt().sel.clear();
            }
            // ep_del.draw_type = self.draw_range;
            evt_proc.sel_proc = Some(proc_del.clone());
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        match cmd.cmd_type {
            CmdType::DelNextChar | CmdType::DelPrevChar if is_selected_org => {}
            _ => {
                let mut proc = Proc { cmd: cmd.clone(), ..Proc::default() };
                proc.cur_s = self.win_mgr.curt().cur;
                self.set_box_sel_vec(&proc_del, &mut proc);

                let act_type = match &cmd.cmd_type {
                    CmdType::DelNextChar => self.delete(&mut proc),
                    CmdType::DelPrevChar => self.backspace(&mut proc),
                    CmdType::InsertRow => self.insert_row(),
                    CmdType::Cut => self.cut(proc_del),
                    CmdType::InsertStr(_) | CmdType::InsertBox(_) => {
                        if let CmdType::InsertStr(ref str) = cmd.cmd_type {
                            self.edit_proc_set_insert_str(str, &mut proc);
                        } else if let CmdType::InsertBox(ref box_sel_vec) = cmd.cmd_type {
                            proc.box_sel_vec = box_sel_vec.clone();
                        }
                        self.insert_str(&mut proc);
                        ActType::Next
                    }
                    CmdType::DelBox(box_sel_vec) => {
                        proc.box_sel_vec = box_sel_vec.clone();
                        self.undo_del_box(box_sel_vec);
                        ActType::Next
                    }
                    // In case of replace, only registration of Evt process
                    CmdType::ReplaceExec(search_str, replace_str, idx_set) => {
                        self.replace(&mut proc, search_str, replace_str, idx_set);
                        ActType::Next
                    }
                    _ => return ActType::Cancel,
                };
                if act_type != ActType::Next {
                    return act_type;
                }
                if cmd.cmd_type != CmdType::Cut {
                    proc.cur_e = self.win_mgr.curt().cur;
                    // ep.draw_type = self.draw_range;
                    evt_proc.proc = Some(proc.clone());
                }
            }
        }
        self.research();

        self.state.is_changed = true;
        self.set_change_info_edit(&evt_proc);

        // Register edit history
        if self.cmd.cmd_type != CmdType::Undo && self.cmd.cmd_type != CmdType::Redo {
            self.history.clear_redo_vec();
            self.history.undo_vec.push(evt_proc);
        }

        self.scroll();
        self.scroll_horizontal();

        return ActType::Next;
    }

    pub fn slice_box_sel(&mut self) -> (String, Vec<(SelRange, String)>) {
        let (sy, ey) = if self.win_mgr.curt().sel.is_selected() { (self.win_mgr.curt().sel.sy, self.win_mgr.curt().sel.ey) } else { (self.box_insert.vec.first().unwrap().0.sy, self.box_insert.vec.last().unwrap().0.sy) };

        let (sy, ey) = (min(sy, ey), max(sy, ey));
        let mut string = String::new();
        let mut vec: Vec<(SelRange, String)> = vec![];

        Log::debug("sy", &sy);
        Log::debug("ey", &ey);
        Log::debug("self.sel.get_range()", &self.win_mgr.curt().sel.get_range());

        for y in sy..=ey {
            let mut row_sel = self.win_mgr.curt().sel.get_range();
            let mut slice_str = String::new();
            if row_sel.is_selected_width() {
                let (slice_string, sx, ex) = self.get_disp_x_range_string(self.buf.char_vec_row(y));
                Log::debug("sx", &sx);
                Log::debug("ex", &ex);
                Log::debug("slice_string", &slice_string);
                slice_str = slice_string;
                row_sel.sx = sx;
                row_sel.ex = ex;
            } else {
                // Range selection with a width of 0
                row_sel.sx = self.win_mgr.curt().cur.x;
                row_sel.ex = self.win_mgr.curt().cur.x;
                row_sel.s_disp_x = self.win_mgr.curt().cur.disp_x;
                row_sel.e_disp_x = self.win_mgr.curt().cur.disp_x;
            }
            row_sel.sy = y;
            row_sel.ey = y;
            vec.push((row_sel, slice_str.clone()));

            string.push_str(&slice_str);
            string.push_str(&NL::get_nl(&self.h_file.nl));
        }

        Log::debug("string", &string);
        Log::debug("vec", &vec);

        (string, vec)
    }

    pub fn get_disp_x_range_string(&mut self, vec: Vec<char>) -> (String, usize, usize) {
        let (mut width, mut cur_x_s, mut cur_x_e) = (0, USIZE_UNDEFINED, 0);

        let sel = self.win_mgr.curt().sel.get_range();
        let mut rtn = String::new();
        for (idx, c) in vec.iter().enumerate() {
            if *c == NEW_LINE_LF || *c == NEW_LINE_CR {
                break;
            }
            let width_org = width;
            let c_len = get_char_width(c, width + self.win_mgr.curt().offset.disp_x);
            width += c_len;
            cur_x_e += 1;

            if sel.s_disp_x < width && width_org < sel.e_disp_x {
                rtn.push(*c);
                if cur_x_s == USIZE_UNDEFINED {
                    cur_x_s = idx;
                }
            }
            if width >= sel.e_disp_x {
                break;
            }
        }
        (rtn, cur_x_s, cur_x_e)
    }

    pub fn set_box_sel_vec(&mut self, ep_del: &Proc, ep: &mut Proc) {
        if self.box_insert.mode == BoxInsertMode::Insert || self.win_mgr.curt().sel.mode == SelMode::BoxSelect {
            if ep_del.box_sel_vec.is_empty() {
                if self.box_insert.mode == BoxInsertMode::Insert && !self.box_insert.vec.is_empty() {
                    ep.box_sel_vec = self.box_insert.vec.clone();
                    ep.str = self.box_insert.get_str(&NL::get_nl(&self.h_file.nl));
                }
            } else {
                ep.box_sel_vec = ep_del.box_sel_vec.clone();
            }
        }
    }

    pub fn exit_box_mode(&mut self) {
        Log::debug_key("exit_box_mode");
        self.win_mgr.curt().sel.mode = SelMode::Normal;
        self.box_insert.mode = BoxInsertMode::Normal;
    }

    pub fn edit_proc_set_insert_str(&mut self, str: &str, proc: &mut Proc) {
        Log::debug_key("edit_proc_set_insert_str");
        if self.box_insert.mode == BoxInsertMode::Insert {
            // paste
            if str.is_empty() {
                // highlight data reset
                if self.get_clipboard(proc) {
                    self.exit_box_mode();
                } else {
                    // Move cur.y to the beginning of a Box insert
                    self.win_mgr.curt().cur.y = self.box_insert.vec.first().unwrap().0.sy;
                }
                if !self.box_insert.vec.is_empty() {
                    self.set_box_str_vec("", proc);
                    self.box_insert.vec = proc.box_sel_vec.clone();
                }
                // Range selection with a width of 0
            } else {
                self.set_box_sel(proc);
                self.set_box_str_vec(str, proc);
                self.box_insert.vec = proc.box_sel_vec.clone();
            }
        } else {
            // paste
            if str.is_empty() {
                self.get_clipboard(proc);
            } else {
                proc.str = if str == TAB_CHAR.to_string() { get_tab_str() } else { str.to_string() };
            }
        }
    }

    pub fn del_sel_range(&mut self, ep: &Proc) {
        let sel = self.win_mgr.curt().sel.get_range();
        self.buf.remove_range(sel, ep);
        match self.win_mgr.curt().sel.mode {
            SelMode::Normal => self.set_cur_target_by_x(sel.sy, sel.sx, false),
            SelMode::BoxSelect => {
                let sel = ep.box_sel_vec.last().unwrap().0;
                self.set_cur_target_by_x(sel.sy, sel.sx, false);
            }
        }
    }

    pub fn set_box_sel(&mut self, ep: &mut Proc) {
        Log::debug_key("set_box_sel");

        let (slice_str, vec) = self.slice_box_sel();
        ep.str = slice_str;
        ep.box_sel_vec = vec;

        Log::debug("ep", &ep);
    }

    pub fn set_box_str_vec(&mut self, ins_str: &str, ep: &mut Proc) {
        for i in 0..=ep.box_sel_vec.len() - 1 {
            if !ins_str.is_empty() {
                ep.box_sel_vec[i].1 = ins_str.to_string();
            };
            ep.box_sel_vec[i].0.sx = self.win_mgr.curt().cur.x;
            ep.box_sel_vec[i].0.s_disp_x = self.win_mgr.curt().cur.disp_x;
            ep.box_sel_vec[i].0.ex = ep.box_sel_vec[i].0.sx + ins_str.chars().count();
            ep.box_sel_vec[i].0.e_disp_x = ep.box_sel_vec[i].0.s_disp_x + get_str_width(ins_str);
        }
    }
}
