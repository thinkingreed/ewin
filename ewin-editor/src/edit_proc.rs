use crate::{ewin_core::_cfg::keys::KeyCmd, ewin_core::def::*, ewin_core::log::Log, ewin_core::model::*, ewin_core::util::*, model::*};

impl Editor {
    pub fn edit_proc(&mut self, keycmd: KeyCmd) {
        Log::debug_s("edit_proc");
        if self.check_evtproc(&keycmd) {
            return;
        }
        let mut evt_proc = EvtProc::default();
        let is_selected_org = self.sel.is_selected_width();
        let mut ep_del = Proc::default();

        // selected range delete
        if self.sel.is_selected_width() && self.is_edit_del_keycmd(&keycmd) {
            ep_del = Proc { keycmd: if keycmd == KeyCmd::DeleteNextChar { KeyCmd::DeleteNextChar } else { KeyCmd::DeletePrevChar }, ..Proc::default() };
            ep_del.cur_s = Cur { y: self.sel.sy, x: self.sel.sx, disp_x: self.sel.s_disp_x };
            ep_del.cur_e = self.cur;
            match self.sel.mode {
                SelMode::Normal => ep_del.str = self.buf.slice(self.sel.get_range()),
                SelMode::BoxSelect => self.set_box_sel(&mut ep_del),
            }
            ep_del.sel = self.sel;
            self.del_sel_range(&ep_del);
            self.sel.clear();
            self.set_draw_range_each_process(DrawType::After(self.cur.y));
            ep_del.draw_type = self.draw_type;
            evt_proc.sel_proc = Some(ep_del.clone());
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        match keycmd {
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar if is_selected_org => {}
            _ => {
                let mut ep = Proc { keycmd: keycmd.clone(), ..Proc::default() };

                ep.cur_s = self.cur;
                self.set_box_sel_vec(&ep_del, &mut ep);

                match &keycmd {
                    KeyCmd::InsertStr(str) => self.edit_proc_set_insert_str(str, &mut ep),
                    KeyCmd::InsertBox(box_sel_vec) => ep.box_sel_vec = box_sel_vec.clone(),
                    _ => {}
                }
                match &keycmd {
                    KeyCmd::DeleteNextChar => self.delete(&mut ep),
                    KeyCmd::DeletePrevChar => self.backspace(&mut ep),
                    KeyCmd::InsertLine => self.enter(),
                    KeyCmd::Cut => self.cut(ep_del),
                    KeyCmd::InsertStr(_) | KeyCmd::InsertBox(_) => self.insert_str(&mut ep),
                    KeyCmd::DelBox(box_sel_vec) => self.undo_del_box(&box_sel_vec),
                    // In case of replace, only registration of Evt process
                    KeyCmd::ReplaceExec(is_regex, replace_str, search_map) => self.replace(&mut ep, *is_regex, replace_str.clone(), search_map.clone()),
                    _ => {}
                }
                if keycmd != KeyCmd::Cut {
                    ep.cur_e = self.cur;
                    ep.draw_type = self.draw_type;
                    evt_proc.evt_proc = Some(ep.clone());
                }
            }
        }
        // Register edit history
        if self.keycmd != KeyCmd::Undo && self.keycmd != KeyCmd::Redo {
            self.history.undo_vec.push(evt_proc);
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn slice_box_sel(&mut self) -> (String, Vec<(SelRange, String)>) {
        let (sy, ey) = if self.sel.is_selected() { (self.sel.sy, self.sel.ey) } else { (self.box_insert.vec.first().unwrap().0.sy, self.box_insert.vec.last().unwrap().0.sy) };

        let mut string = String::new();
        let mut vec: Vec<(SelRange, String)> = vec![];

        for y in sy..=ey {
            let mut row_sel = self.sel.clone();
            let mut slice_str = String::new();
            if row_sel.is_selected_width() {
                let (slice_string, sx, ex) = self.get_disp_x_range_string(self.buf.char_vec_line(y));
                slice_str = slice_string;
                row_sel.sx = sx;
                row_sel.ex = ex;
            } else {
                // Range selection with a width of 0
                row_sel.sx = self.cur.x;
                row_sel.ex = self.cur.x;
                row_sel.s_disp_x = self.cur.disp_x;
                row_sel.e_disp_x = self.cur.disp_x;
            }
            row_sel.sy = y;
            row_sel.ey = y;
            vec.push((row_sel, slice_str.clone()));

            string.push_str(&slice_str);
            string.push_str(&&NL::get_nl(&self.h_file.nl));
        }

        return (string, vec);
    }

    pub fn get_disp_x_range_string(&mut self, vec: Vec<char>) -> (String, usize, usize) {
        let (mut width, mut cur_x_s, mut cur_x_e) = (0, USIZE_UNDEFINED, 0);

        let mut rtn = String::new();
        for (idx, c) in vec.iter().enumerate() {
            if *c == EOF_MARK || *c == NEW_LINE_LF || *c == NEW_LINE_CR {
                break;
            }
            let width_org = width;
            let c_len = get_char_width(&c, width + self.offset_disp_x);
            width += c_len;
            cur_x_e += 1;

            if self.sel.s_disp_x < width && width_org < self.sel.e_disp_x {
                rtn.push(*c);
                if cur_x_s == USIZE_UNDEFINED {
                    cur_x_s = idx;
                }
            }
            if width >= self.sel.e_disp_x {
                break;
            }
        }
        return (rtn, cur_x_s, cur_x_e);
    }

    pub fn set_box_sel_vec(&mut self, ep_del: &Proc, ep: &mut Proc) {
        if self.box_insert.mode == BoxInsertMode::Insert || self.sel.mode == SelMode::BoxSelect {
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
    pub fn is_edit_del_keycmd(&mut self, keycmd: &KeyCmd) -> bool {
        match keycmd {
            KeyCmd::InsertStr(_) | KeyCmd::Tab | KeyCmd::InsertLine | KeyCmd::Cut | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => return true,
            _ => return false,
        }
    }
    pub fn exit_box_mode(&mut self) {
        self.sel.mode = SelMode::Normal;
        self.box_insert.mode = BoxInsertMode::Normal;
    }
    pub fn edit_proc_set_insert_str(&mut self, str: &String, ep: &mut Proc) {
        if self.box_insert.mode == BoxInsertMode::Insert {
            // paste
            if str.is_empty() {
                let is_box_insert_exit = self.get_clipboard(ep);
                // highlight data reset
                if is_box_insert_exit {
                    self.exit_box_mode();
                } else {
                    // Move cur.y to the beginning of a Box insert
                    self.cur.y = self.box_insert.vec.first().unwrap().0.sy;
                }
                if !self.box_insert.vec.is_empty() {
                    self.set_box_str_vec(&"".to_string(), ep);
                    self.box_insert.vec = ep.box_sel_vec.clone();
                }
                // Range selection with a width of 0
            } else {
                self.set_box_sel(ep);
                self.set_box_str_vec(str, ep);
                self.box_insert.vec = ep.box_sel_vec.clone();
            }
        } else {
            // paste
            if str.is_empty() {
                self.get_clipboard(ep);
            } else {
                ep.str = if str == &TAB_CHAR.to_string() { get_tab_str() } else { str.to_string() };
            }
        }
    }
    pub fn set_evtproc(&mut self, ep: &Proc, cur: &Cur) {
        self.cur.y = cur.y;
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
        self.draw_type = ep.draw_type;
    }

    pub fn check_evtproc(&mut self, keycmd: &KeyCmd) -> bool {
        if keycmd == &KeyCmd::DeleteNextChar {
            // End of last line
            if !self.sel.is_selected() && self.cur.y == self.buf.len_lines() - 1 && self.cur.x == self.buf.len_line_chars(self.cur.y) - 1 {
                self.draw_type = DrawType::Not;
                return true;
            }
        } else if keycmd == &KeyCmd::DeletePrevChar {
            // For the starting point
            if !self.sel.is_selected() && self.cur.y == 0 && self.cur.x == 0 {
                self.draw_type = DrawType::Not;
                return true;
            }
        }
        return false;
    }
    pub fn del_sel_range(&mut self, ep: &Proc) {
        let sel = self.sel.get_range();
        self.buf.remove_range(sel, ep);
        self.set_cur_target(sel.sy, sel.sx, false);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_box_sel(&mut self, ep: &mut Proc) {
        Log::debug_key("set_box_sel");

        let (slice_str, vec) = self.slice_box_sel();

        ep.str = slice_str.clone();
        ep.box_sel_vec = vec.clone();

        Log::debug("ep", &ep);
    }

    pub fn set_box_str_vec(&mut self, ins_str: &String, ep: &mut Proc) {
        for i in 0..=ep.box_sel_vec.len() - 1 {
            if !ins_str.is_empty() {
                ep.box_sel_vec[i].1 = ins_str.clone();
            };
            ep.box_sel_vec[i].0.sx = self.cur.x;
            ep.box_sel_vec[i].0.s_disp_x = self.cur.disp_x;
            ep.box_sel_vec[i].0.ex = ep.box_sel_vec[i].0.sx + ins_str.chars().count();
            ep.box_sel_vec[i].0.e_disp_x = ep.box_sel_vec[i].0.s_disp_x + get_str_width(ins_str);
        }
    }

    /*
    pub fn set_box_ins_str_vec(&mut self, ins_str: &String, ep: &EvtProc) {
        if self.box_sel.insert_vec.is_empty() {
            self.box_sel.insert_vec = ep.box_sel_vec.clone();
            for i in 0..=self.box_sel.insert_vec.len() - 1 {
                self.box_sel.insert_vec[i].1 = ins_str.clone();
                self.box_sel.insert_vec[i].0.ex = self.box_sel.insert_vec[i].0.sx + ins_str.chars().count();
                self.box_sel.insert_vec[i].0.e_disp_x = self.box_sel.insert_vec[i].0.s_disp_x + get_str_width(ins_str);
            }
        } else {
            for i in 0..=self.box_sel.insert_vec.len() - 1 {
                self.box_sel.insert_vec[i].1 = ins_str.clone();
                self.box_sel.insert_vec[i].0.sx = self.box_sel.insert_vec[i].0.ex;
                self.box_sel.insert_vec[i].0.s_disp_x = self.box_sel.insert_vec[i].0.e_disp_x;
                self.box_sel.insert_vec[i].0.ex = self.box_sel.insert_vec[i].0.sx + ins_str.chars().count();
                self.box_sel.insert_vec[i].0.e_disp_x = self.box_sel.insert_vec[i].0.s_disp_x + get_str_width(ins_str);
            }
        }
    }
     */
}
