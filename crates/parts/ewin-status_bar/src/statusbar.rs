use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::def::*;
use ewin_editor::model::*;
use ewin_key::model::*;
use ewin_key::sel_range::*;
use ewin_state::term::*;
use ewin_view::view::*;
use std::ops::Range;

impl StatusBar {
    pub fn get_opt_vec(editor: &Editor) -> Vec<StatusBarCont> {
        Log::info_key("StatusBar::get_opt_vec");

        let mut opt_vec = vec![];
        let read_only = StatusBarCont::new(if State::get().curt_state().editor.is_read_only { Lang::get().unable_to_edit.to_string() } else { "".to_string() });
        opt_vec.push(read_only);

        let select_mode = StatusBarCont::new(match editor.win_mgr.curt_ref().sel.mode {
            SelMode::Normal => "".to_string(),
            SelMode::BoxSelect => Lang::get().box_select.to_string(),
        });
        opt_vec.push(select_mode);

        let box_sel_mode = StatusBarCont::new(match editor.box_insert.mode {
            BoxInsertMode::Normal => "".to_string(),
            BoxInsertMode::Insert => Lang::get().box_insert.to_string(),
        });
        opt_vec.push(box_sel_mode);

        let mouse_disable = StatusBarCont::new(match State::get().curt_state().editor.mouse {
            Mouse::Enable => "".to_string(),
            Mouse::Disable => Lang::get().mouse_disable.to_string(),
        });
        opt_vec.push(mouse_disable);

        let search_idx = if editor.search.idx == USIZE_UNDEFINED { 1 } else { editor.search.idx + 1 };
        let search_str = if editor.search.ranges.is_empty() { "".to_string() } else { format!("{}({}/{})", &Lang::get().search, search_idx, editor.search.ranges.len()) };
        let search = StatusBarCont::new(search_str);
        opt_vec.push(search);

        let key_record = StatusBarCont::new(if State::get().curt_mut_state().editor.key_macro.is_record { Lang::get().key_recording.to_string() } else { "".to_string() });
        Log::debug("key_record", &key_record);
        opt_vec.push(key_record);

        opt_vec.reverse();
        return opt_vec;
    }

    pub fn get_editor_conts(editor: &Editor) -> (StatusBarCont, Vec<StatusBarCont>) {
        let cur_cont = StatusBar::get_cur_cont(editor);
        let opt_vec = StatusBar::get_opt_vec(editor);
        return (cur_cont, opt_vec);
    }

    pub fn get_cur_cont(editor: &Editor) -> StatusBarCont {
        return StatusBarCont::new(StatusBar::get_cur_str(editor));
    }

    pub fn get_cur_str(editor: &Editor) -> String {
        let cur = editor.win_mgr.curt_ref().cur;
        let len_lines = editor.buf.len_rows();
        let len_line_chars = editor.buf.len_row_chars(editor.win_mgr.curt_ref().cur.y);

        let row_str = format!("{}({}/{})", &Lang::get().row, (cur.y + 1), len_lines);
        let len_line_chars = if len_line_chars == 0 {
            0
        } else if editor.win_mgr.curt_ref().cur.y == len_lines - 1 {
            len_line_chars
        } else {
            // -1 is new line code
            len_line_chars - 1
        };
        let col_str = format!("{}({}/{})", &Lang::get().col, cur.x + 1, len_line_chars);
        let cur_posi = format!("{}{}", row_str, col_str,);
        return cur_posi;
    }

    pub fn new() -> Self {
        StatusBar { ..StatusBar::default() }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBar {
    pub view: View,
    pub cur_area: (usize, usize),
    pub enc_nl_area: (usize, usize),
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar { view: View::default(), cur_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), enc_nl_area: (USIZE_UNDEFINED, USIZE_UNDEFINED) }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct StatusBarCont {
    pub area: Range<usize>,
    pub disp_str: String,
}

impl StatusBarCont {
    pub fn new(disp_str: String) -> Self {
        StatusBarCont { disp_str, ..StatusBarCont::default() }
    }
}
