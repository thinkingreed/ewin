use crate::{
    ewin_com::{model::*, util::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::def::*;
use ewin_editor::model::*;
use std::{io::Write, ops::Range};

impl StatusBar {
    pub fn draw(str_vec: &mut Vec<String>, tab: &mut Tab, h_file: &HeaderFile) {
        Log::info_key("StatusBar.draw");
        let cols = get_term_size().0;

        let mut normal_vec = vec![];
        let enc_nl = StatusBarCont::new(format!("{}({})", h_file.enc, h_file.nl));
        normal_vec.push(enc_nl.clone());
        let cur = StatusBarCont::new(StatusBar::get_cur_str(&tab.editor));
        normal_vec.push(cur.clone());
        normal_vec.reverse();

        let mut normal_str = String::new();
        for normal in &normal_vec {
            normal_str.push_str(&normal.disp_str);
        }
        let mut opt_str = String::new();
        let mut opt_vec = StatusBar::get_opt_vec(&tab.editor);
        opt_vec.reverse();
        for opt in opt_vec {
            if !opt.disp_str.is_empty() {
                opt_str.push_str(&format!("[{}]", opt.disp_str));
            }
        }
        let normal_str_w = get_str_width(&normal_str);
        let opt_str_w = get_str_width(&opt_str);
        let other_w = cols - normal_str_w - opt_str_w;
        let normal_w_s = other_w + opt_str_w;

        tab.sbar.cur_area = (normal_w_s, normal_w_s + get_str_width(&cur.disp_str) - 1);
        tab.sbar.enc_nl_area = (tab.sbar.cur_area.1 + 1, tab.sbar.cur_area.1 + 1 + get_str_width(&enc_nl.disp_str));

        let mut msg_str = String::new();
        msg_str.push_str(&format!("{}{}", Colors::get_default_fg_bg(), " ".repeat(other_w)));
        msg_str.push_str(&format!("{}{}", Colors::get_sbar_inversion_fg_bg(), opt_str));
        msg_str.push_str(&format!("{}{}", Colors::get_sbar_fg_bg(), normal_str));
        msg_str = format!("{}{}{}", MoveTo(0, tab.sbar.row_posi as u16), Clear(ClearType::CurrentLine), msg_str);

        str_vec.push(msg_str);
        str_vec.push(Colors::get_default_fg_bg());
    }

    pub fn draw_only<T: Write>(out: &mut T, tab: &mut Tab, h_file: &HeaderFile) {
        Log::debug_key("StatusBar.draw_only");
        let mut v: Vec<String> = vec![];
        StatusBar::draw(&mut v, tab, h_file);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn get_opt_vec(editor: &Editor) -> Vec<StatusBarCont> {
        Log::info_key("StatusBar::get_opt_vec");

        let mut opt_vec = vec![];
        let read_only = StatusBarCont::new(if editor.state.is_read_only { Lang::get().unable_to_edit.to_string() } else { "".to_string() });
        opt_vec.push(read_only);

        let select_mode = StatusBarCont::new(match editor.sel.mode {
            SelMode::Normal => "".to_string(),
            SelMode::BoxSelect => Lang::get().box_select.to_string(),
        });
        opt_vec.push(select_mode);

        let box_sel_mode = StatusBarCont::new(match editor.box_insert.mode {
            BoxInsertMode::Normal => "".to_string(),
            BoxInsertMode::Insert => Lang::get().box_insert.to_string(),
        });
        opt_vec.push(box_sel_mode);

        let mouse_disable = StatusBarCont::new(match editor.state.mouse {
            Mouse::Enable => "".to_string(),
            Mouse::Disable => Lang::get().mouse_disable.to_string(),
        });
        opt_vec.push(mouse_disable);

        let search_idx = if editor.search.idx == USIZE_UNDEFINED { 1 } else { editor.search.idx + 1 };
        let search_str = if editor.search.ranges.is_empty() { "".to_string() } else { format!("{}({}/{})", &Lang::get().search, search_idx, editor.search.ranges.len()) };
        let search = StatusBarCont::new(search_str);
        opt_vec.push(search);

        let key_record = StatusBarCont::new(if editor.state.key_macro.is_record { Lang::get().key_recording.to_string() } else { "".to_string() });
        Log::debug("key_record", &key_record);
        Log::debug("editor.state.key_macro.is_record", &editor.state.key_macro.is_record);
        opt_vec.push(key_record);

        return opt_vec;
    }

    pub fn get_cur_str(editor: &Editor) -> String {
        let cur = editor.cur;
        let len_lines = editor.buf.len_rows();
        let len_line_chars = editor.buf.len_row_chars(editor.cur.y);

        let row_str = format!("{}({}/{})", &Lang::get().row, (cur.y + 1), len_lines);
        let len_line_chars = if len_line_chars == 0 {
            0
        } else if editor.cur.y == len_lines - 1 {
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
    // Position on the terminal
    pub row_num: usize,
    // 0 index
    pub row_posi: usize,
    pub col_num: usize,
    pub cur_area: (usize, usize),
    pub enc_nl_area: (usize, usize),
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar { row_num: STATUSBAR_ROW_NUM, row_posi: 0, col_num: 0, cur_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), enc_nl_area: (USIZE_UNDEFINED, USIZE_UNDEFINED) }
    }
}

#[derive(Debug, Default, Clone)]
pub struct StatusBarCont {
    pub area: Range<usize>,
    pub disp_str: String,
}

impl StatusBarCont {
    pub fn new(disp_str: String) -> Self {
        StatusBarCont { disp_str, ..StatusBarCont::default() }
    }
}
