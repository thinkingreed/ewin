use crate::{_cfg::keys::KeyCmd, log::*, model::*, sel_range::SelMode};

impl Editor {
    pub fn shift_move_com(&mut self) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match self.keycmd {
            KeyCmd::CursorUpSelect => self.cur_up(),
            KeyCmd::CursorDownSelect => self.cur_down(),
            KeyCmd::CursorLeftSelect => self.cur_left(),
            KeyCmd::CursorRightSelect => self.cur_right(),
            KeyCmd::CursorRowHomeSelect => self.cur_home(),
            KeyCmd::CursorRowEndSelect => self.cur_end(),
            _ => {}
        }

        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.draw_type = DrawType::get_type(self.sel.mode, self.cur_y_org, self.cur.y);
        self.sel.check_overlap();
    }

    pub fn box_select_mode(&mut self) {
        Log::debug_key("box_select_mode");
        self.sel.clear();
        self.sel.mode = match self.sel.mode {
            SelMode::Normal => SelMode::BoxSelect,
            SelMode::BoxSelect => SelMode::Normal,
        };
        if self.sel.mode == SelMode::BoxSelect {
            // Initial processing for Box Insert without moving the cursor
            self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);
            self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        }
    }

    pub fn record_key(&mut self) {
        match self.keycmd {
            // Ctrl
            KeyCmd::Copy | KeyCmd::Cut | KeyCmd::AllSelect | KeyCmd::InsertStr(_) | KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd => self.macros.key_macro_vec.push(KeyMacro { keys: self.keys, ..KeyMacro::default() }),
            // Shift
            KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => self.macros.key_macro_vec.push(KeyMacro { keys: self.keys, ..KeyMacro::default() }),
            KeyCmd::FindBack => self.macros.key_macro_vec.push(KeyMacro { keys: self.keys, search: Search { str: self.search.str.clone(), ..Search::default() } }),
            // Raw
            KeyCmd::InsertLine | KeyCmd::DeletePrevChar | KeyCmd::DeleteNextChar | KeyCmd::CursorPageUp | KeyCmd::CursorPageDown | KeyCmd::Tab | KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => self.macros.key_macro_vec.push(KeyMacro { keys: self.keys, ..KeyMacro::default() }),
            KeyCmd::FindNext => self.macros.key_macro_vec.push(KeyMacro { keys: self.keys, search: Search { str: self.search.str.clone(), ..Search::default() } }),
            _ => {}
        }
    }
}
