use crate::{
    _cfg::{cfg::TabType, keys::KeyCmd},
    def::*,
    global::CFG,
    log::Log,
    model::*,
    sel_range::{SelMode, SelRange},
    util::*,
};

impl Editor {
    pub fn slice_box_sel(&mut self) -> (String, Vec<(SelRange, String)>) {
        let sel = self.sel.get_range();

        Log::debug("sel", &sel);

        let mut string = String::new();
        let mut vec: Vec<(SelRange, String)> = vec![];

        for y in sel.sy..=sel.ey {
            let mut row_sel = sel.clone();
            let mut slice_str = String::new();
            if row_sel.is_selected() {
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
        Log::debug("box_slice_str", &string);
        Log::debug("box_sel_vec", &vec);

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

    pub fn edit_proc(&mut self, keycmd: KeyCmd) {
        if self.check_evtproc(&keycmd) {
            return;
        }
        let is_selected_org = self.sel.is_selected();
        let mut ep_del = EvtProc::default();
        // selected range delete

        // if self.sel.is_selected() {
        if keycmd != KeyCmd::Undo && keycmd != KeyCmd::Redo && self.sel.is_selected() {
            ep_del = EvtProc { keycmd: if keycmd == KeyCmd::DeleteNextChar { KeyCmd::DeleteNextChar } else { KeyCmd::DeletePrevChar }, ..EvtProc::default() };
            ep_del.cur_s = Cur { y: self.sel.sy, x: self.sel.sx, disp_x: self.sel.s_disp_x };
            ep_del.cur_e = self.cur;
            match self.sel.mode {
                SelMode::Normal => ep_del.str = self.buf.slice(self.sel.get_range()),
                SelMode::BoxSelect => self.set_box_sel(&mut ep_del),
            }
            ep_del.sel = self.sel;
            self.del_sel_range(&ep_del);
            self.sel.clear();
            ep_del.d_range = self.d_range;
            self.history.regist_edit(self.keycmd.clone(), &ep_del);
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        if !(is_selected_org && (keycmd == KeyCmd::DeleteNextChar || keycmd == KeyCmd::DeletePrevChar)) {
            let mut ep = EvtProc { keycmd: keycmd.clone(), ..EvtProc::default() };

            ep.cur_s = self.cur;
            if ep_del.box_sel_vec.is_empty() {
                if self.box_sel.mode == BoxInsertMode::Insert {
                    ep.box_sel_vec = self.box_sel.clipboard_box_sel_vec.clone();
                    ep.str = self.box_sel.clipboard_str.clone();
                }
            } else {
                ep.box_sel_vec = ep_del.box_sel_vec.clone();
            }

            match &keycmd {
                KeyCmd::InsertStr(str) => {
                    // self.set_box_sel_mode(&ep_del);

                    if self.box_sel.mode == BoxInsertMode::Insert {
                        // Range selection with a width of 0
                        if !is_selected_org {
                            self.set_box_sel(&mut ep);
                        }
                        self.set_box_ins_str_vec(str, &ep);
                        ep.box_sel_vec = self.box_sel.insert_vec.clone();
                    } else {
                        if str == &TAB_CHAR.to_string() {
                            ep.str = self.get_tab_str();
                        } else {
                            // str empty is Paste
                            if !str.is_empty() {
                                ep.str = str.to_string();
                            }
                        }
                    }
                }
                KeyCmd::InsertBox(box_sel_vec) => {
                    ep.box_sel_vec = box_sel_vec.clone();
                }
                _ => {}
            }
            match &keycmd {
                KeyCmd::DeleteNextChar => self.delete(&mut ep),
                KeyCmd::DeletePrevChar => self.backspace(&mut ep),
                KeyCmd::InsertLine => self.enter(),
                KeyCmd::CutSelect => self.cut(ep_del),
                KeyCmd::InsertStr(_) | KeyCmd::InsertBox(_) => self.insert_str(&mut ep),
                KeyCmd::DelBox(box_sel_vec) => self.undo_del_box(&box_sel_vec),
                // In case of replace, only registration of Evt process
                KeyCmd::ReplaceExec(search_str, replace_str) => self.replace(&mut ep, search_str, replace_str),
                _ => {}
            }
            if keycmd != KeyCmd::CutSelect {
                ep.cur_e = self.cur;
                ep.d_range = self.d_range;
                self.history.regist_edit(self.keycmd.clone(), &ep);
            }
        }
        self.cancel_mode();

        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_evtproc(&mut self, ep: &EvtProc, cur: &Cur) {
        self.cur.y = cur.y;
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
        self.d_range = ep.d_range;
    }

    pub fn check_evtproc(&mut self, keycmd: &KeyCmd) -> bool {
        if keycmd == &KeyCmd::DeleteNextChar {
            // End of last line
            if !self.sel.is_selected() {
                if self.cur.y == self.buf.len_lines() - 1 && self.cur.x == self.buf.len_line_chars(self.cur.y) - 1 {
                    self.d_range.draw_type = DrawType::Not;
                    return true;
                }
            }
        } else if keycmd == &KeyCmd::DeletePrevChar {
            // For the starting point
            if !self.sel.is_selected() {
                if self.cur.y == 0 && self.cur.x == 0 {
                    self.d_range.draw_type = DrawType::Not;
                    return true;
                }
            }
        }
        return false;
    }
    pub fn del_sel_range(&mut self, ep: &EvtProc) {
        let sel = self.sel.get_range();
        self.buf.remove_range(sel, ep);
        self.set_cur_target(sel.sy, sel.sx, false);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn get_tab_str(&mut self) -> String {
        let cfg = &CFG.get().unwrap().try_lock().unwrap();
        let tab_type: &TabType = &cfg.general.editor.tab.tab_type;
        let tab_width: usize = cfg.general.editor.tab.width;
        return match tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => " ".repeat(tab_width),
        };
    }

    pub fn set_box_sel(&mut self, ep: &mut EvtProc) {
        let (slice_str, vec) = self.slice_box_sel();
        ep.str = slice_str.clone();
        ep.box_sel_vec = vec.clone();

        //   if keycmd == KeyCmd::CutSelect {
        self.box_sel.clipboard_str = slice_str;
        self.box_sel.clipboard_box_sel_vec = vec;
    }

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
}
